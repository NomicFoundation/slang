use inflector::Inflector;
use semver::{BuildMetadata, Prerelease, Version};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::dataset::SolidityProject;

pub fn parse_version(project: &SolidityProject) -> LanguageVersion {
    let mut version = Version::parse(&project.compiler_version).unwrap();
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;
    LanguageVersion::try_from(version).unwrap()
}

pub fn parse_evm_target(project: &SolidityProject) -> EvmTarget {
    EvmTarget::try_from(project.evm_version.to_pascal_case().as_str()).unwrap()
}
