mod diff;
mod model;

pub mod passes;

use diff::IrModelDiff;
pub use model::IrModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelWithBuilder {
    pub target: IrModel,
    pub builder: IrModelDiff,
}

impl ModelWithBuilder {
    pub fn new(source: &IrModel, target: IrModel) -> Self {
        let builder = IrModelDiff::diff(source, &target);
        Self { target, builder }
    }
}

#[derive(Serialize)]
pub struct ModelWithTransformer {
    pub target: IrModel,
    pub transformer: IrModelDiff,
}

impl ModelWithTransformer {
    pub fn new(source: &IrModel, target: IrModel) -> Self {
        let transformer = IrModelDiff::diff(source, &target);
        Self {
            target,
            transformer,
        }
    }
}
