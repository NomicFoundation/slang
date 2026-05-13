use std::rc::Rc;

use slang_solidity_v2_ast::{abi, ast};
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use super::semantic::File;
use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> (&'static SolidityProject, Vec<File>, Rc<SemanticContext>) {
    let (project, files) = super::semantic::setup(project);
    let semantic = super::semantic::test((project, files.clone()));
    (project, files, Rc::new(semantic))
}

pub fn run(
    project: &'static SolidityProject,
    files: Vec<File>,
    semantic: Rc<SemanticContext>,
) -> Vec<abi::ContractAbi> {
    test((project, files, semantic))
}

pub fn test(
    (_project, files, semantic): (&'static SolidityProject, Vec<File>, Rc<SemanticContext>),
) -> Vec<abi::ContractAbi> {
    files
        .iter()
        .flat_map(|file| ast::create_source_unit(file.ir_root(), &semantic).compute_contracts_abi())
        .collect()
}

// `compute_contracts_abi` already filters out abstract contracts (and
// interfaces/libraries are not in `SourceUnit::contracts()` at all), so the
// length of its output is exactly the number of concrete contracts whose ABI
// could be computed.
pub fn count_concrete_contracts(contracts: &[abi::ContractAbi]) -> usize {
    contracts.len()
}
