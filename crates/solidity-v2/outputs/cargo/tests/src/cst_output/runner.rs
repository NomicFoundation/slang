use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_testing_utils::cst_renderer::render;

use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus};

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

    snapshots::generate_snapshots(&test_dir, &mut fs, "generated", |version, target| {
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
    })?;

    Ok(())
}
