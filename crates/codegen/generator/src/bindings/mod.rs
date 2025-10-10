use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use language_definition::model;
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
