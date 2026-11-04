use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A built-in function is given named arguments, eg. `abi.encode({a: 1})`.
/// Its parameters have no names to match them to.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedArgumentsToBuiltInFunction;

impl DiagnosticExtensions for NamedArgumentsToBuiltInFunction {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/named-arguments-to-built-in-function"
    }

    fn message(&self) -> String {
        "Built-in functions cannot take named arguments.".to_owned()
    }
}
