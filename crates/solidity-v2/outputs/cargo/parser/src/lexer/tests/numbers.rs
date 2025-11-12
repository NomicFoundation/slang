use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn decimal_literal() {
    test(
        "1",
        &[(L::DecimalLiteral, 0..1)],
        &[(L::DecimalLiteral, 0..1)],
    );

    test(
        "1e1",
        &[(L::DecimalLiteral, 0..3)],
        &[(L::DecimalLiteral, 0..3)],
    );

    test(
        "1e-1",
        &[(L::DecimalLiteral, 0..4)],
        &[(L::DecimalLiteral, 0..4)],
    );

    test(
        "1.",
        &[(L::DecimalLiteral, 0..2)],
        &[(L::DecimalLiteral, 0..1), (L::Period, 1..2)],
    );

    test(
        "1.2",
        &[(L::DecimalLiteral, 0..3)],
        &[(L::DecimalLiteral, 0..3)],
    );

    test(
        "1.e1",
        &[(L::DecimalLiteral, 0..4)],
        &[
            (L::DecimalLiteral, 0..1),
            (L::Period, 1..2),
            (L::Identifier, 2..4),
        ],
    );

    test(
        "1.e-1",
        &[(L::DecimalLiteral, 0..5)],
        &[
            (L::DecimalLiteral, 0..1),
            (L::Period, 1..2),
            (L::Identifier, 2..3),
            (L::Minus, 3..4),
            (L::DecimalLiteral, 4..5),
        ],
    );

    test(
        "1.2e3",
        &[(L::DecimalLiteral, 0..5)],
        &[(L::DecimalLiteral, 0..5)],
    );

    test(
        "1.2e-3",
        &[(L::DecimalLiteral, 0..6)],
        &[(L::DecimalLiteral, 0..6)],
    );

    test(".", &[(L::Period, 0..1)], &[(L::Period, 0..1)]);

    test(
        ".2",
        &[(L::DecimalLiteral, 0..2)],
        &[(L::DecimalLiteral, 0..2)],
    );

    test(
        ".e1",
        &[(L::Period, 0..1), (L::Identifier, 1..3)],
        &[(L::Period, 0..1), (L::Identifier, 1..3)],
    );

    test(
        ".e-1",
        &[
            (L::Period, 0..1),
            (L::Identifier, 1..2),
            (L::Minus, 2..3),
            (L::DecimalLiteral, 3..4),
        ],
        &[
            (L::Period, 0..1),
            (L::Identifier, 1..2),
            (L::Minus, 2..3),
            (L::DecimalLiteral, 3..4),
        ],
    );

    test(
        ".2e3",
        &[(L::DecimalLiteral, 0..4)],
        &[(L::DecimalLiteral, 0..4)],
    );

    test(
        ".2e-3",
        &[(L::DecimalLiteral, 0..5)],
        &[(L::DecimalLiteral, 0..5)],
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
        Version::new(0, 4, 26),
        ContextKind::Solidity,
    );

    test_lexer_output(
        source,
        expected_modern,
        Version::new(0, 5, 0),
        ContextKind::Solidity,
    );
}
