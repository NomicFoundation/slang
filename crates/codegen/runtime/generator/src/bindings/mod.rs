use std::collections::BTreeMap;

use anyhow::Result;
use codegen_language_definition::model::BuiltIn;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Serialize;

use crate::LanguageModel;

#[derive(Default, Serialize)]
pub struct BindingsModel {
    binding_rules_source: String,
    built_ins_source: BTreeMap<Version, String>,
}

impl BindingsModel {
    pub fn from_language(language: &LanguageModel) -> Result<Self> {
        // We use `CodegenFileSystem` here to ensure the rules are rebuilt if the rules file changes
        let binding_rules_dir = language.definition.binding_rules_file.unwrap_parent();
        let mut fs = CodegenFileSystem::new(binding_rules_dir)?;
        let binding_rules_source = fs.read_file(&language.definition.binding_rules_file)?;

        let mut built_ins_source = BTreeMap::new();
        let mut last_contents = None;
        for version in &language.definition.versions {
            let built_ins = filter_built_ins_for_version(&language.definition.built_ins, version);
            let contents = (language.render_built_ins)(&built_ins);
            match last_contents {
                None => last_contents = Some(contents),
                Some(ref last) if last == &contents => (),
                _ => {
                    let last = last_contents.replace(contents);
                    built_ins_source.insert(version.clone(), last.unwrap());
                }
            }
        }
        if let Some(version) = language.definition.versions.last() {
            built_ins_source.insert(version.clone(), last_contents.unwrap());
        }

        Ok(Self {
            binding_rules_source,
            built_ins_source,
        })
    }
}

fn filter_built_ins_for_version<'a>(
    built_ins: &'a [BuiltIn],
    version: &Version,
) -> Vec<&'a BuiltIn> {
    built_ins
        .iter()
        .filter(|built_in| {
            built_in
                .enabled()
                .map_or(true, |enabled| enabled.contains(version))
        })
        .collect()
}
