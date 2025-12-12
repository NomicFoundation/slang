mod nodes;
pub use nodes::*;

pub use super::ir2_flat_contracts as input;

#[path = "visitor.generated.rs"]
pub mod visitor;
