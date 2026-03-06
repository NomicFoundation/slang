#[path = "node_checker.generated.rs"]
mod node_checker;

use std::ops::Range;

pub use node_checker::{NodeChecker, NodeCheckerError};
use semver::Version;
use slang_solidity::cst::Cursor;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{Parser as V2Parser, ParserError};

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

    fn text_range(&self) -> Range<usize> {
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

pub fn compare_with_v1_cursor(
    source: &str,
    root_cursor: &Cursor,
    version: &Version,
) -> Vec<ComparisonError> {
    let lang_version = LanguageVersion::try_from(version.clone())
        .unwrap_or_else(|_| panic!("Unsupported language version: {version}."));
    let v2_output = V2Parser::parse(source, lang_version);

    match v2_output {
        Ok(v2_tree) => v2_tree
            .check_node(&root_cursor.node())
            .into_iter()
            .map(ComparisonError::NodeCheckerError)
            .collect(),
        Err(error) => vec![ComparisonError::ParsingError(error)],
    }
}
