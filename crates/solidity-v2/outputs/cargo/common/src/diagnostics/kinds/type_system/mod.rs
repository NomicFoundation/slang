mod array_length_fractional;
mod array_length_negative;
mod array_length_not_constant;
mod array_length_too_large;
mod array_length_zero;
mod cannot_call_via_contract_type_name;
mod constant_arithmetic_error;
mod expression_not_a_value;
mod expression_not_callable;
mod fallback_function_mutability;
mod fallback_function_signature;
mod incompatible_conditional_branches;
mod incompatible_constant_operator;
mod invalid_base;
mod literal_too_large;
mod partially_applied_function_used_as_value;
mod receive_function_parameters;
mod storage_layout_base_non_integer;
mod storage_layout_base_not_constant;
mod storage_layout_base_out_of_range;

pub use array_length_fractional::ArrayLengthFractional;
pub use array_length_negative::ArrayLengthNegative;
pub use array_length_not_constant::ArrayLengthNotConstant;
pub use array_length_too_large::ArrayLengthTooLarge;
pub use array_length_zero::ArrayLengthZero;
pub use cannot_call_via_contract_type_name::CannotCallViaContractTypeName;
pub use constant_arithmetic_error::ConstantArithmeticError;
pub use expression_not_a_value::{ExpressionNotAValue, NotAValueKind};
pub use expression_not_callable::ExpressionNotCallable;
pub use fallback_function_mutability::FallbackFunctionMutability;
pub use fallback_function_signature::FallbackFunctionSignature;
pub use incompatible_conditional_branches::IncompatibleConditionalBranches;
pub use incompatible_constant_operator::IncompatibleConstantOperator;
pub use invalid_base::InvalidBase;
pub use literal_too_large::LiteralTooLarge;
pub use partially_applied_function_used_as_value::PartiallyAppliedFunctionUsedAsValue;
pub use receive_function_parameters::ReceiveFunctionParameters;
use serde::Serialize;
pub use storage_layout_base_non_integer::StorageLayoutBaseNonInteger;
pub use storage_layout_base_not_constant::StorageLayoutBaseNotConstant;
pub use storage_layout_base_out_of_range::StorageLayoutBaseOutOfRange;

use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::utils::define_diagnostic_kind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::TypeSystem;

    /// Group of diagnostics about the type system — type mismatches, invalid
    /// conversions, operator type errors, function-call type mismatches, and ABI
    /// encoding constraints that are properties of a type.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum TypeSystemDiagnosticKind {
        /// A base in an inheritance list is not a contract or interface.
        InvalidBase(InvalidBase),

        /// A fallback function is declared `pure` or `view`.
        FallbackFunctionMutability(FallbackFunctionMutability),
        /// A fallback function has a signature other than the two accepted forms.
        FallbackFunctionSignature(FallbackFunctionSignature),
        /// A receive function declares parameters.
        ReceiveFunctionParameters(ReceiveFunctionParameters),
        /// A binary operator in a compile-time constant expression has no
        /// result type for its operand types.
        IncompatibleConstantOperator(IncompatibleConstantOperator),
        /// Compile-time constant evaluation produced a value that doesn't fit the
        /// result type of an arithmetic or bitwise operator.
        ConstantArithmeticError(ConstantArithmeticError),
        /// An array length expression is not a compile-time constant.
        ArrayLengthNotConstant(ArrayLengthNotConstant),
        /// An array length expression evaluates to zero.
        ArrayLengthZero(ArrayLengthZero),
        /// An array length expression evaluates to a fractional value.
        ArrayLengthFractional(ArrayLengthFractional),
        /// An array length expression evaluates to a negative value.
        ArrayLengthNegative(ArrayLengthNegative),
        /// An array length expression evaluates to a value larger than
        /// `2**256 - 1`.
        ArrayLengthTooLarge(ArrayLengthTooLarge),
        /// A storage layout base slot expression is not a compile-time constant.
        StorageLayoutBaseNotConstant(StorageLayoutBaseNotConstant),
        /// A storage layout base slot expression evaluates to a non-integer
        /// (e.g. fractional) value.
        StorageLayoutBaseNonInteger(StorageLayoutBaseNonInteger),
        /// A storage layout base slot expression evaluates to a value outside
        /// the range of `uint256`.
        StorageLayoutBaseOutOfRange(StorageLayoutBaseOutOfRange),
        /// A function is called through a contract/interface type name (eg.
        /// `C.f()`) rather than through an instance.
        CannotCallViaContractTypeName(CannotCallViaContractTypeName),
        /// The callee of a call is not callable.
        ExpressionNotCallable(ExpressionNotCallable),
        /// An expression in a position that requires a value names a built-in,
        /// `super`, or an uncalled `new` instead. Its `kind` says which.
        ExpressionNotAValue(ExpressionNotAValue),
        /// A partially applied function is used as a value rather than called.
        PartiallyAppliedFunctionUsedAsValue(PartiallyAppliedFunctionUsedAsValue),
        /// A number literal too large for any EVM type is used as a value.
        LiteralTooLarge(LiteralTooLarge),
        /// The two branches of a conditional have no common type.
        IncompatibleConditionalBranches(IncompatibleConditionalBranches),
    }
}
