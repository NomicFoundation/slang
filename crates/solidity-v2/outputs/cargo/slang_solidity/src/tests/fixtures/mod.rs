use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;

use crate::compilation::builder::{CompilationBuilder, CompilationBuilderConfig};
use crate::compilation::unit::CompilationUnit;

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
            ) -> std::rc::Rc<$crate::compilation::unit::CompilationUnit> {
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
    fn read_file(&mut self, file_id: &str) -> Result<String, String> {
        self.files
            .iter()
            .find(|file| file.id == file_id)
            .map(|file| file.contents.to_owned())
            .ok_or_else(|| format!("fixture file not found: {file_id}"))
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path: &str,
    ) -> Result<String, String> {
        import_path
            .strip_prefix(|c: char| matches!(c, '"' | '\''))
            .and_then(|p| p.strip_suffix(|c: char| matches!(c, '"' | '\'')))
            .map(|p| p.to_owned())
            .ok_or_else(|| format!("failed to resolve import: {import_path}"))
    }
}

pub(super) fn build_compilation_unit_from_fixture(
    files: &[FixtureFile<'_>],
) -> Rc<CompilationUnit> {
    let version = LanguageVersion::V0_8_30;
    let mut builder = CompilationBuilder::create(version, FixtureBuildConfig { files });

    for file in files {
        builder.add_file(file.id.to_owned());
    }

    Rc::new(builder.build())
}

// Fixture build tests

#[test]
fn test_build_abi_with_tuples_fixture() {
    let unit = AbiWithTuples::build_compilation_unit();
    assert_eq!(1, unit.files().len());
}

#[test]
fn test_build_chained_imports_fixture() {
    let unit = ChainedImports::build_compilation_unit();
    assert_eq!(3, unit.files().len());
}

#[test]
fn test_build_counter_fixture() {
    let unit = Counter::build_compilation_unit();
    assert_eq!(3, unit.files().len());
}

#[test]
fn test_build_full_abi_fixture() {
    let unit = FullAbi::build_compilation_unit();
    assert_eq!(1, unit.files().len());
}

#[test]
fn test_build_overrides_fixture() {
    let unit = Overrides::build_compilation_unit();
    assert_eq!(1, unit.files().len());
}

#[test]
fn test_build_storage_layout_fixture() {
    let unit = StorageLayout::build_compilation_unit();
    assert_eq!(1, unit.files().len());
}
