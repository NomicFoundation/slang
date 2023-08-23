use semver::Version;
use slang_solidity::{
    kinds::TokenKind::*,
    language::{Language, LexicalContext},
};

#[test]
fn test_next_token() {
    let version = Version::parse("0.4.11").unwrap();
    let language = Language::new(version).unwrap();

    for (s, k) in &[
        ("contract", ContractKeyword),
        (" ", Whitespace),
        ("X", Identifier),
        ("{", OpenBrace),
        ("}", CloseBrace),
        ("+=", PlusEqual),
        ("1", DecimalLiteral),
        ("\n", EndOfLine),
        ("// single line\n", SingleLineComment),
        ("/* multi-line\n   comment */ blah", MultilineComment),
    ] {
        assert_eq!(language.scan(LexicalContext::Default, s), Some(*k));
    }
}
