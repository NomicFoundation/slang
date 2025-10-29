use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::binder::{Binder, Definition, FileScope, ParametersScope, Scope, ScopeId};
use crate::backend::ir::ir2_flat_contracts::visitor::Visitor;
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::compilation::{CompilationUnit, File};
use crate::cst::{NodeId, TerminalNode};
use crate::utils::versions::VERSION_0_5_0;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

/// In this pass all definitions are collected with their naming identifiers.
/// Also lexical (and other kinds of) scopes are identified and linked together,
/// and definitions are registered into them for later lookup. The pass
/// instantiates a `Binder` object which will store all this information as well
/// as references and typing information for the nodes, to be resolved in later
/// passes.
pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(compilation_unit.language_version());
    for (file_id, source_unit) in &files {
        let file = compilation_unit.file(file_id).unwrap();
        pass.visit_file(file, source_unit);
    }
    let binder = pass.binder;

    Output {
        compilation_unit,
        files,
        binder,
    }
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass {
    language_version: Version,
    current_file: Option<Rc<File>>, // needed to resolve imports on the file
    scope_stack: Vec<ScopeFrame>,
    binder: Binder,
}

impl Pass {
    fn new(language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            current_file: None,
            scope_stack: Vec::new(),
            binder: Binder::new(),
        }
    }

    fn visit_file(&mut self, file: Rc<File>, source_unit: &input_ir::SourceUnit) {
        assert!(self.current_file.is_none());

        self.current_file = Some(file);
        input_ir::visitor::accept_source_unit(source_unit, self);
        self.current_file = None;

        assert!(self.scope_stack.is_empty());
    }

    fn enter_scope(&mut self, scope: Scope) -> ScopeId {
        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
        scope_id
    }

    fn replace_scope(&mut self, scope: Scope) -> ScopeId {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("scope stack cannot be empty");
        };

        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(ScopeFrame {
            structural_scope_id,
            lexical_scope_id: scope_id,
        });
        scope_id
    }

    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            structural_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_scope_id(&self) -> ScopeId {
        let Some(ScopeFrame {
            lexical_scope_id, ..
        }) = self.scope_stack.last()
        else {
            unreachable!("empty scope stack");
        };
        *lexical_scope_id
    }

    fn current_scope(&mut self) -> &mut Scope {
        let scope_id = self.current_scope_id();
        self.binder.get_scope_mut(scope_id)
    }

    fn current_file_scope(&mut self) -> &mut FileScope {
        let Scope::File(file_scope) = self.current_scope() else {
            unreachable!("current scope is not a file scope");
        };
        file_scope
    }

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.binder
            .insert_definition_in_scope(definition, self.current_scope_id());
    }

    fn resolve_import_path(&self, import_path: &Rc<TerminalNode>) -> Option<String> {
        let import_path_node_id = import_path.id();
        let current_file = self
            .current_file
            .as_ref()
            .expect("import directive must be visited in the context of a current file");
        current_file
            .resolved_import_by_node_id(import_path_node_id)
            .map(|file_id| file_id.to_string())

        // TODO(validation): emit an error/warning if the file cannot be resolved
    }

    // Collects *all* the sequential parameters making and registering
    // definitions for named ones and return the constructed parameters scope ID
    // to link with the enclosing function definition
    fn collect_parameters(&mut self, parameters: &input_ir::Parameters) -> ScopeId {
        let mut scope = ParametersScope::new();
        for parameter in parameters {
            scope.add_parameter(parameter.name.as_ref(), parameter.id());
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.id(), name);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        self.binder.insert_scope(Scope::Parameters(scope))
    }

    // Collect parameters in error definition
    fn collect_error_parameters(&mut self, parameters: &input_ir::ErrorParameters) -> ScopeId {
        let mut scope = ParametersScope::new();
        for parameter in parameters {
            scope.add_parameter(parameter.name.as_ref(), parameter.id());
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.id(), name);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        self.binder.insert_scope(Scope::Parameters(scope))
    }

    // Collect parameters in event definition
    fn collect_event_parameters(&mut self, parameters: &input_ir::EventParameters) -> ScopeId {
        let mut scope = ParametersScope::new();
        for parameter in parameters {
            scope.add_parameter(parameter.name.as_ref(), parameter.id());
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.id(), name);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        self.binder.insert_scope(Scope::Parameters(scope))
    }

    // This is used to collect only named parameters and insert their
    // definitions into an existing scope. Used mostly for return parameters,
    // where position and types are not used for binding.
    fn collect_named_parameters_into_scope(
        &mut self,
        parameters: &input_ir::Parameters,
        scope_id: ScopeId,
    ) {
        for parameter in parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.id(), name);
                self.binder.insert_definition_in_scope(definition, scope_id);
            }
        }
    }

    fn register_constructor_parameters(&mut self, constructor_parameters_scope_id: ScopeId) {
        let current_scope_node_id = self.current_scope().node_id();
        let Definition::Contract(contract_definition) =
            self.binder.get_definition_mut(current_scope_node_id)
        else {
            unreachable!("the current scope is not a contract");
        };
        // TODO(validation): there should be a single constructor, so the
        // current value should be None
        contract_definition.constructor_parameters_scope_id = Some(constructor_parameters_scope_id);
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        let Some(current_file) = &self.current_file else {
            unreachable!("visiting SourceUnit without a current file being set");
        };
        let scope = Scope::new_file(node.id(), current_file.id());
        self.enter_scope(scope);

        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        if let Some(alias) = &node.alias {
            let definition = Definition::new_import(node.id(), alias, imported_file_id);
            self.insert_definition_in_current_scope(definition);
        } else if let Some(imported_file_id) = imported_file_id {
            self.current_file_scope()
                .add_imported_file(imported_file_id);
        }

        false
    }

    fn enter_named_import(&mut self, node: &input_ir::NamedImport) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        let definition = Definition::new_import(node.id(), &node.alias, imported_file_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_import_deconstruction(&mut self, node: &input_ir::ImportDeconstruction) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        for symbol in &node.symbols {
            let identifier = if let Some(alias) = &symbol.alias {
                alias
            } else {
                &symbol.name
            };
            let definition = Definition::new_imported_symbol(
                symbol.id(),
                identifier,
                symbol.name.unparse(),
                imported_file_id.clone(),
            );
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        match node.kind {
            input_ir::FunctionKind::Regular
            | input_ir::FunctionKind::Constructor
            | input_ir::FunctionKind::Fallback
            | input_ir::FunctionKind::Receive
            | input_ir::FunctionKind::Unnamed => {
                let parameters_scope_id = self.collect_parameters(&node.parameters);

                if let Some(name) = &node.name {
                    let visibility = (&node.visibility).into();
                    let definition =
                        Definition::new_function(node.id(), name, parameters_scope_id, visibility);

                    let current_scope_node_id = self.current_scope().node_id();
                    let enclosing_definition =
                        self.binder.find_definition_by_id(current_scope_node_id);
                    let enclosing_contract_name =
                        if let Some(Definition::Contract(contract_definition)) =
                            enclosing_definition
                        {
                            Some(contract_definition.identifier.unparse())
                        } else {
                            None
                        };

                    if enclosing_contract_name
                        .is_some_and(|contract_name| contract_name == name.unparse())
                    {
                        // TODO(validation): for Solidity >= 0.5.0 there cannot be a
                        // function with the same name as the enclosing contract. In any
                        // case, if the names match we don't register the definition in
                        // the current scope.
                        self.binder.insert_definition_no_scope(definition);

                        // For Solidity < 0.5.0, a function with the same name as the
                        // enclosing contract is a constructor, and we need to register the
                        // constructor parameters scope.
                        if self.language_version < VERSION_0_5_0 {
                            self.register_constructor_parameters(parameters_scope_id);
                        }
                    } else {
                        self.insert_definition_in_current_scope(definition);
                    }
                } else if matches!(node.kind, input_ir::FunctionKind::Constructor) {
                    // Register the constructor to resolve named parameters when
                    // constructing this contract
                    self.register_constructor_parameters(parameters_scope_id);
                }

                let function_scope =
                    Scope::new_function(node.id(), self.current_scope_id(), parameters_scope_id);
                let function_scope_id = self.enter_scope(function_scope);

                if let Some(returns) = &node.returns {
                    self.collect_named_parameters_into_scope(returns, function_scope_id);
                }
            }

            input_ir::FunctionKind::Modifier => {
                let Some(name) = &node.name else {
                    unreachable!("expected a name for the modifier");
                };

                let definition = Definition::new_modifier(node.id(), name);
                self.insert_definition_in_current_scope(definition);

                let modifier_scope = Scope::new_modifier(node.id(), self.current_scope_id());
                let modifier_scope_id = self.enter_scope(modifier_scope);
                self.collect_named_parameters_into_scope(&node.parameters, modifier_scope_id);
            }
        }
        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let enum_scope = Scope::new_enum(node.id());
        let enum_scope_id = self.binder.insert_scope(enum_scope);
        for member in &node.members {
            let definition = Definition::new_enum_member(member.id(), member);
            self.binder
                .insert_definition_in_scope(definition, enum_scope_id);
        }

        false
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let struct_scope = Scope::new_struct(node.id());
        let struct_scope_id = self.binder.insert_scope(struct_scope);
        for member in &node.members {
            let definition = Definition::new_struct_member(member.id(), &member.name);
            self.binder
                .insert_definition_in_scope(definition, struct_scope_id);
        }

        true
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        let parameters_scope_id = self.collect_error_parameters(&node.members);
        let definition = Definition::new_error(node.id(), &node.name, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        let parameters_scope_id = self.collect_event_parameters(&node.parameters);
        let definition = Definition::new_event(node.id(), &node.name, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_state_variable_definition(
        &mut self,
        node: &input_ir::StateVariableDefinition,
    ) -> bool {
        let is_constant = matches!(node.mutability, input_ir::StateVariableMutability::Constant);
        let is_public = matches!(node.visibility, input_ir::StateVariableVisibility::Public);
        // Public state variables define a getter, so we don't register them as
        // constants here
        let definition = if is_constant && !is_public {
            Definition::new_constant(node.id(), &node.name)
        } else {
            let visibility = (&node.visibility).into();
            Definition::new_state_variable(node.id(), &node.name, visibility)
        };
        self.insert_definition_in_current_scope(definition);

        // there may be more definitions in the type of the state variable (eg.
        // key/value names in mappings)
        true
    }

    fn enter_constant_definition(&mut self, node: &input_ir::ConstantDefinition) -> bool {
        let definition = Definition::new_constant(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let definition = Definition::new_user_defined_value_type(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn leave_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) {
        if self.language_version >= VERSION_0_5_0 {
            // In Solidity >= 0.5.0 this definition should only be available for
            // statements after this one. So we open a new scope that replaces
            // but is linked to the current one
            let scope = Scope::new_block(node.id(), self.current_scope_id());
            self.replace_scope(scope);
        }

        let definition = Definition::new_variable(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);
    }

    fn leave_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) {
        if self.language_version >= VERSION_0_5_0 {
            // In Solidity >= 0.5.0 the definitions should only be available for
            // statements after this one. So we open a new scope that replaces
            // but is linked to the current one
            let scope = Scope::new_block(node.id(), self.current_scope_id());
            self.replace_scope(scope);
        }

        let is_untyped_declaration = node.var_keyword;
        for element in &node.elements {
            let Some(tuple_member) = &element.member else {
                continue;
            };
            let definition = match tuple_member {
                input_ir::TupleMember::TypedTupleMember(typed_tuple_member) => {
                    Definition::new_variable(typed_tuple_member.id(), &typed_tuple_member.name)
                }
                input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
                    if !is_untyped_declaration {
                        continue;
                    }
                    Definition::new_variable(untyped_tuple_member.id(), &untyped_tuple_member.name)
                }
            };
            self.insert_definition_in_current_scope(definition);
        }
    }

    fn enter_block(&mut self, node: &input_ir::Block) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            let scope = Scope::new_block(node.id(), self.current_scope_id());
            self.enter_scope(scope);
        }
        true
    }

    fn leave_block(&mut self, node: &input_ir::Block) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.id());
        }
    }

    fn enter_for_statement(&mut self, node: &input_ir::ForStatement) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            // For Solidity >= 0.5.0 open a new block here to hold declarations
            // in the initialization clause.
            let scope = Scope::new_block(node.id(), self.current_scope_id());
            self.enter_scope(scope);
        }
        true
    }

    fn leave_for_statement(&mut self, node: &input_ir::ForStatement) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.id());
        }
    }

    fn leave_try_statement(&mut self, node: &input_ir::TryStatement) {
        if self.language_version < VERSION_0_5_0 {
            return;
        }
        if let Some(returns) = &node.returns {
            // For Solidity >= 0.5.0, collect the parameters in the returns
            // declaration of the try statement and make them available in the
            // body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(returns, body_scope_id);
        }
    }

    fn leave_catch_clause(&mut self, node: &input_ir::CatchClause) {
        if let Some(error) = &node.error {
            // For Solidity >= 0.5.0, collect the parameters in the catch
            // declaration and make them available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(&error.parameters, body_scope_id);
        }
    }

    fn enter_mapping_type(&mut self, node: &input_ir::MappingType) -> bool {
        if let Some(name) = &node.key_type.name {
            let definition = Definition::new_type_parameter(node.key_type.id(), name);
            self.binder.insert_definition_no_scope(definition);
        }
        if let Some(name) = &node.value_type.name {
            let definition = Definition::new_type_parameter(node.value_type.id(), name);
            self.binder.insert_definition_no_scope(definition);
        }

        true
    }

    fn enter_function_type(&mut self, node: &input_ir::FunctionType) -> bool {
        for parameter in &node.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_type_parameter(parameter.id(), name);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                if let Some(name) = &parameter.name {
                    let definition = Definition::new_type_parameter(parameter.id(), name);
                    self.binder.insert_definition_no_scope(definition);
                }
            }
        }

        false
    }

    fn enter_yul_block(&mut self, node: &input_ir::YulBlock) -> bool {
        let scope = Scope::new_yul_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_yul_block(&mut self, node: &input_ir::YulBlock) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        let definition = Definition::new_yul_function(node.id(), &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_yul_function(node.id(), self.current_scope_id());
        let scope_id = self.enter_scope(scope);

        for parameter in &node.parameters {
            let definition = Definition::new_yul_parameter(parameter.id(), parameter);
            self.binder.insert_definition_in_scope(definition, scope_id);
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                let definition = Definition::new_yul_variable(parameter.id(), parameter);
                self.binder.insert_definition_in_scope(definition, scope_id);
            }
        }

        true
    }

    fn leave_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_label(&mut self, node: &input_ir::YulLabel) -> bool {
        let definition = Definition::new_yul_label(node.id(), &node.label);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &input_ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            let definition = Definition::new_yul_variable(variable.id(), variable);
            self.insert_definition_in_current_scope(definition);
        }
        // TODO: we maybe want to enter a new scope here, but that should be
        // only relevant for validation (ie. to avoid referencing a variable
        // before declaring it). If we do that, we need to take special care of
        // where we insert label and function definitions, since those are
        // hoisted in the block.

        false
    }

    fn enter_yul_for_statement(&mut self, node: &input_ir::YulForStatement) -> bool {
        // Visit the initialization block first
        input_ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block such that iterator and body blocks link to it
        self.enter_scope_for_node_id(node.initialization.id());
        input_ir::visitor::accept_yul_expression(&node.condition, self);
        input_ir::visitor::accept_yul_block(&node.iterator, self);
        input_ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.id());

        // We already visited our children
        false
    }
}
