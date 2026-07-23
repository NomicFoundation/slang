use std::sync::Arc;

use slang_solidity_v2_ast::ast::visitor::{accept_source_unit, Visitor};
use slang_solidity_v2_ast::ast::{self, Identifier};
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use super::semantic::File;

pub struct Input {
    pub(crate) files: Vec<File>,
    pub(crate) semantic: Arc<SemanticContext>,
}

#[allow(unused)]
pub struct Output {
    pub(crate) files: Vec<File>,
    pub(crate) semantic: Arc<SemanticContext>,
    pub(crate) identifier_count: usize,
}

pub fn setup(project: &str) -> Input {
    let semantic_input = super::semantic::setup(project);
    let files = semantic_input.files.clone();
    let semantic_output = super::semantic::test(semantic_input);

    Input {
        files,
        semantic: Arc::new(semantic_output),
    }
}

pub fn run(input: Input) -> Output {
    test(input)
}

pub fn test(input: Input) -> Output {
    let mut walk = NodeWalk::default();

    for file in &input.files {
        let source_unit = ast::create_source_unit(file.ir_root(), &input.semantic);
        accept_source_unit(&source_unit, &mut walk);
    }

    Output {
        files: input.files,
        semantic: input.semantic,
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
