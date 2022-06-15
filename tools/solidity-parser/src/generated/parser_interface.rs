use super::tree_interface::*;
use chumsky::prelude::{BoxedParser, Simple};
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
#[allow(dead_code)]
pub struct Parsers {
    pub add_sub_operator: ParserType<add_sub_operator::N>,
    pub assignment_operator: ParserType<assignment_operator::N>,
    pub break_statement: ParserType<break_statement::N>,
    pub comment: ParserType<comment::N>,
    pub continue_statement: ParserType<continue_statement::N>,
    pub data_location: ParserType<data_location::N>,
    pub equality_comparison_operator: ParserType<equality_comparison_operator::N>,
    pub line_comment: ParserType<line_comment::N>,
    pub mul_div_mod_operator: ParserType<mul_div_mod_operator::N>,
    pub order_comparison_operator: ParserType<order_comparison_operator::N>,
    pub positional_argument_list: ParserType<positional_argument_list::N>,
    pub shift_operator: ParserType<shift_operator::N>,
    pub state_mutability_specifier: ParserType<state_mutability_specifier::N>,
    pub unary_prefix_operator: ParserType<unary_prefix_operator::N>,
    pub unary_suffix_operator: ParserType<unary_suffix_operator::N>,
    pub unchecked_block: ParserType<unchecked_block::N>,
    pub visibility_specifier: ParserType<visibility_specifier::N>,
    pub whitespace: ParserType<whitespace::N>,
    pub yul_break_statement: ParserType<yul_break_statement::N>,
    pub yul_continue_statement: ParserType<yul_continue_statement::N>,
    pub yul_leave_statement: ParserType<yul_leave_statement::N>,
    pub ignore: ParserType<ignore::N>,
    pub ascii_escape: ParserType<ascii_escape::N>,
    pub boolean_literal: ParserType<boolean_literal::N>,
    pub decimal_integer: ParserType<decimal_integer::N>,
    pub fixed_bytes_type: ParserType<fixed_bytes_type::N>,
    pub fixed_type: ParserType<fixed_type::N>,
    pub hex_character: ParserType<hex_character::N>,
    pub identifier_start: ParserType<identifier_start::N>,
    pub number_unit: ParserType<number_unit::N>,
    pub pragma_directive: ParserType<pragma_directive::N>,
    pub reserved_keyword: ParserType<reserved_keyword::N>,
    pub signed_integer_type: ParserType<signed_integer_type::N>,
    pub yul_decimal_number_literal: ParserType<yul_decimal_number_literal::N>,
    pub yul_evm_builtin_function_name: ParserType<yul_evm_builtin_function_name::N>,
    pub yul_hex_literal: ParserType<yul_hex_literal::N>,
    pub yul_keyword: ParserType<yul_keyword::N>,
    pub decimal_exponent: ParserType<decimal_exponent::N>,
    pub decimal_float: ParserType<decimal_float::N>,
    pub hex_byte_escape: ParserType<hex_byte_escape::N>,
    pub hex_number: ParserType<hex_number::N>,
    pub identifier_part: ParserType<identifier_part::N>,
    pub possibly_separated_pairs_of_hex_digits:
        ParserType<possibly_separated_pairs_of_hex_digits::N>,
    pub ufixed_type: ParserType<ufixed_type::N>,
    pub unicode_escape: ParserType<unicode_escape::N>,
    pub unsigned_integer_type: ParserType<unsigned_integer_type::N>,
    pub yul_reserved_word: ParserType<yul_reserved_word::N>,
    pub decimal_number: ParserType<decimal_number::N>,
    pub elementary_type: ParserType<elementary_type::N>,
    pub escape_sequence: ParserType<escape_sequence::N>,
    pub hex_string_literal: ParserType<hex_string_literal::N>,
    pub keyword: ParserType<keyword::N>,
    pub raw_identifier: ParserType<raw_identifier::N>,
    pub double_quoted_ascii_string_literal: ParserType<double_quoted_ascii_string_literal::N>,
    pub double_quoted_unicode_string_literal: ParserType<double_quoted_unicode_string_literal::N>,
    pub elementary_type_with_payable: ParserType<elementary_type_with_payable::N>,
    pub elementary_type_without_payable: ParserType<elementary_type_without_payable::N>,
    pub numeric_literal: ParserType<numeric_literal::N>,
    pub reserved_word: ParserType<reserved_word::N>,
    pub single_quoted_ascii_string_literal: ParserType<single_quoted_ascii_string_literal::N>,
    pub single_quoted_unicode_string_literal: ParserType<single_quoted_unicode_string_literal::N>,
    pub yul_identifier: ParserType<yul_identifier::N>,
    pub ascii_string_literal: ParserType<ascii_string_literal::N>,
    pub assembly_flags: ParserType<assembly_flags::N>,
    pub identifier: ParserType<identifier::N>,
    pub unicode_string_literal: ParserType<unicode_string_literal::N>,
    pub yul_function_call: ParserType<yul_function_call::N>,
    pub yul_function_definition: ParserType<yul_function_definition::N>,
    pub yul_path: ParserType<yul_path::N>,
    pub enum_definition: ParserType<enum_definition::N>,
    pub identifier_path: ParserType<identifier_path::N>,
    pub import_path: ParserType<import_path::N>,
    pub literal: ParserType<literal::N>,
    pub named_argument: ParserType<named_argument::N>,
    pub parameter_declaration: ParserType<parameter_declaration::N>,
    pub selected_import: ParserType<selected_import::N>,
    pub user_defined_value_type_definition: ParserType<user_defined_value_type_definition::N>,
    pub yul_literal: ParserType<yul_literal::N>,
    pub mapping_type: ParserType<mapping_type::N>,
    pub named_argument_list: ParserType<named_argument_list::N>,
    pub non_empty_parameter_list: ParserType<non_empty_parameter_list::N>,
    pub override_specifier: ParserType<override_specifier::N>,
    pub parameter_list: ParserType<parameter_list::N>,
    pub selecting_import_directive: ParserType<selecting_import_directive::N>,
    pub simple_import_directive: ParserType<simple_import_directive::N>,
    pub star_import_directive: ParserType<star_import_directive::N>,
    pub yul_expression: ParserType<yul_expression::N>,
    pub argument_list: ParserType<argument_list::N>,
    pub catch_clause: ParserType<catch_clause::N>,
    pub function_type: ParserType<function_type::N>,
    pub import_directive: ParserType<import_directive::N>,
    pub method_attribute: ParserType<method_attribute::N>,
    pub state_variable_attribute: ParserType<state_variable_attribute::N>,
    pub yul_assignment: ParserType<yul_assignment::N>,
    pub yul_for_statement: ParserType<yul_for_statement::N>,
    pub yul_if_statement: ParserType<yul_if_statement::N>,
    pub yul_switch_statement: ParserType<yul_switch_statement::N>,
    pub yul_variable_declaration: ParserType<yul_variable_declaration::N>,
    pub inheritance_specifier: ParserType<inheritance_specifier::N>,
    pub modifier_invocation: ParserType<modifier_invocation::N>,
    pub type_name: ParserType<type_name::N>,
    pub yul_statement: ParserType<yul_statement::N>,
    pub constructor_attribute: ParserType<constructor_attribute::N>,
    pub error_parameter: ParserType<error_parameter::N>,
    pub event_parameter: ParserType<event_parameter::N>,
    pub fallback_function_attribute: ParserType<fallback_function_attribute::N>,
    pub function_attribute: ParserType<function_attribute::N>,
    pub inheritance_specifier_list: ParserType<inheritance_specifier_list::N>,
    pub primary_expression: ParserType<primary_expression::N>,
    pub receive_function_attribute: ParserType<receive_function_attribute::N>,
    pub struct_definition: ParserType<struct_definition::N>,
    pub using_directive: ParserType<using_directive::N>,
    pub variable_declaration: ParserType<variable_declaration::N>,
    pub yul_block: ParserType<yul_block::N>,
    pub assembly_statement: ParserType<assembly_statement::N>,
    pub directive: ParserType<directive::N>,
    pub error_definition: ParserType<error_definition::N>,
    pub event_definition: ParserType<event_definition::N>,
    pub index_access_expression: ParserType<index_access_expression::N>,
    pub variable_declaration_tuple: ParserType<variable_declaration_tuple::N>,
    pub member_access_expression: ParserType<member_access_expression::N>,
    pub function_call_options_expression: ParserType<function_call_options_expression::N>,
    pub function_call_expression: ParserType<function_call_expression::N>,
    pub unary_prefix_expression: ParserType<unary_prefix_expression::N>,
    pub unary_suffix_expression: ParserType<unary_suffix_expression::N>,
    pub exponentiation_expression: ParserType<exponentiation_expression::N>,
    pub mul_div_mod_expression: ParserType<mul_div_mod_expression::N>,
    pub add_sub_expression: ParserType<add_sub_expression::N>,
    pub shift_expression: ParserType<shift_expression::N>,
    pub bit_and_expression: ParserType<bit_and_expression::N>,
    pub bit_x_or_expression: ParserType<bit_x_or_expression::N>,
    pub bit_or_expression: ParserType<bit_or_expression::N>,
    pub order_comparison_expression: ParserType<order_comparison_expression::N>,
    pub equality_comparison_expression: ParserType<equality_comparison_expression::N>,
    pub and_expression: ParserType<and_expression::N>,
    pub or_expression: ParserType<or_expression::N>,
    pub conditional_expression: ParserType<conditional_expression::N>,
    pub assignment_expression: ParserType<assignment_expression::N>,
    pub expression: ParserType<expression::N>,
    pub constant_definition: ParserType<constant_definition::N>,
    pub do_while_statement: ParserType<do_while_statement::N>,
    pub emit_statement: ParserType<emit_statement::N>,
    pub expression_statement: ParserType<expression_statement::N>,
    pub if_statement: ParserType<if_statement::N>,
    pub return_statement: ParserType<return_statement::N>,
    pub revert_statement: ParserType<revert_statement::N>,
    pub state_variable_declaration: ParserType<state_variable_declaration::N>,
    pub try_statement: ParserType<try_statement::N>,
    pub variable_declaration_statement: ParserType<variable_declaration_statement::N>,
    pub while_statement: ParserType<while_statement::N>,
    pub simple_statement: ParserType<simple_statement::N>,
    pub for_statement: ParserType<for_statement::N>,
    pub statement: ParserType<statement::N>,
    pub block: ParserType<block::N>,
    pub constructor_definition: ParserType<constructor_definition::N>,
    pub fallback_function_definition: ParserType<fallback_function_definition::N>,
    pub function_definition: ParserType<function_definition::N>,
    pub modifier_definition: ParserType<modifier_definition::N>,
    pub receive_function_definition: ParserType<receive_function_definition::N>,
    pub contract_body_element: ParserType<contract_body_element::N>,
    pub contract_definition: ParserType<contract_definition::N>,
    pub interface_definition: ParserType<interface_definition::N>,
    pub library_definition: ParserType<library_definition::N>,
    pub definition: ParserType<definition::N>,
    pub source_unit: ParserType<source_unit::N>,
}
