use anyhow::Result;

mod chained_imports;
mod counter;
mod overrides;

pub(crate) use chained_imports::ChainedImports;
pub(crate) use counter::Counter;
pub(crate) use overrides::Overrides;

#[test]
fn test_build_chained_imports_fixture() -> Result<()> {
    let unit = ChainedImports::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(3, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_counter_fixture() -> Result<()> {
    let unit = Counter::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(3, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_overrides_fixture() -> Result<()> {
    let unit = Overrides::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}
