use codegen_v2_cst::structured_cst::model::StructuredCstModel;
use codegen_v2_parser::parser::ParserModel;
use codegen_v2_semantic::built_ins::{BuiltInContextModel, build_built_ins_model};
use codegen_v2_semantic::ir::ModelWithBuilder;
use codegen_v2_semantic::ir::builder::build_v2_ir_model;
use indexmap::IndexSet;
use language_v2_definition::model::{Identifier, Language};
use semver::Version;
use serde::Serialize;

#[derive(Serialize)]
pub struct RuntimeModel {
    language: LanguageModel,

    parser: ParserModel,
    structured_cst_model: StructuredCstModel,
    ir_language_model: ModelWithBuilder,
}

impl RuntimeModel {
    pub fn from_language(language: &Language) -> Self {
        Self {
            language: LanguageModel::from_language(language),
            parser: ParserModel::from_language(language),

            structured_cst_model: StructuredCstModel::from_language(language),
            ir_language_model: build_v2_ir_model(language),
        }
    }
}

#[derive(Serialize)]
pub struct LanguageModel {
    name: String,
    versions: IndexSet<Version>,
    evm_targets: IndexSet<Identifier>,
    built_ins: Vec<BuiltInContextModel>,
}

impl LanguageModel {
    pub fn from_language(language: &Language) -> Self {
        Self {
            name: language.name.to_string(),
            versions: language.versions.clone(),
            evm_targets: language.evm_targets.clone(),
            built_ins: build_built_ins_model(language),
        }
    }
}
