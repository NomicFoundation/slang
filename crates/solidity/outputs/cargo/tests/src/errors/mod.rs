use semver::Version;
use slang_solidity::language::Language;

#[test]
fn unsupported_language_version() {
    let version = Version::parse("0.0.0").unwrap();
    let error = Language::new(version).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Unsupported Solidity language version '0.0.0'."
    );
}
