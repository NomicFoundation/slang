use std::cmp::min;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use itertools::Itertools;
use semver::Version;
use slang_solidity::kinds::RuleKind;
use slang_solidity::language::Language;

use crate::datasets::{DataSet, SourceFile};
use crate::events::{Events, TestOutcome};
use crate::ShardingOptions;

pub struct TestSelection<'d> {
    pub directories: Vec<&'d String>,
    pub files_count: usize,
}

pub(crate) fn select_tests<'d>(
    dataset: &'d DataSet,
    sharding_options: &ShardingOptions,
) -> TestSelection<'d> {
    let ShardingOptions {
        shards_count,
        shard_index,
    } = sharding_options;

    let directories_iter = dataset.directories().keys();
    let directories_count = dataset.directories().len();

    let directories = match (shards_count, shard_index) {
        (Some(shards_count), Some(shard_index)) => {
            let mut shard_size = directories_count / shards_count;
            if shards_count * shard_size != directories_count {
                // If the division is not exact, add one to compensate for the fractions:
                shard_size += 1;
            }

            let from = min(shard_index * shard_size, directories_count - 1);
            let length = min(shard_size, directories_count - from);

            directories_iter.skip(from).take(length).collect_vec()
        }

        (None, None) => directories_iter.collect_vec(),

        _ => unreachable!("Arguments should require each other."),
    };

    let files_count: usize = directories
        .iter()
        .map(|dir| dataset.directories()[*dir].len())
        .sum();

    TestSelection {
        directories,
        files_count,
    }
}

pub fn run_test(file: &SourceFile, events: &Events) -> Result<()> {
    let Some(version) = extract_compiler_version(&file.compiler) else {
        events.test(TestOutcome::Incompatible);
        return Ok(());
    };

    if !file.path.exists() {
        // Index can be out of date:
        events.test(TestOutcome::NotFound);
        return Ok(());
    }

    let source = file
        .path
        .read_to_string()?
        // Remove unicode character inserted by sanctuary (illegal in Solidity):
        .replace("[email protected]", "[email-protected]");

    let language = Language::new(version.to_owned())?;
    let output = language.parse(RuleKind::SourceUnit, &source);

    if output.is_valid() {
        events.test(TestOutcome::Passed);
        return Ok(());
    }

    events.test(TestOutcome::Failed);

    let with_color = true;
    let source_id = file.path.strip_repo_root()?.unwrap_str();

    for error in output.errors() {
        let report = error.to_error_report(source_id, &source, with_color);

        events.parse_error(format!("[{version}] {report}"));
    }

    Ok(())
}

fn extract_compiler_version(compiler: &str) -> Option<Version> {
    if compiler.starts_with("vyper:") {
        // Ignore contracts not compiled by "solc":
        return None;
    }

    // Otherwise, it is "solc", and that field will contain the compiler version:
    // Note: trip the "v" prefix if it is present (e.g. "v0.8.0" -> "0.8.0").
    let version = compiler.strip_prefix('v').unwrap_or(compiler);

    let Ok(version) = Version::parse(version) else {
        panic!("Unrecognized compiler/version: '{compiler}'");
    };

    if &version < Language::SUPPORTED_VERSIONS.first().unwrap() {
        // Version is too early:
        return None;
    }

    Some(version)
}
