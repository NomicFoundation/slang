use std::ops::Range;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A single diagnostic produced while parsing or compiling a source unit.
///
/// A `Diagnostic` pairs the location information (`file_id` and `text_range`)
/// with a [`DiagnosticKind`] that describes the specific problem.
///
/// Build one by passing any leaf diagnostic (or group enum) as the `kind`
/// argument; the `impl Into<DiagnosticKind>` bound takes care of the
/// conversion via the `From` chain in [`crate::diagnostics::kinds`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    file_id: String,
    text_range: Range<usize>,
    kind: DiagnosticKind,
}

impl Diagnostic {
    /// Creates a new diagnostic anchored at `text_range` within the file
    /// identified by `file_id`. Any leaf diagnostic type (or group enum) can
    /// be passed as `kind` thanks to the `From` chain defined in
    /// [`crate::diagnostics::kinds`].
    pub(super) fn new(
        file_id: String,
        text_range: Range<usize>,
        kind: impl Into<DiagnosticKind>,
    ) -> Self {
        Self {
            file_id,
            text_range,
            kind: kind.into(),
        }
    }

    /// Returns the identifier of the file this diagnostic was emitted for.
    pub fn file_id(&self) -> &str {
        &self.file_id
    }

    /// Returns the byte range within the source text that this diagnostic
    /// points at.
    pub fn text_range(&self) -> &Range<usize> {
        &self.text_range
    }

    /// Returns the classification of this diagnostic — the specific leaf
    /// payload wrapped inside its group enum.
    pub fn kind(&self) -> &DiagnosticKind {
        &self.kind
    }
}

impl DiagnosticExtensions for Diagnostic {
    fn severity(&self) -> DiagnosticSeverity {
        self.kind.severity()
    }

    fn code(&self) -> &'static str {
        self.kind.code()
    }

    fn message(&self) -> String {
        self.kind.message()
    }
}
