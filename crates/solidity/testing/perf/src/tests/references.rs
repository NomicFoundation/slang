use slang_solidity::bindings::BindingGraphBuilder;

pub fn setup() -> BindingGraphBuilder {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(binding_graph_builder: BindingGraphBuilder) {
    let binding_graph = binding_graph_builder.resolve();

    let definition_count = binding_graph
        .all_definitions()
        .filter(|definition| definition.get_file().is_user())
        .count();

    assert_eq!(definition_count, 882, "Failed to fetch all definitions");

    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for reference in binding_graph.all_references() {
        if reference.get_file().is_system() {
            // skip built-ins
            continue;
        }
        reference_count += 1;

        let definitions = reference.definitions();
        if !definitions.is_empty() {
            resolved_references += 1;
        }
    }

    assert_eq!(reference_count, 1652, "Failed to fetch all references");

    assert_eq!(
        resolved_references, 1490,
        "Failed to resolve all references"
    );
}
