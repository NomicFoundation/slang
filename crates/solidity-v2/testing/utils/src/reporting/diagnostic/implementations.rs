use std::ops::Range;

use itertools::Itertools;
use slang_solidity_v2_common::versions::LanguageVersionSpecifier;
use slang_solidity_v2_cst::structured_cst::validation::SyntaxVersionError;
use slang_solidity_v2_parser::ParserError;

use super::{Diagnostic, Severity};

impl Diagnostic for SyntaxVersionError {
    fn text_range(&self) -> Range<usize> {
        self.range.clone()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match &self.enabled {
            LanguageVersionSpecifier::From { from } => {
                format!("This syntax was introduced in version '{from}'.")
            }
            LanguageVersionSpecifier::Till { till } => {
                format!("This syntax was deprecated in version '{till}'.")
            }
            LanguageVersionSpecifier::Range { from, till } => {
                format!("This syntax was introduced in version '{from}', and deprecated in version '{till}'.")
            }
        }
    }
}

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
