// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn funcalls() -> Result<()> {
    run("expressions", "funcalls.sol")
}

#[test]
fn funcalls_named_args() -> Result<()> {
    run("expressions", "funcalls_named_args.sol")
}
