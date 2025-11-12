use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn single_line_comment() {
    test("//", &[(L::SingleLineComment, 0..2)]);
    test("// ", &[(L::SingleLineComment, 0..3)]);
    test("// foo", &[(L::SingleLineComment, 0..6)]);
}

#[test]
fn single_line_nat_spec_comment() {
    test("///", &[(L::SingleLineNatSpecComment, 0..3)]);
    test("/// ", &[(L::SingleLineNatSpecComment, 0..4)]);
    test("/// foo", &[(L::SingleLineNatSpecComment, 0..7)]);
}

#[test]
fn multi_line_comment() {
    test("/**/", &[(L::MultiLineComment, 0..4)]);
    test("/* */", &[(L::MultiLineComment, 0..5)]);
    test("/* **/", &[(L::MultiLineComment, 0..6)]);
    test("/* * */", &[(L::MultiLineComment, 0..7)]);
    test("/* / */", &[(L::MultiLineComment, 0..7)]);
    test("/* foo */", &[(L::MultiLineComment, 0..9)]);

    test(
        "/**/ /**/",
        &[
            // TODO(v2): lexer is greedy, and tries to match the longest possible token by default.
            // Grammar should be fixed to consider this input as two separate comments with whitespace in between:
            // (L::MultiLineComment, 0..4),
            // (L::Whitespace, 4..5),
            // (L::MultiLineComment, 5..9),
            (L::MultiLineNatSpecComment, 0..9),
        ],
    );

    test("/", &[(L::Slash, 0..1)]);
    test("/*", &[(L::Slash, 0..1), (L::Asterisk, 1..2)]);
    test("/* ", &[(L::UNRECOGNIZED, 0..3)]);
    test("/* *", &[(L::UNRECOGNIZED, 0..4)]);
}

#[test]
fn multi_line_nat_spec_comment() {
    test("/***/", &[(L::MultiLineNatSpecComment, 0..5)]);
    test("/** */", &[(L::MultiLineNatSpecComment, 0..6)]);
    test("/** */", &[(L::MultiLineNatSpecComment, 0..6)]);
    test("/*** */", &[(L::MultiLineNatSpecComment, 0..7)]);
    test("/****/", &[(L::MultiLineNatSpecComment, 0..6)]);
    test("/** * */", &[(L::MultiLineNatSpecComment, 0..8)]);
    test("/** / */", &[(L::MultiLineNatSpecComment, 0..8)]);
    test("/** foo */", &[(L::MultiLineNatSpecComment, 0..10)]);

    test(
        "/***/ /***/",
        &[
            (L::MultiLineNatSpecComment, 0..5),
            (L::Whitespace, 5..6),
            (L::MultiLineNatSpecComment, 6..11),
        ],
    );

    test("/", &[(L::Slash, 0..1)]);
    test("/*", &[(L::Slash, 0..1), (L::Asterisk, 1..2)]);
    test("/**", &[(L::Slash, 0..1), (L::AsteriskAsterisk, 1..3)]);

    // TODO(v2): bug in 'logos' crate, only impacting invalid inputs:
    // https://github.com/maciejhirsz/logos/issues/509
    test("/** ", &[(L::Slash, 0..4)]);
    test("/** *", &[(L::Slash, 0..5)]);
}

fn test(source: &str, expected: &[(L, Range<usize>)]) {
    test_lexer_output(
        source,
        expected,
        Version::new(0, 8, 0),
        ContextKind::Solidity,
    );
}
