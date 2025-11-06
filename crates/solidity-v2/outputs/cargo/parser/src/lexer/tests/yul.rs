use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn yul_identifier() {
    test(
        "foo",
        &[(L::YulIdentifier, 0..3)],
        &[(L::YulIdentifier, 0..3)],
    );

    test(
        "foo.bar",
        &[(L::YulIdentifier, 0..7)],
        &[
            (L::YulIdentifier, 0..3),
            (L::Period, 3..4),
            (L::YulIdentifier, 4..7),
        ],
    );
}

fn test(
    source: &str,
    expected_combined: &[(L, Range<usize>)],
    expected_separate: &[(L, Range<usize>)],
) {
    test_lexer_output(
        source,
        expected_separate,
        Version::new(0, 5, 7),
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_combined,
        Version::new(0, 5, 8),
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_combined,
        Version::new(0, 6, 12),
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_separate,
        Version::new(0, 7, 0),
        ContextKind::Yul,
    );
}
