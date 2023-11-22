// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn destructure_empty() -> Result<()> {
    return run("UsingDirective", "destructure_empty");
}

#[test]
fn destructure_multiple() -> Result<()> {
    return run("UsingDirective", "destructure_multiple");
}

#[test]
fn destructure_single() -> Result<()> {
    return run("UsingDirective", "destructure_single");
}

#[test]
fn path_named() -> Result<()> {
    return run("UsingDirective", "path_named");
}

#[test]
fn path_unnamed() -> Result<()> {
    return run("UsingDirective", "path_unnamed");
}

#[test]
fn user_defined_operator() -> Result<()> {
    return run("UsingDirective", "user_defined_operator");
}
