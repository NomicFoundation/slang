use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function changes the data location
/// of a return variable of a non-`external` function it overrides.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideReturnLocationDiffers;

impl DiagnosticExtensions for OverrideReturnLocationDiffers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-return-location-differs"
    }

    fn message(&self) -> String {
        "Data locations of return variables have to be the same when overriding non-external functions, but they differ.".to_string()
    }
}
