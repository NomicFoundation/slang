// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn all_built_ins() -> Result<()> {
    run("yul", "all_built_ins")
}

#[test]
fn blocks() -> Result<()> {
    run("yul", "blocks")
}

#[test]
fn built_ins() -> Result<()> {
    run("yul", "built_ins")
}

#[test]
fn conditionals() -> Result<()> {
    run("yul", "conditionals")
}

#[test]
fn constant_access_from_functions() -> Result<()> {
    run("yul", "constant_access_from_functions")
}

#[test]
fn external_variables() -> Result<()> {
    run("yul", "external_variables")
}

#[test]
fn functions() -> Result<()> {
    run("yul", "functions")
}

#[test]
fn identifiers_with_dots() -> Result<()> {
    run("yul", "identifiers_with_dots")
}

#[test]
fn labels() -> Result<()> {
    run("yul", "labels")
}

#[test]
fn legacy_built_ins() -> Result<()> {
    run("yul", "legacy_built_ins")
}

#[test]
fn loops() -> Result<()> {
    run("yul", "loops")
}

#[test]
fn slot_offset_members() -> Result<()> {
    run("yul", "slot_offset_members")
}

#[test]
fn slot_suffix() -> Result<()> {
    run("yul", "slot_suffix")
}

#[test]
fn solidity_built_in_doesn_t_bind() -> Result<()> {
    run("yul", "solidity_built_in_doesn_t_bind")
}

#[test]
fn stack_assign() -> Result<()> {
    run("yul", "stack_assign")
}

#[test]
fn variables() -> Result<()> {
    run("yul", "variables")
}
