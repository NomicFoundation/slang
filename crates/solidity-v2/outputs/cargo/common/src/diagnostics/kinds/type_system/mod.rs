mod array_length_fractional;
mod array_length_negative;
mod array_length_not_constant;
mod array_length_too_large;
mod array_length_zero;
mod call_options_already_set;
mod call_options_not_allowed;
mod cannot_call_via_contract_type_name;
mod constant_arithmetic_error;
mod contract_too_large_for_storage;
mod empty_array_literal;
mod encoded_arguments_not_inline_tuple;
mod explicit_conversion_not_allowed;
mod expression_not_a_value;
mod expression_not_an_lvalue;
mod expression_not_callable;
mod fallback_function_mutability;
mod fallback_function_signature;
mod incompatible_conditional_branches;
mod incompatible_constant_operator;
mod invalid_argument_type;
mod invalid_base;
mod invalid_decoded_types;
mod invalid_encoded_function;
mod invalid_utf8_string_literal;
mod literal_too_large;
mod modifier_outside_function_header;
mod named_arguments_in_conversion;
mod named_arguments_to_built_in_function;
mod partially_applied_function_used_as_value;
mod receive_function_parameters;
mod storage_layout_base_non_integer;
mod storage_layout_base_not_constant;
mod storage_layout_base_out_of_range;
mod storage_layout_base_past_end;
mod struct_with_mapping_constructed;
mod too_few_arguments;
mod type_too_large_for_storage;
mod unknown_named_argument;
mod write_to_constant;
mod wrong_argument_count;
mod wrong_argument_count_in_conversion;

pub use array_length_fractional::ArrayLengthFractional;
pub use array_length_negative::ArrayLengthNegative;
pub use array_length_not_constant::ArrayLengthNotConstant;
pub use array_length_too_large::ArrayLengthTooLarge;
pub use array_length_zero::ArrayLengthZero;
pub use call_options_already_set::CallOptionsAlreadySet;
pub use call_options_not_allowed::CallOptionsNotAllowed;
pub use cannot_call_via_contract_type_name::CannotCallViaContractTypeName;
pub use constant_arithmetic_error::ConstantArithmeticError;
pub use contract_too_large_for_storage::ContractTooLargeForStorage;
pub use empty_array_literal::EmptyArrayLiteral;
pub use encoded_arguments_not_inline_tuple::EncodedArgumentsNotInlineTuple;
pub use explicit_conversion_not_allowed::ExplicitConversionNotAllowed;
pub use expression_not_a_value::{ExpressionNotAValue, NotAValueKind};
pub use expression_not_an_lvalue::ExpressionNotAnLValue;
pub use expression_not_callable::ExpressionNotCallable;
pub use fallback_function_mutability::FallbackFunctionMutability;
pub use fallback_function_signature::FallbackFunctionSignature;
pub use incompatible_conditional_branches::IncompatibleConditionalBranches;
pub use incompatible_constant_operator::IncompatibleConstantOperator;
pub use invalid_argument_type::InvalidArgumentType;
pub use invalid_base::InvalidBase;
pub use invalid_decoded_types::InvalidDecodedTypes;
pub use invalid_encoded_function::InvalidEncodedFunction;
pub use invalid_utf8_string_literal::InvalidUtf8StringLiteral;
pub use literal_too_large::LiteralTooLarge;
pub use modifier_outside_function_header::ModifierOutsideFunctionHeader;
pub use named_arguments_in_conversion::NamedArgumentsInConversion;
pub use named_arguments_to_built_in_function::NamedArgumentsToBuiltInFunction;
pub use partially_applied_function_used_as_value::PartiallyAppliedFunctionUsedAsValue;
pub use receive_function_parameters::ReceiveFunctionParameters;
use serde::Serialize;
pub use storage_layout_base_non_integer::StorageLayoutBaseNonInteger;
pub use storage_layout_base_not_constant::StorageLayoutBaseNotConstant;
pub use storage_layout_base_out_of_range::StorageLayoutBaseOutOfRange;
pub use storage_layout_base_past_end::StorageLayoutBasePastEnd;
pub use struct_with_mapping_constructed::StructWithMappingConstructed;
pub use too_few_arguments::TooFewArguments;
pub use type_too_large_for_storage::TypeTooLargeForStorage;
pub use unknown_named_argument::UnknownNamedArgument;
pub use write_to_constant::WriteToConstant;
pub use wrong_argument_count::WrongArgumentCount;
pub use wrong_argument_count_in_conversion::WrongArgumentCountInConversion;

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
        /// A contract's state variables, laid out from its storage layout base
        /// slot, extend past the end of storage.
        StorageLayoutBasePastEnd(StorageLayoutBasePastEnd),
        /// A state variable stores a type that extends past the end of storage.
        TypeTooLargeForStorage(TypeTooLargeForStorage),
        /// A contract's state variables extend past the end of storage.
        ContractTooLargeForStorage(ContractTooLargeForStorage),
        /// A function is called through a contract/interface type name (eg.
        /// `C.f()`) rather than through an instance.
        CannotCallViaContractTypeName(CannotCallViaContractTypeName),
        /// The callee of a call is not callable.
        ExpressionNotCallable(ExpressionNotCallable),
        /// An expression references a modifier outside a function header.
        ModifierOutsideFunctionHeader(ModifierOutsideFunctionHeader),
        /// Call options are set on a function that is not called by a message
        /// call (eg. an internal or library function).
        CallOptionsNotAllowed(CallOptionsNotAllowed),
        /// Call options are set on an expression that already has them (eg.
        /// `c.f{value: 1}{gas: 2}`).
        CallOptionsAlreadySet(CallOptionsAlreadySet),
        /// A call passes a different number of arguments than the callee
        /// declares parameters.
        WrongArgumentCount(WrongArgumentCount),
        /// A call passes fewer arguments than the leading parameters of a
        /// callee that takes any number of arguments after them.
        TooFewArguments(TooFewArguments),
        /// An argument of a call does not implicitly convert to the type of its
        /// parameter.
        InvalidArgumentType(InvalidArgumentType),
        /// A named argument of a call names none of the callee's parameters.
        UnknownNamedArgument(UnknownNamedArgument),
        /// An explicit type conversion is not given exactly one argument.
        WrongArgumentCountInConversion(WrongArgumentCountInConversion),
        /// An explicit type conversion is given named arguments.
        NamedArgumentsInConversion(NamedArgumentsInConversion),
        /// A built-in function is given named arguments.
        NamedArgumentsToBuiltInFunction(NamedArgumentsToBuiltInFunction),
        /// The second argument of `abi.decode` is not a tuple of type names.
        InvalidDecodedTypes(InvalidDecodedTypes),
        /// The first argument of `abi.encodeCall` is not an external function.
        InvalidEncodedFunction(InvalidEncodedFunction),
        /// The second argument of `abi.encodeCall` has a tuple type without
        /// being a tuple expression.
        EncodedArgumentsNotInlineTuple(EncodedArgumentsNotInlineTuple),
        /// A struct containing a mapping is constructed.
        StructWithMappingConstructed(StructWithMappingConstructed),
        /// An expression in a position that requires a value names a built-in,
        /// `super`, or an uncalled `new` instead. Its `kind` says which.
        ExpressionNotAValue(ExpressionNotAValue),
        /// A partially applied function is used as a value rather than called.
        PartiallyAppliedFunctionUsedAsValue(PartiallyAppliedFunctionUsedAsValue),
        /// A number literal too large for any EVM type is used as a value.
        LiteralTooLarge(LiteralTooLarge),
        /// The two branches of a conditional have no common type.
        IncompatibleConditionalBranches(IncompatibleConditionalBranches),
        /// An array literal has no elements to deduce its type from.
        EmptyArrayLiteral(EmptyArrayLiteral),
        /// An expression in a position that is written to does not denote a
        /// writable storage location.
        ExpressionNotAnLValue(ExpressionNotAnLValue),
        /// A `constant` declaration is written to.
        WriteToConstant(WriteToConstant),
        /// An explicit type conversion is not allowed from the type of its
        /// argument to the target type.
        ExplicitConversionNotAllowed(ExplicitConversionNotAllowed),
        /// A string literal that is not valid UTF-8 is converted to `string`.
        InvalidUtf8StringLiteral(InvalidUtf8StringLiteral),
    }
}
