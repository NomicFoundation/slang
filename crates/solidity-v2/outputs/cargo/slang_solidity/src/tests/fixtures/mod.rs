use std::sync::Arc;

use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::ast::{Definition, LibraryDefinition};
use crate::compilation::{CompilationUnit, FileId, ImportResolver};
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::utils::LanguageVersion;

mod counter;

pub(super) use counter::Counter;

pub(super) struct FixtureFile {
    pub(crate) id: FileId,
    pub(crate) contents: &'static str,
}

#[macro_export]
macro_rules! define_fixture {
    // Recursive case: consume one file definition.
    (@accum [$($acc:expr),*] ; $name:ident ; file : $k:literal, $v:expr $(, $($rest:tt)*)?) => {
        define_fixture!(
            @accum [$($acc,)* $crate::tests::fixtures::FixtureFile { id: $k.into(), contents: $v }] ;
            $name ;
            $($($rest)*)?);
    };

    // Base case: emit the declaration
    (@accum [$($acc:expr),*] ; $name:ident ;) => {
        pub(crate) struct $name;

        impl $name {
            pub(crate) fn build_compilation_unit(
            ) -> std::sync::Arc<$crate::compilation::CompilationUnit> {
                let files = vec![$($acc),*];
                $crate::tests::fixtures::build_compilation_unit_from_fixture(&files)
            }
        }
    };

    // Entry point
    ($name:ident, $($rest:tt)*) => {
        define_fixture!(@accum [] ; $name ; $($rest)*);
    };
}

struct FixtureImportResolver;

impl ImportResolver for FixtureImportResolver {
    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(import_path.into())
    }
}

pub(super) fn build_compilation_unit_from_fixture(files: &[FixtureFile]) -> Arc<CompilationUnit> {
    let unit = CompilationUnit::create(
        LanguageVersion::LATEST,
        EvmTarget::LATEST,
        files
            .iter()
            .map(|file| (file.id.clone(), file.contents.to_owned())),
        FixtureImportResolver,
    );

    assert!(
        unit.diagnostics().is_empty(),
        "expected no diagnostics, but found: {:#?}",
        unit.diagnostics()
    );

    Arc::new(unit)
}

pub(super) fn find_library(unit: &CompilationUnit, name: &str) -> LibraryDefinition {
    unit.all_definitions()
        .filter_map(|definition| match definition {
            Definition::Library(library) => Some(library),
            _ => None,
        })
        .find(|library| library.name().name() == name)
        .unwrap_or_else(|| panic!("library `{name}` is declared"))
}

// Fixture build tests

#[test]
fn test_build_counter_fixture() {
    let unit = Counter::build_compilation_unit();
    assert_eq!(3, unit.files().count());
}
