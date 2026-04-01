use semver::{BuildMetadata, Prerelease};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{SourceUnit, SourceUnitMember};
use slang_solidity_v2_parser::Parser;

use crate::dataset::SolidityProject;

pub fn setup(project: &str) -> &'static SolidityProject {
    crate::tests::setup::setup(project)
}

pub fn run(project: &SolidityProject) {
    test(project);
}

pub fn test(project: &SolidityProject) -> Vec<SourceUnit> {
    let lang_version = parse_version(project);
    let mut source_units = Vec::new();
    for source in project.sources.values() {
        let source_unit = Parser::parse(source, lang_version).unwrap();
        source_units.push(source_unit);
    }
    assert!(!source_units.is_empty());

    source_units
}

pub fn count_contracts(source_units: &Vec<SourceUnit>) -> usize {
    let mut contract_count = 0;
    for source_unit in source_units {
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

fn parse_version(project: &SolidityProject) -> LanguageVersion {
    let mut version = semver::Version::parse(&project.compiler_version).unwrap();
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;
    LanguageVersion::try_from(version).unwrap()
}
