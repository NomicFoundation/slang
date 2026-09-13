use std::sync::Arc;

use crate::ast::{Definition, InterfaceDefinition, LibraryDefinition};
use crate::compilation::{CompilationUnit, FileId};
use crate::tests::support;

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

pub(super) fn build_compilation_unit_from_fixture(files: &[FixtureFile]) -> Arc<CompilationUnit> {
    let unit = support::compile(files.iter().map(|file| (file.id.clone(), file.contents)));

    assert!(
        unit.diagnostics().is_empty(),
        "expected no diagnostics, but found: {:#?}",
        unit.diagnostics()
    );

    Arc::new(unit)
}

pub(super) fn find_interface(unit: &CompilationUnit, name: &str) -> InterfaceDefinition {
    find_definition(unit, name, |definition| match definition {
        Definition::Interface(interface) if interface.name().name() == name => Some(interface),
        _ => None,
    })
}

pub(super) fn find_library(unit: &CompilationUnit, name: &str) -> LibraryDefinition {
    find_definition(unit, name, |definition| match definition {
        Definition::Library(library) if library.name().name() == name => Some(library),
        _ => None,
    })
}

fn find_definition<T>(
    unit: &CompilationUnit,
    name: &str,
    select: impl FnMut(Definition) -> Option<T>,
) -> T {
    unit.all_definitions()
        .find_map(select)
        .unwrap_or_else(|| panic!("`{name}` is declared"))
}

// Fixture build tests

#[test]
fn test_build_counter_fixture() {
    let unit = Counter::build_compilation_unit();
    assert_eq!(3, unit.files().count());
}
