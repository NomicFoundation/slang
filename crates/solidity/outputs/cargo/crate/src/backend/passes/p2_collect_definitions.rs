use std::collections::HashMap;
use std::rc::Rc;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, FileScope, InterfaceDefinition, LibraryDefinition,
};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::compilation::{CompilationUnit, File};

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
    binder: Binder,
}

impl Pass {
    fn new() -> Self {
        Self {
            current_file: None,
            binder: Binder::new(),
        }
    }

    fn visit_file(&mut self, file: Rc<File>, source_unit: &input_ir::SourceUnit) {
        assert!(self.current_file.is_none());

        self.current_file = Some(file);
        input_ir::visitor::accept_source_unit(source_unit, self);
        self.current_file = None;
    }

    fn current_file_scope(&mut self) -> &mut FileScope {
        let Some(current_file) = &self.current_file else {
            unreachable!("visiting SourceUnit without a current file being set");
        };
        self.binder.get_file_scope_mut(current_file.id())
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
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, _node: &input_ir::SourceUnit) -> bool {
        let Some(current_file) = &self.current_file else {
            unreachable!("visiting SourceUnit without a current file being set");
        };
        self.binder.insert_file_scope(current_file.id());

        true
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        let definition = Definition::Contract(ContractDefinition {
            node_id: node.node_id,
            identifier: Rc::clone(&node.name),
        });

        self.current_file_scope().insert_definition(&definition);
        self.binder.insert_definition(definition);

        true
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        let definition = Definition::Library(LibraryDefinition {
            node_id: node.node_id,
            identifier: Rc::clone(&node.name),
        });

        self.current_file_scope().insert_definition(&definition);
        self.binder.insert_definition(definition);

        true
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        let definition = Definition::Interface(InterfaceDefinition {
            node_id: node.node_id,
            identifier: Rc::clone(&node.name),
        });

        self.current_file_scope().insert_definition(&definition);
        self.binder.insert_definition(definition);

        true
    }

    fn enter_path_import(&mut self, node: &input_ir::PathImport) -> bool {
        if let Some(imported_file_id) = self.resolve_import_path(&node.path) {
            if node.alias.is_none() {
                self.current_file_scope()
                    .add_imported_file(imported_file_id);
            } else {
                // TODO: add the definition for the namespace
            }
        }
        false
    }
}
