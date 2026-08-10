use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when two functions visible under the same name declare
/// parameter lists that an external call cannot tell apart, so a call naming
/// them could never be disambiguated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateFunctionDefinition;

impl DiagnosticExtensions for DuplicateFunctionDefinition {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/duplicate-function-definition"
    }

    fn message(&self) -> String {
        "Function with same name and parameter types defined twice.".to_owned()
    }
}
