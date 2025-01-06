// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn constructor_call_parent() -> Result<()> {
    run("contracts", "constructor_call_parent")
}

#[test]
fn constructor_invocation() -> Result<()> {
    run("contracts", "constructor_invocation")
}

#[test]
fn constructor_modifier() -> Result<()> {
    run("contracts", "constructor_modifier")
}

#[test]
fn constructors() -> Result<()> {
    run("contracts", "constructors")
}

#[test]
fn diamond() -> Result<()> {
    run("contracts", "diamond")
}

#[test]
fn fallback_receive() -> Result<()> {
    run("contracts", "fallback_receive")
}

#[test]
fn inheritance() -> Result<()> {
    run("contracts", "inheritance")
}

#[test]
fn inherited_state_vars() -> Result<()> {
    run("contracts", "inherited_state_vars")
}

#[test]
fn internal_visibility() -> Result<()> {
    run("contracts", "internal_visibility")
}

#[test]
fn legacy_constructors() -> Result<()> {
    run("contracts", "legacy_constructors")
}

#[test]
fn legacy_function_options() -> Result<()> {
    run("contracts", "legacy_function_options")
}

#[test]
fn multi_inheritance() -> Result<()> {
    run("contracts", "multi_inheritance")
}

#[test]
fn public_array_getters() -> Result<()> {
    run("contracts", "public_array_getters")
}

#[test]
fn public_getter_members() -> Result<()> {
    run("contracts", "public_getter_members")
}

#[test]
fn public_getters() -> Result<()> {
    run("contracts", "public_getters")
}

#[test]
fn public_inherited_getter() -> Result<()> {
    run("contracts", "public_inherited_getter")
}

#[test]
fn public_mapping_getters() -> Result<()> {
    run("contracts", "public_mapping_getters")
}

#[test]
fn public_struct_getter() -> Result<()> {
    run("contracts", "public_struct_getter")
}

#[test]
fn qualified_inherited() -> Result<()> {
    run("contracts", "qualified_inherited")
}

#[test]
fn qualified_parent_call() -> Result<()> {
    run("contracts", "qualified_parent_call")
}

#[test]
fn super_deep() -> Result<()> {
    run("contracts", "super_deep")
}

#[test]
fn super_linearisation() -> Result<()> {
    run("contracts", "super_linearisation")
}

#[test]
fn super_scope() -> Result<()> {
    run("contracts", "super_scope")
}

#[test]
fn this_scope() -> Result<()> {
    run("contracts", "this_scope")
}

#[test]
fn unnamed_function() -> Result<()> {
    run("contracts", "unnamed_function")
}

#[test]
fn virtual_lookup() -> Result<()> {
    run("contracts", "virtual_lookup")
}

#[test]
fn virtual_methods() -> Result<()> {
    run("contracts", "virtual_methods")
}

#[test]
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}
