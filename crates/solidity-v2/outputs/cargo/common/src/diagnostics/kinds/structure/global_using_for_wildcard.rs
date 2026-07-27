use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A `global` `using` directive targets the wildcard type (`*`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GlobalUsingForWildcard;

impl DiagnosticExtensions for GlobalUsingForWildcard {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/global-using-for-wildcard"
    }

    fn message(&self) -> String {
        "Functions cannot be attached globally to '*'. The types have to be specified explicitly."
            .to_string()
    }
}
