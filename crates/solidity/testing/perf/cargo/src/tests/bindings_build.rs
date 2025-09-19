use std::rc::Rc;

use slang_solidity::bindings::BindingGraph;
use slang_solidity::compilation::CompilationUnit;

#[derive(Clone)]
pub struct BuiltBindingGraph {
    pub unit: Rc<CompilationUnit>,
    pub binding_graph: Rc<BindingGraph>,
}

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project))
}

pub fn run(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
    test(unit)
}

pub fn test(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
    let binding_graph = unit.binding_graph().to_owned();
    BuiltBindingGraph {
        unit,
        binding_graph,
    }
}
