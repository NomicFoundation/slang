use std::{ops::Range, str::FromStr};

use crate::lexer::LexemeKind;

// TODO(v2): Replace `LexemeKind` with `TerminalKind` in the public API, since lexemes are internal types.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedEof {
        offset: usize,
        expected: Vec<LexemeKind>,
    },
    UnexpectedTerminal {
        range: Range<usize>,
        terminal: LexemeKind,
        expected: Vec<LexemeKind>,
    },
    ExtraTerminal {
        range: Range<usize>,
        terminal: LexemeKind,
    },
}

impl From<lalrpop_util::ParseError<usize, LexemeKind, ()>> for ParserError {
    fn from(value: lalrpop_util::ParseError<usize, LexemeKind, ()>) -> Self {
        /// This function transforms the `String` representation returned by LALRPOP into an instance of `LexemeKind`
        ///
        /// TODO(v2): We may be able to improve on this if there's room for returning a discriminant instead of a string representation.
        /// [Ongoing discussion](https://github.com/lalrpop/lalrpop/issues/1089#issuecomment-4011323139)
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
                    offset: location,
                    expected: from_str(expected),
                }
            }
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (left, token, right),
                expected,
            } => Self::UnexpectedTerminal {
                range: left..right,
                terminal: token,
                expected: from_str(expected),
            },
            lalrpop_util::ParseError::ExtraToken {
                token: (left, token, right),
            } => Self::ExtraTerminal {
                range: left..right,
                terminal: token,
            },
            lalrpop_util::ParseError::User { .. } => panic!("The parser should never return a user error, since we're not using any custom error types in our grammar"),
            lalrpop_util::ParseError::InvalidToken { .. } => panic!("The parser should never return an invalid token error, since it's not using the default lexer"),
        }
    }
}
