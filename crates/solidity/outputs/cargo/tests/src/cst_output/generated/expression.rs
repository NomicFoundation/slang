// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

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
fn delete() -> Result<()> {
    run("Expression", "delete")
}

#[test]
fn delete_conditional() -> Result<()> {
    run("Expression", "delete_conditional")
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
fn member_access_chain() -> Result<()> {
    run("Expression", "member_access_chain")
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
fn new_expression() -> Result<()> {
    run("Expression", "new_expression")
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
fn underscore_is_identifier() -> Result<()> {
    run("Expression", "underscore_is_identifier")
}

#[test]
fn unicode_string_literal() -> Result<()> {
    run("Expression", "unicode_string_literal")
}
