use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when call options are set on an expression that already
/// has them, eg. `c.f{value: 1}{gas: 2}()`, `(c.f{value: 1}){gas: 2}()` or
/// `new C{value: 1}{salt: s}()`. All options must be set in a single `{...}`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CallOptionsAlreadySet;

impl DiagnosticExtensions for CallOptionsAlreadySet {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/call-options-already-set"
    }

    fn message(&self) -> String {
        "Call options have already been set; combine them into a single '{...}'.".to_owned()
    }
}
