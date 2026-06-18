use std::sync::Arc;

use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit, FileId};
use crate::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use crate::utils::LanguageVersion;

mod counter;

pub(super) use counter::Counter;

pub(super) struct FixtureFile {
    pub(crate) id: FileId,
    pub(crate) contents: &'static str,
}

#[macro_export]
macro_rules! define_fixture {
    // Recursive case: consume one file definition
    (@accum [$($acc:expr),*] ; $name:ident ; file : $k:literal, $v:literal $(, $($rest:tt)*)?) => {
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

struct FixtureBuildConfig<'a> {
    files: &'a [FixtureFile],
}

impl CompilationBuilderConfig for FixtureBuildConfig<'_> {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        self.files
            .iter()
            .find(|file| file.id == *file_id)
            .map(|file| file.contents.to_owned())
            .ok_or_else(|| MissingFile {
                reason: "Fixture file not found".to_string(),
            })
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(import_path.into())
    }
}

pub(super) fn build_compilation_unit_from_fixture(files: &[FixtureFile]) -> Arc<CompilationUnit> {
    let version = LanguageVersion::LATEST;
    let target = EvmTarget::LATEST;
    let mut builder = CompilationBuilder::create(version, target, FixtureBuildConfig { files });

    for file in files {
        builder.add_file(file.id.clone());
    }

    let unit = builder.build();

    assert!(
        unit.diagnostics().is_empty(),
        "expected no diagnostics, but found: {:#?}",
        unit.diagnostics()
    );

    Arc::new(unit)
}

// Fixture build tests

#[test]
fn test_build_counter_fixture() {
    let unit = Counter::build_compilation_unit();
    assert_eq!(3, unit.files().count());
}
