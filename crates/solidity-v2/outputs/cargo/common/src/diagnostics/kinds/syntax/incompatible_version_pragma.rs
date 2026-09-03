use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::versions::LanguageVersion;

/// Diagnostic emitted when a `pragma solidity` directive names a set of versions
/// that does not include the language version the source is being compiled with.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleVersionPragma {
    /// The language version the source is being compiled with, which this pragma
    /// does not admit.
    pub language_version: LanguageVersion,
}

impl DiagnosticExtensions for IncompatibleVersionPragma {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/incompatible-version-pragma"
    }

    fn message(&self) -> String {
        format!(
            "This pragma is incompatible with the compilation language version '{version}'.",
            version = self.language_version,
        )
    }
}
