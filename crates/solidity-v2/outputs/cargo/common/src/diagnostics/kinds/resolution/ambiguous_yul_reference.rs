use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A Yul identifier matches more than one declaration. Unlike Solidity, Yul
/// performs no overload resolution, so the reference cannot be narrowed down
/// even when it is being called with arguments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AmbiguousYulReference {
    pub name: String,
}

impl DiagnosticExtensions for AmbiguousYulReference {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/ambiguous-yul-reference"
    }

    fn message(&self) -> String {
        format!(
            "Multiple matching identifiers for '{}'. Resolving overloaded identifiers is not supported.",
            self.name
        )
    }
}
