use slang_solidity_v2_cst::structured_cst::nodes::{SourceUnit, SourceUnitMember};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::dataset::SolidityProject;
use crate::tests::slang_v2::common::parse_version;

pub type Input = &'static SolidityProject;

pub struct Output {
    pub(crate) source_units: Vec<(String, SourceUnit)>,
}

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

pub fn run(project: Input) -> Output {
    let lang_version = parse_version(project);
    let mut source_units = Vec::new();
    for (key, source) in &project.sources {
        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse(key, source, lang_version);
        assert!(diagnostics.is_empty());

        source_units.push((key.clone(), source_unit));
    }
    assert!(!source_units.is_empty());

    Output { source_units }
}

pub fn test(project: Input) -> Output {
    run(project)
}

pub fn count_contracts(output: &Output) -> usize {
    let mut contract_count = 0;
    for (_, source_unit) in &output.source_units {
        for member in &source_unit.members.elements {
            match member {
                SourceUnitMember::ContractDefinition(_)
                | SourceUnitMember::InterfaceDefinition(_)
                | SourceUnitMember::LibraryDefinition(_) => contract_count += 1,
                _ => {}
            }
        }
    }
    contract_count
}
