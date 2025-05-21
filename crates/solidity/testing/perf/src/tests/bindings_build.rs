use std::rc::Rc;

use slang_solidity::bindings::BindingGraph;
use slang_solidity::compilation::CompilationUnit;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::parser::setup(project))
}

pub fn run(unit: Rc<CompilationUnit>) -> Rc<BindingGraph> {
    unit.binding_graph().to_owned()
}
