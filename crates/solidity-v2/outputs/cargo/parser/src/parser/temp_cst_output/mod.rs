#[path = "node_checker.generated.rs"]
pub mod node_checker;

use std::fmt::Debug;
use std::str::FromStr;

use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use semver::Version;
use slang_solidity::cst::{Cursor, Node, NonterminalKind, TextRange};
use slang_solidity::parser::Parser;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::temp_cst_output::node_checker::{NodeChecker, NodeCheckerError};
use crate::Parser as ParserV2;

/// We use this trait only to combine Debug and NodeChecker
trait NodeCheckerDebug: NodeChecker + Debug {}

impl<T> NodeCheckerDebug for T where T: NodeChecker + Debug {}

/// Compare V1 and V2 only for non terminals and versions supported by both
pub fn compare_with_v1_output(
    parser_name: &str,
    test_dir: &std::path::PathBuf,
    fs: &mut CodegenFileSystem,
    source: &String,
    version: Version,
) -> Result<bool, anyhow::Error> {
    // Get the output for v2
    let parser = ParserV2::new();
    let v2_output: Result<Box<dyn NodeCheckerDebug>, _> = match parser_name {
        // For now we only have a few parsers, having all parsers with LALRPOP is expensive
        "SourceUnit" => parser
            .parse(&source, LanguageVersion::V0_8_30)
            .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>),
        "Expression" => parser
            .parse_expression(&source, LanguageVersion::V0_8_30)
            .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>),
        "ContractDefinition" => parser
            .parse_contract_definition(&source, LanguageVersion::V0_8_30)
            .map(|node| Box::new(node) as Box<dyn NodeCheckerDebug>),
        _ => {
            // Ignore everything else
            return Ok(true);
        }
    };

    // Get the output for V1
    let tested_kind = NonterminalKind::from_str(parser_name)
        .unwrap_or_else(|_| panic!("No such parser: {parser_name}"));
    let v1_output = Parser::create(version.clone())?.parse_nonterminal(tested_kind, &source);

    let status = if v2_output.is_ok() {
        "success"
    } else {
        "failure"
    };
    let snapshot_path = test_dir
        .join("v2/generated")
        .join(format!("{version}-{status}.yml"));

    match v2_output {
        Ok(parsed_checker) => {
            let mut s = String::new();

            // Print AST
            s.push_str(&format!("{parsed_checker:#?}"));

            // check V1 validity
            if v1_output.is_valid() {
                // Check for errors
                let checked =
                    parsed_checker.check_node(&Node::Nonterminal(v1_output.tree().clone()));

                if !checked.is_empty() {
                    s.push_str(&"\n----------------\n");
                    for err in &checked {
                        s.push_str(&format!("{}\n\n", err.err));
                    }
                }
                fs.write_file_raw(&snapshot_path, s)?;
                Ok(checked.is_empty())
                // assert!(
                //     checked.is_empty(),
                //     "The AST is different between both parsers",
                // );
            } else {
                s.push_str(&"\n----------------\n");
                s.push_str("\nV1 Parser: Invalid\n");
                fs.write_file_raw(&snapshot_path, s)?;
                Ok(v1_output.is_valid())
                // assert!(
                //     v1_output.is_valid(),
                //     "V1 parser is not valid, but V2 Parser is"
                // );
            }
        }
        Err(err) => {
            // We don't care about the errors for now, we just write them
            fs.write_file_raw(&snapshot_path, format!("{err:#?}"))?;

            Ok(!v1_output.is_valid())
            // assert!(!v1_output.is_valid(), "V1 parser is valid, but V2 is not");
        }
    }
}

pub fn compare_with_v1_cursor(source: String, root_cursor: Cursor) -> Vec<NodeCheckerError> {
    let parser = ParserV2::new();
    let v2_output = parser.parse(&source, LanguageVersion::V0_8_30);

    if let Ok(v2_tree) = v2_output {
        v2_tree.check_node(&root_cursor.node())
    } else {
        vec![NodeCheckerError::new(
            String::from("V2 was invalid!!"),
            TextRange::default(),
        )]
    }
}
