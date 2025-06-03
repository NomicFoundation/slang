use std::path::Path;

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
        let binding_rules_source =
            Path::repo_path(&language.binding_rules_file).read_to_string()?;

        Ok(Self {
            binding_rules_source,
        })
    }
}
