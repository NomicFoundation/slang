use crate::cst_output::run;
use anyhow::Result;

#[test]
fn empty_file() -> Result<()> {
    return run("source_unit", "empty_file");
}
