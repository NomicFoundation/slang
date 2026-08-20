use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A call's operand matches several declarations and none of them accepts the
/// arguments given.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NoMatchingCallableDeclaration;

impl DiagnosticExtensions for NoMatchingCallableDeclaration {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/no-matching-callable-declaration"
    }

    fn message(&self) -> String {
        "No matching declaration found after argument-dependent lookup.".to_owned()
    }
}
