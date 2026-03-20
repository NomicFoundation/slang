use semver::{BuildMetadata, Prerelease};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::SourceUnitMember;
use slang_solidity_v2_parser::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    let lang_version = parse_version(project);
    for source in project.sources.values() {
        let _result = Parser::parse(source, lang_version).unwrap();
    }
}

pub fn test(project: &SolidityProject) -> usize {
    let lang_version = parse_version(project);
    let mut contract_count = 0;
    for source in project.sources.values() {
        let source_unit = Parser::parse(source, lang_version).unwrap();
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
