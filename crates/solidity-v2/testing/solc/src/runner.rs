use std::path::Path;

use anyhow::{Context, Result};
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity_v2_common::collections::OrderedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::UnresolvedImport;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_testing_utils::import_resolver::{ImportResolver, SourceMap};
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::evm_target::{FUTURE_EVM_VERSION, ParsedTarget, default_evm_target, resolve_evm_target};
use crate::test_case::IsolTestCase;

/// The result of running `slang` against a single semantic test.
pub enum Outcome {
    /// `slang` compiled the test with no error diagnostics.
    Passed,
    /// The test was not compiled cleanly.
    Failed { diagnostics: Vec<String> },
}

/// Parses and runs a single semantic test file located at `test_path`, pinned
/// to the given `language_version`.
///
/// A test `slang` rejects is an [`Outcome::Failed`]; an `Err` means the harness
/// itself couldn't run the test.
pub fn run_test(test_path: &Path, language_version: LanguageVersion) -> Result<Outcome> {
    let test_case = IsolTestCase::parse(test_path)?;

    if test_case.files.is_empty() {
        return Ok(Outcome::Failed {
            diagnostics: vec!["no source files found in test".to_owned()],
        });
    }

    run_test_case(&test_case, language_version)
        .with_context(|| format!("Failed to run test: {test_path:?}"))
}

fn run_test_case(test_case: &IsolTestCase, language_version: LanguageVersion) -> Result<Outcome> {
    let files = &test_case.files;
    let config = TestConfig::new(files);

    let resolved = resolve_evm_target(
        test_case.evm_version.as_deref(),
        default_evm_target(language_version)?,
    )?;

    let evm_target = match resolved {
        ParsedTarget::Target(target) => target,
        ParsedTarget::FutureSpec => {
            return Ok(Outcome::Failed {
                diagnostics: vec![format!(
                    "test requires the unreleased '{FUTURE_EVM_VERSION}' EVM version, which slang has no target for"
                )],
            });
        }
    };

    let mut builder = CompilationBuilder::create(language_version, evm_target, config);

    // Add every source, so files that aren't reachable via imports (e.g.
    // sibling sources in a multi-source test) are still analyzed.
    builder.add_files(
        files
            .iter()
            .map(|(file_id, contents)| (FileId::from(file_id.as_str()), contents.clone())),
    );

    let unit = builder.build();
    let diagnostics = unit.diagnostics();

    if diagnostics.is_empty() {
        return Ok(Outcome::Passed);
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

    Ok(Outcome::Failed {
        diagnostics: rendered,
    })
}

/// Resolves the imports between the in-memory sources of an [`IsolTestCase`]
/// for the `slang` compilation builder.
///
/// Import resolution reuses the shared [`ImportResolver`] (also used by the
/// sourcify runner), with each source registered under its own name.
struct TestConfig {
    resolver: ImportResolver,
}

impl TestConfig {
    fn new(files: &OrderedMap<String, String>) -> Self {
        let source_maps = files
            .keys()
            .map(|id| SourceMap {
                // Our source names are already the "virtual" paths that imports
                // refer to, so the id and virtual path are the same.
                source_id: id.clone(),
                virtual_path: id.clone(),
            })
            .collect();

        Self {
            resolver: ImportResolver {
                import_remaps: Vec::new(),
                source_maps,
            },
        }
    }
}

impl CompilationBuilderConfig for TestConfig {
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
