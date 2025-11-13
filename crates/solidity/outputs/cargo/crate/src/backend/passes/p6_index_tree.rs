use std::collections::HashMap;

use super::p5_resolve_references::Output as Input;
use crate::backend::binder::Binder;
use crate::backend::ir::ir2_flat_contracts as input_ir;
use crate::backend::ir::ir2_flat_contracts::index::{self, TreeIndex};
use crate::backend::types::TypeRegistry;
use crate::backend::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
    pub index: TreeIndex,
}

#[inline(never)]
pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let binder = input.binder;
    let types = input.types;

    let mut index = TreeIndex::new();
    for source_unit in files.values() {
        index::register_source_unit(source_unit, &mut index);
    }

    Output {
        compilation_unit,
        files,
        binder,
        types,
        index,
    }
}
