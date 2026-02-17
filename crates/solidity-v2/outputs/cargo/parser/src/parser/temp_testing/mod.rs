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
    /// Return a new V2Tester instance for the given parser name.
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
    phantom: std::marker::PhantomData<T>,
}

impl<NT, T: ParserV2<NonTerminal = NT>> V2TesterImpl<NT, T> {
    pub fn new() -> Self {
        Self {
            last_output: None,
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
            Some(ref last) if last == &v2_output => return Ok(()),
            _ => &*self.last_output.insert(v2_output),
        };

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
                        parsed_checker.check_node(&Node::Nonterminal(Rc::clone(v1_output.tree())));

                    if !checked.is_empty() {
                        write_errors(&mut s, &checked, source_id, source)?;

                        fs.write_file_raw(&diff_path, s)?;
                        // TODO(v2): This should fail, but some errors in V1 are causing V2 to produce different outputs
                        // panic!("V2 parser produced a different output than V1 output.");
                    }
                } else {
                    writeln!(s, "V1 Parser: Invalid")?;
                    writeln!(s, "V2 Parser: Valid")?;
                    fs.write_file_raw(&diff_path, s)?;
                    // TODO(v2): This should fail, but for now we really don't care about validation in V2
                }
            }
            Err(_) if v1_output.is_valid() => {
                writeln!(s, "V1 Parser: Valid")?;
                writeln!(s, "V2 Parser: Invalid")?;
                fs.write_file_raw(&diff_path, s)?;
                panic!("V2 parser produced invalid output against V1 valid output. ");
            }
            Err(_) => {
                // TODO(v2): Both are invalid, compare the errors
            }
        }

        Ok(())
    }
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

pub fn compare_with_v1_cursor(source: &str, root_cursor: &Cursor) -> Vec<NodeCheckerError> {
    let v2_output = SourceUnitParser::parse(source, LanguageVersion::V0_8_30);

    match v2_output {
        Ok(v2_tree) => v2_tree.check_node(&root_cursor.node()),
        Err(error_message) => vec![NodeCheckerError::new(error_message, TextRange::default())],
    }
}
