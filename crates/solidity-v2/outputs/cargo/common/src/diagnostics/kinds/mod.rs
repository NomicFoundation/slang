mod utils;

pub mod compilation;
pub mod resolution;
pub mod structure;
pub mod syntax;
pub mod type_system;

use std::cmp::Ordering;

use compilation::CompilationDiagnosticKind;
use resolution::ResolutionDiagnosticKind;
use serde::Serialize;
use structure::StructureDiagnosticKind;
use syntax::SyntaxDiagnosticKind;
use type_system::TypeSystemDiagnosticKind;

use super::extensions::DiagnosticExtensions;
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
        /// A diagnostic about structural shape.
        Structure(StructureDiagnosticKind),
        /// A diagnostic produced for undeclared identifiers, duplicate
        /// definitions, import failures, shadowing, ambiguous references
        /// and scope errors.
        Resolution(ResolutionDiagnosticKind),
        /// A diagnostic about the type system.
        TypeSystem(TypeSystemDiagnosticKind),
    }
}

impl Ord for DiagnosticKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.severity()
            .cmp(&other.severity())
            .then(self.code().cmp(other.code()))
    }
}

impl PartialOrd for DiagnosticKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
