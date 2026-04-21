use std::collections::BTreeSet;
use std::ops::Range;

use slang_solidity_v2_common::versions::LanguageVersionSpecifier;
use slang_solidity_v2_cst::terminals::TerminalKind;

use crate::lexer::LexemeKind;

#[derive(Clone, Debug, PartialEq)]
pub enum ParserError {
    UnexpectedEof {
        offset: usize,
        expected: BTreeSet<TerminalKind>,
    },
    UnexpectedTerminal {
        range: Range<usize>,
        found: TerminalKind,
        expected: BTreeSet<TerminalKind>,
    },
    ExtraTerminal {
        range: Range<usize>,
        found: TerminalKind,
    },
    SyntaxVersion {
        range: Range<usize>,
        enabled: LanguageVersionSpecifier,
    },
}

impl From<lalrpop_util::ParseError<usize, LexemeKind, ()>> for ParserError {
    fn from(value: lalrpop_util::ParseError<usize, LexemeKind, ()>) -> Self {
        /// This function transforms the `String` representation returned by LALRPOP into an instance of `LexemeKind`
        ///
        /// TODO(v2): We may be able to improve on this if there's room for returning a discriminant instead of a string representation.
        /// [Ongoing discussion](https://github.com/lalrpop/lalrpop/issues/1089#issuecomment-4011323139)
        fn convert_expectations(expected: &[String]) -> BTreeSet<TerminalKind> {
            expected
                .iter()
                .map(|str| str.strip_prefix("L_").unwrap())
                .map(|str| str.parse::<LexemeKind>().unwrap())
                .map(|lexeme| TerminalKind::from(&lexeme))
                .collect()
        }

        match value {
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                Self::UnexpectedEof {
                    offset: location,
                    expected: convert_expectations(&expected),
                }
            }
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (left, lexeme, right),
                expected,
            } => Self::UnexpectedTerminal {
                range: left..right,
                found: TerminalKind::from(&lexeme),
                expected: convert_expectations(&expected),
            },
            lalrpop_util::ParseError::ExtraToken {
                token: (left, lexeme, right),
            } => Self::ExtraTerminal {
                range: left..right,
                found: TerminalKind::from(&lexeme),
            },
            lalrpop_util::ParseError::User { .. } => panic!("The parser should never return a user error, since we're not using any custom error types in our grammar"),
            lalrpop_util::ParseError::InvalidToken { .. } => panic!("The parser should never return an invalid token error, since it's not using the default lexer"),
        }
    }
}
