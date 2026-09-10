use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly reference to a constant that is declared
/// later in the same file and whose value is not a literal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulForwardReferencedConstant;

impl DiagnosticExtensions for YulForwardReferencedConstant {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-forward-referenced-constant"
    }

    fn message(&self) -> String {
        "Constant variables with non-literal values cannot be forward referenced from inline assembly."
            .to_owned()
    }
}
