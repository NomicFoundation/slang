use std::collections::HashMap;

use crate::backend::l1_structured_ast::{builder, SourceUnit};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

pub fn run(input: CompilationUnit) -> Result<Output, String> {
    let mut files = HashMap::new();
    for file in &input.files() {
        files.insert(file.id().into(), builder::build_source_unit(file.tree())?);
    }
    Ok(Output {
        compilation_unit: input,
        files,
    })
}
