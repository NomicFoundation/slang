use semver::Version;

use crate::kinds::LexicalContextType;
use crate::kinds::TokenKind::*;
use crate::language::Language;
use crate::lexer::Lexer;
use crate::support::ParserContext;

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
        let mut input = ParserContext::new(s);

        assert_eq!(
            language.next_token::<LexicalContextType::Default>(&mut input),
            Some(*k)
        );
    }
}
