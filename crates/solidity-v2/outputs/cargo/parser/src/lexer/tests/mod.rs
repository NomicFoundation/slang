mod keywords;
mod numbers;
mod strings;
mod trivia;
mod yul;

use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;
use crate::lexer::lexemes::LexemeKind;

fn test_lexer_output(
    source: &str,
    expected: &[(LexemeKind, Range<usize>)],
    version: Version,
    kind: ContextKind,
) {
    let mut lexer = Lexer::new(kind, source, version);

    let actual: Vec<_> = std::iter::from_fn(|| {
        lexer
            .next_lexeme()
            .map(|lexeme| (lexeme.kind, lexeme.range))
    })
    .collect();

    assert_eq!(actual, expected, "for source: {source:#?}");
}
