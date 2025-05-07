mod generated;

use std::collections::HashMap;

pub use generated::*;

use crate::compilation;

pub struct CompilationUnit {
    pub files: HashMap<String, SourceUnit>,
}

impl CompilationUnit {
    pub fn build(input: &compilation::CompilationUnit) -> Result<Self, String> {
        let mut files = HashMap::new();
        for file in &input.files() {
            files.insert(file.id().into(), builder::build_source_unit(file.tree())?);
        }
        Ok(Self { files })
    }
}
