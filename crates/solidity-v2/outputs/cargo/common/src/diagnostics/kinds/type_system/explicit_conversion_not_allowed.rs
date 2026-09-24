use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an explicit type conversion is not allowed from the
/// type of its argument to the target type, eg. `uint8(300)`, `int(myEnum)` or
/// `address(bytes32(0))`.
///
/// TODO: name the source and target types in the message (eg. `from 'uint16'
/// to 'int8'`). This needs a type display printer usable from the passes, with
/// data locations, `address payable` and literal values, which the
/// `SemanticContext` ABI printers lack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExplicitConversionNotAllowed;

impl DiagnosticExtensions for ExplicitConversionNotAllowed {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/explicit-conversion-not-allowed"
    }

    fn message(&self) -> String {
        "Explicit type conversion is not allowed from the argument's type to the target type."
            .to_owned()
    }
}
