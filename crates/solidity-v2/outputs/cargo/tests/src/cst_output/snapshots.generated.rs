// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

mod assembly_statement {
    use super::*;

    #[test]
    fn simple() -> Result<()> {
        run("AssemblyStatement", "simple")
    }

    #[test]
    fn with_flags() -> Result<()> {
        run("AssemblyStatement", "with_flags")
    }
}

mod block {
    use super::*;

    #[test]
    fn postfix_recovery_regression() -> Result<()> {
        run("Block", "postfix_recovery_regression")
    }

    #[test]
    fn unchecked() -> Result<()> {
        run("Block", "unchecked")
    }
}

mod break_statement {
    use super::*;

    #[test]
    fn error_recovery() -> Result<()> {
        run("BreakStatement", "error_recovery")
    }

    #[test]
    fn valid() -> Result<()> {
        run("BreakStatement", "valid")
    }
}

mod conditional_expression {
    use super::*;

    #[test]
    fn identifier_base() -> Result<()> {
        run("ConditionalExpression", "identifier_base")
    }

    #[test]
    fn nested_base() -> Result<()> {
        run("ConditionalExpression", "nested_base")
    }

    #[test]
    fn nested_conditions() -> Result<()> {
        run("ConditionalExpression", "nested_conditions")
    }

    #[test]
    fn recursive() -> Result<()> {
        run("ConditionalExpression", "recursive")
    }
}

mod constant_definition {
    use super::*;

    #[test]
    fn int() -> Result<()> {
        run("ConstantDefinition", "int")
    }
}

mod constructor_definition {
    use super::*;

    #[test]
    fn override_attribute() -> Result<()> {
        run("ConstructorDefinition", "override_attribute")
    }

    #[test]
    fn simple() -> Result<()> {
        run("ConstructorDefinition", "simple")
    }

    #[test]
    fn virtual_attribute() -> Result<()> {
        run("ConstructorDefinition", "virtual_attribute")
    }
}

mod contract_definition {
    use super::*;

    #[test]
    fn abstract_contract() -> Result<()> {
        run("ContractDefinition", "abstract_contract")
    }

    #[test]
    fn constructor_contextual() -> Result<()> {
        run("ContractDefinition", "constructor_contextual")
    }

    #[test]
    fn emit_contextual() -> Result<()> {
        run("ContractDefinition", "emit_contextual")
    }

    #[test]
    fn empty_contract() -> Result<()> {
        run("ContractDefinition", "empty_contract")
    }

    #[test]
    fn function_multiple_delimiters() -> Result<()> {
        run("ContractDefinition", "function_multiple_delimiters")
    }

    #[test]
    fn header_comment() -> Result<()> {
        run("ContractDefinition", "header_comment")
    }

    #[test]
    fn inheritance_specifier() -> Result<()> {
        run("ContractDefinition", "inheritance_specifier")
    }

    #[test]
    fn member_constructor_definition() -> Result<()> {
        run("ContractDefinition", "member_constructor_definition")
    }

    #[test]
    fn member_enum_definition() -> Result<()> {
        run("ContractDefinition", "member_enum_definition")
    }

    #[test]
    fn member_error_definition() -> Result<()> {
        run("ContractDefinition", "member_error_definition")
    }

    #[test]
    fn member_event_definition() -> Result<()> {
        run("ContractDefinition", "member_event_definition")
    }

    #[test]
    fn member_fallback_function_definition() -> Result<()> {
        run("ContractDefinition", "member_fallback_function_definition")
    }

    #[test]
    fn member_function_definition() -> Result<()> {
        run("ContractDefinition", "member_function_definition")
    }

    #[test]
    fn member_modifier_definition() -> Result<()> {
        run("ContractDefinition", "member_modifier_definition")
    }

    #[test]
    fn member_receive_function_definition() -> Result<()> {
        run("ContractDefinition", "member_receive_function_definition")
    }

    #[test]
    fn member_state_variable_declaration() -> Result<()> {
        run("ContractDefinition", "member_state_variable_declaration")
    }

    #[test]
    fn member_struct_definition() -> Result<()> {
        run("ContractDefinition", "member_struct_definition")
    }

    #[test]
    fn member_unnamed_function_definition() -> Result<()> {
        run("ContractDefinition", "member_unnamed_function_definition")
    }

    #[test]
    fn member_unnamed_function_with_attrs_definition() -> Result<()> {
        run(
            "ContractDefinition",
            "member_unnamed_function_with_attrs_definition",
        )
    }

    #[test]
    fn member_user_defined_value_type_definition() -> Result<()> {
        run(
            "ContractDefinition",
            "member_user_defined_value_type_definition",
        )
    }

    #[test]
    fn member_using_directive() -> Result<()> {
        run("ContractDefinition", "member_using_directive")
    }

    #[test]
    fn missing_field_type() -> Result<()> {
        run("ContractDefinition", "missing_field_type")
    }

    #[test]
    fn recovery_testbed() -> Result<()> {
        run("ContractDefinition", "recovery_testbed")
    }

    #[test]
    fn storage_specifier_after_inheritance() -> Result<()> {
        run("ContractDefinition", "storage_specifier_after_inheritance")
    }

    #[test]
    fn storage_specifier_before_inheritance() -> Result<()> {
        run("ContractDefinition", "storage_specifier_before_inheritance")
    }

    #[test]
    fn storage_specifier_only() -> Result<()> {
        run("ContractDefinition", "storage_specifier_only")
    }

    #[test]
    fn unicode_in_doc_comments() -> Result<()> {
        run("ContractDefinition", "unicode_in_doc_comments")
    }

    #[test]
    fn unterminated_body() -> Result<()> {
        run("ContractDefinition", "unterminated_body")
    }

    #[test]
    fn zero_length_input() -> Result<()> {
        run("ContractDefinition", "zero_length_input")
    }
}

mod contract_members {
    use super::*;

    #[test]
    fn constructor() -> Result<()> {
        run("ContractMembers", "constructor")
    }

    #[test]
    fn local_expression() -> Result<()> {
        run("ContractMembers", "local_expression")
    }

    #[test]
    fn mismatched_delimiter() -> Result<()> {
        run("ContractMembers", "mismatched_delimiter")
    }

    #[test]
    fn separated_recovery() -> Result<()> {
        run("ContractMembers", "separated_recovery")
    }

    #[test]
    fn state_variable_constant_function() -> Result<()> {
        run("ContractMembers", "state_variable_constant_function")
    }

    #[test]
    fn state_variable_pure_function() -> Result<()> {
        run("ContractMembers", "state_variable_pure_function")
    }

    #[test]
    fn state_variable_view_function() -> Result<()> {
        run("ContractMembers", "state_variable_view_function")
    }
}

mod decimal_number_expression {
    use super::*;

    #[test]
    fn bare_period() -> Result<()> {
        run("DecimalNumberExpression", "bare_period")
    }

    #[test]
    fn days_unit() -> Result<()> {
        run("DecimalNumberExpression", "days_unit")
    }

    #[test]
    fn decimal_eof() -> Result<()> {
        run("DecimalNumberExpression", "decimal_eof")
    }

    #[test]
    fn decimal_no_unit() -> Result<()> {
        run("DecimalNumberExpression", "decimal_no_unit")
    }

    #[test]
    fn decimal_trailing_dollar() -> Result<()> {
        run("DecimalNumberExpression", "decimal_trailing_dollar")
    }

    #[test]
    fn decimal_trailing_ident_start() -> Result<()> {
        run("DecimalNumberExpression", "decimal_trailing_ident_start")
    }

    #[test]
    fn decimal_trailing_unit() -> Result<()> {
        run("DecimalNumberExpression", "decimal_trailing_unit")
    }

    #[test]
    fn ether_unit() -> Result<()> {
        run("DecimalNumberExpression", "ether_unit")
    }

    #[test]
    fn exponent_trailing_ident_start() -> Result<()> {
        run("DecimalNumberExpression", "exponent_trailing_ident_start")
    }

    #[test]
    fn float() -> Result<()> {
        run("DecimalNumberExpression", "float")
    }

    #[test]
    fn float_ident_after_period() -> Result<()> {
        run("DecimalNumberExpression", "float_ident_after_period")
    }

    #[test]
    fn float_no_fraction() -> Result<()> {
        run("DecimalNumberExpression", "float_no_fraction")
    }

    #[test]
    fn float_no_mantissa() -> Result<()> {
        run("DecimalNumberExpression", "float_no_mantissa")
    }

    #[test]
    fn float_with_exponent() -> Result<()> {
        run("DecimalNumberExpression", "float_with_exponent")
    }

    #[test]
    fn float_with_negative_exponent() -> Result<()> {
        run("DecimalNumberExpression", "float_with_negative_exponent")
    }

    #[test]
    fn integer() -> Result<()> {
        run("DecimalNumberExpression", "integer")
    }

    #[test]
    fn integer_ident_after_period() -> Result<()> {
        run("DecimalNumberExpression", "integer_ident_after_period")
    }

    #[test]
    fn integer_period_ident_exponent_like() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "integer_period_ident_exponent_like",
        )
    }

    #[test]
    fn integer_period_ident_negative_exponent_like() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "integer_period_ident_negative_exponent_like",
        )
    }

    #[test]
    fn integer_with_exponent() -> Result<()> {
        run("DecimalNumberExpression", "integer_with_exponent")
    }

    #[test]
    fn integer_with_negative_exponent() -> Result<()> {
        run("DecimalNumberExpression", "integer_with_negative_exponent")
    }

    #[test]
    fn integer_with_separators() -> Result<()> {
        run("DecimalNumberExpression", "integer_with_separators")
    }

    #[test]
    fn leading_period_ident_after_decimal() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "leading_period_ident_after_decimal",
        )
    }

    #[test]
    fn leading_period_ident_after_period() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "leading_period_ident_after_period",
        )
    }

    #[test]
    fn leading_period_ident_exponent_like() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "leading_period_ident_exponent_like",
        )
    }

    #[test]
    fn leading_period_ident_negative_exponent_like() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "leading_period_ident_negative_exponent_like",
        )
    }

    #[test]
    fn leading_period_with_exponent() -> Result<()> {
        run("DecimalNumberExpression", "leading_period_with_exponent")
    }

    #[test]
    fn leading_period_with_negative_exponent() -> Result<()> {
        run(
            "DecimalNumberExpression",
            "leading_period_with_negative_exponent",
        )
    }

    #[test]
    fn years_unit() -> Result<()> {
        run("DecimalNumberExpression", "years_unit")
    }
}

mod enum_definition {
    use super::*;

    #[test]
    fn multiple_members() -> Result<()> {
        run("EnumDefinition", "multiple_members")
    }

    #[test]
    fn no_members() -> Result<()> {
        run("EnumDefinition", "no_members")
    }
}

mod error_definition {
    use super::*;

    #[test]
    fn top_level() -> Result<()> {
        run("ErrorDefinition", "top_level")
    }
}

mod event_definition {
    use super::*;

    #[test]
    fn no_parens() -> Result<()> {
        run("EventDefinition", "no_parens")
    }

    #[test]
    fn transfer() -> Result<()> {
        run("EventDefinition", "transfer")
    }
}

mod exponentiation_expression {
    use super::*;

    #[test]
    fn associativity() -> Result<()> {
        run("ExponentiationExpression", "associativity")
    }
}

mod expression {
    use super::*;

    #[test]
    fn _0() -> Result<()> {
        run("Expression", "_0")
    }

    #[test]
    fn _a() -> Result<()> {
        run("Expression", "_a")
    }

    #[test]
    fn add_mul() -> Result<()> {
        run("Expression", "add_mul")
    }

    #[test]
    fn address_call() -> Result<()> {
        run("Expression", "address_call")
    }

    #[test]
    fn address_payable_call() -> Result<()> {
        run("Expression", "address_payable_call")
    }

    #[test]
    fn areturn() -> Result<()> {
        run("Expression", "areturn")
    }

    #[test]
    fn chained_assignment() -> Result<()> {
        run("Expression", "chained_assignment")
    }

    #[test]
    fn condition_and_assignment_first_position() -> Result<()> {
        run("Expression", "condition_and_assignment_first_position")
    }

    #[test]
    fn condition_and_assignment_second_position() -> Result<()> {
        run("Expression", "condition_and_assignment_second_position")
    }

    #[test]
    fn condition_and_assignment_third_position() -> Result<()> {
        run("Expression", "condition_and_assignment_third_position")
    }

    #[test]
    fn delete() -> Result<()> {
        run("Expression", "delete")
    }

    #[test]
    fn delete_conditional() -> Result<()> {
        run("Expression", "delete_conditional")
    }

    #[test]
    fn elementary_type_under_call() -> Result<()> {
        run("Expression", "elementary_type_under_call")
    }

    #[test]
    fn function_call() -> Result<()> {
        run("Expression", "function_call")
    }

    #[test]
    fn function_call_argument_has_type_name_as_prefix() -> Result<()> {
        run(
            "Expression",
            "function_call_argument_has_type_name_as_prefix",
        )
    }

    #[test]
    fn function_call_chain() -> Result<()> {
        run("Expression", "function_call_chain")
    }

    #[test]
    fn function_call_member_access() -> Result<()> {
        run("Expression", "function_call_member_access")
    }

    #[test]
    fn function_call_options() -> Result<()> {
        run("Expression", "function_call_options")
    }

    #[test]
    fn function_call_options_split() -> Result<()> {
        run("Expression", "function_call_options_split")
    }

    #[test]
    fn identifier_call() -> Result<()> {
        run("Expression", "identifier_call")
    }

    #[test]
    fn incomplete_operand() -> Result<()> {
        run("Expression", "incomplete_operand")
    }

    #[test]
    fn index_access() -> Result<()> {
        run("Expression", "index_access")
    }

    #[test]
    fn index_access_chain() -> Result<()> {
        run("Expression", "index_access_chain")
    }

    #[test]
    fn index_slice_end() -> Result<()> {
        run("Expression", "index_slice_end")
    }

    #[test]
    fn index_slice_start() -> Result<()> {
        run("Expression", "index_slice_start")
    }

    #[test]
    fn index_slice_start_end() -> Result<()> {
        run("Expression", "index_slice_start_end")
    }

    #[test]
    fn index_slice_unbounded() -> Result<()> {
        run("Expression", "index_slice_unbounded")
    }

    #[test]
    fn keyword_alias() -> Result<()> {
        run("Expression", "keyword_alias")
    }

    #[test]
    fn keyword_apply() -> Result<()> {
        run("Expression", "keyword_apply")
    }

    #[test]
    fn keyword_auto() -> Result<()> {
        run("Expression", "keyword_auto")
    }

    #[test]
    fn keyword_calldata() -> Result<()> {
        run("Expression", "keyword_calldata")
    }

    #[test]
    fn keyword_constructor() -> Result<()> {
        run("Expression", "keyword_constructor")
    }

    #[test]
    fn keyword_copyof() -> Result<()> {
        run("Expression", "keyword_copyof")
    }

    #[test]
    fn keyword_define() -> Result<()> {
        run("Expression", "keyword_define")
    }

    #[test]
    fn keyword_emit() -> Result<()> {
        run("Expression", "keyword_emit")
    }

    #[test]
    fn keyword_fallback() -> Result<()> {
        run("Expression", "keyword_fallback")
    }

    #[test]
    fn keyword_finney() -> Result<()> {
        run("Expression", "keyword_finney")
    }

    #[test]
    fn keyword_immutable() -> Result<()> {
        run("Expression", "keyword_immutable")
    }

    #[test]
    fn keyword_implements() -> Result<()> {
        run("Expression", "keyword_implements")
    }

    #[test]
    fn keyword_macro() -> Result<()> {
        run("Expression", "keyword_macro")
    }

    #[test]
    fn keyword_mutable() -> Result<()> {
        run("Expression", "keyword_mutable")
    }

    #[test]
    fn keyword_override() -> Result<()> {
        run("Expression", "keyword_override")
    }

    #[test]
    fn keyword_partial() -> Result<()> {
        run("Expression", "keyword_partial")
    }

    #[test]
    fn keyword_promise() -> Result<()> {
        run("Expression", "keyword_promise")
    }

    #[test]
    fn keyword_receive() -> Result<()> {
        run("Expression", "keyword_receive")
    }

    #[test]
    fn keyword_reference() -> Result<()> {
        run("Expression", "keyword_reference")
    }

    #[test]
    fn keyword_sealed() -> Result<()> {
        run("Expression", "keyword_sealed")
    }

    #[test]
    fn keyword_sizeof() -> Result<()> {
        run("Expression", "keyword_sizeof")
    }

    #[test]
    fn keyword_supports() -> Result<()> {
        run("Expression", "keyword_supports")
    }

    #[test]
    fn keyword_szabo() -> Result<()> {
        run("Expression", "keyword_szabo")
    }

    #[test]
    fn keyword_typedef() -> Result<()> {
        run("Expression", "keyword_typedef")
    }

    #[test]
    fn keyword_ufixed() -> Result<()> {
        run("Expression", "keyword_ufixed")
    }

    #[test]
    fn keyword_unchecked() -> Result<()> {
        run("Expression", "keyword_unchecked")
    }

    #[test]
    fn keyword_virtual() -> Result<()> {
        run("Expression", "keyword_virtual")
    }

    #[test]
    fn member_access() -> Result<()> {
        run("Expression", "member_access")
    }

    #[test]
    fn member_access_address() -> Result<()> {
        run("Expression", "member_access_address")
    }

    #[test]
    fn member_access_addresses() -> Result<()> {
        run("Expression", "member_access_addresses")
    }

    #[test]
    fn member_access_chain() -> Result<()> {
        run("Expression", "member_access_chain")
    }

    #[test]
    fn member_access_chain_access() -> Result<()> {
        run("Expression", "member_access_chain_access")
    }

    #[test]
    fn member_access_function_call() -> Result<()> {
        run("Expression", "member_access_function_call")
    }

    #[test]
    fn member_access_index_access() -> Result<()> {
        run("Expression", "member_access_index_access")
    }

    #[test]
    fn member_access_integer() -> Result<()> {
        run("Expression", "member_access_integer")
    }

    #[test]
    fn member_access_options() -> Result<()> {
        run("Expression", "member_access_options")
    }

    #[test]
    fn member_access_rational() -> Result<()> {
        run("Expression", "member_access_rational")
    }

    #[test]
    fn member_access_rational_leading_period() -> Result<()> {
        run("Expression", "member_access_rational_leading_period")
    }

    #[test]
    fn member_access_super() -> Result<()> {
        run("Expression", "member_access_super")
    }

    #[test]
    fn member_access_this() -> Result<()> {
        run("Expression", "member_access_this")
    }

    #[test]
    fn mixed_compound_assignment() -> Result<()> {
        run("Expression", "mixed_compound_assignment")
    }

    #[test]
    fn new_expression() -> Result<()> {
        run("Expression", "new_expression")
    }

    #[test]
    fn new_expression_chain() -> Result<()> {
        run("Expression", "new_expression_chain")
    }

    #[test]
    fn new_expression_options() -> Result<()> {
        run("Expression", "new_expression_options")
    }

    #[test]
    fn overlapping_operators() -> Result<()> {
        run("Expression", "overlapping_operators")
    }

    #[test]
    fn paren_expression_options() -> Result<()> {
        run("Expression", "paren_expression_options")
    }

    #[test]
    fn postfix_decrement() -> Result<()> {
        run("Expression", "postfix_decrement")
    }

    #[test]
    fn prefix_decrement() -> Result<()> {
        run("Expression", "prefix_decrement")
    }

    #[test]
    fn prefix_minus() -> Result<()> {
        run("Expression", "prefix_minus")
    }

    #[test]
    fn prefix_plus() -> Result<()> {
        run("Expression", "prefix_plus")
    }

    #[test]
    fn returna() -> Result<()> {
        run("Expression", "returna")
    }

    #[test]
    fn returns() -> Result<()> {
        run("Expression", "returns")
    }

    #[test]
    fn two_conditions_false_case() -> Result<()> {
        run("Expression", "two_conditions_false_case")
    }

    #[test]
    fn two_conditions_true_case() -> Result<()> {
        run("Expression", "two_conditions_true_case")
    }

    #[test]
    fn underscore_is_identifier() -> Result<()> {
        run("Expression", "underscore_is_identifier")
    }

    #[test]
    fn unicode_string_literal() -> Result<()> {
        run("Expression", "unicode_string_literal")
    }
}

mod fallback_function_definition {
    use super::*;

    #[test]
    fn simple() -> Result<()> {
        run("FallbackFunctionDefinition", "simple")
    }
}

mod function_call_expression {
    use super::*;

    #[test]
    fn empty_named_arguments() -> Result<()> {
        run("FunctionCallExpression", "empty_named_arguments")
    }

    #[test]
    fn payable_conversion() -> Result<()> {
        run("FunctionCallExpression", "payable_conversion")
    }
}

mod function_definition {
    use super::*;

    #[test]
    fn constant_state_mutability() -> Result<()> {
        run("FunctionDefinition", "constant_state_mutability")
    }

    #[test]
    fn from_contextual_keyword() -> Result<()> {
        run("FunctionDefinition", "from_contextual_keyword")
    }

    #[test]
    fn overridden() -> Result<()> {
        run("FunctionDefinition", "overridden")
    }

    #[test]
    fn pure_state_mutability() -> Result<()> {
        run("FunctionDefinition", "pure_state_mutability")
    }
}

mod function_type {
    use super::*;

    #[test]
    fn basic() -> Result<()> {
        run("FunctionType", "basic")
    }

    #[test]
    fn constant_state_mutability() -> Result<()> {
        run("FunctionType", "constant_state_mutability")
    }

    #[test]
    fn pure_state_mutability() -> Result<()> {
        run("FunctionType", "pure_state_mutability")
    }
}

mod hex_number_expression {
    use super::*;

    #[test]
    fn hex_consecutive_underscores() -> Result<()> {
        run("HexNumberExpression", "hex_consecutive_underscores")
    }

    #[test]
    fn hex_invalid_alpha_digit() -> Result<()> {
        run("HexNumberExpression", "hex_invalid_alpha_digit")
    }

    #[test]
    fn hex_leading_underscore() -> Result<()> {
        run("HexNumberExpression", "hex_leading_underscore")
    }

    #[test]
    fn hex_multiple_digits() -> Result<()> {
        run("HexNumberExpression", "hex_multiple_digits")
    }

    #[test]
    fn hex_no_digits() -> Result<()> {
        run("HexNumberExpression", "hex_no_digits")
    }

    #[test]
    fn hex_no_unit() -> Result<()> {
        run("HexNumberExpression", "hex_no_unit")
    }

    #[test]
    fn hex_trailing_ident_start() -> Result<()> {
        run("HexNumberExpression", "hex_trailing_ident_start")
    }

    #[test]
    fn hex_trailing_letter() -> Result<()> {
        run("HexNumberExpression", "hex_trailing_letter")
    }

    #[test]
    fn hex_trailing_underscore() -> Result<()> {
        run("HexNumberExpression", "hex_trailing_underscore")
    }

    #[test]
    fn hex_trailing_unit() -> Result<()> {
        run("HexNumberExpression", "hex_trailing_unit")
    }

    #[test]
    fn hex_unit() -> Result<()> {
        run("HexNumberExpression", "hex_unit")
    }

    #[test]
    fn hex_uppercase_prefix() -> Result<()> {
        run("HexNumberExpression", "hex_uppercase_prefix")
    }

    #[test]
    fn hex_with_underscores() -> Result<()> {
        run("HexNumberExpression", "hex_with_underscores")
    }
}

mod hex_string_literals {
    use super::*;

    #[test]
    fn all_separated_pairs() -> Result<()> {
        run("HexStringLiterals", "all_separated_pairs")
    }

    #[test]
    fn invalid_consecutive_separators() -> Result<()> {
        run("HexStringLiterals", "invalid_consecutive_separators")
    }

    #[test]
    fn invalid_leading_separator() -> Result<()> {
        run("HexStringLiterals", "invalid_leading_separator")
    }

    #[test]
    fn invalid_separator_after_single_char() -> Result<()> {
        run("HexStringLiterals", "invalid_separator_after_single_char")
    }

    #[test]
    fn invalid_trailing_separator() -> Result<()> {
        run("HexStringLiterals", "invalid_trailing_separator")
    }

    #[test]
    fn multiple() -> Result<()> {
        run("HexStringLiterals", "multiple")
    }

    #[test]
    fn no_separators() -> Result<()> {
        run("HexStringLiterals", "no_separators")
    }

    #[test]
    fn single() -> Result<()> {
        run("HexStringLiterals", "single")
    }

    #[test]
    fn single_trailing_ident() -> Result<()> {
        run("HexStringLiterals", "single_trailing_ident")
    }

    #[test]
    fn some_separated_pairs() -> Result<()> {
        run("HexStringLiterals", "some_separated_pairs")
    }
}

mod import_directive {
    use super::*;

    #[test]
    fn destructure_import_empty() -> Result<()> {
        run("ImportDirective", "destructure_import_empty")
    }

    #[test]
    fn destructure_import_multiple() -> Result<()> {
        run("ImportDirective", "destructure_import_multiple")
    }

    #[test]
    fn destructure_import_single() -> Result<()> {
        run("ImportDirective", "destructure_import_single")
    }

    #[test]
    fn named_import() -> Result<()> {
        run("ImportDirective", "named_import")
    }

    #[test]
    fn path_import() -> Result<()> {
        run("ImportDirective", "path_import")
    }

    #[test]
    fn path_import_with_alias() -> Result<()> {
        run("ImportDirective", "path_import_with_alias")
    }
}

mod interface_definition {
    use super::*;

    #[test]
    fn sample_counter() -> Result<()> {
        run("InterfaceDefinition", "sample_counter")
    }
}

mod mapping_type {
    use super::*;

    #[test]
    fn named_both() -> Result<()> {
        run("MappingType", "named_both")
    }

    #[test]
    fn named_key() -> Result<()> {
        run("MappingType", "named_key")
    }

    #[test]
    fn named_value() -> Result<()> {
        run("MappingType", "named_value")
    }

    #[test]
    fn stray_delimiter() -> Result<()> {
        run("MappingType", "stray_delimiter")
    }

    #[test]
    fn unnamed() -> Result<()> {
        run("MappingType", "unnamed")
    }
}

mod modifier_definition {
    use super::*;

    #[test]
    fn override_attr() -> Result<()> {
        run("ModifierDefinition", "override_attr")
    }
}

mod new_expression {
    use super::*;

    #[test]
    fn array_1d() -> Result<()> {
        run("NewExpression", "array_1d")
    }

    #[test]
    fn array_1d_expression() -> Result<()> {
        run("NewExpression", "array_1d_expression")
    }

    #[test]
    fn array_2d() -> Result<()> {
        run("NewExpression", "array_2d")
    }

    #[test]
    fn identifier_path() -> Result<()> {
        run("NewExpression", "identifier_path")
    }
}

mod pragma_directive {
    use super::*;

    #[test]
    fn abi_coder_unkown_identifier() -> Result<()> {
        run("PragmaDirective", "abi_coder_unkown_identifier")
    }

    #[test]
    fn abi_coder_v1() -> Result<()> {
        run("PragmaDirective", "abi_coder_v1")
    }

    #[test]
    fn abi_coder_v2() -> Result<()> {
        run("PragmaDirective", "abi_coder_v2")
    }

    #[test]
    fn experimental_abiencoderv2() -> Result<()> {
        run("PragmaDirective", "experimental_abiencoderv2")
    }

    #[test]
    fn experimental_smtchecker() -> Result<()> {
        run("PragmaDirective", "experimental_smtchecker")
    }

    #[test]
    fn experimental_string() -> Result<()> {
        run("PragmaDirective", "experimental_string")
    }

    #[test]
    fn experimental_string_version() -> Result<()> {
        run("PragmaDirective", "experimental_string_version")
    }

    #[test]
    fn experimental_unknown_identifier() -> Result<()> {
        run("PragmaDirective", "experimental_unknown_identifier")
    }

    #[test]
    fn version() -> Result<()> {
        run("PragmaDirective", "version")
    }

    #[test]
    fn version_string() -> Result<()> {
        run("PragmaDirective", "version_string")
    }
}

mod receive_function_definition {
    use super::*;

    #[test]
    fn simple() -> Result<()> {
        run("ReceiveFunctionDefinition", "simple")
    }
}

mod return_statement {
    use super::*;

    #[test]
    fn invalid_terminator() -> Result<()> {
        run("ReturnStatement", "invalid_terminator")
    }
}

mod source_unit {
    use super::*;

    #[test]
    fn adjacent_multiline_comments() -> Result<()> {
        run("SourceUnit", "adjacent_multiline_comments")
    }

    #[test]
    fn adjacent_natspec_comments() -> Result<()> {
        run("SourceUnit", "adjacent_natspec_comments")
    }

    #[test]
    fn braces_inside_assembly() -> Result<()> {
        run("SourceUnit", "braces_inside_assembly")
    }

    #[test]
    fn empty_file() -> Result<()> {
        run("SourceUnit", "empty_file")
    }

    #[test]
    fn empty_multiline_comment_1_asterisk() -> Result<()> {
        run("SourceUnit", "empty_multiline_comment_1_asterisk")
    }

    #[test]
    fn empty_multiline_comment_2_asterisks() -> Result<()> {
        run("SourceUnit", "empty_multiline_comment_2_asterisks")
    }

    #[test]
    fn empty_multiline_comment_3_asterisks() -> Result<()> {
        run("SourceUnit", "empty_multiline_comment_3_asterisks")
    }

    #[test]
    fn empty_multiline_comment_4_asterisks() -> Result<()> {
        run("SourceUnit", "empty_multiline_comment_4_asterisks")
    }

    #[test]
    fn empty_multiline_over_natspec() -> Result<()> {
        run("SourceUnit", "empty_multiline_over_natspec")
    }

    #[test]
    fn end_of_file_trivia() -> Result<()> {
        run("SourceUnit", "end_of_file_trivia")
    }

    #[test]
    fn end_of_file_trivia_incomplete() -> Result<()> {
        run("SourceUnit", "end_of_file_trivia_incomplete")
    }

    #[test]
    fn end_of_file_trivia_incomplete_natspec() -> Result<()> {
        run("SourceUnit", "end_of_file_trivia_incomplete_natspec")
    }

    #[test]
    fn end_of_file_trivia_unexpected_after() -> Result<()> {
        run("SourceUnit", "end_of_file_trivia_unexpected_after")
    }

    #[test]
    fn error_definition() -> Result<()> {
        run("SourceUnit", "error_definition")
    }

    #[test]
    fn everything() -> Result<()> {
        run("SourceUnit", "everything")
    }

    #[test]
    fn layout_at() -> Result<()> {
        run("SourceUnit", "layout_at")
    }

    #[test]
    fn leading_trivia_multi_line() -> Result<()> {
        run("SourceUnit", "leading_trivia_multi_line")
    }

    #[test]
    fn leading_trivia_multi_line_natspec_comment() -> Result<()> {
        run("SourceUnit", "leading_trivia_multi_line_natspec_comment")
    }

    #[test]
    fn leading_trivia_multi_line_trailing_double_star() -> Result<()> {
        run(
            "SourceUnit",
            "leading_trivia_multi_line_trailing_double_star",
        )
    }

    #[test]
    fn leading_trivia_new_line() -> Result<()> {
        run("SourceUnit", "leading_trivia_new_line")
    }

    #[test]
    fn leading_trivia_single_line_comment() -> Result<()> {
        run("SourceUnit", "leading_trivia_single_line_comment")
    }

    #[test]
    fn leading_trivia_single_line_comment_empty() -> Result<()> {
        run("SourceUnit", "leading_trivia_single_line_comment_empty")
    }

    #[test]
    fn leading_trivia_single_line_natspec_comment() -> Result<()> {
        run("SourceUnit", "leading_trivia_single_line_natspec_comment")
    }

    #[test]
    fn leading_trivia_single_line_natspec_comment_empty() -> Result<()> {
        run(
            "SourceUnit",
            "leading_trivia_single_line_natspec_comment_empty",
        )
    }

    #[test]
    fn leading_trivia_whitespace() -> Result<()> {
        run("SourceUnit", "leading_trivia_whitespace")
    }

    #[test]
    fn partial_definition() -> Result<()> {
        run("SourceUnit", "partial_definition")
    }

    #[test]
    fn pratt_precedence_recovery() -> Result<()> {
        run("SourceUnit", "pratt_precedence_recovery")
    }

    #[test]
    fn revert_statement() -> Result<()> {
        run("SourceUnit", "revert_statement")
    }

    #[test]
    fn safe_math() -> Result<()> {
        run("SourceUnit", "safe_math")
    }

    #[test]
    fn state_variable_function() -> Result<()> {
        run("SourceUnit", "state_variable_function")
    }

    #[test]
    fn top_level_event() -> Result<()> {
        run("SourceUnit", "top_level_event")
    }

    #[test]
    fn top_level_function() -> Result<()> {
        run("SourceUnit", "top_level_function")
    }

    #[test]
    fn trailing_trivia() -> Result<()> {
        run("SourceUnit", "trailing_trivia")
    }

    #[test]
    fn trailing_trivia_multi_line_spanning_multiple_lines() -> Result<()> {
        run(
            "SourceUnit",
            "trailing_trivia_multi_line_spanning_multiple_lines",
        )
    }

    #[test]
    fn trailing_trivia_multi_line_without_newline() -> Result<()> {
        run("SourceUnit", "trailing_trivia_multi_line_without_newline")
    }

    #[test]
    fn trailing_trivia_only_until_newline() -> Result<()> {
        run("SourceUnit", "trailing_trivia_only_until_newline")
    }

    #[test]
    fn try_expression() -> Result<()> {
        run("SourceUnit", "try_expression")
    }

    #[test]
    fn try_options() -> Result<()> {
        run("SourceUnit", "try_options")
    }

    #[test]
    fn tuple_assignment() -> Result<()> {
        run("SourceUnit", "tuple_assignment")
    }

    #[test]
    fn tuple_assignment_empty() -> Result<()> {
        run("SourceUnit", "tuple_assignment_empty")
    }

    #[test]
    fn tuple_deconstruction_statement() -> Result<()> {
        run("SourceUnit", "tuple_deconstruction_statement")
    }

    #[test]
    fn unrecognized_lexeme_after_contract() -> Result<()> {
        run("SourceUnit", "unrecognized_lexeme_after_contract")
    }

    #[test]
    fn unreserved_keywords() -> Result<()> {
        run("SourceUnit", "unreserved_keywords")
    }

    #[test]
    fn unterminated_double_quote_string() -> Result<()> {
        run("SourceUnit", "unterminated_double_quote_string")
    }

    #[test]
    fn unterminated_single_quote_string() -> Result<()> {
        run("SourceUnit", "unterminated_single_quote_string")
    }

    #[test]
    fn using_directive() -> Result<()> {
        run("SourceUnit", "using_directive")
    }
}

mod state_variable_definition {
    use super::*;

    #[test]
    fn transient() -> Result<()> {
        run("StateVariableDefinition", "transient")
    }
}

mod statements {
    use super::*;

    #[test]
    fn compound_tokens() -> Result<()> {
        run("Statements", "compound_tokens")
    }

    #[test]
    fn contextual_keywords() -> Result<()> {
        run("Statements", "contextual_keywords")
    }

    #[test]
    fn delete_identifier() -> Result<()> {
        run("Statements", "delete_identifier")
    }

    #[test]
    fn delete_index() -> Result<()> {
        run("Statements", "delete_index")
    }

    #[test]
    fn invalid_termination() -> Result<()> {
        run("Statements", "invalid_termination")
    }

    #[test]
    fn recovery_ignore_multiple_empty_matches() -> Result<()> {
        run("Statements", "recovery_ignore_multiple_empty_matches")
    }
}

mod string_literal {
    use super::*;

    #[test]
    fn double_quote_unicode() -> Result<()> {
        run("StringLiteral", "double_quote_unicode")
    }

    #[test]
    fn escape_arbitrary_ascii() -> Result<()> {
        run("StringLiteral", "escape_arbitrary_ascii")
    }

    #[test]
    fn escape_arbitrary_unicode() -> Result<()> {
        run("StringLiteral", "escape_arbitrary_unicode")
    }

    #[test]
    fn escape_ascii() -> Result<()> {
        run("StringLiteral", "escape_ascii")
    }

    #[test]
    fn escape_cr_double_quote() -> Result<()> {
        run("StringLiteral", "escape_cr_double_quote")
    }

    #[test]
    fn escape_cr_single_quote() -> Result<()> {
        run("StringLiteral", "escape_cr_single_quote")
    }

    #[test]
    fn escape_crlf_double_quote() -> Result<()> {
        run("StringLiteral", "escape_crlf_double_quote")
    }

    #[test]
    fn escape_crlf_single_quote() -> Result<()> {
        run("StringLiteral", "escape_crlf_single_quote")
    }

    #[test]
    fn escape_hex() -> Result<()> {
        run("StringLiteral", "escape_hex")
    }

    #[test]
    fn escape_hex_invalid() -> Result<()> {
        run("StringLiteral", "escape_hex_invalid")
    }

    #[test]
    fn escape_lf_double_quote() -> Result<()> {
        run("StringLiteral", "escape_lf_double_quote")
    }

    #[test]
    fn escape_lf_single_quote() -> Result<()> {
        run("StringLiteral", "escape_lf_single_quote")
    }

    #[test]
    fn escape_unicode() -> Result<()> {
        run("StringLiteral", "escape_unicode")
    }

    #[test]
    fn escape_unicode_invalid() -> Result<()> {
        run("StringLiteral", "escape_unicode_invalid")
    }

    #[test]
    fn single_quote_unicode() -> Result<()> {
        run("StringLiteral", "single_quote_unicode")
    }

    #[test]
    fn tabs_double_quote() -> Result<()> {
        run("StringLiteral", "tabs_double_quote")
    }

    #[test]
    fn tabs_single_quote() -> Result<()> {
        run("StringLiteral", "tabs_single_quote")
    }
}

mod string_literals {
    use super::*;

    #[test]
    fn both_quotes() -> Result<()> {
        run("StringLiterals", "both_quotes")
    }

    #[test]
    fn double_quote() -> Result<()> {
        run("StringLiterals", "double_quote")
    }

    #[test]
    fn double_quote_empty() -> Result<()> {
        run("StringLiterals", "double_quote_empty")
    }

    #[test]
    fn double_quote_unicode() -> Result<()> {
        run("StringLiterals", "double_quote_unicode")
    }

    #[test]
    fn single_quote() -> Result<()> {
        run("StringLiterals", "single_quote")
    }

    #[test]
    fn single_quote_empty() -> Result<()> {
        run("StringLiterals", "single_quote_empty")
    }

    #[test]
    fn single_quote_unicode() -> Result<()> {
        run("StringLiterals", "single_quote_unicode")
    }

    #[test]
    fn single_trailing_ident() -> Result<()> {
        run("StringLiterals", "single_trailing_ident")
    }
}

mod struct_definition {
    use super::*;

    #[test]
    fn member_function_pointer() -> Result<()> {
        run("StructDefinition", "member_function_pointer")
    }

    #[test]
    fn no_members() -> Result<()> {
        run("StructDefinition", "no_members")
    }
}

mod throw_statement {
    use super::*;

    #[test]
    fn throw() -> Result<()> {
        run("ThrowStatement", "throw")
    }
}

mod try_statement {
    use super::*;

    #[test]
    fn method_call() -> Result<()> {
        run("TryStatement", "method_call")
    }

    #[test]
    fn method_call_with_body() -> Result<()> {
        run("TryStatement", "method_call_with_body")
    }

    #[test]
    fn method_call_with_options() -> Result<()> {
        run("TryStatement", "method_call_with_options")
    }

    #[test]
    fn method_call_with_options_and_body() -> Result<()> {
        run("TryStatement", "method_call_with_options_and_body")
    }
}

mod tuple_deconstruction_statement {
    use super::*;

    #[test]
    fn abi_decode_array_type() -> Result<()> {
        run("TupleDeconstructionStatement", "abi_decode_array_type")
    }

    #[test]
    fn abi_decode_singleton_type() -> Result<()> {
        run("TupleDeconstructionStatement", "abi_decode_singleton_type")
    }

    #[test]
    fn empty() -> Result<()> {
        run("TupleDeconstructionStatement", "empty")
    }

    #[test]
    fn empty_tuple() -> Result<()> {
        run("TupleDeconstructionStatement", "empty_tuple")
    }

    #[test]
    fn empty_var_tuple() -> Result<()> {
        run("TupleDeconstructionStatement", "empty_var_tuple")
    }

    #[test]
    fn ignored_members() -> Result<()> {
        run("TupleDeconstructionStatement", "ignored_members")
    }

    #[test]
    fn invalid_termination() -> Result<()> {
        run("TupleDeconstructionStatement", "invalid_termination")
    }

    #[test]
    fn with_location() -> Result<()> {
        run("TupleDeconstructionStatement", "with_location")
    }

    #[test]
    fn with_type() -> Result<()> {
        run("TupleDeconstructionStatement", "with_type")
    }

    #[test]
    fn with_type_and_location() -> Result<()> {
        run("TupleDeconstructionStatement", "with_type_and_location")
    }

    #[test]
    fn with_var() -> Result<()> {
        run("TupleDeconstructionStatement", "with_var")
    }
}

mod tuple_expression {
    use super::*;

    #[test]
    fn empty() -> Result<()> {
        run("TupleExpression", "empty")
    }

    #[test]
    fn full() -> Result<()> {
        run("TupleExpression", "full")
    }

    #[test]
    fn missing_elements() -> Result<()> {
        run("TupleExpression", "missing_elements")
    }
}

mod type_name {
    use super::*;

    #[test]
    fn address_payable() -> Result<()> {
        run("TypeName", "address_payable")
    }

    #[test]
    fn byte() -> Result<()> {
        run("TypeName", "byte")
    }

    #[test]
    fn bytes_invalid_size_as_identifier() -> Result<()> {
        run("TypeName", "bytes_invalid_size_as_identifier")
    }

    #[test]
    fn bytes_no_size_reserved() -> Result<()> {
        run("TypeName", "bytes_no_size_reserved")
    }

    #[test]
    fn bytes_valid_size() -> Result<()> {
        run("TypeName", "bytes_valid_size")
    }

    #[test]
    fn int_invalid_size_as_identifier() -> Result<()> {
        run("TypeName", "int_invalid_size_as_identifier")
    }

    #[test]
    fn int_no_size() -> Result<()> {
        run("TypeName", "int_no_size")
    }

    #[test]
    fn int_valid_size() -> Result<()> {
        run("TypeName", "int_valid_size")
    }

    #[test]
    fn uint_invalid_size_as_identifier() -> Result<()> {
        run("TypeName", "uint_invalid_size_as_identifier")
    }

    #[test]
    fn uint_no_size() -> Result<()> {
        run("TypeName", "uint_no_size")
    }

    #[test]
    fn uint_valid_size() -> Result<()> {
        run("TypeName", "uint_valid_size")
    }
}

mod unicode_string_literals {
    use super::*;

    #[test]
    fn multiple() -> Result<()> {
        run("UnicodeStringLiterals", "multiple")
    }

    #[test]
    fn single() -> Result<()> {
        run("UnicodeStringLiterals", "single")
    }

    #[test]
    fn single_trailing_ident() -> Result<()> {
        run("UnicodeStringLiterals", "single_trailing_ident")
    }
}

mod unnamed_function_definition {
    use super::*;

    #[test]
    fn constant_attribute() -> Result<()> {
        run("UnnamedFunctionDefinition", "constant_attribute")
    }

    #[test]
    fn internal_attribute() -> Result<()> {
        run("UnnamedFunctionDefinition", "internal_attribute")
    }

    #[test]
    fn private_attribute() -> Result<()> {
        run("UnnamedFunctionDefinition", "private_attribute")
    }

    #[test]
    fn public_attribute() -> Result<()> {
        run("UnnamedFunctionDefinition", "public_attribute")
    }
}

mod user_defined_value_type_definition {
    use super::*;

    #[test]
    fn bool() -> Result<()> {
        run("UserDefinedValueTypeDefinition", "bool")
    }
}

mod using_deconstruction_symbol {
    use super::*;

    #[test]
    fn identifier_path() -> Result<()> {
        run("UsingDeconstructionSymbol", "identifier_path")
    }

    #[test]
    fn identifier_path_as_operator() -> Result<()> {
        run("UsingDeconstructionSymbol", "identifier_path_as_operator")
    }

    #[test]
    fn single_id() -> Result<()> {
        run("UsingDeconstructionSymbol", "single_id")
    }

    #[test]
    fn single_id_as_operator() -> Result<()> {
        run("UsingDeconstructionSymbol", "single_id_as_operator")
    }
}

mod using_directive {
    use super::*;

    #[test]
    fn destructure_empty() -> Result<()> {
        run("UsingDirective", "destructure_empty")
    }

    #[test]
    fn destructure_multiple() -> Result<()> {
        run("UsingDirective", "destructure_multiple")
    }

    #[test]
    fn destructure_single() -> Result<()> {
        run("UsingDirective", "destructure_single")
    }

    #[test]
    fn path_named() -> Result<()> {
        run("UsingDirective", "path_named")
    }

    #[test]
    fn path_unnamed() -> Result<()> {
        run("UsingDirective", "path_unnamed")
    }

    #[test]
    fn user_defined_operator() -> Result<()> {
        run("UsingDirective", "user_defined_operator")
    }
}

mod variable_declaration_statement {
    use super::*;

    #[test]
    fn keyword_abicoder() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_abicoder")
    }

    #[test]
    fn keyword_bytes() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_bytes")
    }

    #[test]
    fn keyword_bytes1() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_bytes1")
    }

    #[test]
    fn keyword_bytes11() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_bytes11")
    }

    #[test]
    fn keyword_experimental() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_experimental")
    }

    #[test]
    fn keyword_solidity() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_solidity")
    }

    #[test]
    fn keyword_ufixed() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_ufixed")
    }

    #[test]
    fn keyword_ufixed184x80() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_ufixed184x80")
    }

    #[test]
    fn keyword_ufixed8x0() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_ufixed8x0")
    }

    #[test]
    fn keyword_ufixed8x8() -> Result<()> {
        run("VariableDeclarationStatement", "keyword_ufixed8x8")
    }

    #[test]
    fn var() -> Result<()> {
        run("VariableDeclarationStatement", "var")
    }
}

mod version_pragma {
    use super::*;

    #[test]
    fn alternatives() -> Result<()> {
        run("VersionPragma", "alternatives")
    }

    #[test]
    fn double_quotes_string() -> Result<()> {
        run("VersionPragma", "double_quotes_string")
    }

    #[test]
    fn equal_operator() -> Result<()> {
        run("VersionPragma", "equal_operator")
    }

    #[test]
    fn exact_version() -> Result<()> {
        run("VersionPragma", "exact_version")
    }

    #[test]
    fn less_than_operator() -> Result<()> {
        run("VersionPragma", "less_than_operator")
    }

    #[test]
    fn multiple_exact_versions() -> Result<()> {
        run("VersionPragma", "multiple_exact_versions")
    }

    #[test]
    fn multiple_strings() -> Result<()> {
        run("VersionPragma", "multiple_strings")
    }

    #[test]
    fn nested_expressions() -> Result<()> {
        run("VersionPragma", "nested_expressions")
    }

    #[test]
    fn ranges() -> Result<()> {
        run("VersionPragma", "ranges")
    }

    #[test]
    fn single_quote_string() -> Result<()> {
        run("VersionPragma", "single_quote_string")
    }

    #[test]
    fn with_trivia() -> Result<()> {
        run("VersionPragma", "with_trivia")
    }
}

mod yul_block {
    use super::*;

    #[test]
    fn function_def() -> Result<()> {
        run("YulBlock", "function_def")
    }

    #[test]
    fn ignore_unknown_delim() -> Result<()> {
        run("YulBlock", "ignore_unknown_delim")
    }

    #[test]
    fn multiple_stack_assignments() -> Result<()> {
        run("YulBlock", "multiple_stack_assignments")
    }
}

mod yul_expression {
    use super::*;

    #[test]
    fn decimal_literal() -> Result<()> {
        run("YulExpression", "decimal_literal")
    }

    #[test]
    fn decimal_trailing_dollar() -> Result<()> {
        run("YulExpression", "decimal_trailing_dollar")
    }

    #[test]
    fn decimal_trailing_ident_start() -> Result<()> {
        run("YulExpression", "decimal_trailing_ident_start")
    }

    #[test]
    fn false_keyword() -> Result<()> {
        run("YulExpression", "false_keyword")
    }

    #[test]
    fn function_call() -> Result<()> {
        run("YulExpression", "function_call")
    }

    #[test]
    fn hex_literal() -> Result<()> {
        run("YulExpression", "hex_literal")
    }

    #[test]
    fn hex_trailing_ident_start() -> Result<()> {
        run("YulExpression", "hex_trailing_ident_start")
    }

    #[test]
    fn hex_trailing_letter() -> Result<()> {
        run("YulExpression", "hex_trailing_letter")
    }

    #[test]
    fn identifier_path() -> Result<()> {
        run("YulExpression", "identifier_path")
    }

    #[test]
    fn identifier_with_dot() -> Result<()> {
        run("YulExpression", "identifier_with_dot")
    }

    #[test]
    fn true_keyword() -> Result<()> {
        run("YulExpression", "true_keyword")
    }
}

mod yul_function_call_expression {
    use super::*;

    #[test]
    fn built_in_and() -> Result<()> {
        run("YulFunctionCallExpression", "built_in_and")
    }
}

mod yul_label {
    use super::*;

    #[test]
    fn single_label() -> Result<()> {
        run("YulLabel", "single_label")
    }
}

mod yul_leave_statement {
    use super::*;

    #[test]
    fn leave() -> Result<()> {
        run("YulLeaveStatement", "leave")
    }
}

mod yul_stack_assignment_statement {
    use super::*;

    #[test]
    fn equal_colon_separated() -> Result<()> {
        run("YulStackAssignmentStatement", "equal_colon_separated")
    }

    #[test]
    fn single_variable() -> Result<()> {
        run("YulStackAssignmentStatement", "single_variable")
    }
}

mod yul_statement {
    use super::*;

    #[test]
    fn label() -> Result<()> {
        run("YulStatement", "label")
    }

    #[test]
    fn switch_decimal_trailing_case() -> Result<()> {
        run("YulStatement", "switch_decimal_trailing_case")
    }

    #[test]
    fn var_assign_colon_and_equals() -> Result<()> {
        run("YulStatement", "var_assign_colon_and_equals")
    }
}

mod yul_statements {
    use super::*;

    #[test]
    fn function_pointer() -> Result<()> {
        run("YulStatements", "function_pointer")
    }
}

mod yul_variable_assignment_statement {
    use super::*;

    #[test]
    fn colon_equal_separated() -> Result<()> {
        run("YulVariableAssignmentStatement", "colon_equal_separated")
    }

    #[test]
    fn identifier_and() -> Result<()> {
        run("YulVariableAssignmentStatement", "identifier_and")
    }
}

mod yul_variable_declaration_statement {
    use super::*;

    #[test]
    fn colon_equal_separated() -> Result<()> {
        run("YulVariableDeclarationStatement", "colon_equal_separated")
    }

    #[test]
    fn identifier_with_dots() -> Result<()> {
        run("YulVariableDeclarationStatement", "identifier_with_dots")
    }

    #[test]
    fn keyword_bytes() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_bytes")
    }

    #[test]
    fn keyword_bytes1() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_bytes1")
    }

    #[test]
    fn keyword_bytes11() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_bytes11")
    }

    #[test]
    fn keyword_ufixed184x80() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_ufixed184x80")
    }

    #[test]
    fn keyword_ufixed8x0() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_ufixed8x0")
    }

    #[test]
    fn keyword_ufixed8x8() -> Result<()> {
        run("YulVariableDeclarationStatement", "keyword_ufixed8x8")
    }

    #[test]
    fn multiple_variables() -> Result<()> {
        run("YulVariableDeclarationStatement", "multiple_variables")
    }

    #[test]
    fn multiple_variables_with_value() -> Result<()> {
        run(
            "YulVariableDeclarationStatement",
            "multiple_variables_with_value",
        )
    }
}
