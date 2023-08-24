use semver::Version;
use slang_solidity::{
    kinds::TokenKind::*,
    language::{Language, LexicalContext},
};

#[test]
fn test_next_token() {
    let version = Version::parse("0.8.0").unwrap();
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
        ("unicode'abc'", UnicodeStringLiteral),
        ("unicode'abc'ZZ", Identifier), // TODO: This needs to be further checked against solc
        ("hex'abcd'", HexStringLiteral),
        ("hex'abcd'ZZz", HexKeyword), // TODO: This needs to be further checked against solc
        ("// single line\n", SingleLineComment),
        ("/* multi-line\n   comment */ blah", MultilineComment),
        ("/* multi-line comment **/ blah", MultilineComment),
    ] {
        assert_eq!(language.scan(LexicalContext::Default, s), Some(*k));
    }

    assert_eq!(language.scan(LexicalContext::Default, "0ZZ"), None);
    assert_eq!(language.scan(LexicalContext::Default, "0xabZZ"), None);
    assert_eq!(language.scan(LexicalContext::Default, "'abc'ZZ"), None);
}
