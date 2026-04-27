pub mod compilation;
pub mod syntax;

use compilation::CompilationDiagnosticKind;
use syntax::SyntaxDiagnosticKind;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Top-level classification of a diagnostic by the subsystem that produced
/// it. Callers that only care about rendering should rely on
/// [`DiagnosticExtensions`] instead of matching on this enum directly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// A diagnostic produced while driving the compilation pipeline.
    Compilation(CompilationDiagnosticKind),
    /// A diagnostic produced while parsing source text.
    Syntax(SyntaxDiagnosticKind),
}

impl DiagnosticExtensions for DiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::Compilation(d) => d.severity(),
            Self::Syntax(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Compilation(d) => d.code(),
            Self::Syntax(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Compilation(d) => d.message(),
            Self::Syntax(d) => d.message(),
        }
    }
}
