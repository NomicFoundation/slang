mod bytecode_dependency_validator_exhausted;
mod cyclic_bytecode_dependency;
mod cyclic_constant_definition;
mod cyclic_constant_dependency;
mod cyclic_dependency_validator_exhausted;
mod cyclic_inheritance;
mod linearisation_impossible;
mod missing_override_specifier;
mod override_changes_modifier_signature;
mod override_changes_state_mutability;
mod override_parameter_location_differs;
mod override_return_location_differs;
mod override_return_types_differ;
mod override_visibility_differs;
mod overriding_non_virtual_member;
mod overriding_public_state_variable;
mod public_state_variable_overrides_non_external;
mod recursive_struct;
mod recursive_struct_validator_exhausted;
mod unimplemented_override_of_implemented;

pub use bytecode_dependency_validator_exhausted::BytecodeDependencyValidatorExhausted;
pub use cyclic_bytecode_dependency::CyclicBytecodeDependency;
pub use cyclic_constant_definition::CyclicConstantDefinition;
pub use cyclic_constant_dependency::CyclicConstantDependency;
pub use cyclic_dependency_validator_exhausted::CyclicDependencyValidatorExhausted;
pub use cyclic_inheritance::CyclicInheritance;
pub use linearisation_impossible::LinearisationImpossible;
pub use missing_override_specifier::MissingOverrideSpecifier;
pub use override_changes_modifier_signature::OverrideChangesModifierSignature;
pub use override_changes_state_mutability::OverrideChangesStateMutability;
pub use override_parameter_location_differs::OverrideParameterLocationDiffers;
pub use override_return_location_differs::OverrideReturnLocationDiffers;
pub use override_return_types_differ::OverrideReturnTypesDiffer;
pub use override_visibility_differs::OverrideVisibilityDiffers;
pub use overriding_non_virtual_member::OverridingNonVirtualMember;
pub use overriding_public_state_variable::OverridingPublicStateVariable;
pub use public_state_variable_overrides_non_external::PublicStateVariableOverridesNonExternal;
pub use recursive_struct::RecursiveStruct;
pub use recursive_struct_validator_exhausted::RecursiveStructValidatorExhausted;
use serde::Serialize;
pub use unimplemented_override_of_implemented::UnimplementedOverrideOfImplemented;

use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::utils::define_diagnostic_kind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Semantic;

    /// Group of diagnostics produced by semantic analysis.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum SemanticDiagnosticKind {
        /// Contract bytecode dependency graph traversal exceeded the depth
        /// limit.
        BytecodeDependencyValidatorExhausted(BytecodeDependencyValidatorExhausted),
        /// A contract references its own bytecode through a cycle of `new`
        /// or `type(...).creationCode` / `type(...).runtimeCode` uses.
        CyclicBytecodeDependency(CyclicBytecodeDependency),
        /// Compile-time constant evaluation hit a cycle or exceeded the
        /// recursion limit.
        CyclicConstantDefinition(CyclicConstantDefinition),
        /// A constant value depends on a cyclic chain of constants.
        CyclicConstantDependency(CyclicConstantDependency),
        /// Constant dependency graph traversal exceeded the depth limit.
        CyclicDependencyValidatorExhausted(CyclicDependencyValidatorExhausted),
        /// A contract's or interface's inheritance hierarchy contains a cycle.
        CyclicInheritance(CyclicInheritance),
        /// The inheritance hierarchy cannot be linearised into a consistent
        /// method resolution order.
        LinearisationImpossible(LinearisationImpossible),
        /// A struct from which a by-value cycle is reachable, so it would
        /// have infinite size.
        RecursiveStruct(RecursiveStruct),
        /// Recursive-struct detection gave up on a by-value path longer than
        /// its depth limit.
        RecursiveStructValidatorExhausted(RecursiveStructValidatorExhausted),

        /// A member overrides an inherited one without an `override` specifier.
        MissingOverrideSpecifier(MissingOverrideSpecifier),
        /// A member overrides an inherited `public` state variable.
        OverridingPublicStateVariable(OverridingPublicStateVariable),
        /// A member overrides an inherited function or modifier that is not `virtual`.
        OverridingNonVirtualMember(OverridingNonVirtualMember),
        /// A `public` state variable overrides a function that is not `external`.
        PublicStateVariableOverridesNonExternal(PublicStateVariableOverridesNonExternal),
        /// An overriding function or modifier changes the visibility of the
        /// overridden one.
        OverrideVisibilityDiffers(OverrideVisibilityDiffers),
        /// A function or modifier without a body overrides an implemented one.
        UnimplementedOverrideOfImplemented(UnimplementedOverrideOfImplemented),
        /// An overriding modifier declares different parameter types than the
        /// modifier it overrides.
        OverrideChangesModifierSignature(OverrideChangesModifierSignature),
        /// An overriding function or public state variable returns different
        /// types than the function it overrides.
        OverrideReturnTypesDiffer(OverrideReturnTypesDiffer),
        /// An overriding function changes the data location of a parameter of a
        /// non-`external` function.
        OverrideParameterLocationDiffers(OverrideParameterLocationDiffers),
        /// An overriding function changes the data location of a return variable
        /// of a non-`external` function.
        OverrideReturnLocationDiffers(OverrideReturnLocationDiffers),
        /// An overriding function or public state variable loosens the state
        /// mutability of the function it overrides.
        OverrideChangesStateMutability(OverrideChangesStateMutability),
    }
}
