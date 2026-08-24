//! Builds a complete [`CompilationUnit`] through the public v2 API.
//!
//! The sibling modules in this directory each measure one pipeline stage in
//! isolation. This one instead drives the whole pipeline the way consumers do:
//! parsing, IR building, and semantic analysis, all behind
//! `CompilationBuilder::build()`.

use slang_solidity_v2::compilation::{
    CompilationBuilder, CompilationBuilderConfig, CompilationUnit, FileId,
};
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};

use crate::dataset::SolidityProject;
use crate::tests::slang_v2::common::{parse_evm_target, parse_version};

pub type Input = &'static SolidityProject;
pub type Output = CompilationUnit;

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

pub fn run(project: Input) -> Output {
    let mut builder = CompilationBuilder::create(
        parse_version(project),
        parse_evm_target(project),
        ProjectConfig { project },
    );

    // Add every source of the project, rather than just its entrypoint. This
    // matches the workload of the per-stage benchmarks in this directory, which
    // all operate on the full source list, and it mirrors consumers that
    // already know their complete file set up front (e.g. a build tool).
    for file_id in project.sources.keys() {
        builder.add_file(file_id.as_str().into());
    }

    let unit = builder.build();

    assert!(
        unit.diagnostics().is_empty(),
        "compilation produced diagnostics: {diagnostics:#?}",
        diagnostics = unit.diagnostics()
    );

    unit
}

pub fn test(project: Input) -> Output {
    run(project)
}

/// Serves the builder callbacks entirely from the project's in-memory sources,
/// so that benchmarks never touch the filesystem.
struct ProjectConfig {
    project: &'static SolidityProject,
}

impl CompilationBuilderConfig for ProjectConfig {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        self.project
            .sources
            .get(file_id.as_str())
            .cloned()
            .ok_or_else(|| MissingFile {
                reason: format!("'{file_id}' is not part of this project"),
            })
    }

    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        self.project
            .import_resolver
            .resolve_import(source_file_id.as_str(), import_path)
            .map(FileId::from)
            .ok_or_else(|| UnresolvedImport {
                reason: format!("can't resolve '{import_path}' from '{source_file_id}'"),
            })
    }
}

/// Counts the ABI entries produced for the unit's concrete contracts, which is
/// the same value that the `compute_contracts_abi` stage benchmark reports.
pub fn count_concrete_contracts(output: &Output) -> usize {
    output.compute_contracts_abi().len()
}

/// Counts the references the binder resolved across every file of the unit.
pub fn count_resolved_references(output: &Output) -> usize {
    output.all_references().count()
}
