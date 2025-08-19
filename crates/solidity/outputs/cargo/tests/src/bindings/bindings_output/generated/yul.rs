// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "yul";

#[test]
fn all_built_ins() -> Result<()> {
    run(T, "all_built_ins")
}

#[test]
fn blocks() -> Result<()> {
    run(T, "blocks")
}

#[test]
fn built_ins() -> Result<()> {
    run(T, "built_ins")
}

#[test]
fn catch_params() -> Result<()> {
    run(T, "catch_params")
}

#[test]
fn conditionals() -> Result<()> {
    run(T, "conditionals")
}

#[test]
fn constant_access_from_functions() -> Result<()> {
    run(T, "constant_access_from_functions")
}

#[test]
fn constructor_params() -> Result<()> {
    run(T, "constructor_params")
}

#[test]
fn external_variables() -> Result<()> {
    run(T, "external_variables")
}

#[test]
fn fallback_params() -> Result<()> {
    run(T, "fallback_params")
}

#[test]
fn for_init_variables() -> Result<()> {
    run(T, "for_init_variables")
}

#[test]
fn functions() -> Result<()> {
    run(T, "functions")
}

#[test]
fn identifiers_with_dots() -> Result<()> {
    run(T, "identifiers_with_dots")
}

#[test]
fn imported_constant() -> Result<()> {
    run(T, "imported_constant")
}

#[test]
fn imported_deconstructed_constants() -> Result<()> {
    run(T, "imported_deconstructed_constants")
}

#[test]
fn inherited_constant() -> Result<()> {
    run(T, "inherited_constant")
}

#[test]
fn labels() -> Result<()> {
    run(T, "labels")
}

#[test]
fn legacy_built_ins() -> Result<()> {
    run(T, "legacy_built_ins")
}

#[test]
fn loops() -> Result<()> {
    run(T, "loops")
}

#[test]
fn modifier_params() -> Result<()> {
    run(T, "modifier_params")
}

#[test]
fn nested_functions() -> Result<()> {
    run(T, "nested_functions")
}

#[test]
fn slot_of_public_var_override() -> Result<()> {
    run(T, "slot_of_public_var_override")
}

#[test]
fn slot_of_return_params() -> Result<()> {
    run(T, "slot_of_return_params")
}

#[test]
fn slot_offset_members() -> Result<()> {
    run(T, "slot_offset_members")
}

#[test]
fn slot_suffix() -> Result<()> {
    run(T, "slot_suffix")
}

#[test]
fn solidity_built_in_doesn_t_bind() -> Result<()> {
    run(T, "solidity_built_in_doesn_t_bind")
}

#[test]
fn stack_assign() -> Result<()> {
    run(T, "stack_assign")
}

#[test]
fn try_params() -> Result<()> {
    run(T, "try_params")
}

#[test]
fn variables() -> Result<()> {
    run(T, "variables")
}
