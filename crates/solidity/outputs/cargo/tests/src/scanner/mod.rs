use semver::Version;
use slang_solidity::kinds::TokenKind::{self, *};
use slang_solidity::language::{Language, LexicalContext};
use slang_solidity::lexer::{KeywordScan, ScannedToken};

#[test]
fn test_next_token() {
    let language = Language::new(Version::new(0, 8, 0)).unwrap();

    for (s, k) in &[
        (" ", Whitespace),
        ("{", OpenBrace),
        ("}", CloseBrace),
        ("+=", PlusEqual),
        ("1", DecimalLiteral),
        ("\n", EndOfLine),
        ("unicode'abc'", UnicodeStringLiteral),
        ("hex'abcd'", HexStringLiteral),
        ("'abc'ZZ", AsciiStringLiteral), // with an identifier afterwards
        ("unicode'abc'ZZ", UnicodeStringLiteral), // with an identifier afterwards
        ("hex'abcd'ZZz", HexStringLiteral), // with an identifier afterwards
        ("// single line\n", SingleLineComment),
        ("/* multi-line\n   comment */ blah", MultilineComment),
        ("/* multi-line comment **/ blah", MultilineComment),
        ("0ZZ", SKIPPED),
        ("0xabZZ", SKIPPED),
    ] {
        assert_eq!(
            language.scan(LexicalContext::Default, s),
            Some(ScannedToken::Single(*k))
        );
    }

    for (s, kw) in [
        (
            "contract",
            KeywordScan::Reserved(TokenKind::ContractKeyword),
        ),
        ("X", KeywordScan::Absent),
        ("from", KeywordScan::Present(TokenKind::FromKeyword)),
    ] {
        assert_eq!(
            language.scan(LexicalContext::Default, s),
            Some(ScannedToken::IdentifierOrKeyword {
                identifier: TokenKind::Identifier,
                kw
            })
        );
    }
}
