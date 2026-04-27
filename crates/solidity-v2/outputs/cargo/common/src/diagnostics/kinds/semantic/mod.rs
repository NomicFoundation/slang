mod duplicate_mutability_keyword;

pub use duplicate_mutability_keyword::DuplicateMutabilityKeyword;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Group of diagnostics produced by semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticDiagnosticKind {
    /// More than one mutability keyword was provided on a function or
    /// function-type declaration.
    DuplicateMutabilityKeyword(DuplicateMutabilityKeyword),
}

impl DiagnosticExtensions for SemanticDiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::DuplicateMutabilityKeyword(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::DuplicateMutabilityKeyword(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::DuplicateMutabilityKeyword(d) => d.message(),
        }
    }
}
