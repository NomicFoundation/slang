use semver::Version;
use slang_solidity::{
    kinds::{TokenKind, TokenKind::*},
    language::Language,
    support::stream::Stream,
};

#[test]
fn test_next_token() {
    let version = Version::parse("0.4.11").unwrap();
    let language = Language::new(version).unwrap();

    let mut stream = Stream::new("contract X {} \n\n// single line \n\n/* multi line */\n\n");
    let mut actual = vec![];
    while let Some(token) = language.default_next_token(&mut stream) {
        actual.push(token);
    }
    let expected: Vec<TokenKind> = vec![
        ContractKeyword,
        Whitespace,
        Identifier,
        Whitespace,
        OpenBrace,
        CloseBrace,
        Whitespace,
        EndOfLine,
        EndOfLine,
        SingleLineComment,
        EndOfLine,
        EndOfLine,
        MultilineComment,
        EndOfLine,
        EndOfLine,
    ];
    assert_eq!(expected, actual);
}
