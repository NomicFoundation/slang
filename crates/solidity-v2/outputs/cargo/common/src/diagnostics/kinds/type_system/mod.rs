mod fallback_function_mutability;
mod fallback_function_signature;
mod invalid_base;
mod invalid_function_type_visibility;

pub use fallback_function_mutability::FallbackFunctionMutability;
pub use fallback_function_signature::FallbackFunctionSignature;
pub use invalid_base::InvalidBase;
pub use invalid_function_type_visibility::InvalidFunctionTypeVisibility;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::TypeSystem;

    /// Group of diagnostics about the type system — type mismatches, invalid
    /// conversions, operator type errors, function-call type mismatches, and ABI
    /// encoding constraints that are properties of a type.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum TypeSystemDiagnosticKind {
        /// A base in an inheritance list is not a contract or interface.
        InvalidBase(InvalidBase),
        /// A function type has a visibility other than `internal` or `external`.
        InvalidFunctionTypeVisibility(InvalidFunctionTypeVisibility),

        /// A fallback function is declared `pure` or `view`.
        FallbackFunctionMutability(FallbackFunctionMutability),
        /// A fallback function has a signature other than the two accepted forms.
        FallbackFunctionSignature(FallbackFunctionSignature),
    }
}
