#[path = "nodes.generated.rs"]
mod nodes;
pub use nodes::*;

mod node_extensions;
pub use node_extensions::*;

use super::ir2_flat_contracts as input;

#[path = "visitor.generated.rs"]
pub mod visitor;
