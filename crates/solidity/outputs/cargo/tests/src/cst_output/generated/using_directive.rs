// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn destructure_empty() -> Result<()> {
    run("UsingDirective", "destructure_empty")
}

#[test]
fn destructure_multiple() -> Result<()> {
    run("UsingDirective", "destructure_multiple")
}

#[test]
fn destructure_single() -> Result<()> {
    run("UsingDirective", "destructure_single")
}

#[test]
fn path_named() -> Result<()> {
    run("UsingDirective", "path_named")
}

#[test]
fn path_unnamed() -> Result<()> {
    run("UsingDirective", "path_unnamed")
}

#[test]
fn user_defined_operator() -> Result<()> {
    run("UsingDirective", "user_defined_operator")
}
