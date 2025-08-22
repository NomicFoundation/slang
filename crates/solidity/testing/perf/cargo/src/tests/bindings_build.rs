use std::rc::Rc;

use slang_solidity::bindings::BindingGraph;
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_utils::config::Library;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project, Library::Slang).unwrap())
}

pub fn run(unit: Rc<CompilationUnit>) -> Rc<BindingGraph> {
    unit.binding_graph().to_owned()
}
