use semver::Version;

use crate::bindings::BindingGraphBuilder;

#[allow(clippy::needless_pass_by_value)]
pub fn add_built_ins(_binding_graph_builder: &mut BindingGraphBuilder, _version: Version) {
    unreachable!("Built-ins are Solidity-specific")
}
