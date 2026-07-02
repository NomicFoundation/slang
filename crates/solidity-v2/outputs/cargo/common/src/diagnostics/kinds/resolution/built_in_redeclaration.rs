use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul declaration (variable, function, or function
/// parameter/return) reuses the name of a Yul built-in, which is reserved.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BuiltInRedeclaration {
    /// The reserved built-in name that was used as a declaration.
    pub name: String,
}

impl DiagnosticExtensions for BuiltInRedeclaration {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/built-in-redeclaration"
    }

    fn message(&self) -> String {
        format!(
            "Cannot use the built-in name '{}' as a declaration.",
            self.name
        )
    }
}
