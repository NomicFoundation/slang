use std::ops::Range;

use semver::Version;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn unreserved_keyword() {
    test(
        "pragma solidity 0.4.26;",
        &[
            (L::PragmaKeyword_Reserved, 0..6),
            (L::Whitespace, 6..7),
            (L::SolidityKeyword_Unreserved, 7..15),
            (L::Whitespace, 15..16),
            (L::VersionSpecifier, 16..17),
            (L::Period, 17..18),
            (L::VersionSpecifier, 18..19),
            (L::Period, 19..20),
            (L::VersionSpecifier, 20..22),
            (L::Semicolon, 22..23),
        ],
        &[
            (L::PragmaKeyword_Reserved, 0..6),
            (L::Whitespace, 6..7),
            (L::SolidityKeyword_Unreserved, 7..15),
            (L::Whitespace, 15..16),
            (L::VersionSpecifier, 16..17),
            (L::Period, 17..18),
            (L::VersionSpecifier, 18..19),
            (L::Period, 19..20),
            (L::VersionSpecifier, 20..22),
            (L::Semicolon, 22..23),
        ],
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
