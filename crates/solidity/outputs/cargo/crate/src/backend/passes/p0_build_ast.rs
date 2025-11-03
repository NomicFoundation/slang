use std::collections::HashMap;

use crate::backend::ir::ir1_structured_ast::{builder, SourceUnit};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

pub fn run(input: CompilationUnit) -> Output {
    let mut files = HashMap::new();
    for file in &input.files() {
        if let Some(source_unit) = builder::build_source_unit(file.syntax_tree()) {
            files.insert(file.id().into(), source_unit);
        }
    }
    Output {
        compilation_unit: input,
        files,
    }
}
