mod cyclic_inheritance;
mod linearisation_impossible;

pub use cyclic_inheritance::CyclicInheritance;
pub use linearisation_impossible::LinearisationImpossible;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Semantic;

    /// Group of diagnostics about semantic constraints.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum SemanticDiagnosticKind {
        /// A contract's or interface's inheritance hierarchy contains a cycle.
        CyclicInheritance(CyclicInheritance),
        /// The inheritance hierarchy cannot be linearised into a consistent
        /// method resolution order.
        LinearisationImpossible(LinearisationImpossible),
    }
}
