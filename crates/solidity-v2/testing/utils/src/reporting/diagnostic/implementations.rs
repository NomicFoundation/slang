use std::ops::Range;

use itertools::Itertools;

use slang_solidity_v2_parser::ParserError;

use super::{Diagnostic, Severity};

impl Diagnostic for ParserError {
    fn text_range(&self) -> Range<usize> {
        match self {
            ParserError::UnexpectedEof { offset, .. } => *offset..*offset,
            ParserError::UnexpectedTerminal { range, .. } => range.clone(),
            ParserError::ExtraTerminal { range, .. } => range.clone(),
        }
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match self {
            ParserError::UnexpectedEof { expected, .. } => {
                format!(
                    "Unexpected end of file. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::UnexpectedTerminal {
                terminal, expected, ..
            } => {
                format!(
                    "Unexpected {terminal}. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::ExtraTerminal { terminal, .. } => {
                format!("Unexpected {terminal}. End of file was expected")
            }
        }
    }
}
