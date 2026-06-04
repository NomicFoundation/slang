use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::diagnostics_output::targets::{SlangTarget, SolcTarget, TestTarget};
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

    run_target(&slang_target, &test_dir, &mut fs, &files, &versions)?;
    run_target(&solc_target, &test_dir, &mut fs, &files, &versions)?;

    Ok(())
}

fn run_target(
    target: &dyn TestTarget,
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    files: &BTreeMap<String, String>,
    versions: &BTreeSet<Version>,
) -> Result<()> {
    let mut last_report = None;

    for version in versions {
        let errors = target.get_errors(files, version)?;
        let status = if errors.is_empty() {
            "success"
        } else {
            "failure"
        };

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
                    .join(format!("{version}-{status}.txt"));
                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }
    }

    Ok(())
}
