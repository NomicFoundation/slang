#[path = "node_checker.generated.rs"]
pub mod node_checker;

use std::fmt::{Debug, Write};
use std::path::Path;

use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use semver::Version;
use slang_solidity::cst::{Cursor, Node, TextRange};
use slang_solidity::parser::ParseOutput;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::temp_testing::node_checker::{NodeChecker, NodeCheckerError};
use crate::Parser as ParserV2;

/// We use this trait only to combine Debug and `NodeChecker`
trait NodeCheckerDebug: NodeChecker + Debug {}

impl<T> NodeCheckerDebug for T where T: NodeChecker + Debug {}

/// Compare V1 and V2 only for non terminals and versions supported by both
pub fn test_v2_output(
    parser_name: &str,
    test_dir: &Path,
    fs: &mut CodegenFileSystem,
    source_id: &str,
    source: &String,
    version: &Version,
    v1_output: &ParseOutput,
) -> Result<(), anyhow::Error> {
    // TODO(v2): We only check 0.8.30 for now, we should eventually check all versions
    if *version != Version::new(0, 8, 30) {
        return Ok(());
    }

    // Get the output for v2
    let v2_output: Result<Box<dyn NodeCheckerDebug>, _> = match parser_name {
        // For now we only have a few parsers, having all parsers with LALRPOP is expensive
        "SourceUnit" => ParserV2::parse(&source, LanguageVersion::V0_8_30)
            .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>),
        "Expression" => ParserV2::parse_expression(&source, LanguageVersion::V0_8_30)
            .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>),
        "ContractDefinition" => {
            ParserV2::parse_contract_definition(&source, LanguageVersion::V0_8_30)
                .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>)
        }
        _ => {
            // Ignore everything else
            return Ok(());
        }
    };

    let status = if v2_output.is_ok() {
        "success"
    } else {
        "failure"
    };

    let snapshot_path = test_dir
        .join("v2/generated")
        .join(format!("{version}-{status}.yml"));

    let mut s = String::new();

    match &v2_output {
        Ok(parsed_checker) => {
            // Print AST
            writeln!(s, "{parsed_checker:#?}")?;
        }
        Err(err) => {
            // We don't care about the errors for now, we just write them
            writeln!(s, "{err:#?}")?;
        }
    };

    fs.write_file_raw(&snapshot_path, s)?;

    // Now check the diff between V1 and V2

    let diff_path = test_dir
        .join("diff/generated")
        .join(format!("{version}.txt"));

    let mut s = String::new();
    match &v2_output {
        Ok(parsed_checker) => {
            // check V1 validity
            if v1_output.is_valid() {
                // Check for errors
                let checked =
                    parsed_checker.check_node(&Node::Nonterminal(v1_output.tree().clone()));

                if !checked.is_empty() {
                    write_errors(&mut s, &checked, source_id, &source)?;

                    fs.write_file_raw(&diff_path, s)?;
                    assert!(
                        checked.is_empty(),
                        "The AST is different between both parsers",
                    );
                }
            } else {
                writeln!(s, "V1 Parser: Invalid")?;
                writeln!(s, "V2 Parser: Valid")?;
                fs.write_file_raw(&diff_path, s)?;
                assert!(
                    v1_output.is_valid(),
                    "V1 parser is not valid, but V2 Parser is"
                );
            }
        }
        Err(_) if v1_output.is_valid() => {
            writeln!(s, "V1 Parser: Valid")?;
            writeln!(s, "V2 Parser: Invalid")?;
            fs.write_file_raw(&diff_path, s)?;

            assert!(!v1_output.is_valid(), "V1 parser is valid, but V2 is not");
        }
        Err(_) => {
            // TODO(v2): Both are invalid, compare the errors
        }
    };
    Ok(())
}

fn write_errors(
    w: &mut String,
    errors: &Vec<NodeCheckerError>,
    source_id: &str,
    source: &str,
) -> Result<()> {
    if errors.is_empty() {
        return Ok(());
    }

    writeln!(w, "Errors: # {count} total", count = errors.len())?;

    for error in errors {
        writeln!(w, "  - >")?;
        for line in slang_solidity::diagnostic::render(error, source_id, source, false).lines() {
            writeln!(w, "    {line}")?;
        }
    }

    Ok(())
}

pub fn compare_with_v1_cursor(source: String, root_cursor: Cursor) -> Vec<NodeCheckerError> {
    let v2_output = ParserV2::parse(&source, LanguageVersion::V0_8_30);

    match v2_output {
        Ok(v2_tree) => v2_tree.check_node(&root_cursor.node()),
        Err(error_message) => vec![NodeCheckerError::new(error_message, TextRange::default())],
    }
}
