use std::sync::Arc;

use crate::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use crate::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use crate::utils::LanguageVersion;

mod counter;

pub(super) use counter::Counter;

pub(super) struct FixtureFile<'a> {
    pub(crate) id: &'a str,
    pub(crate) contents: &'a str,
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
        pub(crate) struct $name;

        impl $name {
            const FILES: &[$crate::tests::fixtures::FixtureFile<'_>] = &[$($acc),*];

            pub(crate) fn build_compilation_unit(
            ) -> std::sync::Arc<$crate::compilation::CompilationUnit> {
                $crate::tests::fixtures::build_compilation_unit_from_fixture(Self::FILES)
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
    fn read_file(&mut self, file_id: &str) -> Result<String, MissingFile> {
        self.files
            .iter()
            .find(|file| file.id == file_id)
            .map(|file| file.contents.to_owned())
            .ok_or_else(|| MissingFile {
                reason: "Fixture file not found".to_string(),
            })
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path: &str,
    ) -> Result<String, UnresolvedImport> {
        Ok(import_path.to_owned())
    }
}

pub(super) fn build_compilation_unit_from_fixture(
    files: &[FixtureFile<'_>],
) -> Arc<CompilationUnit> {
    let version = LanguageVersion::LATEST;
    let mut builder = CompilationBuilder::create(version, FixtureBuildConfig { files });

    for file in files {
        builder.add_file(file.id.to_owned());
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
    assert_eq!(3, unit.file_ids().len());
}
