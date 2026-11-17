use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an expression references a modifier (eg. `m;`,
/// `m(1)` or `C.m`). A modifier is only invoked from a function header, so it
/// has no meaning anywhere else, a statement included.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModifierOutsideFunctionHeader;

impl DiagnosticExtensions for ModifierOutsideFunctionHeader {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/modifier-outside-function-header"
    }

    fn message(&self) -> String {
        "A modifier can only be referenced in a function header.".to_owned()
    }
}
