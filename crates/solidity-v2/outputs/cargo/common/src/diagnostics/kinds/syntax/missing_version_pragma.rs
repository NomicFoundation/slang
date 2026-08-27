use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a source file holds no `pragma solidity` directive at
/// all, leaving the version it was written for unrecorded.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingVersionPragma;

impl DiagnosticExtensions for MissingVersionPragma {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Warning
    }

    fn code(&self) -> &'static str {
        "syntax/missing-version-pragma"
    }

    fn message(&self) -> String {
        "Source file does not specify required compiler version.".to_string()
    }
}
