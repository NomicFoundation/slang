use semver::Version;
use slang_testlang::parser::Parser;

#[test]
fn unsupported_language_version() {
    let version = Version::parse("0.0.0").unwrap();
    let error = Parser::new(version).unwrap_err();

    assert_eq!(error.to_string(), "Unsupported language version '0.0.0'.");
}
