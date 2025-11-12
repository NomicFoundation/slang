use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn double_quoted_string_literal() {
    test("\"\"", &[(L::DoubleQuotedStringLiteral, 0..2)]);
    test("\"foo\"", &[(L::DoubleQuotedStringLiteral, 0..5)]);

    test("\"", &[(L::UNRECOGNIZED, 0..1)]);
    test("\" ", &[(L::UNRECOGNIZED, 0..2)]);
}

#[test]
fn single_quoted_string_literal() {
    test("''", &[(L::SingleQuotedStringLiteral, 0..2)]);
    test("'foo'", &[(L::SingleQuotedStringLiteral, 0..5)]);

    test("'", &[(L::UNRECOGNIZED, 0..1)]);
    test("' ", &[(L::UNRECOGNIZED, 0..2)]);
}

fn test(source: &str, expected: &[(L, Range<usize>)]) {
    test_lexer_output(
        source,
        expected,
        Version::new(0, 8, 0),
        ContextKind::Solidity,
    );
}
