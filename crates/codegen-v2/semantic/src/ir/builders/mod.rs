use std::collections::BTreeMap;

use language_v2_definition::model::Language;
use serde::Serialize;

use super::{IrModel, ModelWithBuilder, ModelWithTransformer};

mod ir1_structured_ast;
mod ir2_flat_contracts;

#[derive(Serialize)]
#[serde(untagged)]
pub enum GenericModelV2 {
    ModelWithBuilder(ModelWithBuilder),
    ModelWithTransformer(ModelWithTransformer),
}

pub fn build_v2_ir_models(language: &Language) -> BTreeMap<String, GenericModelV2> {
    let mut ir_models = BTreeMap::new();

    // IR0: CST:
    let cst_model = IrModel::from_language(language);

    // IR1: structured AST:
    let ir1_structured_ast_model = ir1_structured_ast::build_from(&cst_model);

    // IR2: flat contract specifiers:
    let ir2_flat_contracts_model = ir2_flat_contracts::build_from(&ir1_structured_ast_model.target);

    ir_models.insert(
        "ir1_structured_ast".to_string(),
        GenericModelV2::ModelWithBuilder(ir1_structured_ast_model),
    );
    ir_models.insert(
        "ir2_flat_contracts".to_string(),
        GenericModelV2::ModelWithTransformer(ir2_flat_contracts_model),
    );

    ir_models
}
