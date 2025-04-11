mod ir_model;
mod transformed;

pub use ir_model::IrModel;
use serde::Serialize;
use transformed::TransformedIrModel;

#[derive(Serialize)]
pub struct ModelWrapper {
    pub target: IrModel,
    pub builder: Option<TransformedIrModel>,
    pub transformer: Option<TransformedIrModel>,
}

impl ModelWrapper {
    pub fn with_builder(source: &IrModel, target: IrModel) -> Self {
        let builder = Some(TransformedIrModel::diff(source, &target));
        Self {
            target,
            builder,
            transformer: None,
        }
    }

    pub fn with_transformer(source: &IrModel, target: IrModel) -> Self {
        let transformer = Some(TransformedIrModel::diff(source, &target));
        Self {
            target,
            builder: None,
            transformer,
        }
    }
}
