use anyhow::Result;
use codegen_language_definition::model;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct BindingsModel {
    binding_rules_source: String,
}

impl BindingsModel {
    pub fn from_language(language: &model::Language) -> Result<Self> {
        // We use `CodegenFileSystem` here to ensure the rules are rebuilt if the rules file changes
        let binding_rules_dir = language.binding_rules_file.unwrap_parent();
        let mut fs = CodegenFileSystem::new(binding_rules_dir)?;
        let binding_rules_source = fs.read_file(&language.binding_rules_file)?;

        Ok(Self {
            binding_rules_source,
        })
    }
}
