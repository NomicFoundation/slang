use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_utils::config::Library;

use crate::compilation_builder::create_compilation_unit;
use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> &'static SolidityProject {
    super::setup::setup(project, Library::Slang).unwrap()
}

pub fn run(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    let unit = create_compilation_unit(project).unwrap();

    assert_ne!(unit.files().len(), 0);
    for file in unit.files() {
        assert!(file.errors().is_empty());
    }
    unit.into()
}
