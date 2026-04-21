use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::syntax::SyntaxDiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::versions::LanguageVersionSpecifier;

/// Diagnostic emitted when a piece of syntax is not available for the
/// currently selected language version (either introduced later or
/// deprecated earlier).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsupportedSyntax {
    /// The range of language versions in which this syntax is valid.
    pub supported_in: LanguageVersionSpecifier,
}

impl DiagnosticExtensions for UnsupportedSyntax {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/unsupported-syntax"
    }

    fn message(&self) -> String {
        match &self.supported_in {
            LanguageVersionSpecifier::From { from } => {
                format!("This syntax was introduced in version '{from}'.")
            }
            LanguageVersionSpecifier::Till { till } => {
                format!("This syntax was deprecated in version '{till}'.")
            }
            LanguageVersionSpecifier::Range { from, till } => {
                format!(
                    "This syntax was introduced in version '{from}', and deprecated in version '{till}'."
                )
            }
        }
    }
}

impl From<UnsupportedSyntax> for DiagnosticKind {
    fn from(d: UnsupportedSyntax) -> Self {
        Self::Syntax(SyntaxDiagnosticKind::UnsupportedSyntax(d))
    }
}
