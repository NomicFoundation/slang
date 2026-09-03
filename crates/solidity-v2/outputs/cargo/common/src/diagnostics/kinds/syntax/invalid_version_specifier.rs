use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a version literal inside a `pragma solidity`
/// directive cannot be read as a version.
///
/// The grammar is looser than the version syntax it is meant to spell out: a
/// specifier may mix digits and wildcards (`1x`), and a quoted literal may hold
/// anything at all (`"Y"`, `"0.8.beta"`, `""`).
///
/// A literal that names a version Slang cannot compile is *not* reported here —
/// it is a perfectly well-formed version, and
/// [`crate::diagnostics::kinds::syntax::IncompatibleVersionPragma`] covers it.
/// That includes a literal with more components than a version has (`0.8.36.0`)
/// and one whose numbers are far larger than any release (`4294967296`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvalidVersionSpecifier {
    /// The specifier as written, with any surrounding quotes removed.
    pub specifier: String,
}

impl DiagnosticExtensions for InvalidVersionSpecifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/invalid-version-specifier"
    }

    fn message(&self) -> String {
        format!(
            "'{specifier}' is not a valid version specifier.",
            specifier = self.specifier
        )
    }
}
