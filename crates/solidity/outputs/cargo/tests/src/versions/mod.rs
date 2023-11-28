use semver::Version;
use slang_solidity::language::Language;

#[test]
fn list_supported_versions() {
    let versions = Language::SUPPORTED_VERSIONS;

    assert!(!versions.is_empty());
    assert!(!versions.contains(&Version::new(0, 0, 0)));
    assert!(versions.contains(&Version::new(0, 4, 11)));
}
