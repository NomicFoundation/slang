mod cyclic_constant_definition;
mod cyclic_constant_dependency;
mod cyclic_dependency_validator_exhausted;

pub use cyclic_constant_definition::CyclicConstantDefinition;
pub use cyclic_constant_dependency::CyclicConstantDependency;
pub use cyclic_dependency_validator_exhausted::CyclicDependencyValidatorExhausted;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Semantic;

    /// Group of diagnostics produced by semantic analysis.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum SemanticDiagnosticKind {
        /// Compile-time constant evaluation hit a cycle or exceeded the
        /// recursion limit.
        CyclicConstantDefinition(CyclicConstantDefinition),
        /// A constant value depends on a cyclic chain of constants.
        CyclicConstantDependency(CyclicConstantDependency),
        /// Constant dependency graph traversal exceeded the depth limit.
        CyclicDependencyValidatorExhausted(CyclicDependencyValidatorExhausted),
    }
}
