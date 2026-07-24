use std::path::Path;

use anyhow::Result;
use infra_utils::solc::default_evm_version;
use semver::Version;
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_testing_utils::import_resolver::{ImportRemap, ImportResolver, SourceMap};
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::test_case::{IsolTestCase, parse_evm_target_name, resolve_evm_target};

/// The result of running `slang` against a single semantic test.
pub enum Outcome {
    /// `slang` compiled the test with no error diagnostics.
    Passed,
    /// The test was not compiled cleanly.
    Failed { diagnostics: Vec<String> },
}

/// Parses and runs a single semantic test file located at `test_path`, pinned
/// to the given `language_version`.
pub fn run_test(test_path: &Path, language_version: LanguageVersion) -> Outcome {
    let test_case = match IsolTestCase::parse(test_path) {
        Ok(test_case) => test_case,
        Err(error) => {
            return Outcome::Failed {
                diagnostics: vec![format!("could not parse test file: {error}")],
            };
        }
    };

    if test_case.files.is_empty() {
        return Outcome::Failed {
            diagnostics: vec!["no source files found in test".to_owned()],
        };
    }

    run_test_case(&test_case, language_version)
}

/// The EVM target `solc` of the given language version defaults to when a test
/// doesn't specify one.
fn default_evm_target(language_version: LanguageVersion) -> EvmTarget {
    let version: Version = language_version.into();
    parse_evm_target_name(default_evm_version(&version)).unwrap_or(EvmTarget::LATEST)
}

fn run_test_case(test_case: &IsolTestCase, language_version: LanguageVersion) -> Outcome {
    let files: SortedMap<String, String> = test_case.files.iter().cloned().collect();
    let config = TestConfig::new(&files, &test_case.remappings);

    let evm_target = resolve_evm_target(
        test_case.evm_version.as_deref(),
        default_evm_target(language_version),
    );

    let mut builder = CompilationBuilder::create(language_version, evm_target, config);

    // Add every source as a root, so files that aren't reachable via imports
    // (e.g. sibling sources in a multi-source test) are still analyzed.
    for (file_id, _) in &test_case.files {
        builder.add_file(FileId::from(file_id.as_str()));
    }

    let unit = builder.build();
    let diagnostics = unit.diagnostics();

    if diagnostics.is_empty() {
        return Outcome::Passed;
    }

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let source = files
                .get(file_id.as_str())
                .map(String::as_str)
                .unwrap_or_default();
            diagnostic::render(diagnostic, file_id.as_str(), source, false)
        })
        .collect();

    Outcome::Failed {
        diagnostics: rendered,
    }
}

/// Feeds the in-memory sources of an [`IsolTestCase`] to the `slang` compilation
/// builder, resolving imports between them.
///
/// Import resolution reuses the shared [`ImportResolver`] (also used by the
/// sourcify runner): each source is registered under its own name, and any
/// `ExternalSource: <name>=<path>` remappings become import remaps.
struct TestConfig {
    files: SortedMap<String, String>,
    resolver: ImportResolver,
}

impl TestConfig {
    fn new(files: &SortedMap<String, String>, remappings: &[(String, String)]) -> Self {
        let source_maps = files
            .keys()
            .map(|id| SourceMap {
                // Our source names are already the "virtual" paths that imports
                // refer to, so the id and virtual path are the same.
                source_id: id.clone(),
                virtual_path: id.clone(),
            })
            .collect();

        let import_remaps = remappings
            .iter()
            .map(|(name, path)| ImportRemap::new_from_components(None, name, path))
            .collect();

        Self {
            files: files.clone(),
            resolver: ImportResolver {
                import_remaps,
                source_maps,
            },
        }
    }
}

impl CompilationBuilderConfig for TestConfig {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        self.files
            .get(file_id.as_str())
            .cloned()
            .ok_or_else(|| MissingFile {
                reason: format!("no source registered for '{file_id}'"),
            })
    }

    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        self.resolver
            .resolve_import(source_file_id.as_str(), import_path)
            .map(FileId::from)
            .ok_or_else(|| UnresolvedImport {
                reason: format!("could not resolve import '{import_path}'"),
            })
    }
}
