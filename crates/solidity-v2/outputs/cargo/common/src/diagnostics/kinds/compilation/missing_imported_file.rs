use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an `import` directive resolves to a file that the
/// configured `read_file` callback could not provide. It is anchored at the
/// import path, complementing the [`super::MissingFile`] reported on the
/// imported file itself.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingImportedFile {
    /// The resolved identifier of the imported file that is missing.
    pub imported_file_id: String,
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
