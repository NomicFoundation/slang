use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference to an immutable variable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulImmutableAccess;

impl DiagnosticExtensions for YulImmutableAccess {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-immutable-access"
    }

    fn message(&self) -> String {
        "Assembly access to immutable variables is not supported.".to_owned()
    }
}
