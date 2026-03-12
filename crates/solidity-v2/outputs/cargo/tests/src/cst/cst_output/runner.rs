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
use solidity_v2_testing_utils::reporting::diagnostic;
use solidity_v2_testing_utils::v1_comparison::parser::{NodeChecker, NodeCheckerError};

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("cst_output/from_v1")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input/generated/input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str().to_owned();
    let source = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();

    let mut last_output = None;
    let mut last_diff = None;

    // TODO(v2): Test all breaking versions
    // _SLANG_V2_PARSER_VERSION_
    let tested_versions = [LanguageVersion::V0_8_30];

    for lang_version in tested_versions {
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

        match &last_diff {
            // Skip if the diff is the same as last time
            Some(ref last) if last == &diff => (),
            _ => {
                let (is_same, should_panic, diff_report) = &diff;

                let diff_path = test_dir.join("diff/generated").join(format!(
                    "{lang_version}-{diff}.txt",
                    diff = if *is_same { "same" } else { "diff" }
                ));

                fs.write_file_raw(&diff_path, diff_report)?;

                if let Some(panic_message) = should_panic {
                    panic!("{panic_message}");
                }
                last_diff = Some(diff);
            }
        }
    }

    Ok(())
}

/// Given V1 and V2 parser output, compare them and produce a report. The report includes:
/// - Whether the outputs are the same (ignoring errors)
/// - An optional panic message, if `None` then it shouldn't panic.
/// - A string report describing the differences, which may include error messages if they differ.
fn diff_report(
    v1_output: &ParseOutput,
    v2_output: &Result<SourceUnit, ParserError>,
    source_id: &str,
    source: &str,
) -> (bool, Option<String>, String) {
    match v2_output {
        Ok(structured_cst) => {
            // check V1 validity
            if v1_output.is_valid() {
                // Check for errors
                let errors =
                    structured_cst.check_node(&Node::Nonterminal(Rc::clone(v1_output.tree())));

                if errors.is_empty() {
                    (
                        true,
                        None,
                        "V2 parser produced the same output as V1 output.".to_string(),
                    )
                } else {
                    let formatted_errors = write_errors(&errors, source_id, source);

                    // TODO(v2): This is forced not to panic since some tests in V1 produce different outputs,
                    // in particular `state_variable_function`
                    (false, None, formatted_errors)
                }
            } else {
                // TODO(v2): This is forced not to panic, since V2 has no validation yet, but we
                // do want to be aware of these cases, so we write them in the diff report and we can review them later.
                (
                    false,
                    None,
                    "V1 Parser: Invalid\nV2 Parser: Valid\n".to_string(),
                )
            }
        }
        // TODO(v2): We force this not to panic, since we need lexical context switching to work for some
        // tests to pass
        Err(_) if v1_output.is_valid() => (
            false,
            None,
            "V1 Parser: Valid\nV2 Parser: Invalid\n".to_string(),
        ),
        Err(_) => {
            // TODO(v2): Both are invalid, compare the errors
            (
                true,
                None,
                "Both V1 and V2 produced invalid output.\n".to_string(),
            )
        }
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
