use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::parser::ParseError;

use crate::compilation_builder::CompilationBuilder;
use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> &'static SolidityProject {
    super::setup::setup(project)
}

pub fn run(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    let mut builder = CompilationBuilder::new(project).unwrap();

    let unit = builder.build().unwrap();
    assert_ne!(unit.files().len(), 0);
    for file in unit.files() {
        assert_eq!(file.errors(), &Vec::<ParseError>::new());
    }
    unit.into()
}
