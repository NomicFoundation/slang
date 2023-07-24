mod datasets;
mod reporting;

use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use slang_solidity::{language::Language, syntax::nodes::ProductionKind};
use solidity_language::SolidityLanguageExtensions;
use solidity_testing_utils::version_pragmas::extract_version_pragmas;

use crate::{
    datasets::{get_all_datasets, Dataset},
    reporting::Reporter,
};

fn main() {
    // Fail the parent process if a child thread panics:
    std::panic::catch_unwind(|| -> Result<()> {
        let language = &LanguageDefinition::load_solidity()?;
        let versions = language.collect_version_breaks();

        for dataset in get_all_datasets()? {
            process_dataset(&dataset, &versions)?;
        }

        return Ok(());
    })
    .unwrap()
    .unwrap();
}

fn process_dataset(dataset: &impl Dataset, versions: &BTreeSet<Version>) -> Result<()> {
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
            process_source_file(file_path, versions, &reporter)?;
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
    versions: &BTreeSet<Version>,
    reporter: &Reporter,
) -> Result<()> {
    let source_id = file_path.to_str().unwrap();
    let source = &std::fs::read_to_string(file_path)?;

    let latest_version = versions.iter().max().unwrap();
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

    for version in versions {
        if !pragmas.iter().all(|pragma| pragma.matches(version)) {
            continue;
        }

        let language = Language::new(version.to_owned())?;
        let output = language.parse(ProductionKind::SourceUnit, source)?;

        reporter.report_test_result(source_id, source, &version, &output);
    }

    return Ok(());
}
