//! Unit tests for how the lexer splits comments into lexemes.
//!
//! Trivia isn't part of the CST, so the exact boundaries of comments can't be checked with snapshots.
//! Neither can `\r` line endings, since the repository normalizes them to `\n`. Whether sources are
//! accepted or rejected is tested in the `cst_output` snapshots.

use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::LexemeKind::{
    self, EndOfLine, Identifier, MultiLineComment, MultiLineNatSpecComment, PragmaMultiLineComment,
    PragmaMultiLineNatSpecComment, PragmaSingleLineComment, PragmaSingleLineNatSpecComment,
    SingleLineComment, SingleLineNatSpecComment, Whitespace, YulMultiLineComment,
    YulMultiLineNatSpecComment, YulSingleLineComment, YulSingleLineNatSpecComment,
};
use crate::lexer::definition::Lexer;

fn lex(source: &str) -> Vec<(LexemeKind, &str)> {
    let mut lexer = Lexer::new(source, LanguageVersion::LATEST);
    std::iter::from_fn(|| lexer.next_lexeme())
        .map(|lexeme| (lexeme.kind, &source[lexeme.range]))
        .collect()
}

#[test]
fn natspec_comments() {
    for (source, expected) in [
        // Each `///` line is a comment of its own, without its line break: the parser merges
        // consecutive ones
        (
            "/// a\n  /// b\nx",
            &[
                (SingleLineNatSpecComment, "/// a"),
                (EndOfLine, "\n"),
                (Whitespace, "  "),
                (SingleLineNatSpecComment, "/// b"),
                (EndOfLine, "\n"),
                (Identifier, "x"),
            ][..],
        ),
        (
            "/// a\r\n///\r\t/// c",
            &[
                (SingleLineNatSpecComment, "/// a"),
                (EndOfLine, "\r\n"),
                (SingleLineNatSpecComment, "///"),
                (EndOfLine, "\r"),
                (Whitespace, "\t"),
                (SingleLineNatSpecComment, "/// c"),
            ],
        ),
        // Slashes within a line are fine
        (
            "/// a/b ////",
            &[(SingleLineNatSpecComment, "/// a/b ////")],
        ),
        ("/// /c", &[(SingleLineNatSpecComment, "/// /c")]),
        // `////` is a regular comment
        ("////", &[(SingleLineComment, "////")]),
        ("//// a", &[(SingleLineComment, "//// a")]),
        (
            "/// a\n//// b",
            &[
                (SingleLineNatSpecComment, "/// a"),
                (EndOfLine, "\n"),
                (SingleLineComment, "//// b"),
            ],
        ),
    ] {
        assert_eq!(lex(source), expected, "{source:?}");
    }
}

#[test]
fn regular_comments() {
    for (source, expected) in [
        // Each `//` line is a comment of its own, including `////` and empty `//` lines
        (
            "// a\n  //\n//// b",
            &[
                (SingleLineComment, "// a"),
                (EndOfLine, "\n"),
                (Whitespace, "  "),
                (SingleLineComment, "//"),
                (EndOfLine, "\n"),
                (SingleLineComment, "//// b"),
            ][..],
        ),
        // Lines of only slashes are regular comments too
        ("//////", &[(SingleLineComment, "//////")]),
        // `/**/` and `/***` are regular comments, but `/**` is a `NatSpec` one
        ("/**/", &[(MultiLineComment, "/**/")]),
        ("/***/", &[(MultiLineComment, "/***/")]),
        ("/*** a */", &[(MultiLineComment, "/*** a */")]),
        ("/** a **/", &[(MultiLineNatSpecComment, "/** a **/")]),
        (
            "/**\n * a/\n */",
            &[(MultiLineNatSpecComment, "/**\n * a/\n */")],
        ),
    ] {
        assert_eq!(lex(source), expected, "{source:?}");
    }
}

#[test]
fn pragma_and_yul_contexts_follow_the_same_rules() {
    for (prefix, suffix, natspec, comment, multi_natspec, multi_comment) in [
        (
            "pragma ",
            "\nsolidity ^0.8.0;",
            PragmaSingleLineNatSpecComment,
            PragmaSingleLineComment,
            PragmaMultiLineNatSpecComment,
            PragmaMultiLineComment,
        ),
        (
            "assembly ",
            "\n{}",
            YulSingleLineNatSpecComment,
            YulSingleLineComment,
            YulMultiLineNatSpecComment,
            YulMultiLineComment,
        ),
    ] {
        // Only the comments, dropping the tokens that put the lexer in the other context
        let comments = |source: &str| {
            let mut lexemes = lex(source);
            lexemes.retain(|(kind, _)| {
                [&natspec, &comment, &multi_natspec, &multi_comment].contains(&kind)
            });
            lexemes
                .into_iter()
                .map(|(kind, text)| (kind, text.to_owned()))
                .collect::<Vec<_>>()
        };

        let source = format!("{prefix}/// a\n///\n//// c /** d */ /*** e */{suffix}");
        assert_eq!(
            comments(&source),
            [
                (natspec.clone(), "/// a".to_owned()),
                (natspec.clone(), "///".to_owned()),
                (comment.clone(), "//// c /** d */ /*** e */".to_owned()),
            ],
            "{source:?}"
        );

        let source = format!("{prefix}/** d */ /*** e */ /***/{suffix}");
        assert_eq!(
            comments(&source),
            [
                (multi_natspec.clone(), "/** d */".to_owned()),
                (multi_comment.clone(), "/*** e */".to_owned()),
                (multi_comment.clone(), "/***/".to_owned()),
            ],
            "{source:?}"
        );
    }
}
