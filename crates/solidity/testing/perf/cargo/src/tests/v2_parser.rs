use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    go(project);
}

pub fn test(project: &SolidityProject) -> usize {
    go(project)
}

fn go(project: &SolidityProject) -> usize {
    for source in project.sources.values() {
        Parser::parse(source, LanguageVersion::V0_8_30).expect("V2 parser failed");
    }

    // V2 parser CST does not yet support contract counting
    0
}
