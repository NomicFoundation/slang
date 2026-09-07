use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an overriding function changes the data location
/// of a parameter of a non-`external` function it overrides.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OverrideParameterLocationDiffers;

impl DiagnosticExtensions for OverrideParameterLocationDiffers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/override-parameter-location-differs"
    }

    fn message(&self) -> String {
        "Data locations of parameters have to be the same when overriding non-external functions, but they differ.".to_string()
    }
}
