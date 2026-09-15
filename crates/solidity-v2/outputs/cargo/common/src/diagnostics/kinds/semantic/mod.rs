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
mod yul_forward_referenced_constant;
mod yul_suffix_on_constant;
mod yul_unsupported_constant;

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
pub use yul_forward_referenced_constant::YulForwardReferencedConstant;
pub use yul_suffix_on_constant::YulSuffixOnConstant;
pub use yul_unsupported_constant::YulUnsupportedConstant;

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
        /// An assembly reference reads a constant that is declared later in
        /// the same file and whose value is not a literal.
        YulForwardReferencedConstant(YulForwardReferencedConstant),
        /// An assembly reference accesses a constant through a suffix.
        YulSuffixOnConstant(YulSuffixOnConstant),
        /// An assembly reference reads a constant that is not a direct number
        /// constant.
        YulUnsupportedConstant(YulUnsupportedConstant),
    }
}
