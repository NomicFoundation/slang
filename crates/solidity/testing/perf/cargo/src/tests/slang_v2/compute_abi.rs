use std::sync::Arc;

use slang_solidity_v2_ast::abi::ContractAbi;
use slang_solidity_v2_ast::ast;
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
    pub(crate) abi: Vec<ContractAbi>,
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

/// Computes the ABI of every contract, interface, and library, and renders it as JSON.
pub fn test(input: Input) -> Output {
    let abi: Vec<ContractAbi> = input
        .files
        .iter()
        .flat_map(|file| ast::create_source_unit(file.ir_root(), &input.semantic).compute_abi())
        .collect();
    let json = abi
        .iter()
        .map(|abi| serde_json::to_string(abi).expect("the ABI serializes"))
        .collect();
    Output {
        files: input.files,
        semantic: input.semantic,
        abi,
        json,
    }
}

// Definitions whose ABI cannot be computed are skipped, so comparing this to the
// number of contracts, interfaces, and libraries detects any silently dropped.
pub fn count_abis(output: &Output) -> usize {
    output.abi.len()
}
