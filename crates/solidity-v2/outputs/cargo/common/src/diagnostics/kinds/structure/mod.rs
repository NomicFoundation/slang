mod break_outside_loop;
mod continue_outside_loop;
mod empty_enum;
mod empty_struct;
mod enum_with_too_many_members;
mod function_name_matches_container;
mod invalid_using_directive_container;
mod library_fallback_function;
mod library_receive_function;
mod library_virtual_function;
mod library_virtual_modifier;
mod multiple_constructors;
mod storage_layout_for_abstract_contract;
mod unimplemented_modifier_must_be_virtual;
mod uninitialized_constant;
mod virtual_free_function;
mod virtual_private_function;

pub use break_outside_loop::BreakOutsideLoop;
pub use continue_outside_loop::ContinueOutsideLoop;
pub use empty_enum::EmptyEnum;
pub use empty_struct::EmptyStruct;
pub use enum_with_too_many_members::EnumWithTooManyMembers;
pub use function_name_matches_container::FunctionNameMatchesContainer;
pub use invalid_using_directive_container::InvalidUsingDirectiveContainer;
pub use library_fallback_function::LibraryFallbackFunction;
pub use library_receive_function::LibraryReceiveFunction;
pub use library_virtual_function::LibraryVirtualFunction;
pub use library_virtual_modifier::LibraryVirtualModifier;
pub use multiple_constructors::MultipleConstructors;
use serde::Serialize;
pub use storage_layout_for_abstract_contract::StorageLayoutForAbstractContract;
pub use unimplemented_modifier_must_be_virtual::UnimplementedModifierMustBeVirtual;
pub use uninitialized_constant::UninitializedConstant;
pub use virtual_free_function::VirtualFreeFunction;
pub use virtual_private_function::VirtualPrivateFunction;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Structure;

    /// Group of diagnostics about structural shape.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum StructureDiagnosticKind {
        /// A `break` statement appears outside of any loop.
        BreakOutsideLoop(BreakOutsideLoop),
        /// A `continue` statement appears outside of any loop.
        ContinueOutsideLoop(ContinueOutsideLoop),

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

        /// A struct declares no members.
        EmptyStruct(EmptyStruct),

        /// A library declares a fallback function.
        LibraryFallbackFunction(LibraryFallbackFunction),
        /// A library declares a receive function.
        LibraryReceiveFunction(LibraryReceiveFunction),

        /// A function declared in a library is marked `virtual`.
        LibraryVirtualFunction(LibraryVirtualFunction),
        /// A modifier declared in a library is marked `virtual`.
        LibraryVirtualModifier(LibraryVirtualModifier),

        /// A modifier without an implementation body is not marked `virtual`.
        UnimplementedModifierMustBeVirtual(UnimplementedModifierMustBeVirtual),

        /// A free function is marked `virtual`.
        VirtualFreeFunction(VirtualFreeFunction),
        /// A function is marked both `virtual` and `private`.
        VirtualPrivateFunction(VirtualPrivateFunction),

        /// A `constant` is declared without an initializer value.
        UninitializedConstant(UninitializedConstant),

        /// An abstract contract declares a storage layout (`layout at`) specifier.
        StorageLayoutForAbstractContract(StorageLayoutForAbstractContract),
    }
}
