use semver::Version;
use slang_solidity::parser::Parser;

#[test]
fn unsupported_language_version() {
    let version = Version::parse("0.0.0").unwrap();
    let error = Parser::create(version).unwrap_err();

    assert_eq!(error.to_string(), "Unsupported language version '0.0.0'.");
}
