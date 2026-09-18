use anyhow::{Result, ensure};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_testing_utils::cst_renderer::render;
use solidity_v2_testing_utils::evm_targets::default_evm_target;

use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus, TestCase, TestConfig};

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
    let test_cases: Vec<TestCase> = test_config.test_cases().collect();

    for case in &test_cases {
        ensure!(
            !case.expected_solc_divergence,
            "Not comparing with 'solc' in 'cst_output' tests"
        );
        ensure!(
            case.evm_target == default_evm_target(case.language_version),
            "Parsing doesn't currently depend on the EVM target, so 'cst_output' tests cannot pin it"
        );
    }

    snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_cases,
        "generated",
        |version, target| {
            let output = V2Parser::parse(&file_id, &source, version);
            let status = SnapshotStatus::from_diagnostics(&output.diagnostics);
            let contents = render(&source, source_id, &output, version, target);

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
