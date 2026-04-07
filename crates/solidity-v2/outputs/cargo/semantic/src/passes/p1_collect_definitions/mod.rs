use slang_solidity_v2_ir::ir::visitor::Visitor;
use slang_solidity_v2_ir::ir::{self, NodeId};

use crate::binder::{Binder, Definition, FileScope, ParametersScope, Scope, ScopeId};
use crate::context::InputFile;

/// In this pass all definitions are collected with their naming identifiers.
/// Also lexical (and other kinds of) scopes are identified and linked together,
/// and definitions are registered into them for later lookup. The pass
/// instantiates a `Binder` object which will store all this information as well
/// as references and typing information for the nodes, to be resolved in later
/// passes.
pub fn run(files: &[impl InputFile], binder: &mut Binder) {
    let mut pass = Pass::new(binder);
    for file in files {
        pass.visit_file(file);
    }
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass<'a, F: InputFile> {
    current_file: Option<&'a F>,
    scope_stack: Vec<ScopeFrame>,
    binder: &'a mut Binder,
}

impl<'a, F: InputFile> Pass<'a, F> {
    fn new(binder: &'a mut Binder) -> Self {
        Self {
            current_file: None,
            scope_stack: Vec::new(),
            binder,
        }
    }

    fn visit_file(&mut self, file: &'a F) {
        assert!(self.scope_stack.is_empty());
        assert!(self.current_file.is_none());

        self.current_file = Some(file);
        ir::visitor::accept_source_unit(file.ir_root(), self);
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

    fn resolve_import_path(&self, import_node_id: NodeId) -> Option<String> {
        let current_file = self
            .current_file
            .expect("import directive must be visited in the context of a current file");
        current_file
            .resolved_import_by_node_id(import_node_id)
            .cloned()

        // TODO(validation): emit an error/warning if the file cannot be resolved
    }

    // Collects *all* the sequential parameters making and registering
    // definitions for named ones and return the constructed parameters scope ID
    // to link with the enclosing function definition
    fn collect_parameters(&mut self, parameters: &ir::Parameters) -> ScopeId {
        let mut scope = ParametersScope::new();
        for parameter in parameters {
            scope.add_parameter(parameter.name.as_ref().map(|id| &id.text), parameter.id());
            if parameter.name.is_some() {
                let definition = Definition::new_parameter(parameter);
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
        parameters: &ir::Parameters,
        scope_id: ScopeId,
    ) {
        for parameter in parameters {
            if parameter.name.is_some() {
                let definition = Definition::new_parameter(parameter);
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

impl<F: InputFile> Visitor for Pass<'_, F> {
    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        let current_file = self
            .current_file
            .expect("visiting SourceUnit without a current file being set");
        let scope = Scope::new_file(node.id(), current_file.id());
        self.enter_scope(scope);

        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_path_import(&mut self, node: &ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        if node.alias.is_some() {
            let definition = Definition::new_import(node, imported_file_id);
            self.insert_definition_in_current_scope(definition);
        } else if let Some(imported_file_id) = imported_file_id {
            self.current_file_scope()
                .add_imported_file(imported_file_id);
        }

        false
    }

    fn enter_import_deconstruction(&mut self, node: &ir::ImportDeconstruction) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        for symbol in &node.symbols {
            let definition = Definition::new_imported_symbol(
                symbol,
                symbol.name.unparse().to_owned(),
                imported_file_id.clone(),
            );
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_function_definition(&mut self, node: &ir::FunctionDefinition) -> bool {
        match node.kind {
            ir::FunctionKind::Regular
            | ir::FunctionKind::Constructor
            | ir::FunctionKind::Fallback
            | ir::FunctionKind::Receive => {
                let parameters_scope_id = self.collect_parameters(&node.parameters);

                if let Some(name) = &node.name {
                    let definition = Definition::new_function(
                        node,
                        parameters_scope_id,
                        node.visibility.clone(),
                    );

                    let current_scope_node_id = self.current_scope().node_id();
                    let enclosing_definition =
                        self.binder.find_definition_by_id(current_scope_node_id);
                    let enclosing_contract_name =
                        if let Some(enclosing_definition) = enclosing_definition {
                            if matches!(enclosing_definition, Definition::Contract(_)) {
                                Some(enclosing_definition.identifier().unparse())
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                    if enclosing_contract_name
                        .is_some_and(|contract_name| contract_name == name.unparse())
                    {
                        // TODO(validation): there cannot be a function with the
                        // same name as the enclosing contract (since Solidity
                        // 0.5.0). Regardless, we skip registering the function
                        // symbol in the current scope to avoid interference
                        // with resolution.
                        self.binder.insert_definition_no_scope(definition);
                    } else {
                        self.insert_definition_in_current_scope(definition);
                    }
                } else if matches!(node.kind, ir::FunctionKind::Constructor) {
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

            ir::FunctionKind::Modifier => {
                let definition = Definition::new_modifier(node);
                self.insert_definition_in_current_scope(definition);

                let modifier_scope = Scope::new_modifier(node.id(), self.current_scope_id());
                let modifier_scope_id = self.enter_scope(modifier_scope);
                self.collect_named_parameters_into_scope(&node.parameters, modifier_scope_id);
            }
        }
        true
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_enum_definition(&mut self, node: &ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node);
        self.insert_definition_in_current_scope(definition);

        let enum_scope = Scope::new_enum(node.id());
        let enum_scope_id = self.binder.insert_scope(enum_scope);
        for member in &node.members {
            let definition = Definition::new_enum_member(member);
            self.binder
                .insert_definition_in_scope(definition, enum_scope_id);
        }

        false
    }

    fn enter_struct_definition(&mut self, node: &ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node);
        self.insert_definition_in_current_scope(definition);

        let struct_scope = Scope::new_struct(node.id());
        let struct_scope_id = self.binder.insert_scope(struct_scope);
        for member in &node.members {
            let definition = Definition::new_struct_member(member);
            self.binder
                .insert_definition_in_scope(definition, struct_scope_id);
        }

        true
    }

    fn enter_error_definition(&mut self, node: &ir::ErrorDefinition) -> bool {
        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_error(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &ir::EventDefinition) -> bool {
        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_event(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_state_variable_definition(&mut self, node: &ir::StateVariableDefinition) -> bool {
        let definition = Definition::new_state_variable(node, node.visibility.clone());
        self.insert_definition_in_current_scope(definition);

        // there may be more definitions in the type of the state variable (eg.
        // key/value names in mappings)
        true
    }

    fn enter_constant_definition(&mut self, node: &ir::ConstantDefinition) -> bool {
        let definition = Definition::new_constant(node, self.current_scope_id());
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let definition = Definition::new_user_defined_value_type(node);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        // Open a new scope that replaces but is linked to the current one so
        // definitions declared here are only available for statements after
        // this one.
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.replace_scope(scope);

        match &node.target {
            ir::VariableDeclarationTarget::SingleTypedDeclaration(single) => {
                let definition = Definition::new_variable(&single.declaration);
                self.insert_definition_in_current_scope(definition);
            }
            ir::VariableDeclarationTarget::MultiTypedDeclaration(multi) => {
                for element in &multi.elements {
                    if let Some(member) = &element.member {
                        let definition = Definition::new_variable(member);
                        self.insert_definition_in_current_scope(definition);
                    }
                }
            }
        }
    }

    fn enter_block(&mut self, node: &ir::Block) -> bool {
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        true
    }

    fn leave_block(&mut self, node: &ir::Block) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_for_statement(&mut self, node: &ir::ForStatement) -> bool {
        // Open a new block here to hold declarations in the initialization
        // clause.
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        true
    }

    fn leave_for_statement(&mut self, node: &ir::ForStatement) {
        self.leave_scope_for_node_id(node.id());
    }

    fn leave_try_statement(&mut self, node: &ir::TryStatement) {
        if let Some(returns) = &node.returns {
            // Collect the parameters in the returns declaration of the try
            // statement and make them available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(returns, body_scope_id);
        }
    }

    fn leave_catch_clause(&mut self, node: &ir::CatchClause) {
        if let Some(error) = &node.error {
            // Collect the parameters in the catch declaration and make them
            // available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(&error.parameters, body_scope_id);
        }
    }

    fn enter_mapping_type(&mut self, node: &ir::MappingType) -> bool {
        if node.key_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.key_type);
            self.binder.insert_definition_no_scope(definition);
        }
        if node.value_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.value_type);
            self.binder.insert_definition_no_scope(definition);
        }

        true
    }

    fn enter_function_type(&mut self, node: &ir::FunctionType) -> bool {
        for parameter in &node.parameters {
            if parameter.name.is_some() {
                let definition = Definition::new_type_parameter(parameter);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                if parameter.name.is_some() {
                    let definition = Definition::new_type_parameter(parameter);
                    self.binder.insert_definition_no_scope(definition);
                }
            }
        }

        false
    }

    fn enter_yul_block(&mut self, node: &ir::YulBlock) -> bool {
        let scope = Scope::new_yul_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_yul_block(&mut self, node: &ir::YulBlock) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) -> bool {
        let definition = Definition::new_yul_function(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_yul_function(node.id(), self.current_scope_id());
        let scope_id = self.enter_scope(scope);

        for parameter in &node.parameters {
            let definition = Definition::new_yul_parameter(parameter);
            self.binder.insert_definition_in_scope(definition, scope_id);
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                let definition = Definition::new_yul_variable(parameter);
                self.binder.insert_definition_in_scope(definition, scope_id);
            }
        }

        true
    }

    fn leave_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            let definition = Definition::new_yul_variable(variable);
            self.insert_definition_in_current_scope(definition);
        }

        // TODO: we maybe want to enter a new scope here, but that should be
        // only relevant for validation (ie. to avoid referencing a variable
        // before declaring it). If we do that, we need to take special care of
        // where we insert label and function definitions, since those are
        // hoisted in the block.

        false
    }

    fn enter_yul_for_statement(&mut self, node: &ir::YulForStatement) -> bool {
        // Visit the initialization block first
        ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block such that iterator and body blocks link to it
        self.enter_scope_for_node_id(node.initialization.id());
        ir::visitor::accept_yul_expression(&node.condition, self);
        ir::visitor::accept_yul_block(&node.iterator, self);
        ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.id());

        // We already visited our children
        false
    }
}
