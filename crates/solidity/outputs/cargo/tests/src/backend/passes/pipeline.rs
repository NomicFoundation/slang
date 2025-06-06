use anyhow::{anyhow, Result};
use slang_solidity::backend::passes;

use crate::backend::passes::build_compilation_unit;

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let data = passes::p0_build_ast::run(unit).map_err(|s| anyhow!(s))?;
    let data = passes::p1_flatten_contracts::run(data);
    assert_eq!(2, data.files.len());

    Ok(())
}
