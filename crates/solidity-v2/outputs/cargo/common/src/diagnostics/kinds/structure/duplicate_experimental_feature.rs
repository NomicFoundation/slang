use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the same experimental feature is enabled more than
/// once in the same file, i.e. `pragma experimental ABIEncoderV2;` followed by
/// another `pragma experimental "ABIEncoderV2";`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateExperimentalFeature;

impl DiagnosticExtensions for DuplicateExperimentalFeature {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-experimental-feature"
    }

    fn message(&self) -> String {
        "This experimental feature is already enabled in this file.".to_string()
    }
}
