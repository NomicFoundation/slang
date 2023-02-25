mod datasets;
mod reporting;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use solidity_rust_lib::generated::{kinds::ProductionKind, language::Language};
use solidity_schema::SolidityGrammarExtensions;
use solidity_testing_utils::compatible_versions::filter_compatible_versions;

use crate::{
    datasets::{get_all_datasets, Dataset},
    reporting::Reporter,
};

fn main() {
    // Fail the parent process if a child thread panics:
    std::panic::catch_unwind(|| -> Result<()> {
        let grammar = &Grammar::load_solidity()?;

        for dataset in get_all_datasets()? {
            process_dataset(&dataset, &grammar.versions)?;
        }

        return Ok(());
    })
    .unwrap()
    .unwrap();
}

fn process_dataset(dataset: &impl Dataset, all_versions: &Vec<Version>) -> Result<()> {
    println!();
    println!();
    println!("  🧪 Dataset: {}", dataset.get_title());
    println!();
    println!();

    println!("Preparing dataset...");
    let source_files = dataset.prepare()?;

    println!();
    println!("Processing {} source files...", source_files.len());

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
        std::process::exit(total_errors as i32);
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

    let latest_version = all_versions.last().expect("No versions found.");
    let output =
        &Language::new(latest_version.to_owned()).parse(ProductionKind::SourceUnit, source);

    reporter.report_test_result(source_id, source, latest_version, output);

    let parse_tree = if let Some(parse_tree) = output.parse_tree() {
        parse_tree
    } else {
        // Skip this file if we failed to filter compatible versions.
        return Ok(());
    };

    let compatible_versions = filter_compatible_versions(all_versions, &parse_tree, source).expect(
        &format!("Failed to extract compatible versions from file: {source_id}"),
    );

    let test_versions = filter_test_versions(compatible_versions);

    for version in test_versions {
        let output = &Language::new(version.to_owned()).parse(ProductionKind::SourceUnit, source);

        reporter.report_test_result(source_id, source, &version, output);
    }

    return Ok(());
}

fn filter_test_versions<'a>(
    compatible_versions: BTreeSet<&'a Version>,
) -> impl Iterator<Item = &'a Version> {
    // solc doesn't follow SemVer, and introduces breaking changes during minor version upgrades.
    // Let's run against the latest release of each minor release.
    let mut test_versions = BTreeMap::new();

    for version in compatible_versions {
        let key = version.major ^ version.minor;
        test_versions.insert(key, version);
    }

    return test_versions.into_values();
}
