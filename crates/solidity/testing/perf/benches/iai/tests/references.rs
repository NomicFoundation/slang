use slang_solidity::bindings::Bindings;

pub fn setup() -> Bindings {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(bindings: &Bindings) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for reference in bindings.all_references() {
        reference_count += 1;

        let resolution = reference.jump_to_definition();
        if resolution.is_ok() {
            resolved_references += 1;
        }
    }

    assert!(
        // TODO(#1077): finalize the assertion counts once bindings are fully implemented:
        reference_count >= 1491,
        "Only found {reference_count} references"
    );

    assert!(
        // TODO(#1077): finalize the assertion counts once bindings are fully implemented:
        resolved_references >= 1170,
        "Only resolved {resolved_references} references"
    );
}
