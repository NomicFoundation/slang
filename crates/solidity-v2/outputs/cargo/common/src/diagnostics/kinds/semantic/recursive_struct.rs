use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted at the definition of every struct from which a by-value
/// cycle is reachable, directly or through other structs and fixed-size
/// arrays. Such a struct would be infinitely sized.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecursiveStruct;

impl DiagnosticExtensions for RecursiveStruct {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/recursive-struct"
    }

    fn message(&self) -> String {
        "Recursive struct definition.".to_owned()
    }
}
