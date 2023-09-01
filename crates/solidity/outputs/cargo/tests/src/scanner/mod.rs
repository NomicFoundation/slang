use semver::Version;
use slang_solidity::{
    kinds::TokenKind::*,
    language::{Language, LexicalContext},
};

#[test]
fn test_next_token() {
    let language = Language::new(Version::new(0, 8, 0)).unwrap();

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
        ("0ZZ", SKIPPED),
        ("0xabZZ", SKIPPED),
        ("'abc'ZZ", SKIPPED),
    ] {
        assert_eq!(language.scan(LexicalContext::Default, s), Some(*k));
    }
}
