use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;

use crate::compilation_builder::create_compilation_unit;
use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> &'static SolidityProject {
    super::setup::setup(project)
}

pub fn run(project: &SolidityProject) -> Rc<CompilationUnit> {
    test(project)
}

pub fn test(project: &SolidityProject) -> Rc<CompilationUnit> {
    let unit = create_compilation_unit(project).unwrap();

    assert!(!unit.files().is_empty());
    for file in unit.files() {
        assert!(file.errors().is_empty());
    }
    unit.into()
}
