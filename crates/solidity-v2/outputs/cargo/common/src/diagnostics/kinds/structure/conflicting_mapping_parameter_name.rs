use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a named parameter of a mapping type reuses a name
/// already used by another parameter in the same or a nested mapping type
/// (e.g. `mapping(uint k => mapping(uint k => uint))`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConflictingMappingParameterName {
    /// The name that is used by more than one mapping parameter.
    pub name: String,
}

impl DiagnosticExtensions for ConflictingMappingParameterName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/conflicting-mapping-parameter-name"
    }

    fn message(&self) -> String {
        format!("Conflicting parameter name '{}' in mapping.", self.name)
    }
}
