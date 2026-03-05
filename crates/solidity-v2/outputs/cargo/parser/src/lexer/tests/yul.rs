use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::lexemes::LexemeKind as L;
use crate::lexer::tests::test_lexer_output;

#[test]
fn yul_identifier() {
    test_lexer_output(
        "foo",
        &[(L::YulIdentifier, 0..3)],
        LanguageVersion::V0_8_0,
        ContextKind::Yul,
    );

    test_lexer_output(
        "foo.bar",
        &[
            (L::YulIdentifier, 0..3),
            (L::Period, 3..4),
            (L::YulIdentifier, 4..7),
        ],
        LanguageVersion::V0_8_0,
        ContextKind::Yul,
    );
}
