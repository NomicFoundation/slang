use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use serde::Serialize;

use crate::LanguageModel;

#[derive(Default, Serialize)]
pub struct BindingsModel {
    binding_rules_source: String,
    builtins_source: String,
}

impl BindingsModel {
    pub fn from_language(language: &LanguageModel) -> Result<Self> {
        // We use `CodegenFileSystem` here to ensure the rules are rebuilt if the rules file changes
        let binding_rules_dir = language.definition.binding_rules_file.unwrap_parent();
        let mut fs = CodegenFileSystem::new(binding_rules_dir)?;
        let binding_rules_source = fs.read_file(&language.definition.binding_rules_file)?;
        let builtins_source = (language.render_built_ins)(&language.definition.built_ins);

        Ok(Self {
            binding_rules_source,
            builtins_source,
        })
    }
}
