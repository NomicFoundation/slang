use anyhow::Result;
use codegen_language_definition::model;
use infra_utils::paths::PathExtensions;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct BindingsModel {
    binding_rules_source: String,
}

impl BindingsModel {
    pub fn from_language(language: &model::Language) -> Result<Self> {
        let binding_rules_source = language.binding_rules_file.read_to_string()?;
        println!(
            "cargo:rerun-if-changed={}",
            language.binding_rules_file.to_string_lossy()
        );

        Ok(Self {
            binding_rules_source,
        })
    }
}
