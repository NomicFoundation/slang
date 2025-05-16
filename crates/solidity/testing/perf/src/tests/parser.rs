use std::rc::Rc;

use semver::{BuildMetadata, Prerelease};
use slang_solidity::compilation::CompilationUnit;

use crate::compilation_builder::CompilationBuilder;
use crate::dataset::{load_projects, SolidityProject};

pub fn setup() -> SolidityProject {
    let projects = load_projects().unwrap();
    let project = projects.first().unwrap();
    project.to_owned()
}

pub fn run(project: SolidityProject) -> Rc<CompilationUnit> {
    let mut version = semver::Version::parse(&project.version).unwrap();
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;

    let builder = CompilationBuilder::new(version, project.root, project.entrypoint).unwrap();

    builder.build().unwrap().into()
}
