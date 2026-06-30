mod empty_enum;
mod enum_with_too_many_members;
mod function_name_matches_container;
mod invalid_using_directive_container;
mod library_fallback_function;
mod library_receive_function;
mod multiple_constructors;

pub use empty_enum::EmptyEnum;
pub use enum_with_too_many_members::EnumWithTooManyMembers;
pub use function_name_matches_container::FunctionNameMatchesContainer;
pub use invalid_using_directive_container::InvalidUsingDirectiveContainer;
pub use library_fallback_function::LibraryFallbackFunction;
pub use library_receive_function::LibraryReceiveFunction;
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

        /// An enum declares more than 256 members.
        EnumWithTooManyMembers(EnumWithTooManyMembers),
        /// An enum declares no members.
        EmptyEnum(EmptyEnum),

        /// A library declares a fallback function.
        LibraryFallbackFunction(LibraryFallbackFunction),
        /// A library declares a receive function.
        LibraryReceiveFunction(LibraryReceiveFunction),
    }
}
