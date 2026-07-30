use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function call's named-argument list contains two
/// arguments with the same name.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateNamedArgument {
    /// The name that appears more than once in the argument list.
    pub name: String,
}

impl DiagnosticExtensions for DuplicateNamedArgument {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-named-argument"
    }

    fn message(&self) -> String {
        format!("Duplicate named argument '{}'.", self.name)
    }
}
