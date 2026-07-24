use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A `global` `using` directive appears inside a contract, library or interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GlobalUsingForInsideContract;

impl DiagnosticExtensions for GlobalUsingForInsideContract {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/global-using-for-inside-contract"
    }

    fn message(&self) -> String {
        "\"global\" can only be used at file level.".to_string()
    }
}
