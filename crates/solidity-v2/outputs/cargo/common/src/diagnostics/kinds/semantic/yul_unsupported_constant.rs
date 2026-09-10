use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference to a constant that is not a
/// direct number constant or a chain of references to one.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulUnsupportedConstant;

impl DiagnosticExtensions for YulUnsupportedConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-unsupported-constant"
    }

    fn message(&self) -> String {
        "Only direct number constants and references to such constants are supported by inline assembly."
            .to_owned()
    }
}
