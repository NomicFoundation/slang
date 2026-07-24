use std::hint::black_box;
use std::sync::Arc;

use slang_solidity_v2_ast::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2_ast::ast::{self, Expression, Identifier};
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

    pub(crate) resolved_references_count: usize,
    pub(crate) typed_expressions_count: usize,
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
    let mut walk = AnalysisWalk::default();

    for file in &input.files {
        let source_unit = ast::create_source_unit(file.ir_root(), &input.semantic);
        accept_source_unit(&source_unit, &mut walk);
    }

    Output {
        files: input.files,
        semantic: input.semantic,

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
