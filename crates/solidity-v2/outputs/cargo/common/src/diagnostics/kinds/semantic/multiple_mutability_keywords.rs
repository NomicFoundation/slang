use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::semantic::SemanticDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::terminals::TerminalKind;

/// Diagnostic emitted when a function or function-type declaration carries
/// more than one mutability keyword (e.g. both `pure` and `view`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultipleMutabilityKeywords {
    /// The first mutability keyword seen on the declaration.
    pub first: TerminalKind,
    /// The second (duplicate) mutability keyword that triggered the diagnostic.
    pub second: TerminalKind,
}

impl DiagnosticExtensions for MultipleMutabilityKeywords {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/multiple-mutability-keywords"
    }

    fn message(&self) -> String {
        format!(
            "Only a single mutability keyword can be provided. Found `{second}` after `{first}`.",
            first = self.first,
            second = self.second,
        )
    }
}

impl From<MultipleMutabilityKeywords> for DiagnosticKind {
    fn from(d: MultipleMutabilityKeywords) -> Self {
        Self::Semantic(SemanticDiagnosticKind::MultipleMutabilityKeywords(d))
    }
}
