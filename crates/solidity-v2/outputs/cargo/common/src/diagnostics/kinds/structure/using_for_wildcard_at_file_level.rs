use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A file-level `using` directive targets the wildcard type (`*`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UsingForWildcardAtFileLevel;

impl DiagnosticExtensions for UsingForWildcardAtFileLevel {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/using-for-wildcard-at-file-level"
    }

    fn message(&self) -> String {
        "The type has to be specified explicitly at file level (cannot use '*').".to_string()
    }
}
