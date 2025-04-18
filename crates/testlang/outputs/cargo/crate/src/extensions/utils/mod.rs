#[allow(unreachable_code)]
pub fn infer_language_versions(_input: &str) -> impl Iterator<Item = &'static semver::Version> {
    unreachable!("Infering the language version is Solidity-specific");
    std::iter::empty()
}
