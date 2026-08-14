use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A `using` directive attaching specific functions targets the wildcard type (`*`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UsingForFunctionsWithWildcard;

impl DiagnosticExtensions for UsingForFunctionsWithWildcard {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/using-for-functions-with-wildcard"
    }

    fn message(&self) -> String {
        "The type has to be specified explicitly when attaching specific functions.".to_string()
    }
}
