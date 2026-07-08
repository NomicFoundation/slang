use itertools::Itertools;
use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::terminals::TerminalKind;

/// Diagnostic emitted when a definition declares a mutability that is not
/// allowed in its position, or omits one that is required there.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidMutability {
    /// The mutability keywords that are valid on this definition.
    pub valid: Vec<TerminalKind>,
}

impl DiagnosticExtensions for InvalidMutability {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-mutability"
    }

    fn message(&self) -> String {
        format!(
            "Invalid mutability for this definition. Expected {expected}.",
            expected = self
                .valid
                .iter()
                .map(|kind| format!("'{kind}'"))
                .join(" or ")
        )
    }
}
