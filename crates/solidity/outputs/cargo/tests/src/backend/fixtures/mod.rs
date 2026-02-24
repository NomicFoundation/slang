use anyhow::Result;

mod abi_with_tuples;
mod chained_imports;
mod counter;
mod full_abi;
mod overrides;
mod storage_layout;

pub(crate) use abi_with_tuples::AbiWithTuples;
pub(crate) use chained_imports::ChainedImports;
pub(crate) use counter::Counter;
pub(crate) use full_abi::FullAbi;
pub(crate) use overrides::Overrides;
pub(crate) use storage_layout::StorageLayout;

#[test]
fn test_build_abi_with_tuples_fixture() -> Result<()> {
    let unit = AbiWithTuples::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}

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
fn test_build_full_abi_fixture() -> Result<()> {
    let unit = FullAbi::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_overrides_fixture() -> Result<()> {
    let unit = Overrides::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}

#[test]
fn test_build_storage_layout_fixture() -> Result<()> {
    let unit = StorageLayout::build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(1, semantic_analysis.files().len());

    Ok(())
}
