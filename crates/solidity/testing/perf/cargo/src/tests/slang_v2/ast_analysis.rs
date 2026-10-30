use std::hint::black_box;

use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::ast::{Expression, Identifier};
use slang_solidity_v2::compilation::CompilationUnit;

pub type Input = CompilationUnit;

#[allow(unused)]
pub struct Output {
    pub(crate) unit: CompilationUnit,

    pub(crate) resolved_references_count: usize,
    pub(crate) typed_expressions_count: usize,
}

pub fn setup(project: &str) -> Input {
    super::full_compilation::test(super::full_compilation::setup(project))
}

// Kept out of line: if inlined into the gungraun wrapper, Callgrind mistakes returns into
// the wrapper for fresh entries, and toggles collection off mid-run.
#[inline(never)]
pub fn run(unit: Input) -> Output {
    test(unit)
}

pub fn test(unit: Input) -> Output {
    let mut walk = AnalysisWalk::default();

    for file in unit.files() {
        accept_source_unit(&file.ast(), &mut walk);
    }

    Output {
        unit,

        resolved_references_count: walk.resolved_references_count,
        typed_expressions_count: walk.typed_expressions_count,
    }
}

#[derive(Default)]
struct AnalysisWalk {
    resolved_references_count: usize,
    typed_expressions_count: usize,
}

impl Visitor for AnalysisWalk {
    fn visit_identifier(&mut self, node: &Identifier) {
        if black_box(node.resolve_to_definition()).is_some()
            || black_box(node.resolve_to_built_in()).is_some()
        {
            self.resolved_references_count += 1;
        }
    }

    fn enter_expression(&mut self, node: &Expression) -> bool {
        if black_box(node.get_type()).is_some() {
            self.typed_expressions_count += 1;
        }
        true
    }
}

pub fn count_resolved_references(output: &Output) -> usize {
    output.resolved_references_count
}

pub fn count_typed_expressions(output: &Output) -> usize {
    output.typed_expressions_count
}
