pub mod compilation;
pub mod syntax;
mod utils;

use compilation::CompilationDiagnosticKind;
use serde::Serialize;
use syntax::SyntaxDiagnosticKind;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;

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
