use std::fmt::Write;

use anyhow::{Result, bail};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::diagnostics_output::targets::{SlangTarget, SolcTarget, TargetOutcome, TestTarget};
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

    let test_config = TestConfig::resolve(&test_dir)?;
    let solc_versions: SortedSet<Version> = match test_config.matrix {
        TestMatrix::SingleTargetAllVersions(_) => {
            LanguageVersion::ALL.iter().map(|v| (*v).into()).collect()
        }
        TestMatrix::SingleVersionAllTargets(ref matrix) => {
            SortedSet::from_iter([matrix.version.into()])
        }
    };

    let slang_target = SlangTarget;
    let solc_target = SolcTarget::new(solc_versions)?;

    let slang_outcomes = snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_config,
        &format!("generated/{}", slang_target.name()),
        |version, target| {
            let outcome = slang_target.compile(&files, version, target)?;
            Ok(make_outcome(version, target, &outcome))
        },
    )?;

    let solc_outcomes = snapshots::generate_snapshots(
        &test_dir,
        &mut fs,
        &test_config,
        &format!("generated/{}", solc_target.name()),
        |version, target| {
            let outcome = solc_target.compile(&files, version, target)?;
            Ok(make_outcome(version, target, &outcome))
        },
    )?;

    compare_outcomes(
        group_name,
        test_name,
        &test_config,
        &slang_outcomes,
        &solc_outcomes,
    )
}

fn make_outcome(
    version: LanguageVersion,
    target: slang_solidity_v2_common::evm_targets::EvmTarget,
    outcome: &TargetOutcome,
) -> SnapshotOutcome {
    let status = if outcome.compilation_succeeded {
        SnapshotStatus::Success
    } else {
        SnapshotStatus::Failure
    };

    let diagnostics = &outcome.diagnostics;

    let mut contents = String::new();
    writeln!(contents, "Diagnostics: {count}", count = diagnostics.len()).unwrap();
    for rendered in diagnostics {
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
    config: &TestConfig,
    slang_outcomes: &[SnapshotOutcome],
    solc_outcomes: &[SnapshotOutcome],
) -> Result<()> {
    // Both runs iterate the same axis in the same order.
    assert_eq!(slang_outcomes.len(), solc_outcomes.len());

    let mut report = String::new();
    let mut is_valid = true;

    for (slang, solc) in slang_outcomes.iter().zip(solc_outcomes) {
        assert_eq!(slang.version, solc.version);
        assert_eq!(slang.target, solc.target);

        let (label, expected_divergence) = match &config.matrix {
            TestMatrix::SingleTargetAllVersions(matrix) => (
                slang.version.to_string(),
                matrix
                    .expected_solc_divergence
                    .iter()
                    .any(|specifier| specifier.contains(slang.version)),
            ),
            TestMatrix::SingleVersionAllTargets(matrix) => (
                slang.target.to_string(),
                matrix
                    .expected_solc_divergence
                    .iter()
                    .any(|specifier| specifier.contains(slang.target)),
            ),
        };

        let found_divergence = slang.status != solc.status;

        if found_divergence != expected_divergence {
            is_valid = false;

            writeln!(
                report,
                "  - {label}: slang={slang_status:?}, solc={solc_status:?} ({outcome})",
                slang_status = slang.status,
                solc_status = solc.status,
                outcome = match (found_divergence, expected_divergence) {
                    (false, false) => "statuses match, as expected",
                    (true, true) => "statuses differ, as expected",
                    (true, false) => "ERROR: unexpected status divergence",
                    (false, true) => "ERROR: expected status divergence no longer happens",
                }
            )?;
        }
    }

    if is_valid {
        return Ok(());
    }

    bail!(
        "The status divergence between slang and solc in `{group_name}/{test_name}` doesn't match \
         the `expected_solc_divergence` in its `.tests.config.json`:\
         \n\n{report}\n\n"
    )
}
