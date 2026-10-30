use slang_solidity_v2::ast::Identifier;
use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::compilation::CompilationUnit;

pub type Input = CompilationUnit;

#[allow(unused)]
pub struct Output {
    pub(crate) unit: CompilationUnit,
    pub(crate) identifier_count: usize,
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
    let mut walk = NodeWalk::default();

    for file in unit.files() {
        accept_source_unit(&file.ast(), &mut walk);
    }

    Output {
        unit,
        identifier_count: walk.identifier_count,
    }
}

#[derive(Default)]
struct NodeWalk {
    identifier_count: usize,
}

impl Visitor for NodeWalk {
    fn visit_identifier(&mut self, _node: &Identifier) {
        self.identifier_count += 1;
    }
}

pub fn count_identifiers(output: &Output) -> usize {
    output.identifier_count
}
