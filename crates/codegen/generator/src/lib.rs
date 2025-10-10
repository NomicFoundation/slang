mod ast;
mod bindings;
mod kinds;
mod parser;

pub mod ir;

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use language_definition::model::Language;
use semver::Version;
use serde::Serialize;

use crate::ast::AstModel;
use crate::bindings::BindingsModel;
use crate::kinds::KindsModel;
use crate::parser::ParserModel;

pub struct RuntimeGenerator;

impl RuntimeGenerator {
    pub fn generate_templates_in_place(
        language: &Language,
        fs: &mut CodegenFileSystem,
        dir: &Path,
    ) -> Result<()> {
        let model = RuntimeModel::from_language(language)?;
        CodegenRuntime::render_templates_in_place(fs, dir, model)
    }
}

#[derive(Serialize)]
struct RuntimeModel {
    slang_version: Version,
    language_name: String,
    all_language_versions: BTreeSet<Version>,
    breaking_language_versions: BTreeSet<Version>,

    ast: AstModel,
    bindings: BindingsModel,
    kinds: KindsModel,
    parser: ParserModel,
}

impl RuntimeModel {
    fn from_language(language: &Language) -> Result<Self> {
        Ok(Self {
            slang_version: CargoWorkspace::local_version()?,
            language_name: language.name.to_string(),
            all_language_versions: language.versions.iter().cloned().collect(),
            breaking_language_versions: language.collect_breaking_versions(),

            ast: AstModel::from_language(language),
            bindings: BindingsModel::from_language(language)?,
            parser: ParserModel::from_language(language),
            kinds: KindsModel::from_language(language),
        })
    }
}

impl Default for RuntimeModel {
    fn default() -> Self {
        Self {
            slang_version: Version::new(0, 0, 0),
            language_name: "CodegenRuntime".to_string(),
            all_language_versions: BTreeSet::from([Version::new(0, 0, 0)]),
            breaking_language_versions: BTreeSet::from([Version::new(0, 0, 0)]),

            ast: AstModel::default(),
            bindings: BindingsModel::default(),
            kinds: KindsModel::default(),
            parser: ParserModel::default(),
        }
    }
}
