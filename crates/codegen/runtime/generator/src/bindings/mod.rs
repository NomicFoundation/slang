use std::collections::BTreeMap;

use anyhow::Result;
use codegen_language_definition::model::{BuiltIn, BuiltInField, BuiltInFunction, BuiltInType};
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

fn filter_built_ins_for_version(built_ins: &[BuiltIn], version: &Version) -> Vec<BuiltIn> {
    built_ins
        .iter()
        .filter_map(|built_in| match built_in {
            BuiltIn::BuiltInFunction { item } => filter_built_in_function(item, version)
                .map(|item| BuiltIn::BuiltInFunction { item: item.into() }),
            BuiltIn::BuiltInType { item } => filter_built_in_type(item, version)
                .map(|item| BuiltIn::BuiltInType { item: item.into() }),
            BuiltIn::BuiltInVariable { item } => filter_built_in_field(item, version)
                .map(|item| BuiltIn::BuiltInVariable { item: item.into() }),
        })
        .collect()
}

fn filter_built_in_function(
    function: &BuiltInFunction,
    version: &Version,
) -> Option<BuiltInFunction> {
    if function
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        Some(function.clone())
    } else {
        None
    }
}

fn filter_built_in_field(field: &BuiltInField, version: &Version) -> Option<BuiltInField> {
    if field
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        Some(field.clone())
    } else {
        None
    }
}

fn filter_built_in_type(typ: &BuiltInType, version: &Version) -> Option<BuiltInType> {
    if typ
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        let fields = typ
            .fields
            .iter()
            .filter_map(|field| filter_built_in_field(field, version))
            .collect();
        let functions = typ
            .functions
            .iter()
            .filter_map(|function| filter_built_in_function(function, version))
            .collect();

        Some(BuiltInType {
            name: typ.name.clone(),
            fields,
            functions,
            enabled: typ.enabled.clone(),
        })
    } else {
        None
    }
}
