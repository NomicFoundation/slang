use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an explicit dialect in an assembly statement is not
/// "evmasm" which is the only acceptable value.
///
/// Mirrors solc's `ParserError 4531` ("Only 'evmasm' supported.").
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidAssemblyDialect;

impl DiagnosticExtensions for InvalidAssemblyDialect {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-assembly-dialect"
    }

    fn message(&self) -> String {
        "Invalid assembly dialect. Only 'evmasm' is supported.".to_string()
    }
}
