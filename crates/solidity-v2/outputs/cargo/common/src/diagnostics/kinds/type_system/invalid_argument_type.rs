use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// An argument of a call does not implicitly convert to the type of its
/// parameter, eg. `f(true)` for `function f(uint)`.
///
/// TODO: name the argument and parameter types in the message. This needs a
/// type display printer usable from the passes (see
/// `ExplicitConversionNotAllowed`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidArgumentType;

impl DiagnosticExtensions for InvalidArgumentType {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/invalid-argument-type"
    }

    fn message(&self) -> String {
        "Invalid type for argument: it does not implicitly convert to the parameter's type."
            .to_owned()
    }
}
