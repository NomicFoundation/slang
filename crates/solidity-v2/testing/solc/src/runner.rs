use std::path::Path;

use anyhow::{Context, Result};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_testing_utils::compilation;
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
    // The test's sources, keyed the way the compilation refers to them.
    let files: SortedMap<FileId, String> = test_case
        .files
        .iter()
        .map(|(name, contents)| (FileId::from(name.as_str()), contents.clone()))
        .collect();

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

    let unit = compilation::compile(&files, language_version, evm_target);

    let errors: Vec<_> = unit
        .diagnostics()
        .iter()
        .filter(|diagnostic| match diagnostic.severity() {
            DiagnosticSeverity::Error => true,
            DiagnosticSeverity::Warning => false,
        })
        .map(|diagnostic| {
            let file_id = diagnostic.file_id();
            let source = files.get(file_id).map(String::as_str).unwrap_or_default();
            diagnostic::render(diagnostic, file_id.as_str(), source, false)
        })
        .collect();

    if errors.is_empty() {
        return Ok(Outcome::Passed);
    }

    Ok(Outcome::Failed {
        diagnostics: errors,
    })
}
