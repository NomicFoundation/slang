use std::ops::Range;

use slang_solidity_v2_common::versions::LanguageVersion;

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
        "constructor",
        &[(L::ConstructorKeyword_Unreserved, 0..11)],
        &[(L::ConstructorKeyword_Reserved, 0..11)],
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
        LanguageVersion::V0_4_11,
        ContextKind::Solidity,
    );

    test_lexer_output(
        source,
        expected_modern,
        LanguageVersion::V0_8_0,
        ContextKind::Solidity,
    );
}
