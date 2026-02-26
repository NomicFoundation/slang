use std::str::FromStr;

use slang_solidity_v2_cst::text_index::{TextIndex, TextRange};

use crate::lexer::LexemeKind;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedEof {
        location: TextIndex,
        expected: Vec<LexemeKind>,
    },
    UnexpectedToken {
        range: TextRange,
        token: LexemeKind,
        expected: Vec<LexemeKind>,
    },
    ExtraToken {
        range: TextRange,
        token: LexemeKind,
    },
}

impl From<lalrpop_util::ParseError<usize, LexemeKind, ()>> for ParserError {
    fn from(value: lalrpop_util::ParseError<usize, LexemeKind, ()>) -> Self {
        /// This function transforms the `String` representation returned by LALRPOP into an instance of `LexemeKind`
        ///
        /// TODO(v2): We should consider contributing to LALRPOP so it returns the enum instance itself, I don't think
        /// there's any reason not to do it
        fn from_str(expected: Vec<String>) -> Vec<LexemeKind> {
            expected
                .into_iter()
                .map(|s| {
                    // Remove the "L_" prefix that LALRPOP adds to the lexeme kinds
                    let s = s.strip_prefix("L_").unwrap_or(&s);
                    LexemeKind::from_str(s).unwrap_or(LexemeKind::UNRECOGNIZED)
                })
                .collect()
        }

        match value {
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                Self::UnexpectedEof {
                    location,
                    expected: from_str(expected),
                }
            }
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (left, token, right),
                expected,
            } => Self::UnexpectedToken {
                range: TextRange::from_bytes_range(left..right),
                token,
                expected: from_str(expected),
            },
            lalrpop_util::ParseError::ExtraToken {
                token: (left, token, right),
            } => Self::ExtraToken {
                range: TextRange::from_bytes_range(left..right),
                token,
            },
            lalrpop_util::ParseError::User { .. } => panic!("The parser should never return a user error, since we're not using any custom error types in our grammar"),
            lalrpop_util::ParseError::InvalidToken { .. } => panic!("The parser should never return an invalid token error, since it's not using the default lexer"),
        }
    }
}
