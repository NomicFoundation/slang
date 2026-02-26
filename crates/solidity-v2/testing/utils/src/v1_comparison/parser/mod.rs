#[path = "node_checker.generated.rs"]
mod node_checker;

pub mod testing;
use node_checker::{NodeChecker, NodeCheckerError};
use slang_solidity::cst::Cursor;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::text_index::TextRange;
use slang_solidity_v2_parser::{Parser, ParserError, SourceUnitParser};
pub use testing::V2TesterConstructor;

use crate::reporting::diagnostic::{Diagnostic, Severity};

pub enum ComparisonError {
    ParsingError(ParserError),
    NodeCheckerError(NodeCheckerError),
}

impl Diagnostic for ComparisonError {
    fn message(&self) -> String {
        match self {
            ComparisonError::ParsingError(error) => error.message(),
            ComparisonError::NodeCheckerError(error) => error.message(),
        }
    }

    fn text_range(&self) -> TextRange {
        match self {
            ComparisonError::ParsingError(error) => error.text_range(),
            ComparisonError::NodeCheckerError(error) => error.text_range(),
        }
    }

    fn severity(&self) -> Severity {
        match self {
            ComparisonError::ParsingError(error) => error.severity(),
            ComparisonError::NodeCheckerError(error) => error.severity(),
        }
    }
}

pub fn compare_with_v1_cursor(source: &str, root_cursor: &Cursor) -> Vec<ComparisonError> {
    let v2_output = SourceUnitParser::parse(source, LanguageVersion::V0_8_30);

    match v2_output {
        Ok(v2_tree) => v2_tree
            .check_node(&root_cursor.node())
            .into_iter()
            .map(ComparisonError::NodeCheckerError)
            .collect(),
        Err(error) => vec![ComparisonError::ParsingError(error)],
    }
}
