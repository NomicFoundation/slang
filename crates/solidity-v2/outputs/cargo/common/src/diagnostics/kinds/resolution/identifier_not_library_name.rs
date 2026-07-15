use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The library name in a `using ... for` directive did not resolve to a unique
/// declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifierNotLibraryName;

impl DiagnosticExtensions for IdentifierNotLibraryName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/identifier-not-library-name"
    }

    fn message(&self) -> String {
        "Identifier not found or not a library name.".to_owned()
    }
}
