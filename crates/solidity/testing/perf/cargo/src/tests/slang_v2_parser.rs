use semver::{BuildMetadata, Prerelease};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    let lang_version = parse_version(project);
    for source in project.sources.values() {
        let _result = Parser::parse(source, lang_version).unwrap();
    }
}

pub fn test(project: &SolidityProject) {
    run(project); // no contract counting available in v2 CST yet
}

fn parse_version(project: &SolidityProject) -> LanguageVersion {
    let mut version = semver::Version::parse(&project.compiler_version).unwrap();
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;
    LanguageVersion::try_from(version).unwrap()
}
