use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::versions::LanguageVersionSpecifier;

/// Diagnostic emitted when a piece of syntax is not compatible with the
/// currently selected language version (either introduced later or
/// deprecated earlier).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleSyntaxVersion {
    /// The range of language versions in which this syntax is compatible.
    pub compatible_in: LanguageVersionSpecifier,
}

impl DiagnosticExtensions for IncompatibleSyntaxVersion {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/incompatible-syntax-version"
    }

    fn message(&self) -> String {
        match &self.compatible_in {
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
