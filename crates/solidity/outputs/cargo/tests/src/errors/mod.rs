use semver::Version;
use slang_solidity::{language::Language, syntax::nodes::ProductionKind};

#[test]
fn unsupported_language_version() {
    let version = Version::parse("0.0.0").unwrap();
    let error = Language::new(version).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Unsupported Solidity language version '0.0.0'."
    );
}

#[test]
fn invalid_production_version() {
    let version = Version::parse("0.4.11").unwrap();
    let language = Language::new(version).unwrap();
    let error = language
        .parse(ProductionKind::ConstructorDefinition, "")
        .unwrap_err();

    assert_eq!(
        error.to_string(),
        "Production 'ConstructorDefinition' is not valid in this version of Solidity."
    );
}
