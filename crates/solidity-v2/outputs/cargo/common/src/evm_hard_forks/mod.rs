#[path = "hard_fork.generated.rs"]
mod hard_fork;
mod specifier;

pub use hard_fork::{EvmHardFork, FromStrError};
pub use specifier::EvmHardForkSpecifier;
