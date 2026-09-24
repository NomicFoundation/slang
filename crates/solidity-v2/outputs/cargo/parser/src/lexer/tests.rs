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
fn comments() {
    for (source, expected) in [
        // Consecutive `///` lines are a single comment, whatever their line breaks and indentation,
        // including the line break after each line
        (
            "/// a\n  /// b\nx",
            &[
                (SingleLineNatSpecComment, "/// a\n  /// b\n"),
                (Identifier, "x"),
            ][..],
        ),
        (
            "/// a\r\n/// b\r\t/// c\nx",
            &[
                (SingleLineNatSpecComment, "/// a\r\n/// b\r\t/// c\n"),
                (Identifier, "x"),
            ],
        ),
        // Slashes within a line are fine, and the last line of the source needs no line break
        (
            "/// a/b ////\n/// /c",
            &[(SingleLineNatSpecComment, "/// a/b ////\n/// /c")],
        ),
        // A regular comment, or a blank line, starts a new one
        (
            "/// a\n// b\n/// c",
            &[
                (SingleLineNatSpecComment, "/// a\n"),
                (SingleLineComment, "// b"),
                (EndOfLine, "\n"),
                (SingleLineNatSpecComment, "/// c"),
            ],
        ),
        (
            "/// a\n\n/// b",
            &[
                (SingleLineNatSpecComment, "/// a\n"),
                (EndOfLine, "\n"),
                (SingleLineNatSpecComment, "/// b"),
            ],
        ),
        // Indentation is left out when the next line isn't `///`
        (
            "/// a\n  x",
            &[
                (SingleLineNatSpecComment, "/// a\n"),
                (Whitespace, "  "),
                (Identifier, "x"),
            ],
        ),
        // `////` is a regular comment, which ends a `///` one
        ("//// a", &[(SingleLineComment, "//// a")]),
        (
            "/// a\n//// b",
            &[
                (SingleLineNatSpecComment, "/// a\n"),
                (SingleLineComment, "//// b"),
            ],
        ),
        // Empty `///` lines are part of the comment
        (
            "/// a\n///\n/// b\nx",
            &[
                (SingleLineNatSpecComment, "/// a\n///\n/// b\n"),
                (Identifier, "x"),
            ],
        ),
        (
            "///\n  x",
            &[
                (SingleLineNatSpecComment, "///\n"),
                (Whitespace, "  "),
                (Identifier, "x"),
            ],
        ),
        (
            "/// a\r\n///\r\n//// b",
            &[
                (SingleLineNatSpecComment, "/// a\r\n///\r\n"),
                (SingleLineComment, "//// b"),
            ],
        ),
        (
            "///\r//// b",
            &[
                (SingleLineNatSpecComment, "///\r"),
                (SingleLineComment, "//// b"),
            ],
        ),
        // At the end of the source, an empty `///` line is a comment of its own
        ("///", &[(SingleLineNatSpecComment, "///")]),
        (
            "/// a\n///",
            &[
                (SingleLineNatSpecComment, "/// a\n"),
                (SingleLineNatSpecComment, "///"),
            ],
        ),
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

        let source = format!("{prefix}/// a\n///\n/// b\n//// c /** d */ /*** e */{suffix}");
        assert_eq!(
            comments(&source),
            [
                (natspec.clone(), "/// a\n///\n/// b\n".to_owned()),
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
