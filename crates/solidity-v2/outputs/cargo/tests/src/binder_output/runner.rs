use anyhow::{Result, ensure};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::SortedMap;
use solidity_v2_testing_utils::compilation;

use super::report::binder_report;
use super::report_data::ReportData;
use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus, TestCase, TestConfig};
use crate::utils::multi_part_file::split_multi_file;

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
    let test_cases: Vec<TestCase> = test_config.test_cases().collect();

    ensure!(
        test_cases.iter().all(|case| !case.expected_solc_divergence),
        "Not comparing with 'solc' in 'binder_output' tests"
    );

    snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_cases,
        "generated",
        |version, target| {
            let compilation = compilation::compile(&files, version, target);
            let report_data = ReportData::prepare(&compilation, &files);

            let status = if report_data.all_resolved() {
                SnapshotStatus::from_diagnostics(compilation.diagnostics())
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
