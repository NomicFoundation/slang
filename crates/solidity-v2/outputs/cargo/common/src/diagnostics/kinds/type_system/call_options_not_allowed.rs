use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when call options are set on a function that is not
/// called by a message call, eg. `f{gas: 1000}()` for an internal function,
/// `L.f{gas: 1000}()` for a library function, or `keccak256{gas: 1000}(b)`.
/// Only external calls (eg. `c.f{value: 1}()`), contract creations
/// (`new C{salt: s}()`) and the low-level `call`, `delegatecall` and
/// `staticcall` of an address take them.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CallOptionsNotAllowed;

impl DiagnosticExtensions for CallOptionsNotAllowed {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/call-options-not-allowed"
    }

    fn message(&self) -> String {
        "Call options can only be set on external function calls or contract creations.".to_owned()
    }
}
