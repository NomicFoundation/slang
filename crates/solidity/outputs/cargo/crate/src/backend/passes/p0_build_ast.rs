use crate::backend::ir::ir1_structured_ast::{SourceUnit, builder};
use crate::compilation::File;

pub fn run_file(file: &File) -> Option<SourceUnit> {
    builder::build_source_unit(file.tree())
}
