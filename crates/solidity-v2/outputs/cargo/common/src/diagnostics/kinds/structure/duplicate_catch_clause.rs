use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The kind of catch clause a `try` statement may declare at most once.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CatchClauseKind {
    /// A named `catch Error(...)` clause.
    Error,
    /// A named `catch Panic(...)` clause.
    Panic,
    /// A low-level clause without a selector name: `catch { ... }` or
    /// `catch (bytes ...) { ... }`.
    LowLevel,
}

/// Diagnostic emitted when a `try` statement declares more than one catch clause
/// of the same kind (two `Error`, two `Panic`, or two low-level clauses).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateCatchClause {
    /// The kind of the duplicated catch clause.
    pub kind: CatchClauseKind,
}

impl DiagnosticExtensions for DuplicateCatchClause {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-catch-clause"
    }

    fn message(&self) -> String {
        match self.kind {
            CatchClauseKind::Error => {
                "This try statement already has an \"Error\" catch clause.".to_string()
            }
            CatchClauseKind::Panic => {
                "This try statement already has a \"Panic\" catch clause.".to_string()
            }
            CatchClauseKind::LowLevel => {
                "This try statement already has a low-level catch clause.".to_string()
            }
        }
    }
}
