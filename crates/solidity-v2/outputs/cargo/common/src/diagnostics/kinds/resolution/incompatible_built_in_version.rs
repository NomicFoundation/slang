use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::versions::LanguageVersionSpecifier;

/// A built-in is not compatible with the currently selected language version.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleBuiltInVersion {
    /// The range of language versions in which this built-in is compatible.
    pub compatible_in: LanguageVersionSpecifier,
}

impl DiagnosticExtensions for IncompatibleBuiltInVersion {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/incompatible-built-in-version"
    }

    fn message(&self) -> String {
        match &self.compatible_in {
            LanguageVersionSpecifier::From { from } => {
                format!("This built-in was introduced in version '{from}'.")
            }
            LanguageVersionSpecifier::Till { till } => {
                format!("This built-in was deprecated in version '{till}'.")
            }
            LanguageVersionSpecifier::Range { from, till } => {
                format!(
                    "This built-in was introduced in version '{from}', and deprecated in version '{till}'."
                )
            }
        }
    }
}
