mod ast;
mod bindings;
mod ir;
mod kinds;
mod parser;

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use ir::builders::{GenericModel, build_ir_models};
use language_definition::model::Language;
use semver::Version;
use serde::Serialize;

use crate::ast::AstModel;
use crate::bindings::BindingsModel;
use crate::kinds::KindsModel;
use crate::parser::ParserModel;

#[derive(Serialize)]
pub struct RuntimeModel {
    slang_version: Version,
    language_name: String,
    all_language_versions: BTreeSet<Version>,
    breaking_language_versions: BTreeSet<Version>,

    ast: AstModel,
    bindings: BindingsModel,
    kinds: KindsModel,
    parser: ParserModel,

    ir_languages: BTreeMap<String, GenericModel>,
}

impl RuntimeModel {
    pub fn from_language(language: &Language) -> Result<Self> {
        Ok(Self {
            slang_version: CargoWorkspace::local_version()?,
            language_name: language.name.to_string(),
            all_language_versions: language.versions.iter().cloned().collect(),
            breaking_language_versions: language.collect_grammar_breaking_versions(),

            ast: AstModel::from_language(language),
            bindings: BindingsModel::from_language(language)?,
            parser: ParserModel::from_language(language),
            kinds: KindsModel::from_language(language),

            ir_languages: build_ir_models(language),
        })
    }
}
