use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::files::FileId;

/// Diagnostic emitted when the sources given to a compilation contain the same
/// file ID more than once. The compilation proceeds with the last contents
/// given for the ID, and one of these is reported for each repetition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicatedFileId {
    /// The file ID that was provided more than once.
    pub file_id: FileId,
}

impl DiagnosticExtensions for DuplicatedFileId {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "compilation/duplicated-file-id"
    }

    fn message(&self) -> String {
        format!("Source file provided more than once: {}", self.file_id)
    }
}
