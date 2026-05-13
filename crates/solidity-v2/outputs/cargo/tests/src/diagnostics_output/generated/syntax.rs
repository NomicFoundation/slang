// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

const T: &str = "syntax";

#[test]
fn unexpected_eof() -> Result<()> {
    run(T, "unexpected_eof")
}

#[test]
fn unexpected_terminal() -> Result<()> {
    run(T, "unexpected_terminal")
}

#[test]
fn unsupported_syntax() -> Result<()> {
    run(T, "unsupported_syntax")
}
