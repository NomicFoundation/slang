// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn _0() -> Result<()> {
    return run("Expression", "_0");
}

#[test]
fn _a() -> Result<()> {
    return run("Expression", "_a");
}

#[test]
fn add_mul() -> Result<()> {
    return run("Expression", "add_mul");
}

#[test]
fn address_call() -> Result<()> {
    return run("Expression", "address_call");
}

#[test]
fn address_payable_call() -> Result<()> {
    return run("Expression", "address_payable_call");
}

#[test]
fn areturn() -> Result<()> {
    return run("Expression", "areturn");
}

#[test]
fn conditional_expression_identifier_base() -> Result<()> {
    return run("Expression", "conditional_expression_identifier_base");
}

#[test]
fn conditional_expression_nested_base() -> Result<()> {
    return run("Expression", "conditional_expression_nested_base");
}

#[test]
fn conditional_expression_nested_conditions() -> Result<()> {
    return run("Expression", "conditional_expression_nested_conditions");
}

#[test]
fn conditional_expression_recursive() -> Result<()> {
    return run("Expression", "conditional_expression_recursive");
}

#[test]
fn exponentiation_operator_associativity() -> Result<()> {
    return run("Expression", "exponentiation_operator_associativity");
}

#[test]
fn function_call() -> Result<()> {
    return run("Expression", "function_call");
}

#[test]
fn function_call_argument_has_type_name_as_prefix() -> Result<()> {
    return run(
        "Expression",
        "function_call_argument_has_type_name_as_prefix",
    );
}

#[test]
fn function_call_chain() -> Result<()> {
    return run("Expression", "function_call_chain");
}

#[test]
fn function_call_member_access() -> Result<()> {
    return run("Expression", "function_call_member_access");
}

#[test]
fn function_call_options() -> Result<()> {
    return run("Expression", "function_call_options");
}

#[test]
fn function_call_options_split() -> Result<()> {
    return run("Expression", "function_call_options_split");
}

#[test]
fn identifier_call() -> Result<()> {
    return run("Expression", "identifier_call");
}

#[test]
fn index_access() -> Result<()> {
    return run("Expression", "index_access");
}

#[test]
fn index_access_chain() -> Result<()> {
    return run("Expression", "index_access_chain");
}

#[test]
fn index_slice_end() -> Result<()> {
    return run("Expression", "index_slice_end");
}

#[test]
fn index_slice_start() -> Result<()> {
    return run("Expression", "index_slice_start");
}

#[test]
fn index_slice_start_end() -> Result<()> {
    return run("Expression", "index_slice_start_end");
}

#[test]
fn index_slice_unbounded() -> Result<()> {
    return run("Expression", "index_slice_unbounded");
}

#[test]
fn keyword_alias() -> Result<()> {
    return run("Expression", "keyword_alias");
}

#[test]
fn keyword_apply() -> Result<()> {
    return run("Expression", "keyword_apply");
}

#[test]
fn keyword_auto() -> Result<()> {
    return run("Expression", "keyword_auto");
}

#[test]
fn keyword_calldata() -> Result<()> {
    return run("Expression", "keyword_calldata");
}

#[test]
fn keyword_constructor() -> Result<()> {
    return run("Expression", "keyword_constructor");
}

#[test]
fn keyword_copyof() -> Result<()> {
    return run("Expression", "keyword_copyof");
}

#[test]
fn keyword_define() -> Result<()> {
    return run("Expression", "keyword_define");
}

#[test]
fn keyword_emit() -> Result<()> {
    return run("Expression", "keyword_emit");
}

#[test]
fn keyword_fallback() -> Result<()> {
    return run("Expression", "keyword_fallback");
}

#[test]
fn keyword_finney() -> Result<()> {
    return run("Expression", "keyword_finney");
}

#[test]
fn keyword_immutable() -> Result<()> {
    return run("Expression", "keyword_immutable");
}

#[test]
fn keyword_implements() -> Result<()> {
    return run("Expression", "keyword_implements");
}

#[test]
fn keyword_macro() -> Result<()> {
    return run("Expression", "keyword_macro");
}

#[test]
fn keyword_mutable() -> Result<()> {
    return run("Expression", "keyword_mutable");
}

#[test]
fn keyword_override() -> Result<()> {
    return run("Expression", "keyword_override");
}

#[test]
fn keyword_partial() -> Result<()> {
    return run("Expression", "keyword_partial");
}

#[test]
fn keyword_promise() -> Result<()> {
    return run("Expression", "keyword_promise");
}

#[test]
fn keyword_receive() -> Result<()> {
    return run("Expression", "keyword_receive");
}

#[test]
fn keyword_reference() -> Result<()> {
    return run("Expression", "keyword_reference");
}

#[test]
fn keyword_sealed() -> Result<()> {
    return run("Expression", "keyword_sealed");
}

#[test]
fn keyword_sizeof() -> Result<()> {
    return run("Expression", "keyword_sizeof");
}

#[test]
fn keyword_supports() -> Result<()> {
    return run("Expression", "keyword_supports");
}

#[test]
fn keyword_szabo() -> Result<()> {
    return run("Expression", "keyword_szabo");
}

#[test]
fn keyword_typedef() -> Result<()> {
    return run("Expression", "keyword_typedef");
}

#[test]
fn keyword_unchecked() -> Result<()> {
    return run("Expression", "keyword_unchecked");
}

#[test]
fn keyword_virtual() -> Result<()> {
    return run("Expression", "keyword_virtual");
}

#[test]
fn member_access() -> Result<()> {
    return run("Expression", "member_access");
}

#[test]
fn member_access_chain() -> Result<()> {
    return run("Expression", "member_access_chain");
}

#[test]
fn member_access_function_call() -> Result<()> {
    return run("Expression", "member_access_function_call");
}

#[test]
fn member_access_index_access() -> Result<()> {
    return run("Expression", "member_access_index_access");
}

#[test]
fn overlapping_operators() -> Result<()> {
    return run("Expression", "overlapping_operators");
}

#[test]
fn payable_call() -> Result<()> {
    return run("Expression", "payable_call");
}

#[test]
fn postfix_decrement() -> Result<()> {
    return run("Expression", "postfix_decrement");
}

#[test]
fn prefix_decrement() -> Result<()> {
    return run("Expression", "prefix_decrement");
}

#[test]
fn prefix_minus() -> Result<()> {
    return run("Expression", "prefix_minus");
}

#[test]
fn prefix_plus() -> Result<()> {
    return run("Expression", "prefix_plus");
}

#[test]
fn returna() -> Result<()> {
    return run("Expression", "returna");
}

#[test]
fn returns() -> Result<()> {
    return run("Expression", "returns");
}

#[test]
fn underscore_is_identifier() -> Result<()> {
    return run("Expression", "underscore_is_identifier");
}

#[test]
fn unicode_string_literal() -> Result<()> {
    return run("Expression", "unicode_string_literal");
}
