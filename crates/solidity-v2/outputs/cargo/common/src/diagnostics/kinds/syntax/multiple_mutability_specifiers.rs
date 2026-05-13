use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a definition carries more than one mutability
/// specifier (e.g. both `pure` and `view`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleMutabilitySpecifiers;

impl DiagnosticExtensions for MultipleMutabilitySpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-mutability-specifiers"
    }

    fn message(&self) -> String {
        "Only a single mutability specifier can be provided.".to_string()
    }
}
