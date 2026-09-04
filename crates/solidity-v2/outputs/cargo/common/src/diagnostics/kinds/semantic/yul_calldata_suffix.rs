use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly suffix on a dynamic calldata array that
/// is neither `.offset` nor `.length`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulCalldataSuffix;

impl DiagnosticExtensions for YulCalldataSuffix {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-calldata-suffix"
    }

    fn message(&self) -> String {
        "Calldata variables only support \".offset\" and \".length\".".to_owned()
    }
}
