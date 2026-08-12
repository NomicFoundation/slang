use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::files::FileId;

/// A symbol in an import deconstruction is not declared in the imported file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImportedDeclarationNotFound {
    /// The resolved identifier of the file the import refers to.
    pub imported_file_id: FileId,
}

impl DiagnosticExtensions for ImportedDeclarationNotFound {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/imported-declaration-not-found"
    }

    fn message(&self) -> String {
        format!(
            "Declaration not found in imported file '{}'.",
            self.imported_file_id
        )
    }
}
