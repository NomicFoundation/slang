mod datasets;
mod reporting;

use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use codegen_schema::types::Schema;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use slang_solidity::{syntax::parser::ProductionKind, Language};
use solidity_schema::SoliditySchemaExtensions;
use solidity_testing_utils::version_pragmas::extract_version_pragmas;

use crate::{
    datasets::{get_all_datasets, Dataset},
    reporting::Reporter,
};

fn main() {
    // Fail the parent process if a child thread panics:
    std::panic::catch_unwind(|| -> Result<()> {
        let schema = &Schema::load_solidity()?;

        for dataset in get_all_datasets()? {
            process_dataset(&dataset, &schema.versions)?;
        }

        return Ok(());
    })
    .unwrap()
    .unwrap();
}

fn process_dataset(dataset: &impl Dataset, all_versions: &Vec<Version>) -> Result<()> {
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
        .map(|file_path| {
            process_source_file(file_path, all_versions, &reporter)?;
            reporter.report_file_completed();
            return Ok(());
        })
        .collect::<Result<()>>()?;

    let total_errors = reporter.finish();
    if total_errors > 0 {
        std::process::exit(1);
    }

    return Ok(());
}

fn process_source_file(
    file_path: &PathBuf,
    all_versions: &Vec<Version>,
    reporter: &Reporter,
) -> Result<()> {
    let source_id = file_path.to_str().unwrap();
    let source = &std::fs::read_to_string(file_path)?;

    let latest_version = all_versions.iter().max().unwrap();
    let pragmas = if let Ok(pragmas) = extract_version_pragmas(source, latest_version) {
        pragmas
    } else {
        // Skip this file if we failed to filter compatible versions.
        return Ok(());
    };

    if pragmas.is_empty() {
        // Skip if there are no pragmas in that file.
        return Ok(());
    }

    let mut tested_versions = HashSet::new();

    for version in all_versions {
        if !pragmas.iter().all(|pragma| pragma.matches(version)) {
            continue;
        }

        // solc doesn't follow SemVer, and introduces breaking changes during minor version upgrades.
        // Let's run against the latest release of each minor release.
        let unique_key = version.major ^ version.minor;
        if !tested_versions.insert(unique_key) {
            continue;
        }

        let output = Language::new(version.to_owned())?.parse(ProductionKind::SourceUnit, source);

        reporter.report_test_result(source_id, source, &version, &output);
    }

    return Ok(());
}
