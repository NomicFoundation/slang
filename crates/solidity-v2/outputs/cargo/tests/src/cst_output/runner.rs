use anyhow::{Result, ensure};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_testing_utils::cst_renderer::render;

use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus, TestConfig, TestMatrix};

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("cst_output")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str();
    let file_id = source_id.into();
    let source = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();

    let test_config = TestConfig::resolve(&test_dir)?;
    match &test_config.matrix {
        TestMatrix::SingleVersionAllTargets(matrix) => ensure!(
            matrix.expected_solc_divergence.is_empty(),
            "Not comparing with 'solc' in 'cst_output' tests"
        ),
        TestMatrix::SingleTargetAllVersions(matrix) => ensure!(
            matrix.expected_solc_divergence.is_empty(),
            "Not comparing with 'solc' in 'cst_output' tests"
        ),
    }

    snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_config,
        "generated",
        |version, target| {
            let output = V2Parser::parse(&file_id, &source, version);
            let (ok, contents) = render(&source, source_id, &output);
            let status = if ok {
                SnapshotStatus::Success
            } else {
                SnapshotStatus::Failure
            };
            Ok(SnapshotOutcome {
                version,
                target,
                status,
                contents,
                extension: "yml",
            })
        },
    )?;

    Ok(())
}
