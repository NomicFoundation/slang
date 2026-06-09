mod function_name_matches_container;
mod invalid_using_directive_container;
mod multiple_constructors;

pub use function_name_matches_container::FunctionNameMatchesContainer;
pub use invalid_using_directive_container::InvalidUsingDirectiveContainer;
pub use multiple_constructors::MultipleConstructors;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Structure;

    /// Group of diagnostics about structural shape.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum StructureDiagnosticKind {
        /// Using directives are only allowed at the file level, or inside contracts and libraries.
        InvalidUsingDirectiveContainer(InvalidUsingDirectiveContainer),

        /// A function has the same name as its enclosing container.
        FunctionNameMatchesContainer(FunctionNameMatchesContainer),
        /// A contract defines more than one constructor.
        MultipleConstructors(MultipleConstructors),
    }
}
