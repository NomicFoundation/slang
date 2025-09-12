use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;

use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> &'static SolidityProject {
    super::setup::setup(project)
}

pub fn run(project: &SolidityProject) -> Rc<CompilationUnit> {
    test(project)
}

pub fn test(project: &SolidityProject) -> Rc<CompilationUnit> {
    let unit = project.build_compilation_unit().unwrap();

    assert!(!unit.files().is_empty());
    for file in unit.files() {
        assert!(file.errors().is_empty());
    }
    unit.into()
}
