mod model;
mod diff;

pub use model::IrModel;
use serde::Serialize;
use diff::IrModelDiff;

#[derive(Serialize)]
pub struct ModelWrapper {
    pub target: IrModel,
    pub builder: Option<IrModelDiff>,
    pub transformer: Option<IrModelDiff>,
}

impl ModelWrapper {
    pub fn with_builder(source: &IrModel, target: IrModel) -> Self {
        let builder = Some(IrModelDiff::diff(source, &target));
        Self {
            target,
            builder,
            transformer: None,
        }
    }

    pub fn with_transformer(source: &IrModel, target: IrModel) -> Self {
        let transformer = Some(IrModelDiff::diff(source, &target));
        Self {
            target,
            builder: None,
            transformer,
        }
    }
}
