use std::collections::HashMap;
use std::rc::Rc;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::binder::{Binder, Definition, FileScope, Scope};
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
    scope_stack: Vec<NodeId>,
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
        self.scope_stack.push(scope.node_id());
        self.binder.insert_scope(scope);
    }

    fn leave_scope(&mut self, scope_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(current_scope_id, scope_id);
    }

    fn current_file_scope(&mut self) -> &mut FileScope {
        let Some(current_file) = &self.current_file else {
            unreachable!("attempt to get current file scope without a current file being set");
        };
        self.binder.get_file_scope_mut(current_file.id())
    }

    fn insert_definition_in_current_file_scope(&mut self, definition: Definition) {
        self.current_file_scope().insert_definition(&definition);
        self.binder.insert_definition(definition);
    }

    fn current_scope_id(&self) -> NodeId {
        let Some(scope_id) = self.scope_stack.last() else {
            unreachable!("empty scope stack");
        };
        *scope_id
    }

    fn current_scope(&mut self) -> &mut Scope {
        let scope_id = self.current_scope_id();
        self.binder.get_scope_mut(scope_id)
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
        self.leave_scope(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node.node_id, &node.name);
        self.insert_definition_in_current_file_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node.node_id, &node.name);
        self.insert_definition_in_current_file_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node.node_id, &node.name);
        self.insert_definition_in_current_file_scope(definition);

        let scope = Scope::new_contract(node.node_id, self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope(node.node_id);
    }

    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(&node.path);

        if let Some(alias) = &node.alias {
            let definition =
                Definition::new_import(node.node_id, &alias.identifier, imported_file_id);
            self.insert_definition_in_current_file_scope(definition);
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
        self.insert_definition_in_current_file_scope(definition);

        false
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        if let input_ir::FunctionName::Identifier(name) = &node.name {
            let definition = Definition::new_function(node.node_id, name);
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
}
