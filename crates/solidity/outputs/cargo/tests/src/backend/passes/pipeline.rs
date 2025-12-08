use anyhow::Result;

use crate::backend::sample::build_compilation_unit;

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(3, semantic_analysis.files().len());

    Ok(())
}
