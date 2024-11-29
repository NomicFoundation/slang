use std::{collections::BTreeSet, path::Path};

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
        let binding_rules_dir = &language.binding_rules_dir;
        let binding_rules_source = build_rules(binding_rules_dir, "user-rules.parts")?;
        let built_ins_versions = language.collect_built_ins_versions();
        let file_extension = language.file_extension.clone().unwrap_or_default();

        Ok(Self {
            binding_rules_source,
            built_ins_versions,
            file_extension,
        })
    }
}

// Builds a rules file by concatenating the file parts listed in the given `parts_file`
fn build_rules(rules_dir: &Path, parts_file: &str) -> Result<String> {
    // We use `CodegenFileSystem` here to ensure the rules are rebuilt if the rules file changes
    let mut fs = CodegenFileSystem::new(rules_dir)?;
    let parts_contents = fs.read_file(rules_dir.join(parts_file))?;
    let mut parts = Vec::new();
    for part_name in parts_contents.lines() {
        if part_name.is_empty() {
            continue;
        }
        parts.push(fs.read_file(rules_dir.join(part_name))?);
    }
    Ok(parts.join("\n"))
}
