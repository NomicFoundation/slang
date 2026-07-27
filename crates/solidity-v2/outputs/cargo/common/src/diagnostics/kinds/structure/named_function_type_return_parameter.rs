use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a return parameter of a function type is given a
/// name (e.g. `function () returns (uint x)`), which is not allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedFunctionTypeReturnParameter;

impl DiagnosticExtensions for NamedFunctionTypeReturnParameter {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/named-function-type-return-parameter"
    }

    fn message(&self) -> String {
        "Return parameters in function types may not be named.".to_string()
    }
}
