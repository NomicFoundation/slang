// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn address() -> Result<()> {
    run("using", "address")
}

#[test]
fn binding_enum() -> Result<()> {
    run("using", "binding_enum")
}

#[test]
fn binding_enum_member() -> Result<()> {
    run("using", "binding_enum_member")
}

#[test]
fn casting() -> Result<()> {
    run("using", "casting")
}

#[test]
fn chained_calls() -> Result<()> {
    run("using", "chained_calls")
}

#[test]
fn deconstruction() -> Result<()> {
    run("using", "deconstruction")
}

#[test]
fn elementary() -> Result<()> {
    run("using", "elementary")
}

#[test]
fn elementary_arrays() -> Result<()> {
    run("using", "elementary_arrays")
}

#[test]
fn function_types() -> Result<()> {
    run("using", "function_types")
}

#[test]
fn global() -> Result<()> {
    run("using", "global")
}

#[test]
fn in_contract() -> Result<()> {
    run("using", "in_contract")
}

#[test]
fn in_library() -> Result<()> {
    run("using", "in_library")
}

#[test]
fn inherit_extension() -> Result<()> {
    run("using", "inherit_extension")
}

#[test]
fn inherited_types() -> Result<()> {
    run("using", "inherited_types")
}

#[test]
fn mappings() -> Result<()> {
    run("using", "mappings")
}

#[test]
fn on_interfaces_inherited() -> Result<()> {
    run("using", "on_interfaces_inherited")
}

#[test]
fn on_parameters() -> Result<()> {
    run("using", "on_parameters")
}

#[test]
fn on_state_var_initialization() -> Result<()> {
    run("using", "on_state_var_initialization")
}

#[test]
fn on_super_calls() -> Result<()> {
    run("using", "on_super_calls")
}

#[test]
fn qualified_type() -> Result<()> {
    run("using", "qualified_type")
}

#[test]
fn star() -> Result<()> {
    run("using", "star")
}

#[test]
fn star_in_library() -> Result<()> {
    run("using", "star_in_library")
}

#[test]
fn star_inherited() -> Result<()> {
    run("using", "star_inherited")
}

#[test]
fn top_level() -> Result<()> {
    run("using", "top_level")
}

#[test]
fn uint_alias() -> Result<()> {
    run("using", "uint_alias")
}

#[test]
fn user_types() -> Result<()> {
    run("using", "user_types")
}
