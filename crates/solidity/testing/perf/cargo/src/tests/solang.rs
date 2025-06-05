use std::ffi::OsStr;

use solang::file_resolver::FileResolver;
use solang_parser::diagnostics::Diagnostic;

use crate::dataset::{load_projects, SolidityProject};

pub fn setup(project: &str) -> &'static SolidityProject {
    load_projects().get(project).unwrap()
}

pub fn run(project: &SolidityProject) {
    let mut resolver = FileResolver::default();

    for (fname, source) in &project.sources {
        resolver.set_file_contents(fname, source.clone());
    }

    let result = solang::parse_and_resolve(
        OsStr::new(&project.compilation.entrypoint),
        &mut resolver,
        solang::Target::EVM,
    );

    assert_eq!(result.diagnostics.errors(), Vec::<&Diagnostic>::new());
}
