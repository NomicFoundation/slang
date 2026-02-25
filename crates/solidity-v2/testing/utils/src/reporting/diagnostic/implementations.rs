use itertools::Itertools;
use slang_solidity_v2_cst::text_index::TextRange;
use slang_solidity_v2_parser::ParserError;

use super::{Diagnostic, Severity};

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
