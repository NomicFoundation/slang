mod model;
mod mutator;

pub mod passes;

pub use model::IrModel;
pub use mutator::IrModelMutator;
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelWithBuilder {
    pub target: IrModel,
    pub builder: IrModelMutator,
}

impl From<IrModelMutator> for ModelWithBuilder {
    fn from(mutator: IrModelMutator) -> Self {
        let target = mutator.build_target();
        Self {
            target,
            builder: mutator,
        }
    }
}

#[derive(Serialize)]
pub struct ModelWithTransformer {
    pub target: IrModel,
    pub transformer: IrModelMutator,
}

impl From<IrModelMutator> for ModelWithTransformer {
    fn from(mutator: IrModelMutator) -> Self {
        let target = mutator.build_target();
        Self {
            target,
            transformer: mutator,
        }
    }
}
