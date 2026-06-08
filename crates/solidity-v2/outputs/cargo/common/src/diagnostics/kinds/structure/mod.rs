mod function_name_matches_container;

pub use function_name_matches_container::FunctionNameMatchesContainer;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Structure;

    /// Group of diagnostics about structural shape.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum StructureDiagnosticKind {
        /// A function has the same name as its enclosing container.
        FunctionNameMatchesContainer(FunctionNameMatchesContainer),
    }
}
