pub mod compilation;
pub mod semantic;
pub mod syntax;

use compilation::CompilationDiagnosticKind;
use semantic::SemanticDiagnosticKind;
use syntax::SyntaxDiagnosticKind;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Top-level classification of a diagnostic by the subsystem that produced
/// it. Callers that only care about rendering should rely on
/// [`DiagnosticExtensions`] instead of matching on this enum directly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// A diagnostic produced while parsing source text.
    Syntax(SyntaxDiagnosticKind),
    /// A diagnostic produced while driving the compilation pipeline.
    Compilation(CompilationDiagnosticKind),
    /// A diagnostic produced by semantic analysis of the source.
    Semantic(SemanticDiagnosticKind),
}

impl DiagnosticExtensions for DiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::Syntax(d) => d.severity(),
            Self::Compilation(d) => d.severity(),
            Self::Semantic(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Syntax(d) => d.code(),
            Self::Compilation(d) => d.code(),
            Self::Semantic(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Syntax(d) => d.message(),
            Self::Compilation(d) => d.message(),
            Self::Semantic(d) => d.message(),
        }
    }
}
