mod multiple_mutability_keywords;

pub use multiple_mutability_keywords::MultipleMutabilityKeywords;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Group of diagnostics produced by semantic analysis — validations that
/// require understanding the meaning of the source beyond its syntax, such
/// as disallowed combinations of modifiers or attributes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticDiagnosticKind {
    /// More than one mutability keyword was provided on a function or
    /// function-type declaration.
    MultipleMutabilityKeywords(MultipleMutabilityKeywords),
}

impl DiagnosticExtensions for SemanticDiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::MultipleMutabilityKeywords(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::MultipleMutabilityKeywords(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::MultipleMutabilityKeywords(d) => d.message(),
        }
    }
}
