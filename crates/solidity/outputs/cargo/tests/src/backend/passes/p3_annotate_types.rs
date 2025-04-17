use anyhow::{anyhow, Result};
use slang_solidity::backend::passes::{
    p0_build_ast, p1_flatten_contracts, p2_collect_types, p3_annotate_types,
};

#[test]
fn test_annotate_types_pass() -> Result<()> {
    let unit = super::build_compilation_unit()?;

    let data = p0_build_ast::run(&unit).map_err(|s| anyhow!(s))?;
    let data = p1_flatten_contracts::run(data);
    let data = p2_collect_types::run(data);
    let data = p3_annotate_types::run(data);

    assert_eq!(2, data.files.len());

    Ok(())
}
