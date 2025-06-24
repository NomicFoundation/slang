use std::collections::HashMap;
use std::rc::Rc;

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
    let mut pass = Pass::new();
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

struct Pass {
    current_file: Option<Rc<File>>, // needed to resolve imports on the file
    scope_stack: Vec<ScopeId>,
    binder: Binder,
}

impl Pass {
    fn new() -> Self {
        Self {
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

    fn enter_scope(&mut self, scope: Scope) {
        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(scope_id);
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

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.current_scope().insert_definition(&definition);
        self.binder.insert_definition(definition);
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
        let mut parameters_scope = Scope::new_parameters(node.parameters.node_id);
        for parameter in &node.parameters.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                parameters_scope.insert_definition(&definition);
                self.binder.insert_definition(definition);
            }
        }
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);

        let mut function_scope =
            Scope::new_function(node.node_id, self.current_scope_id(), parameters_scope_id);

        if let Some(returns_declaration) = &node.returns {
            for variable in &returns_declaration.variables.parameters {
                if let Some(name) = &variable.name {
                    let definition = Definition::new_parameter(variable.node_id, name);
                    function_scope.insert_definition(&definition);
                    self.binder.insert_definition(definition);
                }
            }
        }
        let function_scope_id = self.binder.insert_scope(function_scope);

        if let input_ir::FunctionName::Identifier(name) = &node.name {
            let definition = Definition::new_function(
                node.node_id,
                name,
                function_scope_id,
                parameters_scope_id,
            );
            self.insert_definition_in_current_scope(definition);
        }

        true
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        true
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        true
    }

    fn enter_error_definition(&mut self, node: &input_ir::ErrorDefinition) -> bool {
        let mut parameters_scope = Scope::new_parameters(node.members.node_id);
        for parameter in &node.members.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                parameters_scope.insert_definition(&definition);
                self.binder.insert_definition(definition);
            }
        }
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);

        let definition = Definition::new_error(node.node_id, &node.name, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &input_ir::EventDefinition) -> bool {
        let mut parameters_scope = Scope::new_parameters(node.parameters.node_id);
        for parameter in &node.parameters.parameters {
            if let Some(name) = &parameter.name {
                let definition = Definition::new_parameter(parameter.node_id, name);
                parameters_scope.insert_definition(&definition);
                self.binder.insert_definition(definition);
            }
        }
        let parameters_scope_id = self.binder.insert_scope(parameters_scope);

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

        false
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
        // TODO: for Solidity >= 0.5.0 this should create a new scope
        let definition = Definition::new_variable(node.node_id, &node.name);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) -> bool {
        // TODO: for Solidity >= 0.5.0 this should create a new scope
        for element in &node.elements {
            let Some(tuple_member) = &element.member else {
                continue;
            };
            let definition = match tuple_member {
                input_ir::TupleMember::TypedTupleMember(type_tuple_member) => {
                    Definition::new_variable(type_tuple_member.node_id, &type_tuple_member.name)
                }
                input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
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
}
