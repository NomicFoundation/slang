mod ast;
mod bindings;
mod ir;
mod kinds;
mod parser;

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use codegen_v2_cst::structured_cst::model::StructuredCstModel;
use codegen_v2_parser::parser::ParserModel as ParserModelV2;
use codegen_v2_semantic::ir::builder::build_v2_ir_model;
use codegen_v2_semantic::ir::ModelWithBuilder;
use indexmap::IndexSet;
use infra_utils::cargo::CargoWorkspace;
use ir::builders::{build_ir_models, GenericModel};
use language_definition::model::Language;
use language_v2_definition::model::{Identifier, Language as LanguageV2};
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

#[derive(Serialize)]
pub struct RuntimeModelV2 {
    language: LanguageModelV2,

    parser: ParserModelV2,
    structured_cst_model: StructuredCstModel,
    ir_language_model: ModelWithBuilder,
}

impl RuntimeModelV2 {
    pub fn from_language(language: &LanguageV2) -> Self {
        Self {
            language: LanguageModelV2::from_language(language),
            parser: ParserModelV2::from_language(language),

            structured_cst_model: StructuredCstModel::from_language(language),
            ir_language_model: build_v2_ir_model(language),
        }
    }
}

#[derive(Serialize)]
pub struct LanguageModelV2 {
    name: String,
    versions: IndexSet<Version>,
    evm_targets: IndexSet<Identifier>,
    built_ins: Vec<codegen_v2_semantic::built_ins::BuiltInContextModel>,
}

impl LanguageModelV2 {
    pub fn from_language(language: &LanguageV2) -> Self {
        Self {
            name: language.name.to_string(),
            versions: language.versions.clone(),
            evm_targets: language.evm_targets.clone(),
            built_ins: codegen_v2_semantic::built_ins::build_built_ins_model(language),
        }
    }
}
