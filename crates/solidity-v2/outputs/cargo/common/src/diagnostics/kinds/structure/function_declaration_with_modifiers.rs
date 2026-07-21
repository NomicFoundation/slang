use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a function declaration (a function without an
/// implementation body, e.g. in an interface or an abstract contract) has one
/// or more modifier invocations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionDeclarationWithModifiers;

impl DiagnosticExtensions for FunctionDeclarationWithModifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/function-declaration-with-modifiers"
    }

    fn message(&self) -> String {
        "Function declaration cannot have modifiers".to_string()
    }
}
