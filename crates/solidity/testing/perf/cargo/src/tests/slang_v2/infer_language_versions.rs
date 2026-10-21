//! Infers the language versions of every source in the project from its
//! `pragma solidity` directives, through the public v2 API.

use semver::Version;
use slang_solidity_v2::utils::{VersionConstraint, infer_language_versions};

use crate::dataset::SolidityProject;

pub type Input = &'static SolidityProject;
pub type Output = Vec<VersionConstraint>;

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

// Kept out of line: if inlined into the gungraun wrapper, Callgrind mistakes returns into
// the wrapper for fresh entries, and toggles collection off mid-run.
#[inline(never)]
pub fn run(project: Input) -> Output {
    project
        .sources
        .values()
        .map(String::as_str)
        .map(infer_language_versions)
        .collect()
}

pub fn test(project: Input) -> Output {
    let output = run(project);

    // Every source was compiled with the project's compiler version, so each of
    // them has to admit it.
    let compiler_version = Version::parse(&project.compiler_version).unwrap();
    for constraint in &output {
        assert!(
            constraint.matches(compiler_version.clone()),
            "{compiler_version} is not admitted by {constraint:?}"
        );
    }

    output
}

pub fn count_sources(output: &Output) -> usize {
    output.len()
}
