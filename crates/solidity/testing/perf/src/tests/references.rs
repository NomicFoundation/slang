use slang_solidity::bindings::BindingGraph;

pub fn setup() -> BindingGraph {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(binding_graph: BindingGraph) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for reference in binding_graph.all_references() {
        if reference.get_file().is_system() {
            // skip built-ins
            continue;
        }
        reference_count += 1;

        let resolution = reference.resolve_definition();
        if resolution.is_ok() {
            resolved_references += 1;
        }
    }

    assert_eq!(reference_count, 1652, "Failed to fetch all references");

    assert_eq!(
        resolved_references, 1409,
        "Failed to resolve all references"
    );
}
