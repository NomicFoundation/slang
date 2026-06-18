use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the base slot expression of a contract's storage
/// layout (`layout at <expr>`) is not a compile-time constant expression, or
/// cannot be folded by the constant evaluator.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StorageLayoutBaseNotConstant;

impl DiagnosticExtensions for StorageLayoutBaseNotConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/storage-layout-base-not-constant"
    }

    fn message(&self) -> String {
        "The base slot of the storage layout must be a compile-time constant expression.".to_owned()
    }
}
