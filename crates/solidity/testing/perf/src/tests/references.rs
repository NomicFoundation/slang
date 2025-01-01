use slang_solidity::bindings::BindingGraph;

pub fn setup() -> BindingGraph {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(mut binding_graph: BindingGraph) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    binding_graph.resolve();

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
