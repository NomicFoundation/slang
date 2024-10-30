// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn call_public_getter() -> Result<()> {
    run("contracts", "call_public_getter")
}

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
fn fallback_receive() -> Result<()> {
    run("contracts", "fallback_receive")
}

#[test]
fn inheritance() -> Result<()> {
    run("contracts", "inheritance")
}

#[test]
fn internal_visibility() -> Result<()> {
    run("contracts", "internal_visibility")
}

#[test]
fn multi_inheritance() -> Result<()> {
    run("contracts", "multi_inheritance")
}

#[test]
fn public_getters() -> Result<()> {
    run("contracts", "public_getters")
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
