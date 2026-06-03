use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the C3 linearisation of a contract's or interface's
/// inheritance hierarchy cannot be computed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LinearisationImpossible;

impl DiagnosticExtensions for LinearisationImpossible {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/linearisation-impossible"
    }

    fn message(&self) -> String {
        "Linearization of inheritance graph impossible.".to_owned()
    }
}
