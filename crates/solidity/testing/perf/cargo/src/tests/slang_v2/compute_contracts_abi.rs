use std::sync::Arc;

use slang_solidity_v2_ast::{abi, ast};
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
    pub(crate) abi: Vec<abi::ContractAbi>,
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
    let abi = input
        .files
        .iter()
        .flat_map(|file| {
            ast::create_source_unit(file.ir_root(), &input.semantic).compute_contracts_abi()
        })
        .collect();
    Output {
        files: input.files,
        semantic: input.semantic,
        abi,
    }
}

// `compute_contracts_abi` already filters out abstract contracts (and
// interfaces/libraries are not in `SourceUnit::contracts()` at all), so the
// length of its output is exactly the number of concrete contracts whose ABI
// could be computed.
pub fn count_concrete_contracts(output: &Output) -> usize {
    output.abi.len()
}
