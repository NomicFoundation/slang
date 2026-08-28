use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::files::FileId;

/// Diagnostic emitted when an `import` directive resolves to a file that was
/// never added to the compilation. It is anchored at the import path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingImportedFile {
    /// The resolved identifier of the imported file that is missing.
    pub imported_file_id: FileId,
}

impl DiagnosticExtensions for MissingImportedFile {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "compilation/missing-imported-file"
    }

    fn message(&self) -> String {
        format!("Imported file is missing: {}", self.imported_file_id)
    }
}
