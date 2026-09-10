use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at an assembly suffix on a storage variable that is
/// neither `.slot` nor `.offset`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulStorageSuffix;

impl DiagnosticExtensions for YulStorageSuffix {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-storage-suffix"
    }

    fn message(&self) -> String {
        "Storage variables only support \".slot\" and \".offset\".".to_owned()
    }
}
