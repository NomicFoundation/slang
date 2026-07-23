use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an `unchecked` block is used as the un-braced body
/// of an `if`/`else`/`while`/`for`/`do`-`while` statement, rather than directly
/// inside a regular block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UncheckedBlockNotInRegularBlock;

impl DiagnosticExtensions for UncheckedBlockNotInRegularBlock {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/unchecked-block-not-in-regular-block"
    }

    fn message(&self) -> String {
        "\"unchecked\" blocks can only be used inside regular blocks.".to_string()
    }
}
