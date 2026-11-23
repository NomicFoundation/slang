use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An explicit type conversion is given named arguments, eg. `uint({x: 1})`.
/// A conversion takes a single positional argument.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedArgumentsInConversion;

impl DiagnosticExtensions for NamedArgumentsInConversion {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/named-arguments-in-conversion"
    }

    fn message(&self) -> String {
        "A type conversion cannot take named arguments.".to_owned()
    }
}
