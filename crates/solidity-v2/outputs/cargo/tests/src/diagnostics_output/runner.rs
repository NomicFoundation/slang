use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::diagnostics_output::targets::{SlangTarget, SolcTarget, TestTarget};
use crate::utils::multi_part_file::split_multi_file;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompilationStatus {
    Success,
    Failure,
}

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
    let files: BTreeMap<String, String> = multi_part
        .parts
        .iter()
        .map(|part| (part.name.to_string(), part.contents.to_string()))
        .collect();

    let versions = LanguageVersion::ALL
        .iter()
        .map(|v| (*v).into())
        .collect::<BTreeSet<Version>>();

    let slang_target = SlangTarget;
    let solc_target = SolcTarget::new(versions.clone())?;

    let slang_statuses = run_target(&slang_target, &test_dir, &mut fs, &files, &versions)?;
    let solc_statuses = run_target(&solc_target, &test_dir, &mut fs, &files, &versions)?;

    compare_statuses(
        group_name,
        test_name,
        &versions,
        &slang_statuses,
        &solc_statuses,
    )
}

fn run_target(
    target: &dyn TestTarget,
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    files: &BTreeMap<String, String>,
    versions: &BTreeSet<Version>,
) -> Result<Vec<CompilationStatus>> {
    let mut last_report = None;
    let mut statuses = Vec::with_capacity(versions.len());

    for version in versions {
        let errors = target.collect_diagnostics(files, version)?;

        let status = if errors.is_empty() {
            CompilationStatus::Success
        } else {
            CompilationStatus::Failure
        };
        statuses.push(status);

        let mut report = String::new();
        writeln!(report, "Diagnostics: {count}", count = errors.len())?;
        for rendered in &errors {
            writeln!(report)?;
            writeln!(report, "{rendered}")?;
        }

        match last_report {
            Some(ref last) if last == &report => (),
            _ => {
                let snapshot_path = test_dir
                    .join("generated")
                    .join(target.name())
                    .join(format!("{version}-{status:?}.txt").to_lowercase());

                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }
    }

    Ok(statuses)
}

fn compare_statuses(
    group_name: &str,
    test_name: &str,
    versions: &BTreeSet<Version>,
    slang_statuses: &[CompilationStatus],
    solc_statuses: &[CompilationStatus],
) -> Result<()> {
    // Both targets run over the same versions, in the same (sorted) order.
    assert_eq!(versions.len(), slang_statuses.len());
    assert_eq!(versions.len(), solc_statuses.len());

    if slang_statuses == solc_statuses {
        return Ok(());
    }

    let mut message = String::new();
    writeln!(
        message,
        "slang and solc disagree on the compilation status of `{group_name}/{test_name}`."
    )?;
    writeln!(message)?;

    for (index, version) in versions.iter().enumerate() {
        let slang = slang_statuses[index];
        let solc = solc_statuses[index];
        let outcome = if slang == solc { "match" } else { "differ" };

        writeln!(
            message,
            "  {version}: slang={slang:?}, solc={solc:?} ({outcome})"
        )?;
    }

    bail!(message)
}
