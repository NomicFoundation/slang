use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the getter of a public state variable would return
/// a struct whose members are all omitted from it, as mappings and arrays are,
/// leaving it nothing to return.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GetterWithoutReturnValues;

impl DiagnosticExtensions for GetterWithoutReturnValues {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/getter-without-return-values"
    }

    fn message(&self) -> String {
        "The getter of a public state variable cannot return any values, as all the members of its struct are omitted.".to_owned()
    }
}
