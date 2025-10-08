mod diff;
mod model;
mod mutator;

pub mod passes;

use diff::IrModelDiff;
pub use model::IrModel;
pub use mutator::IrModelMutator;
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelWithBuilder {
    pub target: IrModel,
    pub builder: IrModelDiff,
}

impl From<IrModelMutator> for ModelWithBuilder {
    fn from(mutator: IrModelMutator) -> Self {
        let target = mutator.target;
        let builder = mutator.diff;
        Self { target, builder }
    }
}

#[derive(Serialize)]
pub struct ModelWithTransformer {
    pub target: IrModel,
    pub transformer: IrModelDiff,
}

impl From<IrModelMutator> for ModelWithTransformer {
    fn from(mutator: IrModelMutator) -> Self {
        let target = mutator.target;
        let transformer = mutator.diff;
        Self {
            target,
            transformer,
        }
    }
}
