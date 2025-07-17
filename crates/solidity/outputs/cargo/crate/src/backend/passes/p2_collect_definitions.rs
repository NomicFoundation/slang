use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::binder::{Binder, Definition, FileScope, Scope, ScopeId};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::compilation::{CompilationUnit, File};
use crate::cst::NodeId;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

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

const VERSION_0_5_0: Version = Version::new(0, 5, 0);

struct Pass {
    language_version: Version,
    current_file: Option<Rc<File>>, // needed to resolve imports on the file
    scope_stack: Vec<ScopeId>,
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
        self.scope_stack.push(scope_id);
        scope_id
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            current_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_scope_id(&self) -> ScopeId {
        let Some(scope_id) = self.scope_stack.last() else {
            unreachable!("empty scope stack");
        };
        *scope_id
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

    fn current_non_yul_scope_id(&self) -> ScopeId {
        // look for the first non-yul scope in the stack
        for scope_id in self.scope_stack.iter().rev() {
            match self.binder.get_scope_by_id(*scope_id) {
                Scope::YulBlock(_) | Scope::YulFunction(_) => continue,
                _ => return *scope_id,
            }
        }
        unreachable!("cannot find a non Yul scope in the current scope stack");
    }

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.binder
            .insert_definition_in_scope(definition, self.current_scope_id());
    }

    fn resolve_import_path(&self, import_path: &input_ir::StringLiteral) -> Option<String> {
        let import_path_node_id = match import_path {
            input_ir::StringLiteral::SingleQuotedStringLiteral(single_quoted_string) => {
                single_quoted_string.id()
            }
            input_ir::StringLiteral::DoubleQuotedStringLiteral(double_quoted_string) => {
                double_quoted_string.id()
            }
        };
        let current_file = self
            .current_file
            .as_ref()
            .expect("import directive must be visited in the context of a current file");
        current_file
            .resolved_import_by_node_id(import_path_node_id)
            .map(|file_id| file_id.to_string())

        // TODO(validation): emit an error/warning if the file cannot be resolved
    }

    fn collect_parameters(&mut self, parameters: &input_ir::Parameters, scope_id: ScopeId) {
        for parameter in parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                self.binder.insert_definition_in_scope(definition, scope_id);
            }
        }
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        let Some(current_file) = &self.current_file else {
            unreachable!("visiting SourceUnit without a current file being set");
        };
        let scope = Scope::new_file(node.node_id, current_file.id());
        self.enter_scope(scope);

        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        if let Some(alias) = &node.alias {
            let definition =
                Definition::new_import(node.node_id, &alias.identifier, imported_file_id);
            self.insert_definition_in_current_scope(definition);
        } else if let Some(imported_file_id) = imported_file_id {
            self.current_file_scope()
                .add_imported_file(imported_file_id);
        }

        false
    }

    fn enter_named_import(&mut self, node: &input_ir::NamedImport) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        let definition =
            Definition::new_import(node.node_id, &node.alias.identifier, imported_file_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_import_deconstruction(&mut self, node: &input_ir::ImportDeconstruction) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        for symbol in &node.symbols {
            let identifier = if let Some(alias) = &symbol.alias {
                &alias.identifier
            } else {
                &symbol.name
            };
            let definition = Definition::new_imported_symbol(
                symbol.node_id,
                identifier,
                symbol.name.unparse(),
                imported_file_id.clone(),
            );
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        self.collect_parameters(&node.parameters.parameters, parameters_scope_id);

        if let input_ir::FunctionName::Identifier(name) = &node.name {
            let definition = Definition::new_function(node.node_id, name, parameters_scope_id);
            self.insert_definition_in_current_scope(definition);
        }

        let function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);
        let function_scope_id = self.enter_scope(function_scope);

        if let Some(returns_declaration) = &node.returns {
            self.collect_parameters(&returns_declaration.variables.parameters, function_scope_id);
        }

        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        let definition = Definition::new_modifier(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let modifier_scope = Scope::new_modifier(node.node_id, self.current_scope_id());
        let modifier_scope_id = self.enter_scope(modifier_scope);
        if let Some(parameters) = &node.parameters {
            self.collect_parameters(&parameters.parameters, modifier_scope_id);
        }

        true
    }

    fn leave_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) -> bool {
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        self.collect_parameters(&node.parameters.parameters, parameters_scope_id);

        // TODO: register the constructor to resolve named parameters when
        // constructing this contract. Also, for Solidity < 0.5.0, register the
        // constructor which is the function with the same name as the contract.

        let function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);
        self.enter_scope(function_scope);

        true
    }

    fn leave_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_fallback_function_definition(
        &mut self,
        node: &input_ir::FallbackFunctionDefinition,
    ) -> bool {
        // TODO: we don't need a separate scope for the parameters here since
        // the fallback function cannot be invoked with named arguments. Then
        // again, the function scope requires a parameters scope. Maybe make
        // that optional?
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        self.collect_parameters(&node.parameters.parameters, parameters_scope_id);

        let function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);
        let function_scope_id = self.enter_scope(function_scope);

        if let Some(returns_declaration) = &node.returns {
            self.collect_parameters(&returns_declaration.variables.parameters, function_scope_id);
        }

        true
    }

    fn leave_fallback_function_definition(&mut self, node: &input_ir::FallbackFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_receive_function_definition(
        &mut self,
        node: &input_ir::ReceiveFunctionDefinition,
    ) -> bool {
        // TODO: we don't need a separate scope for the parameters here since
        // the receive function cannot be invoked with named arguments. Then
        // again, the function scope requires a parameters scope. Maybe make
        // that optional?
        // TODO: a receive function does not accept parameters. Should we update
        // the language definition?
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        self.collect_parameters(&node.parameters.parameters, parameters_scope_id);

        let function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);
        self.enter_scope(function_scope);

        true
    }

    fn leave_receive_function_definition(&mut self, node: &input_ir::ReceiveFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_unnamed_function_definition(
        &mut self,
        node: &input_ir::UnnamedFunctionDefinition,
    ) -> bool {
        // TODO: we don't need a separate scope for the parameters here since
        // the unnamed function cannot be invoked with named arguments. Then
        // again, the function scope requires a parameters scope. Maybe make
        // that optional?
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        self.collect_parameters(&node.parameters.parameters, parameters_scope_id);

        let function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);
        self.enter_scope(function_scope);

        true
    }

    fn leave_unnamed_function_definition(&mut self, node: &input_ir::UnnamedFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let enum_scope = Scope::new_enum(node.node_id);
        let enum_scope_id = self.binder.insert_scope(enum_scope);
        for member in &node.members {
            let definition = Definition::new_enum_member(member.id(), member);
            self.binder
                .insert_definition_in_scope(definition, enum_scope_id);
        }

        false
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let struct_scope = Scope::new_struct(node.node_id);
        let struct_scope_id = self.binder.insert_scope(struct_scope);
        for member in &node.members {
            let definition = Definition::new_struct_member(member.node_id, &member.name);
            self.binder
                .insert_definition_in_scope(definition, struct_scope_id);
        }

        false
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        let parameters_scope = Scope::new_parameters(node.members.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        for parameter in &node.members.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                self.binder
                    .insert_definition_in_scope(definition, parameters_scope_id);
            }
        }

        let definition = Definition::new_error(node.node_id, &node.name, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        let parameters_scope = Scope::new_parameters(node.parameters.node_id);
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);
        for parameter in &node.parameters.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                self.binder
                    .insert_definition_in_scope(definition, parameters_scope_id);
            }
        }

        let definition = Definition::new_event(node.node_id, &node.name, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_state_variable_definition(
        &mut self,
        node: &input_ir::StateVariableDefinition,
    ) -> bool {
        let is_constant = node.attributes.iter().any(|attribute| {
            matches!(attribute, input_ir::StateVariableAttribute::ConstantKeyword)
        });
        let definition = if is_constant {
            Definition::new_constant(node.node_id, &node.name)
        } else {
            Definition::new_state_variable(node.node_id, &node.name)
        };
        self.insert_definition_in_current_scope(definition);

        // there may be more definitions in the type of the state variable (eg.
        // key/value names in mappings)
        true
    }

    fn enter_constant_definition(&mut self, node: &input_ir::ConstantDefinition) -> bool {
        let definition = Definition::new_constant(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let definition = Definition::new_user_defined_value_type(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) -> bool {
        // TODO(validation): for Solidity >= 0.5.0 this definition should only
        // be available for statements after this one
        let definition = Definition::new_variable(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) -> bool {
        // TODO(validation): for Solidity >= 0.5.0 this definition should only
        // be available for statements after this one
        let is_untyped_declaration = node.var_keyword.is_some();
        for element in &node.elements {
            let Some(tuple_member) = &element.member else {
                continue;
            };
            let definition = match tuple_member {
                input_ir::TupleMember::TypedTupleMember(typed_tuple_member) => {
                    Definition::new_variable(typed_tuple_member.node_id, &typed_tuple_member.name)
                }
                input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
                    if !is_untyped_declaration {
                        continue;
                    }
                    Definition::new_variable(
                        untyped_tuple_member.node_id,
                        &untyped_tuple_member.name,
                    )
                }
            };
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_block(&mut self, node: &input_ir::Block) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            let scope = Scope::new_block(node.node_id, self.current_scope_id());
            self.enter_scope(scope);
        }
        true
    }

    fn leave_block(&mut self, node: &input_ir::Block) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_for_statement(&mut self, node: &input_ir::ForStatement) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            // For Solidity >= 0.5.0 open a new block here to hold declarations
            // in the initialization clause.
            let scope = Scope::new_block(node.node_id, self.current_scope_id());
            self.enter_scope(scope);
        }
        true
    }

    fn leave_for_statement(&mut self, node: &input_ir::ForStatement) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
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
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.node_id).unwrap();
            self.collect_parameters(&returns.variables.parameters, body_scope_id);
        }
    }

    fn leave_catch_clause(&mut self, node: &input_ir::CatchClause) {
        if let Some(error) = &node.error {
            // For Solidity >= 0.5.0, collect the parameters in the catch
            // declaration and make them available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.node_id).unwrap();
            self.collect_parameters(&error.parameters.parameters, body_scope_id);
        }
    }

    fn enter_mapping_type(&mut self, node: &input_ir::MappingType) -> bool {
        if let Some(name) = &node.key_type.name {
            let definition = Definition::new_type_parameter(node.key_type.node_id, name);
            self.binder.insert_definition_no_scope(definition);
        }
        if let Some(name) = &node.value_type.name {
            let definition = Definition::new_type_parameter(node.value_type.node_id, name);
            self.binder.insert_definition_no_scope(definition);
        }

        false
    }

    fn enter_function_type(&mut self, node: &input_ir::FunctionType) -> bool {
        for parameter in &node.parameters.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_type_parameter(parameter.node_id, name);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        if let Some(returns) = &node.returns {
            for parameter in &returns.variables.parameters {
                if let Some(name) = &parameter.name {
                    let definition = Definition::new_type_parameter(parameter.node_id, name);
                    self.binder.insert_definition_no_scope(definition);
                }
            }
        }

        false
    }

    fn enter_yul_block(&mut self, node: &input_ir::YulBlock) -> bool {
        let scope = Scope::new_yul_block(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_yul_block(&mut self, node: &input_ir::YulBlock) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        let definition = Definition::new_yul_function(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        let enclosing_scope_id = self.current_non_yul_scope_id();
        let scope = Scope::new_yul_function(node.node_id, enclosing_scope_id);
        let scope_id = self.enter_scope(scope);

        for parameter in &node.parameters.parameters {
            let definition = Definition::new_yul_parameter(parameter.id(), parameter);
            self.binder.insert_definition_in_scope(definition, scope_id);
        }
        if let Some(returns) = &node.returns {
            for parameter in &returns.variables {
                let definition = Definition::new_yul_variable(parameter.id(), parameter);
                self.binder.insert_definition_in_scope(definition, scope_id);
            }
        }

        true
    }

    fn leave_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_yul_label(&mut self, node: &input_ir::YulLabel) -> bool {
        let definition = Definition::new_yul_label(node.node_id, &node.label);
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
}
