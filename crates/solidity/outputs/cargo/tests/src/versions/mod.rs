use semver::Version;
use slang_solidity::language::Language;

#[test]
fn list_supported_versions() {
    let versions = Language::SUPPORTED_VERSIONS;

    assert_eq!(false, versions.is_empty());
    assert_eq!(false, versions.contains(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, versions.contains(&Version::parse("0.4.11").unwrap()));
}
