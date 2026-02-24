use itertools::Itertools;
use std::str::FromStr;

use slang_solidity_v2_common::diagnostic::{Diagnostic, Severity, TextIndex, TextRange};

use crate::lexer::LexemeKind;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedEOF {
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

impl TryFrom<lalrpop_util::ParseError<usize, LexemeKind, ()>> for ParserError {
    // TODO(v2): Add better errors
    type Error = ();

    fn try_from(
        value: lalrpop_util::ParseError<usize, LexemeKind, ()>,
    ) -> Result<Self, Self::Error> {
        /// This function transforms the `String` representation returned by LALRPOP into an instance of `LexemeKind`
        ///
        /// TODO(v2): We should consider contributing to LALRPOP so it returns the enum instance itself, I don't think
        /// there's any reason not to do it
        fn from_str(expected: Vec<String>) -> Vec<LexemeKind> {
            expected
                .into_iter()
                .map(|s| {
                    let s = &s[2..]; // Remove the "L_" prefix that LALRPOP adds to the lexeme kinds
                    LexemeKind::from_str(s).unwrap_or(LexemeKind::UNRECOGNIZED)
                })
                .collect()
        }

        match value {
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                Ok(Self::UnexpectedEOF {
                    location,
                    expected: from_str(expected),
                })
            }
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (left, token, right),
                expected,
            } => Ok(Self::UnexpectedToken {
                range: TextRange::from_bytes_range(left..right),
                token,
                expected: from_str(expected),
            }),
            lalrpop_util::ParseError::ExtraToken {
                token: (left, token, right),
            } => Ok(Self::ExtraToken {
                range: TextRange::from_bytes_range(left..right),
                token,
            }),
            lalrpop_util::ParseError::User { .. } => Err(()), // We don't use user errors
            lalrpop_util::ParseError::InvalidToken { .. } => Err(()), // We don't use invalid token errors
        }
    }
}

impl Diagnostic for ParserError {
    fn text_range(&self) -> TextRange {
        match self {
            ParserError::UnexpectedEOF { location, .. } => {
                TextRange::from_bytes_range(*location..*location)
            }
            ParserError::UnexpectedToken { range, .. } => range.clone(),
            ParserError::ExtraToken { range, .. } => range.clone(),
        }
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match self {
            ParserError::UnexpectedEOF { expected, .. } => {
                format!(
                    "Unexpected end of file. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::UnexpectedToken {
                token, expected, ..
            } => {
                format!(
                    "Unexpected {token}. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::ExtraToken { token, .. } => {
                format!("Unexpected {token}. End of file was expected")
            }
        }
    }
}
