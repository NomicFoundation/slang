use std::sync::Arc;

use slang_solidity_v2_ast::ast;
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use super::semantic::File;

pub struct Input {
    pub(crate) files: Vec<File>,
    pub(crate) semantic: Arc<SemanticContext>,
}

pub struct Output {
    pub(crate) json: Vec<String>,
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

// Kept out of line: if inlined into the gungraun wrapper, Callgrind mistakes returns into
// the wrapper for fresh entries, and toggles collection off mid-run.
#[inline(never)]
pub fn run(input: Input) -> Output {
    test(input)
}

/// Computes the storage layout of every contract, and renders it as JSON.
pub fn test(input: Input) -> Output {
    let json = input
        .files
        .iter()
        .map(|file| ast::create_source_unit(file.ir_root(), &input.semantic))
        .flat_map(|source_unit| source_unit.contracts())
        .filter_map(|contract| contract.compute_storage_layout())
        .map(|layout| serde_json::to_string(&layout).expect("the storage layout serializes"))
        .collect();

    Output { json }
}

// Contracts whose storage layout cannot be computed are skipped, so comparing this to the
// number of contracts detects any silently dropped.
pub fn count_storage_layouts(output: &Output) -> usize {
    output.json.len()
}
