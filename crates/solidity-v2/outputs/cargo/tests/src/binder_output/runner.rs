use anyhow::{Result, ensure};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2::compilation::{CompilationUnit, FileId, ImportResolver};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;

use super::report::binder_report;
use super::report_data::ReportData;
use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus, TestConfig, TestMatrix};
use crate::utils::multi_part_file::split_multi_file;
use crate::utils::path_resolver;

struct TestImportResolver;

impl ImportResolver for TestImportResolver {
    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        path_resolver::resolve_import(source_file_id, import_path).ok_or_else(|| UnresolvedImport {
            reason: "Unresolved import".to_string(),
        })
    }
}

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("binder_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let multi_part = split_multi_file(&contents);

    let files: SortedMap<FileId, String> = multi_part
        .parts
        .iter()
        .map(|part| (part.name.into(), part.contents.to_string()))
        .collect();

    let test_config = TestConfig::resolve(&test_dir)?;
    match &test_config.matrix {
        TestMatrix::SingleVersionAllTargets(matrix) => ensure!(
            matrix.expected_solc_divergence.is_empty(),
            "Not comparing with 'solc' in 'binder_output' tests"
        ),
        TestMatrix::SingleTargetAllVersions(matrix) => ensure!(
            matrix.expected_solc_divergence.is_empty(),
            "Not comparing with 'solc' in 'binder_output' tests"
        ),
    }

    snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_config,
        "generated",
        |version, target| {
            let compilation =
                CompilationUnit::create(version, target, files.clone(), TestImportResolver);
            let report_data = ReportData::prepare(&compilation, &files);

            let status = if report_data.all_resolved() {
                SnapshotStatus::Success
            } else {
                SnapshotStatus::Failure
            };

            let contents = binder_report(&report_data)?;
            Ok(SnapshotOutcome {
                version,
                target,
                status,
                contents,
                extension: "txt",
            })
        },
    )?;

    Ok(())
}
