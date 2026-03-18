use std::fmt::Write;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity::cst::{Node, NonterminalKind};
use slang_solidity::parser::{ParseOutput, Parser as V1Parser};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit;
use slang_solidity_v2_parser::{Parser as V2Parser, ParserError};
use solidity_v2_language::SolidityDefinition;
use solidity_v2_testing_utils::reporting::diagnostic;
use solidity_v2_testing_utils::v1_comparison::parser::{NodeChecker, NodeCheckerError};

use super::allow_list;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffOutcome {
    /// Both parsers agree (both valid+same, or both invalid)
    Same,
    /// Both valid, but different CST output
    DifferentOutput,
    /// V1 invalid, V2 valid (missing validation in V2)
    V1InvalidV2Valid,
    /// V1 valid, V2 invalid (V2 bug or missing feature)
    V1ValidV2Invalid,
}

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("cst_output")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str().to_owned();
    let source = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();

    let mut last_output = None;
    let mut last_diff = None;

    let tested_versions: Vec<LanguageVersion> = SolidityDefinition::create()
        .collect_all_breaking_versions()
        .into_iter()
        .map(|v| LanguageVersion::try_from(v).unwrap())
        .collect();

    let allow_entry = allow_list::find_entry(parser_name, test_name);
    let mut failures: Vec<String> = Vec::new();

    for &lang_version in &tested_versions {
        let v2_output = V2Parser::parse(&source, lang_version);

        let v2_output = match last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &v2_output => last,
            _ => {
                let (status, content) = match &v2_output {
                    Ok(parsed_cst) => {
                        // Print structured CST
                        ("success", format!("{parsed_cst:#?}\n"))
                    }
                    Err(err) => {
                        // We don't care about the errors for now, we just write them
                        let e = diagnostic::render(err, &source_id, &source, false);
                        ("failure", format!("{e}\n"))
                    }
                };

                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{lang_version}-{status}.txt"));

                fs.write_file_raw(&snapshot_path, content)?;
                last_output.insert(v2_output)
            }
        };

        // Now check the diff between V1 and V2

        let v1_output = V1Parser::create(lang_version.into())?
            .parse_nonterminal(NonterminalKind::SourceUnit, &source);

        let diff = diff_report(&v1_output, v2_output, &source_id, &source);

        let outcome = diff.0;

        match &last_diff {
            // Skip if the diff is the same as last time
            Some(ref last) if last == &diff => (),
            _ => {
                let (outcome, diff_report_text) = &diff;

                let diff_path = test_dir.join("diff/generated").join(format!(
                    "{lang_version}-{label}.txt",
                    label = if *outcome == DiffOutcome::Same {
                        "same"
                    } else {
                        "diff"
                    }
                ));

                fs.write_file_raw(&diff_path, diff_report_text)?;
                last_diff = Some(diff);
            }
        }

        // Check against allow list
        match outcome {
            DiffOutcome::Same => {
                if let Some(entry) = allow_entry {
                    if entry.versions.matches(lang_version) {
                        failures.push(format!(
                            "[{lang_version}] Expected diff (allow list: {:?}) but V1/V2 agree. \
                             Remove or update the allow list entry.",
                            entry.kind,
                        ));
                    }
                }
            }
            _ => {
                if let Some(entry) = allow_entry {
                    if entry.versions.matches(lang_version) && entry.kind.matches_outcome(outcome) {
                        // Expected difference, suppress
                    } else {
                        failures.push(format!(
                            "[{lang_version}] Unexpected diff: {outcome:?}. \
                             Allow list entry exists but doesn't cover this \
                             (kind: {:?}, version predicate doesn't match or wrong outcome).",
                            entry.kind,
                        ));
                    }
                } else {
                    failures.push(format!("[{lang_version}] Unexpected diff: {outcome:?}.",));
                }
            }
        }
    }

    if !failures.is_empty() {
        let mut msg = format!(
            "Test ({parser_name}, {test_name}) had {} failure(s):\n",
            failures.len()
        );
        for f in &failures {
            writeln!(msg, "  - {f}").unwrap();
        }
        panic!("{msg}");
    }

    Ok(())
}

/// Given V1 and V2 parser output, compare them and produce a report.
fn diff_report(
    v1_output: &ParseOutput,
    v2_output: &Result<SourceUnit, ParserError>,
    source_id: &str,
    source: &str,
) -> (DiffOutcome, String) {
    match v2_output {
        Ok(structured_cst) => {
            // check V1 validity
            if v1_output.is_valid() {
                // Check for errors
                let errors =
                    structured_cst.check_node(&Node::Nonterminal(Rc::clone(v1_output.tree())));

                if errors.is_empty() {
                    (
                        DiffOutcome::Same,
                        "V2 parser produced the same output as V1 output.".to_string(),
                    )
                } else {
                    let formatted_errors = write_errors(&errors, source_id, source);
                    (DiffOutcome::DifferentOutput, formatted_errors)
                }
            } else {
                (
                    DiffOutcome::V1InvalidV2Valid,
                    "V1 Parser: Invalid\nV2 Parser: Valid\n".to_string(),
                )
            }
        }
        Err(_) if v1_output.is_valid() => (
            DiffOutcome::V1ValidV2Invalid,
            "V1 Parser: Valid\nV2 Parser: Invalid\n".to_string(),
        ),
        Err(_) => (
            // TODO(v2): Compare the errors to check they are failing for the same reason
            DiffOutcome::Same,
            "Both V1 and V2 produced invalid output.\n".to_string(),
        ),
    }
}

fn write_errors(errors: &Vec<NodeCheckerError>, source_id: &str, source: &str) -> String {
    if errors.is_empty() {
        return String::new();
    }

    let mut s = String::new();
    writeln!(s, "Errors: # {count} total", count = errors.len()).unwrap();

    for error in errors {
        writeln!(s, "  - >").unwrap();
        for line in diagnostic::render(error, source_id, source, false).lines() {
            writeln!(s, "    {line}").unwrap();
        }
    }

    s
}
