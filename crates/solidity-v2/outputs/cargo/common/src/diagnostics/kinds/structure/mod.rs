mod abstract_contract_public_constructor;
mod break_outside_loop;
mod constructor_not_in_contract;
mod continue_outside_loop;
mod contract_should_be_abstract;
mod duplicate_catch_clause;
mod empty_enum;
mod empty_struct;
mod enum_with_too_many_members;
mod free_function_payable;
mod free_function_visibility;
mod function_must_be_implemented;
mod function_name_matches_container;
mod interface_function_cannot_be_implemented;
mod interface_function_not_external;
mod invalid_catch_clause_name;
mod invalid_using_directive_container;
mod library_fallback_function;
mod library_payable_function;
mod library_receive_function;
mod library_virtual_function;
mod library_virtual_modifier;
mod missing_function_visibility;
mod multiple_constructors;
mod non_abstract_contract_internal_constructor;
mod payable_internal_or_private_function;
mod redefined_built_in_error;
mod storage_layout_for_abstract_contract;
mod unimplemented_modifier_must_be_virtual;
mod uninitialized_constant;
mod virtual_free_function;
mod virtual_private_function;

pub use abstract_contract_public_constructor::AbstractContractPublicConstructor;
pub use break_outside_loop::BreakOutsideLoop;
pub use constructor_not_in_contract::ConstructorNotInContract;
pub use continue_outside_loop::ContinueOutsideLoop;
pub use contract_should_be_abstract::ContractShouldBeAbstract;
pub use duplicate_catch_clause::{CatchClauseKind, DuplicateCatchClause};
pub use empty_enum::EmptyEnum;
pub use empty_struct::EmptyStruct;
pub use enum_with_too_many_members::EnumWithTooManyMembers;
pub use free_function_payable::FreeFunctionPayable;
pub use free_function_visibility::FreeFunctionVisibility;
pub use function_must_be_implemented::FunctionMustBeImplemented;
pub use function_name_matches_container::FunctionNameMatchesContainer;
pub use interface_function_cannot_be_implemented::InterfaceFunctionCannotBeImplemented;
pub use interface_function_not_external::InterfaceFunctionNotExternal;
pub use invalid_catch_clause_name::InvalidCatchClauseName;
pub use invalid_using_directive_container::InvalidUsingDirectiveContainer;
pub use library_fallback_function::LibraryFallbackFunction;
pub use library_payable_function::LibraryPayableFunction;
pub use library_receive_function::LibraryReceiveFunction;
pub use library_virtual_function::LibraryVirtualFunction;
pub use library_virtual_modifier::LibraryVirtualModifier;
pub use missing_function_visibility::MissingFunctionVisibility;
pub use multiple_constructors::MultipleConstructors;
pub use non_abstract_contract_internal_constructor::NonAbstractContractInternalConstructor;
pub use payable_internal_or_private_function::PayableInternalOrPrivateFunction;
pub use redefined_built_in_error::RedefinedBuiltInError;
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

        /// A `try` statement has more than one catch clause of the same kind
        /// (two `Error`, two `Panic`, or two low-level clauses).
        DuplicateCatchClause(DuplicateCatchClause),
        /// A `try` statement declares a catch clause with an invalid selector
        /// name (not `Error`, `Panic`, or a low-level clause).
        InvalidCatchClauseName(InvalidCatchClauseName),

        /// A `continue` statement appears outside of any loop.
        ContinueOutsideLoop(ContinueOutsideLoop),

        /// Using directives are only allowed at the file level, or inside contracts and libraries.
        InvalidUsingDirectiveContainer(InvalidUsingDirectiveContainer),

        /// A function has the same name as its enclosing container.
        FunctionNameMatchesContainer(FunctionNameMatchesContainer),

        /// A free function specifies a visibility modifier.
        FreeFunctionVisibility(FreeFunctionVisibility),

        /// A non-free, non-constructor function does not specify a visibility modifier.
        MissingFunctionVisibility(MissingFunctionVisibility),

        /// A function declared in an interface is not `external`.
        InterfaceFunctionNotExternal(InterfaceFunctionNotExternal),

        /// A function declared in an interface has an implementation body.
        InterfaceFunctionCannotBeImplemented(InterfaceFunctionCannotBeImplemented),
        /// A function that requires an implementation body (a free function or a
        /// library function) has none.
        FunctionMustBeImplemented(FunctionMustBeImplemented),

        /// A non-`abstract` contract has an unimplemented function or modifier.
        ContractShouldBeAbstract(ContractShouldBeAbstract),
        /// A contract defines more than one constructor.
        MultipleConstructors(MultipleConstructors),
        /// A constructor is declared outside of a contract (i.e. in an interface or library).
        ConstructorNotInContract(ConstructorNotInContract),

        /// A constructor in an abstract contract is declared `public`.
        AbstractContractPublicConstructor(AbstractContractPublicConstructor),
        /// A constructor in a non-abstract contract is declared `internal`.
        NonAbstractContractInternalConstructor(NonAbstractContractInternalConstructor),

        /// An enum declares more than 256 members.
        EnumWithTooManyMembers(EnumWithTooManyMembers),
        /// An enum declares no members.
        EmptyEnum(EmptyEnum),

        /// A struct declares no members.
        EmptyStruct(EmptyStruct),

        /// An error is defined with a name reserved for a built-in error
        /// (`Error` or `Panic`).
        RedefinedBuiltInError(RedefinedBuiltInError),

        /// A library declares a fallback function.
        LibraryFallbackFunction(LibraryFallbackFunction),
        /// A library declares a receive function.
        LibraryReceiveFunction(LibraryReceiveFunction),
        /// A function declared in a library is marked `payable`.
        LibraryPayableFunction(LibraryPayableFunction),
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

        /// An `internal` or `private` function is marked `payable`.
        PayableInternalOrPrivateFunction(PayableInternalOrPrivateFunction),
        /// A free function is marked `payable`.
        FreeFunctionPayable(FreeFunctionPayable),

        /// A `constant` is declared without an initializer value.
        UninitializedConstant(UninitializedConstant),

        /// An abstract contract declares a storage layout (`layout at`) specifier.
        StorageLayoutForAbstractContract(StorageLayoutForAbstractContract),
    }
}
