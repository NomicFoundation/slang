use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul variable declaration (`let`, or a for-loop
/// initializer) reuses the name of a declaration visible from outside the
/// assembly block — a Solidity declaration (state variable, constant, function,
/// import, ...) or a built-in (e.g. `let msg := 1`). A Yul function, parameter,
/// or return variable may legally shadow such a declaration, so only variable
/// declarations are reported.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExternalDeclarationShadowing;

impl DiagnosticExtensions for ExternalDeclarationShadowing {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/external-declaration-shadowing"
    }

    fn message(&self) -> String {
        "This declaration shadows a declaration outside the inline assembly block.".to_owned()
    }
}
