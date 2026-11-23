use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the getter of a public state variable would return
/// a value with no external representation: an internal function, or a struct
/// that is recursive or holds a mapping or an internal function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidGetterReturnType;

impl DiagnosticExtensions for InvalidGetterReturnType {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-getter-return-type"
    }

    fn message(&self) -> String {
        "The getter of a public state variable cannot return an internal function, a mapping or a recursive type.".to_owned()
    }
}
