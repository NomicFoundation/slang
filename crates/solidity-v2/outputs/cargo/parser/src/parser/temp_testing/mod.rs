#[path = "node_checker.generated.rs"]
pub mod node_checker;

use std::fmt::{Debug, Write};
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;
use semver::Version;
use slang_solidity::cst::{Cursor, Node, TextRange};
use slang_solidity::parser::ParseOutput;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{ContractDefinition, Expression, SourceUnit};

use crate::parser::{ContractDefinitionParser, ExpressionParser, SourceUnitParser};
use crate::temp_testing::node_checker::{NodeChecker, NodeCheckerError};
use crate::Parser as ParserV2;

/// A Tester for V2 parser that compares against V1 outputs.
///
/// It generates snapshots for V2 outputs and diffs against V1 outputs.
pub trait V2Tester {
    fn test_next(
        &mut self,
        test_dir: &Path,
        fs: &mut CodegenFileSystem,
        source_id: &str,
        source: &str,
        version: &Version,
        v1_output: &ParseOutput,
    ) -> Result<()>;
}

pub struct V2TesterConstructor;

impl V2TesterConstructor {
    /// Return a new `V2Tester` instance for the given parser name.
    ///
    /// Only `SourceUnit`, `Expression`, and `ContractDefinition` are supported for now.
    pub fn new_tester(parser_name: &str) -> Box<dyn V2Tester> {
        match parser_name {
            "SourceUnit" => Box::new(V2TesterImpl::<SourceUnit, SourceUnitParser>::new()),
            "Expression" => Box::new(V2TesterImpl::<Expression, ExpressionParser>::new()),
            "ContractDefinition" => {
                Box::new(V2TesterImpl::<ContractDefinition, ContractDefinitionParser>::new())
            }
            _ => Box::new(NonSupportedParserTester),
        }
    }
}

/// A dummy tester that does nothing, used for unsupported parsers.
struct NonSupportedParserTester;

impl V2Tester for NonSupportedParserTester {
    fn test_next(
        &mut self,
        _test_dir: &Path,
        _fs: &mut CodegenFileSystem,
        _source_id: &str,
        _source: &str,
        _version: &Version,
        _v1_output: &ParseOutput,
    ) -> Result<()> {
        Ok(())
    }
}

struct V2TesterImpl<NT, T: ParserV2<NonTerminal = NT>> {
    last_output: Option<Result<NT, String>>,
    last_diff: Option<(bool, Option<String>, String)>,
    phantom: std::marker::PhantomData<T>,
}

impl<NT, T: ParserV2<NonTerminal = NT>> V2TesterImpl<NT, T> {
    pub fn new() -> Self {
        Self {
            last_output: None,
            last_diff: None,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<NT: NodeChecker + Debug + PartialEq, T: ParserV2<NonTerminal = NT>> V2Tester
    for V2TesterImpl<NT, T>
{
    fn test_next(
        &mut self,
        test_dir: &Path,
        fs: &mut CodegenFileSystem,
        source_id: &str,
        source: &str,
        version: &Version,
        v1_output: &ParseOutput,
    ) -> Result<()> {
        // We check version 0.8.30
        // TODO(v2) check all  versions
        if *version != Version::new(0, 8, 30) {
            return Ok(());
        }

        // Get the output for v2
        let lang_version = LanguageVersion::try_from(version.clone())
            .unwrap_or_else(|_| panic!("Unsupported version: {version}"));

        let v2_output: Result<NT, _> = T::parse(source, lang_version);

        let v2_output = match self.last_output {
            // Skip this version if it produces the same output.
            // Note: comparing objects cheaply before expensive serialization.
            Some(ref last) if last == &v2_output => last,
            _ => {
                let status = if v2_output.is_ok() {
                    "success"
                } else {
                    "failure"
                };

                let snapshot_path = test_dir
                    .join("v2/generated")
                    .join(format!("{version}-{status}.txt"));

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
                }

                fs.write_file_raw(&snapshot_path, s)?;
                self.last_output.insert(v2_output)
            }
        };

        // Now check the diff between V1 and V2

        let diff = Self::diff_report(v1_output, v2_output, source_id, source);

        match &self.last_diff {
            // Skip if the diff is the same as last time
            Some(ref last) if last == &diff => (),
            _ => {
                let (is_same, should_panic, diff_report) = &diff;

                let diff_path = test_dir.join("diff/generated").join(format!(
                    "{version}-{diff}.txt",
                    diff = if *is_same { "same" } else { "diff" }
                ));

                fs.write_file_raw(&diff_path, diff_report)?;

                if let Some(panic_message) = should_panic {
                    panic!("{panic_message}");
                }
                self.last_diff = Some(diff);
            }
        }

        Ok(())
    }
}

impl<NT: NodeChecker + Debug + PartialEq, T: ParserV2<NonTerminal = NT>> V2TesterImpl<NT, T> {
    fn diff_report(
        v1_output: &ParseOutput,
        v2_output: &Result<NT, String>,
        source_id: &str,
        source: &str,
    ) -> (bool, Option<String>, String) {
        match v2_output {
            Ok(parsed_checker) => {
                // check V1 validity
                if v1_output.is_valid() {
                    // Check for errors
                    let checked =
                        parsed_checker.check_node(&Node::Nonterminal(Rc::clone(v1_output.tree())));

                    if checked.is_empty() {
                        (
                            true,
                            None,
                            "V2 parser produced the same output as V1 output.".to_string(),
                        )
                    } else {
                        let errors = write_errors(&checked, source_id, source);

                        (
                            false,
                            Some(
                                "V2 parser produced a different output than V1 output.".to_string(),
                            ),
                            errors,
                        )
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
            Err(_) if v1_output.is_valid() => (
                false,
                Some("V2 parser produced invalid output against V1 valid output.".to_string()),
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
}

fn write_errors(errors: &Vec<NodeCheckerError>, source_id: &str, source: &str) -> String {
    if errors.is_empty() {
        return String::new();
    }

    let mut s = String::new();
    writeln!(s, "Errors: # {count} total", count = errors.len()).unwrap();

    for error in errors {
        writeln!(s, "  - >").unwrap();
        for line in slang_solidity::diagnostic::render(error, source_id, source, false).lines() {
            writeln!(s, "    {line}").unwrap();
        }
    }

    s
}

pub fn compare_with_v1_cursor(source: &str, root_cursor: &Cursor) -> Vec<NodeCheckerError> {
    let v2_output = SourceUnitParser::parse(source, LanguageVersion::V0_8_30);

    match v2_output {
        Ok(v2_tree) => v2_tree.check_node(&root_cursor.node()),
        Err(error_message) => vec![NodeCheckerError::new(error_message, TextRange::default())],
    }
}
