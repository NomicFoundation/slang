use slang_solidity::bindings::Bindings;

pub fn setup() -> Bindings {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(bindings: Bindings) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for reference in bindings.all_references() {
        reference_count += 1;

        let resolution = reference.jump_to_definition();
        if resolution.is_ok() {
            resolved_references += 1;
        }
    }

    assert_eq!(reference_count, 1686, "Failed to fetch all references");

    assert_eq!(
        resolved_references, 1353,
        "Failed to resolve all references"
    );
}
