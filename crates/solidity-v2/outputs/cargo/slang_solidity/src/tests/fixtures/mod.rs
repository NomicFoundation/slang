use std::rc::Rc;

use anyhow::Result;

use crate::compilation::{
    CompilationBuilder, CompilationBuilderConfig, CompilationBuilderError, CompilationUnit,
};
use crate::utils::LanguageVersion;

mod abi_with_tuples;
mod chained_imports;
mod counter;
mod full_abi;
mod overrides;
mod storage_layout;

pub(super) use abi_with_tuples::AbiWithTuples;
pub(super) use chained_imports::ChainedImports;
pub(super) use counter::Counter;
pub(super) use full_abi::FullAbi;
pub(super) use overrides::Overrides;
pub(super) use storage_layout::StorageLayout;

pub(super) struct FixtureFile<'a> {
    id: &'a str,
    contents: &'a str,
}

#[macro_export]
macro_rules! define_fixture {
    // Recursive case: consume one file definition
    (@accum [$($acc:expr),*] ; $name:ident ; file : $k:literal, $v:literal $(, $($rest:tt)*)?) => {
        define_fixture!(
            @accum [$($acc,)* $crate::tests::fixtures::FixtureFile { id: $k, contents: $v }] ;
            $name ;
            $($($rest)*)?);
    };

    // Base case: emit the declaration
    (@accum [$($acc:expr),*] ; $name:ident ;) => {
        const FILES: &[$crate::tests::fixtures::FixtureFile<'_>] = &[$($acc),*];
        pub(crate) struct $name;

        impl $name {
            pub(crate) fn build_compilation_unit(
            ) -> anyhow::Result<std::rc::Rc<$crate::compilation::CompilationUnit>> {
                $crate::tests::fixtures::build_compilation_unit_from_fixture(FILES)
            }
        }
    };

    // Entry point
    ($name:ident, $($rest:tt)*) => {
        define_fixture!(@accum [] ; $name ; $($rest)*);
    };
}

struct FixtureBuildConfig<'a> {
    files: &'a [FixtureFile<'a>],
}

impl CompilationBuilderConfig for FixtureBuildConfig<'_> {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        Ok(self
            .files
            .iter()
            .find(|file| file.id == file_id)
            .map(|file| file.contents.to_owned()))
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path: &str,
    ) -> Result<Option<String>> {
        let path = import_path
            .strip_prefix(|c: char| matches!(c, '"' | '\''))
            .and_then(|p| p.strip_suffix(|c: char| matches!(c, '"' | '\'')));
        Ok(path.map(|p| p.to_owned()))
    }
}

pub(super) fn build_compilation_unit_from_fixture(
    files: &[FixtureFile<'_>],
) -> Result<Rc<CompilationUnit>> {
    let version = LanguageVersion::V0_8_30;
    let mut builder = CompilationBuilder::create(version, FixtureBuildConfig { files });

    for file in files {
        builder.add_file(file.id).map_err(|error| match error {
            CompilationBuilderError::ParserError(error) => {
                anyhow::anyhow!("Parser error: {error:?}")
            }
            CompilationBuilderError::UserError(error) => error,
        })?;
    }

    Ok(Rc::new(builder.build()))
}

// Fixture build tests

#[test]
fn test_build_abi_with_tuples_fixture() -> Result<()> {
    let unit = AbiWithTuples::build_compilation_unit()?;
    assert_eq!(1, unit.files().len());
    Ok(())
}

#[test]
fn test_build_chained_imports_fixture() -> Result<()> {
    let unit = ChainedImports::build_compilation_unit()?;
    assert_eq!(3, unit.files().len());
    Ok(())
}

#[test]
fn test_build_counter_fixture() -> Result<()> {
    let unit = Counter::build_compilation_unit()?;
    assert_eq!(3, unit.files().len());
    Ok(())
}

#[test]
fn test_build_full_abi_fixture() -> Result<()> {
    let unit = FullAbi::build_compilation_unit()?;
    assert_eq!(1, unit.files().len());
    Ok(())
}

#[test]
fn test_build_overrides_fixture() -> Result<()> {
    let unit = Overrides::build_compilation_unit()?;
    assert_eq!(1, unit.files().len());
    Ok(())
}

#[test]
fn test_build_storage_layout_fixture() -> Result<()> {
    let unit = StorageLayout::build_compilation_unit()?;
    assert_eq!(1, unit.files().len());
    Ok(())
}
