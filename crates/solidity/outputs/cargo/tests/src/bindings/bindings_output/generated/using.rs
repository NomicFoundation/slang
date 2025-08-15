// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "using";

#[test]
fn address() -> Result<()> {
    run(T, "address")
}

#[test]
fn binding_enum() -> Result<()> {
    run(T, "binding_enum")
}

#[test]
fn binding_enum_member() -> Result<()> {
    run(T, "binding_enum_member")
}

#[test]
fn built_ins_results() -> Result<()> {
    run(T, "built_ins_results")
}

#[test]
fn can_override_built_ins() -> Result<()> {
    run(T, "can_override_built_ins")
}

#[test]
fn casting() -> Result<()> {
    run(T, "casting")
}

#[test]
fn chained_calls() -> Result<()> {
    run(T, "chained_calls")
}

#[test]
fn deconstruction() -> Result<()> {
    run(T, "deconstruction")
}

#[test]
fn duplicate_directives() -> Result<()> {
    run(T, "duplicate_directives")
}

#[test]
fn elementary() -> Result<()> {
    run(T, "elementary")
}

#[test]
fn elementary_arrays() -> Result<()> {
    run(T, "elementary_arrays")
}

#[test]
fn for_fixed_arrays() -> Result<()> {
    run(T, "for_fixed_arrays")
}

#[test]
fn function_types() -> Result<()> {
    run(T, "function_types")
}

#[test]
fn global() -> Result<()> {
    run(T, "global")
}

#[test]
fn global_and_local() -> Result<()> {
    run(T, "global_and_local")
}

#[test]
fn global_multi_file() -> Result<()> {
    run(T, "global_multi_file")
}

#[test]
fn in_contract() -> Result<()> {
    run(T, "in_contract")
}

#[test]
fn in_interface() -> Result<()> {
    run(T, "in_interface")
}

#[test]
fn in_library() -> Result<()> {
    run(T, "in_library")
}

#[test]
fn inherit_extension() -> Result<()> {
    run(T, "inherit_extension")
}

#[test]
fn inherited_types() -> Result<()> {
    run(T, "inherited_types")
}

#[test]
fn mappings() -> Result<()> {
    run(T, "mappings")
}

#[test]
fn named_args() -> Result<()> {
    run(T, "named_args")
}

#[test]
fn on_interfaces_inherited() -> Result<()> {
    run(T, "on_interfaces_inherited")
}

#[test]
fn on_parameters() -> Result<()> {
    run(T, "on_parameters")
}

#[test]
fn on_state_var_initialization() -> Result<()> {
    run(T, "on_state_var_initialization")
}

#[test]
fn on_super_calls() -> Result<()> {
    run(T, "on_super_calls")
}

#[test]
fn overloaded_functions_attached() -> Result<()> {
    run(T, "overloaded_functions_attached")
}

#[test]
fn qualified_type() -> Result<()> {
    run(T, "qualified_type")
}

#[test]
fn star() -> Result<()> {
    run(T, "star")
}

#[test]
fn star_in_library() -> Result<()> {
    run(T, "star_in_library")
}

#[test]
fn star_inherited() -> Result<()> {
    run(T, "star_inherited")
}

#[test]
fn top_level() -> Result<()> {
    run(T, "top_level")
}

#[test]
fn uint_alias() -> Result<()> {
    run(T, "uint_alias")
}

#[test]
fn user_types() -> Result<()> {
    run(T, "user_types")
}
