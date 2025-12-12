use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;

use super::binder_v2_run::BuiltSemanticAnalysis;

pub fn setup(project: &str) -> BuiltSemanticAnalysis {
    let unit: Rc<CompilationUnit> = super::setup::setup(project)
        .build_compilation_unit()
        .unwrap()
        .into();
    let semantic_analysis = Rc::clone(unit.semantic_analysis());
    BuiltSemanticAnalysis {
        unit,
        semantic_analysis,
    }
}
