use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function type has a visibility other than
/// `internal` or `external` (for example `public` or `private`), which are the
/// only visibilities allowed on function types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidFunctionTypeVisibility;

impl DiagnosticExtensions for InvalidFunctionTypeVisibility {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-function-type-visibility"
    }

    fn message(&self) -> String {
        "Invalid visibility, can only be \"external\" or \"internal\".".to_owned()
    }
}
