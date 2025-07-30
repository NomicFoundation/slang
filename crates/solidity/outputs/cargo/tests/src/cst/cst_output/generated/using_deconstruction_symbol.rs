// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "UsingDeconstructionSymbol";

#[test]
fn identifier_path() -> Result<()> {
    run(T, "identifier_path")
}

#[test]
fn identifier_path_as_operator() -> Result<()> {
    run(T, "identifier_path_as_operator")
}

#[test]
fn single_id() -> Result<()> {
    run(T, "single_id")
}

#[test]
fn single_id_as_operator() -> Result<()> {
    run(T, "single_id_as_operator")
}
