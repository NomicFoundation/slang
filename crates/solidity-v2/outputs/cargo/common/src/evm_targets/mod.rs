mod specifier;
#[path = "target.generated.rs"]
mod target;

pub use specifier::EvmTargetSpecifier;
pub use target::{EvmTarget, EvmTargetConversionError};
