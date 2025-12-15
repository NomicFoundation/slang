use std::ops::Range;

use slang_solidity_v2_common::versions::LanguageVersion;

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
        LanguageVersion::V0_5_7,
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_combined,
        LanguageVersion::V0_5_8,
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_combined,
        LanguageVersion::V0_6_12,
        ContextKind::Yul,
    );

    test_lexer_output(
        source,
        expected_separate,
        LanguageVersion::V0_7_0,
        ContextKind::Yul,
    );
}
