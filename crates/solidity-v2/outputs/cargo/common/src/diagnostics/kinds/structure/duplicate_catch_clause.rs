use serde::Serialize;

use crate::catch_clauses::CatchClauseKind;
use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

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
        let kind = match self.kind {
            CatchClauseKind::Error => "an 'Error'",
            CatchClauseKind::Panic => "a 'Panic'",
            CatchClauseKind::LowLevel => "a low-level",
        };
        format!("This try statement already has {kind} catch clause.")
    }
}
