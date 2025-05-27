use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;

use crate::compilation_builder::CompilationBuilder;
use crate::dataset::{load_projects, SolidityProject};

pub fn setup(project: &str) -> &'static SolidityProject {
    load_projects().get(project).unwrap()
}

pub fn run(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    let mut builder = CompilationBuilder::new(project).unwrap();

    let unit = builder.build().unwrap();
    assert_ne!(unit.files().len(), 0);
    unit.into()
}
