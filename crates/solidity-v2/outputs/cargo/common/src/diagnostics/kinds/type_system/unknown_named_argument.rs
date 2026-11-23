use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A named argument of a call names none of the callee's parameters, eg.
/// `f({c: 1})` for `function f(uint a)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnknownNamedArgument {
    pub name: String,
}

impl DiagnosticExtensions for UnknownNamedArgument {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/unknown-named-argument"
    }

    fn message(&self) -> String {
        format!(
            "Named argument '{}' does not match any parameter of the callee.",
            self.name
        )
    }
}
