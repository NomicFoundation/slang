use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an index access on a value omits the index, eg.
/// `a[]` on an array or a mapping. Only a type name can be indexed without
/// one, to name an array type (eg. `uint[]`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MissingIndex;

impl DiagnosticExtensions for MissingIndex {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/missing-index"
    }

    fn message(&self) -> String {
        "Index expression cannot be omitted.".to_owned()
    }
}
