// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "Expression";

#[test]
fn _0() -> Result<()> {
    run(T, "_0")
}

#[test]
fn _a() -> Result<()> {
    run(T, "_a")
}

#[test]
fn add_mul() -> Result<()> {
    run(T, "add_mul")
}

#[test]
fn address_call() -> Result<()> {
    run(T, "address_call")
}

#[test]
fn address_payable_call() -> Result<()> {
    run(T, "address_payable_call")
}

#[test]
fn areturn() -> Result<()> {
    run(T, "areturn")
}

#[test]
fn delete() -> Result<()> {
    run(T, "delete")
}

#[test]
fn delete_conditional() -> Result<()> {
    run(T, "delete_conditional")
}

#[test]
fn elementary_type_under_call() -> Result<()> {
    run(T, "elementary_type_under_call")
}

#[test]
fn exponentiation() -> Result<()> {
    run(T, "exponentiation")
}

#[test]
fn function_call() -> Result<()> {
    run(T, "function_call")
}

#[test]
fn function_call_argument_has_type_name_as_prefix() -> Result<()> {
    run(T, "function_call_argument_has_type_name_as_prefix")
}

#[test]
fn function_call_chain() -> Result<()> {
    run(T, "function_call_chain")
}

#[test]
fn function_call_member_access() -> Result<()> {
    run(T, "function_call_member_access")
}

#[test]
fn function_call_options() -> Result<()> {
    run(T, "function_call_options")
}

#[test]
fn function_call_options_split() -> Result<()> {
    run(T, "function_call_options_split")
}

#[test]
fn hex_literal_expression() -> Result<()> {
    run(T, "hex_literal_expression")
}

#[test]
fn identifier_call() -> Result<()> {
    run(T, "identifier_call")
}

#[test]
fn incomplete_operand() -> Result<()> {
    run(T, "incomplete_operand")
}

#[test]
fn index_access() -> Result<()> {
    run(T, "index_access")
}

#[test]
fn index_access_chain() -> Result<()> {
    run(T, "index_access_chain")
}

#[test]
fn index_slice_end() -> Result<()> {
    run(T, "index_slice_end")
}

#[test]
fn index_slice_start() -> Result<()> {
    run(T, "index_slice_start")
}

#[test]
fn index_slice_start_end() -> Result<()> {
    run(T, "index_slice_start_end")
}

#[test]
fn index_slice_unbounded() -> Result<()> {
    run(T, "index_slice_unbounded")
}

#[test]
fn keyword_alias() -> Result<()> {
    run(T, "keyword_alias")
}

#[test]
fn keyword_apply() -> Result<()> {
    run(T, "keyword_apply")
}

#[test]
fn keyword_auto() -> Result<()> {
    run(T, "keyword_auto")
}

#[test]
fn keyword_calldata() -> Result<()> {
    run(T, "keyword_calldata")
}

#[test]
fn keyword_constructor() -> Result<()> {
    run(T, "keyword_constructor")
}

#[test]
fn keyword_copyof() -> Result<()> {
    run(T, "keyword_copyof")
}

#[test]
fn keyword_define() -> Result<()> {
    run(T, "keyword_define")
}

#[test]
fn keyword_emit() -> Result<()> {
    run(T, "keyword_emit")
}

#[test]
fn keyword_fallback() -> Result<()> {
    run(T, "keyword_fallback")
}

#[test]
fn keyword_finney() -> Result<()> {
    run(T, "keyword_finney")
}

#[test]
fn keyword_immutable() -> Result<()> {
    run(T, "keyword_immutable")
}

#[test]
fn keyword_implements() -> Result<()> {
    run(T, "keyword_implements")
}

#[test]
fn keyword_macro() -> Result<()> {
    run(T, "keyword_macro")
}

#[test]
fn keyword_mutable() -> Result<()> {
    run(T, "keyword_mutable")
}

#[test]
fn keyword_override() -> Result<()> {
    run(T, "keyword_override")
}

#[test]
fn keyword_partial() -> Result<()> {
    run(T, "keyword_partial")
}

#[test]
fn keyword_promise() -> Result<()> {
    run(T, "keyword_promise")
}

#[test]
fn keyword_receive() -> Result<()> {
    run(T, "keyword_receive")
}

#[test]
fn keyword_reference() -> Result<()> {
    run(T, "keyword_reference")
}

#[test]
fn keyword_sealed() -> Result<()> {
    run(T, "keyword_sealed")
}

#[test]
fn keyword_sizeof() -> Result<()> {
    run(T, "keyword_sizeof")
}

#[test]
fn keyword_super() -> Result<()> {
    run(T, "keyword_super")
}

#[test]
fn keyword_supports() -> Result<()> {
    run(T, "keyword_supports")
}

#[test]
fn keyword_szabo() -> Result<()> {
    run(T, "keyword_szabo")
}

#[test]
fn keyword_typedef() -> Result<()> {
    run(T, "keyword_typedef")
}

#[test]
fn keyword_ufixed() -> Result<()> {
    run(T, "keyword_ufixed")
}

#[test]
fn keyword_unchecked() -> Result<()> {
    run(T, "keyword_unchecked")
}

#[test]
fn keyword_virtual() -> Result<()> {
    run(T, "keyword_virtual")
}

#[test]
fn member_access() -> Result<()> {
    run(T, "member_access")
}

#[test]
fn member_access_address() -> Result<()> {
    run(T, "member_access_address")
}

#[test]
fn member_access_addresses() -> Result<()> {
    run(T, "member_access_addresses")
}

#[test]
fn member_access_chain() -> Result<()> {
    run(T, "member_access_chain")
}

#[test]
fn member_access_chain_access() -> Result<()> {
    run(T, "member_access_chain_access")
}

#[test]
fn member_access_function_call() -> Result<()> {
    run(T, "member_access_function_call")
}

#[test]
fn member_access_index_access() -> Result<()> {
    run(T, "member_access_index_access")
}

#[test]
fn member_access_integer() -> Result<()> {
    run(T, "member_access_integer")
}

#[test]
fn member_access_options() -> Result<()> {
    run(T, "member_access_options")
}

#[test]
fn member_access_rational() -> Result<()> {
    run(T, "member_access_rational")
}

#[test]
fn member_access_rational_leading_period() -> Result<()> {
    run(T, "member_access_rational_leading_period")
}

#[test]
fn member_access_super() -> Result<()> {
    run(T, "member_access_super")
}

#[test]
fn member_access_this() -> Result<()> {
    run(T, "member_access_this")
}

#[test]
fn new_expression() -> Result<()> {
    run(T, "new_expression")
}

#[test]
fn new_expression_chain() -> Result<()> {
    run(T, "new_expression_chain")
}

#[test]
fn new_expression_options() -> Result<()> {
    run(T, "new_expression_options")
}

#[test]
fn overlapping_operators() -> Result<()> {
    run(T, "overlapping_operators")
}

#[test]
fn paren_expression_options() -> Result<()> {
    run(T, "paren_expression_options")
}

#[test]
fn postfix_decrement() -> Result<()> {
    run(T, "postfix_decrement")
}

#[test]
fn prefix_decrement() -> Result<()> {
    run(T, "prefix_decrement")
}

#[test]
fn prefix_minus() -> Result<()> {
    run(T, "prefix_minus")
}

#[test]
fn prefix_plus() -> Result<()> {
    run(T, "prefix_plus")
}

#[test]
fn returna() -> Result<()> {
    run(T, "returna")
}

#[test]
fn returns() -> Result<()> {
    run(T, "returns")
}

#[test]
fn string_literal_expression() -> Result<()> {
    run(T, "string_literal_expression")
}

#[test]
fn underscore_is_identifier() -> Result<()> {
    run(T, "underscore_is_identifier")
}

#[test]
fn unicode_string_literal() -> Result<()> {
    run(T, "unicode_string_literal")
}
