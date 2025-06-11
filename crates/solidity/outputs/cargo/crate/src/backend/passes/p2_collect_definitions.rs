use std::collections::HashMap;
use std::rc::Rc;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::compilation::{CompilationUnit, File};

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new();
    for (file_id, source_unit) in &files {
        let file = compilation_unit.file(file_id).unwrap();
        pass.visit_file(file, source_unit);
    }

    Output {
        compilation_unit,
        files,
    }
}

struct Pass {
    current_file: Option<Rc<File>>,
}

impl Pass {
    fn new() -> Self {
        Self { current_file: None }
    }

    fn visit_file(&mut self, file: Rc<File>, source_unit: &input_ir::SourceUnit) {
        assert!(self.current_file.is_none());

        self.current_file = Some(file);
        input_ir::visitor::accept_source_unit(source_unit, self);
        self.current_file = None;
    }
}

impl Visitor for Pass {}
