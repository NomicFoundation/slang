use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A symbol in an import deconstruction is not declared in the imported file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImportedDeclarationNotFound;

impl DiagnosticExtensions for ImportedDeclarationNotFound {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/imported-declaration-not-found"
    }

    fn message(&self) -> String {
        "Declaration not found in imported file.".to_string()
    }
}
