use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul function definition appears in the init block
/// of a for-loop.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulFunctionInForLoopInit;

impl DiagnosticExtensions for YulFunctionInForLoopInit {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/yul-function-in-for-loop-init"
    }

    fn message(&self) -> String {
        "Functions cannot be defined inside a for-loop init block.".to_string()
    }
}
