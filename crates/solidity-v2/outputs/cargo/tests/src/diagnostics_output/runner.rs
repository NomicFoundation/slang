use std::fmt::Write;

use anyhow::{bail, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::diagnostics_output::targets::{SlangTarget, SolcTarget, TestTarget};
use crate::snapshots::{self, SnapshotOutcome, SnapshotStatus, TestConfig, TestMatrix};
use crate::utils::multi_part_file::split_multi_file;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("diagnostics_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let multi_part = split_multi_file(&contents);

    // Use a sorted map so file iteration order is deterministic across runs.
    let files: SortedMap<FileId, String> = multi_part
        .parts
        .iter()
        .map(|part| (part.name.into(), part.contents.to_string()))
        .collect();

    let config = TestConfig::resolve(&test_dir)?;
    let solc_versions: SortedSet<Version> = match config.matrix {
        TestMatrix::SingleTargetAllVersions { .. } => {
            LanguageVersion::ALL.iter().map(|v| (*v).into()).collect()
        }
        TestMatrix::SingleVersionAllTargets { version } => SortedSet::from_iter([version.into()]),
    };

    let slang_target = SlangTarget;
    let solc_target = SolcTarget::new(solc_versions)?;
    let slang_subdir = format!("generated/{}", slang_target.name());
    let solc_subdir = format!("generated/{}", solc_target.name());

    let slang_outcomes =
        snapshots::generate_snapshots(&test_dir, &mut fs, &slang_subdir, |version, target| {
            let errors = slang_target.collect_diagnostics(&files, version, target)?;
            Ok(make_outcome(version, target, &errors))
        })?;
    let solc_outcomes =
        snapshots::generate_snapshots(&test_dir, &mut fs, &solc_subdir, |version, target| {
            let errors = solc_target.collect_diagnostics(&files, version, target)?;
            Ok(make_outcome(version, target, &errors))
        })?;

    compare_outcomes(
        group_name,
        test_name,
        config,
        &slang_outcomes,
        &solc_outcomes,
    )
}

fn make_outcome(
    version: LanguageVersion,
    target: slang_solidity_v2_common::evm_targets::EvmTarget,
    errors: &[String],
) -> SnapshotOutcome {
    let status = if errors.is_empty() {
        SnapshotStatus::Success
    } else {
        SnapshotStatus::Failure
    };

    let mut contents = String::new();
    writeln!(contents, "Diagnostics: {count}", count = errors.len()).unwrap();
    for rendered in errors {
        writeln!(contents).unwrap();
        writeln!(contents, "{rendered}").unwrap();
    }

    SnapshotOutcome {
        version,
        target,
        status,
        contents,
        extension: "txt",
    }
}

fn compare_outcomes(
    group_name: &str,
    test_name: &str,
    config: TestConfig,
    slang_outcomes: &[SnapshotOutcome],
    solc_outcomes: &[SnapshotOutcome],
) -> Result<()> {
    // Both runs iterate the same axis in the same order.
    assert_eq!(slang_outcomes.len(), solc_outcomes.len());
    let statuses_match = slang_outcomes
        .iter()
        .zip(solc_outcomes)
        .all(|(slang, solc)| slang.status == solc.status);
    if statuses_match {
        return Ok(());
    }

    let mut message = String::new();
    writeln!(
        message,
        "slang and solc disagree on the compilation status of `{group_name}/{test_name}`."
    )?;
    writeln!(message)?;

    for (slang, solc) in slang_outcomes.iter().zip(solc_outcomes) {
        debug_assert_eq!(slang.version, solc.version);
        debug_assert_eq!(slang.target, solc.target);

        let label = match config.matrix {
            TestMatrix::SingleTargetAllVersions { .. } => slang.version.to_string(),
            TestMatrix::SingleVersionAllTargets { .. } => slang.target.to_string(),
        };

        let slang_status = slang.status;
        let solc_status = solc.status;
        let outcome = if slang_status == solc_status {
            "match"
        } else {
            "differ"
        };

        writeln!(
            message,
            "  {label}: slang={slang_status:?}, solc={solc_status:?} ({outcome})"
        )?;
    }

    bail!(message)
}
