use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a variable declaration statement is used as the
/// un-braced body of an `if`/`else`/`while`/`for`/`do`-`while` statement,
/// rather than inside a block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VariableDeclarationNotInBlock;

impl DiagnosticExtensions for VariableDeclarationNotInBlock {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/variable-declaration-not-in-block"
    }

    fn message(&self) -> String {
        "Variable declarations can only be used inside blocks.".to_string()
    }
}
