//! Builds a complete [`CompilationUnit`] through the public v2 API.
//!
//! The sibling modules in this directory each measure one pipeline stage in
//! isolation. This one instead drives the whole pipeline the way consumers do:
//! parsing, IR building, and semantic analysis, all behind
//! `CompilationUnit::create()`.

use slang_solidity_v2::compilation::{CompilationUnit, FileId, ImportResolver};
use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;

use crate::dataset::SolidityProject;
use crate::tests::slang_v2::common::{parse_evm_target, parse_version};

pub type Input = &'static SolidityProject;
pub type Output = CompilationUnit;

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

pub fn run(project: Input) -> Output {
    // Pass every source of the project, rather than just its entrypoint. This
    // matches the workload of the per-stage benchmarks in this directory, which
    // all operate on the full source list, and it mirrors consumers that
    // already know their complete file set up front (e.g. a build tool).
    let unit = CompilationUnit::create(
        parse_version(project),
        parse_evm_target(project),
        project
            .sources
            .iter()
            .map(|(file_id, contents)| (FileId::from(file_id.as_str()), contents.clone())),
        ProjectImportResolver { project },
    );

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

/// Resolves the project's imports entirely from its in-memory metadata, so that
/// benchmarks never touch the filesystem.
struct ProjectImportResolver {
    project: &'static SolidityProject,
}

impl ImportResolver for ProjectImportResolver {
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
