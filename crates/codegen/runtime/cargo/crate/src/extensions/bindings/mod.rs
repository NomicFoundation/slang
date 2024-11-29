use semver::Version;

use crate::bindings::BindingGraph;
use crate::parser::ParserInitializationError;

#[allow(clippy::needless_pass_by_value)]
pub fn add_built_ins(
    _binding_graph: &mut BindingGraph,
    _version: &Version,
) -> Result<(), ParserInitializationError> {
    unreachable!("Built-ins are Solidity-specific")
}
