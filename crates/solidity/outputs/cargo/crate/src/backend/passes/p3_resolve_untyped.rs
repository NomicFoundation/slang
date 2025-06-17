use std::collections::HashMap;
use std::rc::Rc;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::{Binder, Reference};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(input.binder);
    for (file_id, source_unit) in &files {
        pass.visit_file(file_id, source_unit);
    }
    let binder = pass.binder;

    Output {
        compilation_unit,
        files,
        binder,
    }
}

struct Pass<'a> {
    current_file_id: Option<&'a str>,
    binder: Binder,
}

impl<'a> Pass<'a> {
    fn new(binder: Binder) -> Self {
        Self {
            current_file_id: None,
            binder,
        }
    }

    fn visit_file<'b: 'a>(&mut self, file_id: &'b str, source_unit: &input_ir::SourceUnit) {
        assert!(self.current_file_id.is_none());

        self.current_file_id = Some(file_id);
        input_ir::visitor::accept_source_unit(source_unit, self);
        self.current_file_id = None;
    }

    fn current_file(&self) -> &str {
        self.current_file_id
            .expect("visiting SourceUnit without a current file being set")
    }

    fn resolve_inheritance_types(&mut self, types: &input_ir::InheritanceTypes) {
        for inheritance_type in types {
            let type_name = &inheritance_type.type_name;

            let leftmost_identifier = &type_name.first().unwrap();
            let definition_id = self
                .binder
                .resolve_in_file_scope(self.current_file(), &leftmost_identifier.unparse());

            let reference = Reference {
                identifier: Rc::clone(leftmost_identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);

            // TODO: resolve the rest of the type name
        }
    }
}

impl Visitor for Pass<'_> {
    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.resolve_inheritance_types(&node.inheritance_types);
        // TODO: save the resolved types as bases of the contract

        true
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        if let Some(inheritance) = &node.inheritance {
            self.resolve_inheritance_types(&inheritance.types);
            // TODO: save the resolved types as bases of the interface
        }

        true
    }
}
