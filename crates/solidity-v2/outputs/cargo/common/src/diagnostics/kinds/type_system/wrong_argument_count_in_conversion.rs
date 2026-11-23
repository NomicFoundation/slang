use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An explicit type conversion is not given exactly one argument, eg.
/// `uint(1, 2)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WrongArgumentCountInConversion;

impl DiagnosticExtensions for WrongArgumentCountInConversion {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/wrong-argument-count-in-conversion"
    }

    fn message(&self) -> String {
        "A type conversion takes exactly one argument.".to_owned()
    }
}
