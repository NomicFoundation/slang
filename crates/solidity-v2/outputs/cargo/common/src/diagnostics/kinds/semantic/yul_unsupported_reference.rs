use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference that reads a declaration that
/// is neither a variable nor a library.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulUnsupportedReference {
    /// The declaration kind, worded like `a function` or `an import`.
    pub kind: &'static str,
}

impl DiagnosticExtensions for YulUnsupportedReference {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-unsupported-reference"
    }

    fn message(&self) -> String {
        format!(
            "This is {}. Only variables and libraries can be referenced in inline assembly.",
            self.kind
        )
    }
}
