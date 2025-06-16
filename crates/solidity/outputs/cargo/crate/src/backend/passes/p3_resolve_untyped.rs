use std::collections::HashMap;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::Binder;
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
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }
    let binder = pass.binder;

    Output {
        compilation_unit,
        files,
        binder,
    }
}

struct Pass {
    binder: Binder,
}

impl Pass {
    fn new(binder: Binder) -> Self {
        Self { binder }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
    }
}

impl Visitor for Pass {}
