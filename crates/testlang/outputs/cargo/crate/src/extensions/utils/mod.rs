/// Stub iterator just to satisfy the compiler.
struct NilIterator;

impl Iterator for NilIterator {
    type Item = &'static semver::Version;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// The compiler will complain that we are not returning a value which impls Iterator if we return nothing, but
// we want to panic instead of actually returing.
#[allow(unreachable_code)]
pub fn infer_language_versions(_input: &str) -> impl Iterator<Item = &'static semver::Version> {
    unreachable!("Infering the language version is Solidity-specific");
    NilIterator {}
}
