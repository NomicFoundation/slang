// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn wrap_unwrap() -> Result<()> {
    run("user_types", "wrap_unwrap")
}
