mod datasets;
mod reporting;

use std::{collections::BTreeSet, ops::ControlFlow, path::Path, process::ExitCode};

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use slang_solidity::{kinds::RuleKind, language::Language};
use solidity_language::SolidityDefinition;
use solidity_testing_utils::version_pragmas::extract_version_pragmas;

use crate::{
    datasets::{get_all_datasets, Dataset},
    reporting::Reporter,
};

fn main() -> Result<ExitCode> {
    let versions = SolidityDefinition::create().collect_breaking_versions();

    for dataset in get_all_datasets()? {
        match process_dataset(&dataset, &versions)? {
            ControlFlow::Continue(..) => {}
            ControlFlow::Break(exit_code) => return Ok(exit_code),
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn process_dataset(
    dataset: &impl Dataset,
    versions: &BTreeSet<Version>,
) -> Result<ControlFlow<ExitCode>> {
    println!();
    println!();
    println!("  🧪 Dataset: {title}", title = dataset.get_title());
    println!();
    println!();

    println!("Preparing dataset...");
    let source_files = dataset.prepare()?;

    println!();
    println!(
        "Processing {count} source files...",
        count = source_files.len()
    );

    let reporter = Reporter::new(source_files.len())?;

    source_files
        .par_iter()
        // Halt as soon as possible if a child panics.
        .panic_fuse()
        .map(|file_path| {
            process_source_file(file_path, versions, &reporter)?;
            reporter.report_file_completed();
            Ok(())
        })
        .collect::<Result<()>>()?;

    let total_errors = reporter.finish();
    if total_errors > 0 {
        println!("There were errors processing the dataset.");
        Ok(ControlFlow::Break(ExitCode::FAILURE))
    } else {
        Ok(ControlFlow::Continue(()))
    }
}

fn process_source_file(
    file_path: &Path,
    versions: &BTreeSet<Version>,
    reporter: &Reporter,
) -> Result<()> {
    let source_id = file_path.unwrap_str();
    let source = &file_path.read_to_string()?;

    let latest_version = versions.iter().max().unwrap();
    let Ok(pragmas) = extract_version_pragmas(source, latest_version) else {
        // Skip this file if we failed to filter compatible versions.
        return Ok(());
    };

    if pragmas.is_empty() {
        // Skip if there are no pragmas in that file.
        return Ok(());
    }

    for version in versions {
        if !pragmas.iter().all(|pragma| pragma.matches(version)) {
            continue;
        }

        let language = Language::new(version.to_owned())?;
        let output = language.parse(RuleKind::SourceUnit, source);

        reporter.report_test_result(source_id, source, version, &output);
    }

    Ok(())
}
