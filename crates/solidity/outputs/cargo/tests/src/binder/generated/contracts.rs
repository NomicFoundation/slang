// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "contracts";

#[test]
fn constructor_call_parent() -> Result<()> {
    run(T, "constructor_call_parent")
}

#[test]
fn constructor_invocation() -> Result<()> {
    run(T, "constructor_invocation")
}

#[test]
fn constructor_modifier() -> Result<()> {
    run(T, "constructor_modifier")
}

#[test]
fn constructors() -> Result<()> {
    run(T, "constructors")
}

#[test]
fn cyclic_inheritance() -> Result<()> {
    run(T, "cyclic_inheritance")
}

#[test]
fn diamond() -> Result<()> {
    run(T, "diamond")
}

#[test]
fn fallback_receive() -> Result<()> {
    run(T, "fallback_receive")
}

#[test]
fn implicit_conversion_to_address() -> Result<()> {
    run(T, "implicit_conversion_to_address")
}

#[test]
fn inheritance() -> Result<()> {
    run(T, "inheritance")
}

#[test]
fn inheritance_arguments_with_inner_constant() -> Result<()> {
    run(T, "inheritance_arguments_with_inner_constant")
}

#[test]
fn inheritance_types() -> Result<()> {
    run(T, "inheritance_types")
}

#[test]
fn inheritance_with_arguments() -> Result<()> {
    run(T, "inheritance_with_arguments")
}

#[test]
fn inherited_legacy_constructor() -> Result<()> {
    run(T, "inherited_legacy_constructor")
}

#[test]
fn inherited_state_vars() -> Result<()> {
    run(T, "inherited_state_vars")
}

#[test]
fn internal_visibility() -> Result<()> {
    run(T, "internal_visibility")
}

#[test]
fn legacy_constructors() -> Result<()> {
    run(T, "legacy_constructors")
}

#[test]
fn legacy_function_options() -> Result<()> {
    run(T, "legacy_function_options")
}

#[test]
fn multi_inheritance() -> Result<()> {
    run(T, "multi_inheritance")
}

#[test]
fn overload_disambiguation_with_this() -> Result<()> {
    run(T, "overload_disambiguation_with_this")
}

#[test]
fn private_member_not_visible() -> Result<()> {
    run(T, "private_member_not_visible")
}

#[test]
fn public_array_getters() -> Result<()> {
    run(T, "public_array_getters")
}

#[test]
fn public_getter_functions() -> Result<()> {
    run(T, "public_getter_functions")
}

#[test]
fn public_getter_members() -> Result<()> {
    run(T, "public_getter_members")
}

#[test]
fn public_getter_selector() -> Result<()> {
    run(T, "public_getter_selector")
}

#[test]
fn public_getters() -> Result<()> {
    run(T, "public_getters")
}

#[test]
fn public_inherited_getter() -> Result<()> {
    run(T, "public_inherited_getter")
}

#[test]
fn public_mapping_getters() -> Result<()> {
    run(T, "public_mapping_getters")
}

#[test]
fn public_nested_mapping_getters() -> Result<()> {
    run(T, "public_nested_mapping_getters")
}

#[test]
fn public_struct_getter() -> Result<()> {
    run(T, "public_struct_getter")
}

#[test]
fn qualified_inherited() -> Result<()> {
    run(T, "qualified_inherited")
}

#[test]
fn qualified_parent_call() -> Result<()> {
    run(T, "qualified_parent_call")
}

#[test]
fn qualified_self() -> Result<()> {
    run(T, "qualified_self")
}

#[test]
fn state_var_override() -> Result<()> {
    run(T, "state_var_override")
}

#[test]
fn storage_layout_constant() -> Result<()> {
    run(T, "storage_layout_constant")
}

#[test]
fn storage_layout_inner_constant() -> Result<()> {
    run(T, "storage_layout_inner_constant")
}

#[test]
fn super_deep() -> Result<()> {
    run(T, "super_deep")
}

#[test]
fn super_linearisation() -> Result<()> {
    run(T, "super_linearisation")
}

#[test]
fn super_scope() -> Result<()> {
    run(T, "super_scope")
}

#[test]
fn this_scope() -> Result<()> {
    run(T, "this_scope")
}

#[test]
fn unnamed_function() -> Result<()> {
    run(T, "unnamed_function")
}

#[test]
fn virtual_lookup() -> Result<()> {
    run(T, "virtual_lookup")
}

#[test]
fn virtual_methods() -> Result<()> {
    run(T, "virtual_methods")
}

#[test]
fn visibility() -> Result<()> {
    run(T, "visibility")
}
