mod datasets;
mod reporting;

use std::path::PathBuf;

use anyhow::{Context, Result};
use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use solidity_rust_lib::generated::language::Language;
use solidity_testing_utils::compatible_versions::filter_compatible_versions;

use crate::{
    datasets::{get_all_datasets, Dataset},
    reporting::Reporter,
};

fn main() {
    // Fail the parent process if a child thread panics:
    std::panic::catch_unwind(|| {
        CodegenContext::with_context(|codegen| {
            let grammar_file = &codegen
                .repo_root
                .join("crates/solidity/inputs/schema/manifest.yml");

            let grammar = &Grammar::from_manifest(codegen, grammar_file);

            for dataset in get_all_datasets()? {
                process_dataset(&dataset, &grammar.manifest.versions)?;
            }

            return Ok(());
        })
        .unwrap();
    })
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
        .map(|file_path| process_source_file(file_path, all_versions, &reporter))
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

    let compatible_versions = {
        let latest_version = all_versions.last().context("No versions found.")?;
        let output = &Language::new(latest_version.to_owned()).parse_source_unit(source);
        reporter.report_test_result(source_id, source, latest_version, output);

        if let Some(parse_tree) = output.parse_tree() {
            filter_compatible_versions(all_versions, &parse_tree, source).context(format!(
                "Failed to extract compatible versions for file: {source_id}"
            ))?
        } else {
            // Skip this file if we failed to filter compatible versions.
            return Ok(());
        }
    };

    for version in compatible_versions {
        let output = &Language::new(version.to_owned()).parse_source_unit(source);
        reporter.report_test_result(source_id, source, version, output);
    }

    reporter.report_file_completed();

    return Ok(());
}
