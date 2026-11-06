use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An entry of an `override` list resolved to a declaration that is neither a
/// contract nor an interface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideNotAContract;

impl DiagnosticExtensions for OverrideNotAContract {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/override-not-a-contract"
    }

    fn message(&self) -> String {
        "An 'override' list can only name contracts and interfaces.".to_owned()
    }
}
