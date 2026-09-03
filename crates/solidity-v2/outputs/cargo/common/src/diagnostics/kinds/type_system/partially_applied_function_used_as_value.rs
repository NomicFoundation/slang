use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function with a bound first argument (eg. `x.f`
/// through a `using` directive) or with call options applied (eg.
/// `this.f{gas: 4}`) is used as a value rather than called, eg. as a branch of
/// a conditional.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PartiallyAppliedFunctionUsedAsValue;

impl DiagnosticExtensions for PartiallyAppliedFunctionUsedAsValue {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/partially-applied-function-used-as-value"
    }

    fn message(&self) -> String {
        "A function with a bound argument or call options can only be called, not used as a value."
            .to_owned()
    }
}
