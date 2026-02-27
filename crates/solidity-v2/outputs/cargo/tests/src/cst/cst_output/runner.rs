use std::fmt::Write;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::cst::{Node, NonterminalKind};
use slang_solidity::parser::Parser;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::SourceUnitParser;
use solidity_v2_testing_utils::reporting::diagnostic;
use solidity_v2_testing_utils::v1_comparison::parser::{NodeChecker, NodeCheckerError};

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let snapshots_crate = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?;

    let test_dir = snapshots_crate
        .join("cst_output/from_v1/SourceUnit")
        .join(parser_name)
        .join(test_name);

    let input_path = test_dir.join("input/generated/input.sol");
    let source_id = input_path.strip_repo_root()?.unwrap_str().to_owned();
    let source = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();

    // Parse with v2 (always SourceUnit, since we wrapped the input)
    let v2_output = SourceUnitParser::parse(&source, LanguageVersion::V0_8_30);

    // Write v2 snapshot
    let (status, v2_snapshot) = match &v2_output {
        Ok(tree) => ("success", format!("{tree:#?}\n")),
        Err(err) => (
            "failure",
            format!("{}\n", diagnostic::render(err, &source_id, &source, false)),
        ),
    };

    fs.write_file_raw(
        test_dir
            .join("generated")
            .join(format!("0.8.30-{status}.txt")),
        &v2_snapshot,
    )?;

    // Parse with v1 (as SourceUnit, since we wrapped the input)
    let v1_parser = Parser::create(Version::new(0, 8, 30))?;
    let v1_output = v1_parser.parse_nonterminal(NonterminalKind::SourceUnit, &source);

    // Compare v1 and v2
    let (diff_status, diff_report) = match &v2_output {
        Ok(tree) => {
            if v1_output.is_valid() {
                let errors = tree.check_node(&Node::Nonterminal(Rc::clone(v1_output.tree())));

                if errors.is_empty() {
                    (
                        "same",
                        "V2 parser produced the same output as V1 output.\n".to_owned(),
                    )
                } else {
                    ("diff", write_errors(&errors, &source_id, &source))
                }
            } else {
                ("diff", "V1 Parser: Invalid\nV2 Parser: Valid\n".to_owned())
            }
        }
        Err(_) if v1_output.is_valid() => {
            ("diff", "V1 Parser: Valid\nV2 Parser: Invalid\n".to_owned())
        }
        Err(_) => (
            "same",
            "Both V1 and V2 produced invalid output.\n".to_owned(),
        ),
    };

    fs.write_file_raw(
        test_dir
            .join("generated")
            .join(format!("0.8.30-{diff_status}.txt")),
        &diff_report,
    )?;

    Ok(())
}

fn write_errors(errors: &[NodeCheckerError], source_id: &str, source: &str) -> String {
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
