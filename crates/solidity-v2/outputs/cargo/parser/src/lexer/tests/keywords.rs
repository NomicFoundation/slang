use std::ops::Range;

use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn unreserved_keyword() {
    test("error", &[(L::ErrorKeyword_Unreserved, 0..5)]);
}

#[test]
fn reserved_keyword() {
    test("enum", &[(L::EnumKeyword_Reserved, 0..4)]);
}

#[test]
fn always_reserved_keyword() {
    test("fallback", &[(L::FallbackKeyword_Reserved, 0..8)]);
}

fn test(source: &str, expected: &[(L, Range<usize>)]) {
    test_lexer_output(
        source,
        expected,
        LanguageVersion::V0_8_0,
        ContextKind::Solidity,
    );
}
