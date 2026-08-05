use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `pragma experimental` string literal names a
/// feature that Slang does not recognize.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnrecognizedExperimentalFeature;

impl DiagnosticExtensions for UnrecognizedExperimentalFeature {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unrecognized-experimental-feature"
    }

    fn message(&self) -> String {
        "Unrecognized experimental feature.".to_string()
    }
}
