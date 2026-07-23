use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a `try` statement declares a catch clause with a
/// selector name other than the permitted ones (`Error`, or `Panic` from 0.8.1).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidCatchClauseName {
    /// Whether `catch Panic(...)` is permitted (true from 0.8.1 onwards). This
    /// determines the set of expected clause names listed in the message.
    pub panic_allowed: bool,
}

impl DiagnosticExtensions for InvalidCatchClauseName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/invalid-catch-clause-name"
    }

    fn message(&self) -> String {
        if self.panic_allowed {
            "Invalid catch clause name. Expected either `catch (...)`, `catch Error(...)`, or `catch Panic(...)`.".to_string()
        } else {
            "Invalid catch clause name. Expected either `catch (...)` or `catch Error(...)`."
                .to_string()
        }
    }
}
