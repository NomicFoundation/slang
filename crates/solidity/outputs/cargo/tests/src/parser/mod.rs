use semver::Version;
use slang_solidity::{kinds::RuleKind, language::Language};

#[test]
fn test_precedence_expression() {
    let language = Language::new(Version::new(0, 8, 0)).unwrap();

    let output = language.parse(RuleKind::ShiftExpression, "1 >> 2");
    assert!(output.is_valid());
    let output = language.parse(RuleKind::ShiftExpression, "1 + 2");
    assert!(!output.is_valid());
    let output = language.parse(RuleKind::ArrayTypeName, "uint[]");
    assert!(output.is_valid());

    // Plus prefix was removed in 0.5.0
    let language = Language::new(Version::new(0, 4, 11)).unwrap();
    let output = language.parse(RuleKind::PrefixExpression, "+1");
    assert!(output.is_valid());
    let language = Language::new(Version::new(0, 8, 0)).unwrap();
    let output = language.parse(RuleKind::PrefixExpression, "+1");
    assert!(!output.is_valid());

    // Changed to right-associative in 0.6.0; the rule should still be valid
    let output = language.parse(RuleKind::ExponentiationExpression, "1 ** 2");
    assert!(output.is_valid());
    let language = Language::new(Version::new(0, 5, 0)).unwrap();
    let output = language.parse(RuleKind::ExponentiationExpression, "1 ** 2");
    assert!(output.is_valid());
}
