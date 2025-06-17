use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::diagnostic::{Diagnostic, Severity};
use slang_solidity::parser::ParseError;

use crate::compilation_builder::CompilationBuilder;
use crate::dataset::{load_projects, SolidityProject};

pub fn setup(project: &str) -> &'static SolidityProject {
    load_projects().get(project).unwrap()
}

pub fn run(project: &'static SolidityProject) -> Rc<CompilationUnit> {
    let mut builder = CompilationBuilder::new(project).unwrap();

    let unit = builder.build().unwrap();
    assert_ne!(unit.files().len(), 0);
    for file in unit.files() {
        let errors: Vec<_> = file
            .errors()
            .iter()
            .filter(|e| e.severity() == Severity::Error)
            .collect();
        debug_assert_eq!(errors, Vec::<&ParseError>::new());
    }
    unit.into()
}
