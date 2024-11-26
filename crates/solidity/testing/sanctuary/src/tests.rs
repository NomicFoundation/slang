use std::cmp::min;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use itertools::Itertools;
use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::bindings::Bindings;
use slang_solidity::cst::{Cursor, NonterminalKind, TextIndex, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};
use slang_solidity::parser::{ParseOutput, Parser};
use slang_solidity::{bindings, transform_built_ins_node};

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
    if !file.path.exists() {
        // Index can be out of date:
        events.test(TestOutcome::NotFound);
        return Ok(());
    }

    // Ignore contracts that rely on obvious, exotic parser bugs fixed in later versions:
    if uses_exotic_parser_bug(&file.path) {
        events.test(TestOutcome::Incompatible);
        return Ok(());
    }

    let Some(version) = extract_compiler_version(&file.compiler) else {
        events.test(TestOutcome::Incompatible);
        return Ok(());
    };

    let source = file.path.read_to_string()?;
    // Heuristic: ignore wrongly scraped sanctuary files that
    // contain HTML with a Cloudflare email obfuscation attribute:
    // https://github.com/tintinweb/smart-contract-sanctuary/issues/32
    // Also ignore wrongly obfuscated `r@` in "\r@..." that leads to an invalid escape sequence (`\[`):
    if source.contains("data-cfemail=") || source.contains("\\[email protected]") {
        events.test(TestOutcome::Incompatible);
        return Ok(());
    }

    let source = source
        // Some files are null terminated. Remove the null character:
        // https://github.com/tintinweb/smart-contract-sanctuary/issues/30
        .trim_end_matches('\0')
        // Remove unicode character inserted by sanctuary (illegal in Solidity):
        // https://github.com/tintinweb/smart-contract-sanctuary/issues/31
        .replace("[email protected]", "[email-protected]")
        // Select contracts from Sanctuary were probably incorrectly web-scraped:
        // https://github.com/tintinweb/smart-contract-sanctuary/issues/32
        .replace("&#39;", "\"");

    let parser = Parser::create(version.clone())?;
    let output = parser.parse(NonterminalKind::SourceUnit, &source);
    let source_id = file.path.strip_repo_root()?.unwrap_str();

    let with_color = true;

    if !output.is_valid() {
        for error in output.errors() {
            let report = slang_solidity::diagnostic::render(error, source_id, &source, with_color);

            events.parse_error(format!("[{version}] {report}"));
        }

        events.test(TestOutcome::Failed);
        return Ok(());
    }

    let unresolved_references = check_bindings(&version, source_id, &output)?;
    if !unresolved_references.is_empty() {
        for unresolved in &unresolved_references {
            let report =
                slang_solidity::diagnostic::render(unresolved, source_id, &source, with_color);
            events.bindings_error(format!("[{version}] {report}"));
        }

        events.test(TestOutcome::Failed);
        return Ok(());
    }

    events.test(TestOutcome::Passed);
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

    if &version < Parser::SUPPORTED_VERSIONS.first().unwrap() {
        // Version is too early:
        return None;
    }

    Some(version)
}

fn uses_exotic_parser_bug(file: &Path) -> bool {
    static CONTRACTS_WITH_EXOTIC_PARSER_BUGS: &[&str] = &[
        // 0.4.24: // Accepts malformed `* /` in multi-line comments:
        // Fixed in 0.4.25: https://github.com/ethereum/solidity/pull/4937
        "ethereum/contracts/mainnet/79/79bb6f4492d5cb13fad8ca0ecfbccd9e2c26ac42_Gateway.sol",
        // 0.4.18: // Accepts unfinished multi-line comments at the end of the file:
        // Fixed in 0.4.25: https://github.com/ethereum/solidity/pull/4937
        "ethereum/contracts/mainnet/7d/7d81c361d6ac60634117dd81ab1b01b8dc795a9d_LILITHCOIN.sol",
        "bsc/contracts/mainnet/a4/a4c2f6D203f91e58956227a002eF1209f53C27Fd_GBNB.sol",
        "bsc/contracts/mainnet/e3/e3472842D6a894dc3e5E1Bc6cE4cEFe16a7df749_WBNB.sol",
        // 0.8.15: Relies on invalidly accepting `indexed indexed` in the event declaration:
        // Fixed in 0.8.18: https://github.com/ethereum/solidity/pull/13816
        "ethereum/contracts/mainnet/d4/d4559E5F507eD935F19208A5D50637898c192Ab3_Factory.sol",
    ];

    CONTRACTS_WITH_EXOTIC_PARSER_BUGS
        .iter()
        .any(|path| file.ends_with(path))
}

fn check_bindings(
    version: &Version,
    source_id: &str,
    output: &ParseOutput,
) -> Result<Vec<UnresolvedReference>> {
    let mut unresolved = Vec::new();
    let bindings = create_bindings(version, source_id, output)?;

    for reference in bindings.all_references() {
        if reference.get_file().is_system() {
            // skip built-ins
            continue;
        }
        // We're not interested in the exact definition a reference resolves
        // to, so we lookup all of them and fail if we find none.
        if reference.definitions().is_empty() {
            let cursor = reference.get_cursor().unwrap();
            unresolved.push(UnresolvedReference { cursor });
        }
    }

    Ok(unresolved)
}

fn create_bindings(version: &Version, source_id: &str, output: &ParseOutput) -> Result<Bindings> {
    let mut bindings = bindings::create_with_resolver(
        version.clone(),
        Arc::new(SingleFileResolver {
            source_id: source_id.into(),
        }),
    );
    let parser = Parser::create(version.clone())?;
    let built_ins_tree = parser
        .parse(
            NonterminalKind::SourceUnit,
            bindings::get_built_ins(version),
        )
        .tree();
    let built_ins_cursor =
        transform_built_ins_node(&built_ins_tree).cursor_with_offset(TextIndex::ZERO);

    bindings.add_system_file("built_ins.sol", built_ins_cursor);
    bindings.add_user_file(source_id, output.create_tree_cursor());
    Ok(bindings)
}

/// Bindings `PathResolver` that always resolves to the given `source_id`.
/// This is useful for Sanctuary since all dependencies are concatenated in the
/// same file, but the import directives are retained.
struct SingleFileResolver {
    source_id: String,
}

impl PathResolver for SingleFileResolver {
    fn resolve_path(&self, _context_path: &str, _path_to_resolve: &str) -> Option<String> {
        Some(self.source_id.clone())
    }
}

struct UnresolvedReference {
    pub cursor: Cursor,
}

impl Diagnostic for UnresolvedReference {
    fn text_range(&self) -> TextRange {
        self.cursor.text_range()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        format!(
            "Unresolved reference to `{symbol}`",
            symbol = self.cursor.node().unparse()
        )
    }
}
