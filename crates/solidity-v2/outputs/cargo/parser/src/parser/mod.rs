use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{ContractDefinition, Expression, SourceUnit};

use crate::lexer::{ContextKind, Lexer};

mod nodes;

lalrpop_mod!(
    // TODO(v2): Remove and allow lints individually if needed
    #[allow(clippy::all)]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(warnings)]
    #[rustfmt::skip]
    pub(crate) grammar, "/parser/grammar.generated.rs"); // synthesized by LALRPOP

// TODO(v2): This is temporary and it's used to compare against V1
pub mod temp_testing;

/// A Solidity Parser
///
/// TODO(v2): Error recovery, for now we just fail
/// TODO(v2): Support multiple versions, for now only 0.8.30 is supported
pub trait Parser {
    /// The type of the non-terminal that this parser produces
    type NonTerminal;

    // TODO(v2): Errors should be something other than `String`
    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, String>;

    // TODO(v2): This is temporary, once the language definition is restricted to only supported versions
    // it won't be needed
    fn check_version(version: LanguageVersion) -> Result<(), String> {
        if version == LanguageVersion::V0_8_30 {
            Ok(())
        } else {
            Err("Only version 0.8.30 is currently supported by the V2 parser".to_string())
        }
    }
}

#[derive(Default)]
pub struct SourceUnitParser;

#[derive(Default)]
pub struct ExpressionParser;

#[derive(Default)]
pub struct ContractDefinitionParser;

impl Parser for SourceUnitParser {
    type NonTerminal = SourceUnit;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, String> {
        Self::check_version(version)?;

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::SourceUnitParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}

impl Parser for ExpressionParser {
    type NonTerminal = Expression;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, String> {
        Self::check_version(version)?;

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}

impl Parser for ContractDefinitionParser {
    type NonTerminal = ContractDefinition;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, String> {
        Self::check_version(version)?;

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ContractDefinitionParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}
