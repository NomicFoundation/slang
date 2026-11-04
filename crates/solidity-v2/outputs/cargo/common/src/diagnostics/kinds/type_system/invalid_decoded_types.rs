use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The second argument of `abi.decode` is not a parenthesized type name or
/// tuple of type names, eg. `abi.decode(data, uint)` or
/// `abi.decode(data, (uint, 1))`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidDecodedTypes;

impl DiagnosticExtensions for InvalidDecodedTypes {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-decoded-types"
    }

    fn message(&self) -> String {
        "The second argument to 'abi.decode' must be a tuple of type names.".to_owned()
    }
}
