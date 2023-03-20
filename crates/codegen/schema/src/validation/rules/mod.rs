pub mod definitions;
pub mod empty_productions;
pub mod references;
pub mod versions;

use super::super::yaml::cst::NodeRef;
use codegen_utils::errors::CodegenErrors;
use std::path::PathBuf;

pub struct Reporter<'ce> {
    path: PathBuf,
    errors: &'ce mut CodegenErrors,
}

impl<'ce> Reporter<'ce> {
    fn new(path: PathBuf, errors: &'ce mut CodegenErrors) -> Self {
        Self { path, errors }
    }

    pub fn report<E: std::error::Error>(&mut self, cst_node: &NodeRef, error: E) {
        self.errors.push(&self.path, cst_node.range(), error);
    }
}
