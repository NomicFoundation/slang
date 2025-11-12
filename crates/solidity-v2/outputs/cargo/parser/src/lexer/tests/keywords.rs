use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn unreserved_keyword() {
    test(
        "error",
        &[(L::ErrorKeyword_Unreserved, 0..5)],
        &[(L::ErrorKeyword_Unreserved, 0..5)],
    );
}

#[test]
fn reserved_keyword() {
    test(
        "enum",
        &[(L::EnumKeyword_Reserved, 0..4)],
        &[(L::EnumKeyword_Reserved, 0..4)],
    );
}

#[test]
fn partially_reserved_keyword() {
    test(
        "fallback",
        &[(L::FallbackKeyword_Unreserved, 0..8)],
        &[(L::FallbackKeyword_Reserved, 0..8)],
    );
}

fn test(
    source: &str,
    expected_legacy: &[(L, Range<usize>)],
    expected_modern: &[(L, Range<usize>)],
) {
    test_lexer_output(
        source,
        expected_legacy,
        Version::new(0, 4, 11),
        ContextKind::Solidity,
    );

    test_lexer_output(
        source,
        expected_modern,
        Version::new(0, 8, 0),
        ContextKind::Solidity,
    );
}
