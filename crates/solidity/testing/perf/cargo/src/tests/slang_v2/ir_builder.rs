use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit as InputSourceUnit;
use slang_solidity_v2_ir::ir::{self, SourceUnit, SourceUnitMember};

use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> (&'static SolidityProject, Vec<(String, InputSourceUnit)>) {
    let project = super::parser::setup(project);
    let parsed = super::parser::test(project);
    (project, parsed)
}

pub fn run(
    project: &'static SolidityProject,
    sources: Vec<(String, InputSourceUnit)>,
) -> Vec<SourceUnit> {
    test(project, sources)
}

pub fn test(
    project: &'static SolidityProject,
    sources: Vec<(String, InputSourceUnit)>,
) -> Vec<SourceUnit> {
    let mut ir_source_units = Vec::new();
    for (name, source) in sources {
        ir_source_units.push(ir::build(
            &source,
            project
                .sources
                .get(&name)
                .expect("Source not found in project"),
        ));
    }
    ir_source_units
}

pub fn count_contracts(source_units: &Vec<SourceUnit>) -> usize {
    let mut contract_count = 0;
    for source_unit in source_units {
        for member in &source_unit.members {
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
