use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit as InputSourceUnit;
use slang_solidity_v2_semantic::ir::{self, SourceUnit, SourceUnitMember};

pub fn setup(project: &str) -> Vec<InputSourceUnit> {
    super::parser::test(super::parser::setup(project))
}

pub fn run(input: Vec<InputSourceUnit>) -> Vec<SourceUnit> {
    let mut ir_source_units = Vec::new();
    for source in input {
        ir_source_units.push(ir::build(&source));
    }
    ir_source_units
}

pub fn test(input: Vec<InputSourceUnit>) -> Vec<SourceUnit> {
    run(input)
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
