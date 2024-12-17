use slang_solidity::bindings::{BindingGraph, Definition};

pub fn lookup_definition_by_name<'a>(
    binding_graph: &'a BindingGraph,
    name: &str,
) -> Option<Definition<'a>> {
    binding_graph
        .all_definitions()
        .find(|definition| definition.get_cursor().node().unparse() == name)
}
