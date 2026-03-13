mod model;
mod mutator;

pub mod builders;

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
