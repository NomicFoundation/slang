use std::collections::BTreeSet;

use anyhow::Result;
use codegen_language_definition::model;
use infra_utils::codegen::CodegenFileSystem;
use semver::Version;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct BindingsModel {
    binding_rules_source: String,
    built_ins_versions: BTreeSet<Version>,
    file_extension: String,
}

impl BindingsModel {
    pub fn from_language(language: &model::Language) -> Result<Self> {
        // We use `CodegenFileSystem` here to ensure the rules are rebuilt if the rules file changes
        let binding_rules_dir = &language.binding_rules_dir;
        let mut fs = CodegenFileSystem::new(binding_rules_dir)?;
        let binding_rules_source = fs.read_file(binding_rules_dir.join("rules.msgb"))?;
        let built_ins_versions = language.collect_built_ins_versions();
        let file_extension = language.file_extension.clone().unwrap_or_default();

        Ok(Self {
            binding_rules_source,
            built_ins_versions,
            file_extension,
        })
    }
}
