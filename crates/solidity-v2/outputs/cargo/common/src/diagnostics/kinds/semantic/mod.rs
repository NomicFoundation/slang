mod bytecode_dependency_validator_exhausted;
mod cyclic_bytecode_dependency;
mod cyclic_constant_definition;
mod cyclic_constant_dependency;
mod cyclic_dependency_validator_exhausted;
mod cyclic_inheritance;
mod linearisation_impossible;
mod recursive_struct;
mod recursive_struct_validator_exhausted;
mod yul_assignment_to_constant;
mod yul_assignment_to_non_variable;
mod yul_assignment_to_offset;
mod yul_assignment_to_state_variable;
mod yul_calldata_array_access;
mod yul_calldata_suffix;
mod yul_external_function_access;
mod yul_forward_referenced_constant;
mod yul_function_pointer_suffix;
mod yul_immutable_access;
mod yul_internal_function_pointer_suffix;
mod yul_multiple_suffixes;
mod yul_storage_suffix;
mod yul_storage_variable_access;
mod yul_suffix_on_constant;
mod yul_unsupported_constant;
mod yul_unsupported_reference;
mod yul_unsupported_suffix;

pub use bytecode_dependency_validator_exhausted::BytecodeDependencyValidatorExhausted;
pub use cyclic_bytecode_dependency::CyclicBytecodeDependency;
pub use cyclic_constant_definition::CyclicConstantDefinition;
pub use cyclic_constant_dependency::CyclicConstantDependency;
pub use cyclic_dependency_validator_exhausted::CyclicDependencyValidatorExhausted;
pub use cyclic_inheritance::CyclicInheritance;
pub use linearisation_impossible::LinearisationImpossible;
pub use recursive_struct::RecursiveStruct;
pub use recursive_struct_validator_exhausted::RecursiveStructValidatorExhausted;
use serde::Serialize;
pub use yul_assignment_to_constant::YulAssignmentToConstant;
pub use yul_assignment_to_non_variable::YulAssignmentToNonVariable;
pub use yul_assignment_to_offset::YulAssignmentToOffset;
pub use yul_assignment_to_state_variable::YulAssignmentToStateVariable;
pub use yul_calldata_array_access::YulCalldataArrayAccess;
pub use yul_calldata_suffix::YulCalldataSuffix;
pub use yul_external_function_access::YulExternalFunctionAccess;
pub use yul_forward_referenced_constant::YulForwardReferencedConstant;
pub use yul_function_pointer_suffix::YulFunctionPointerSuffix;
pub use yul_immutable_access::YulImmutableAccess;
pub use yul_internal_function_pointer_suffix::YulInternalFunctionPointerSuffix;
pub use yul_multiple_suffixes::YulMultipleSuffixes;
pub use yul_storage_suffix::YulStorageSuffix;
pub use yul_storage_variable_access::YulStorageVariableAccess;
pub use yul_suffix_on_constant::YulSuffixOnConstant;
pub use yul_unsupported_constant::YulUnsupportedConstant;
pub use yul_unsupported_reference::YulUnsupportedReference;
pub use yul_unsupported_suffix::YulUnsupportedSuffix;

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
        /// An assembly assignment targets a constant.
        YulAssignmentToConstant(YulAssignmentToConstant),
        /// An assembly assignment targets a declaration that is not a
        /// variable.
        YulAssignmentToNonVariable(YulAssignmentToNonVariable),
        /// An assembly assignment targets the `.offset` of a storage
        /// reference variable.
        YulAssignmentToOffset(YulAssignmentToOffset),
        /// An assembly assignment targets the `.slot` or `.offset` of a state
        /// variable.
        YulAssignmentToStateVariable(YulAssignmentToStateVariable),
        /// An assembly reference accesses a dynamic calldata array without a
        /// suffix.
        YulCalldataArrayAccess(YulCalldataArrayAccess),
        /// An assembly reference accesses a dynamic calldata array through a
        /// suffix other than `.offset` or `.length`.
        YulCalldataSuffix(YulCalldataSuffix),
        /// An assembly reference accesses an external function pointer
        /// without a suffix.
        YulExternalFunctionAccess(YulExternalFunctionAccess),
        /// An assembly reference reads a constant that is declared later in
        /// the same file and whose value is not a literal.
        YulForwardReferencedConstant(YulForwardReferencedConstant),
        /// An assembly reference accesses a function pointer through a suffix
        /// other than `.selector` or `.address`.
        YulFunctionPointerSuffix(YulFunctionPointerSuffix),
        /// An assembly reference accesses an immutable variable.
        YulImmutableAccess(YulImmutableAccess),
        /// An assembly reference accesses an internal function pointer
        /// through `.selector` or `.address`.
        YulInternalFunctionPointerSuffix(YulInternalFunctionPointerSuffix),
        /// An assembly path has more than one suffix.
        YulMultipleSuffixes(YulMultipleSuffixes),
        /// An assembly reference accesses a storage variable through a suffix
        /// other than `.slot` or `.offset`.
        YulStorageSuffix(YulStorageSuffix),
        /// An assembly reference accesses a storage variable without a
        /// suffix.
        YulStorageVariableAccess(YulStorageVariableAccess),
        /// An assembly reference accesses a constant through a suffix.
        YulSuffixOnConstant(YulSuffixOnConstant),
        /// An assembly reference reads a constant that is not a direct number
        /// constant.
        YulUnsupportedConstant(YulUnsupportedConstant),
        /// An assembly reference reads a declaration that is neither a
        /// variable nor a library.
        YulUnsupportedReference(YulUnsupportedReference),
        /// An assembly path item is not a supported suffix for the referenced
        /// declaration.
        YulUnsupportedSuffix(YulUnsupportedSuffix),
    }
}
