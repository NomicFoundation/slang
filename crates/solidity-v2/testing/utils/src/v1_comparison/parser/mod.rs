#[path = "node_checker.generated.rs"]
mod node_checker;

pub use node_checker::{NodeChecker, NodeCheckerError};
use semver::Version;
use slang_solidity::cst::Cursor;
use slang_solidity_v2_common::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser as V2Parser};

use crate::reporting::diagnostic::RenderDiagnostic;

pub enum ComparisonError {
    CompilationDiagnostic(Diagnostic),
    NodeCheckerError(NodeCheckerError),
}

impl RenderDiagnostic for ComparisonError {
    fn text_range(&self) -> std::ops::Range<usize> {
        match self {
            ComparisonError::CompilationDiagnostic(diagnostic) => diagnostic.text_range().clone(),
            ComparisonError::NodeCheckerError(error) => error.text_range(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        match self {
            ComparisonError::CompilationDiagnostic(diagnostic) => {
                DiagnosticExtensions::severity(diagnostic)
            }
            ComparisonError::NodeCheckerError(error) => error.severity(),
        }
    }

    fn message(&self) -> String {
        match self {
            ComparisonError::CompilationDiagnostic(diagnostic) => {
                DiagnosticExtensions::message(diagnostic)
            }
            ComparisonError::NodeCheckerError(error) => error.message(),
        }
    }
}

pub fn compare_with_v1_cursor(
    source: &str,
    root_cursor: &Cursor,
    version: &Version,
) -> Vec<ComparisonError> {
    let lang_version = LanguageVersion::try_from(version.clone()).unwrap();
    let ParseOutput {
        source_unit,
        diagnostics,
    } = V2Parser::parse("v1_comparison", source, lang_version);

    if !diagnostics.is_empty() {
        return diagnostics
            .into_iter()
            .map(ComparisonError::CompilationDiagnostic)
            .collect();
    }

    source_unit
        .check_node(&root_cursor.node())
        .into_iter()
        .map(ComparisonError::NodeCheckerError)
        .collect()
}
