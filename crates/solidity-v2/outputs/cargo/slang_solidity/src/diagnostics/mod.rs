pub mod compilation;

use compilation::CompilationDiagnosticKind;
use serde::Serialize;
use slang_solidity_v2_common::define_diagnostic_kind;
use slang_solidity_v2_parser::diagnostics::SyntaxDiagnosticKind;

define_diagnostic_kind! {
    /// Top-level classification of a diagnostic by the subsystem that produced
    /// it. Callers that only care about rendering should rely on
    /// [`crate::diagnostics::extensions::DiagnosticExtensions`] instead of matching on this enum directly.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum DiagnosticKind {
        /// A diagnostic produced while driving the compilation pipeline.
        Compilation(CompilationDiagnosticKind),
        /// A diagnostic produced while parsing source text.
        Syntax(SyntaxDiagnosticKind),
    }
}

impl From<SyntaxDiagnosticKind> for DiagnosticKind {
    fn from(value: SyntaxDiagnosticKind) -> Self {
        Self::Syntax(value)
    }
}

impl From<CompilationDiagnosticKind> for DiagnosticKind {
    fn from(value: CompilationDiagnosticKind) -> Self {
        Self::Compilation(value)
    }
}
