use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A reference matches more than one declaration and nothing narrows it down to
/// a single one: either the call arguments fit several overloads, or the
/// reference is not a call at all.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AmbiguousReference {
    pub name: String,
}

impl DiagnosticExtensions for AmbiguousReference {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/ambiguous-reference"
    }

    fn message(&self) -> String {
        format!("No unique declaration found for '{}'.", self.name)
    }
}
