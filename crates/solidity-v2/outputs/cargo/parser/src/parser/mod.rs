use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{ContractDefinition, Expression, SourceUnit};

use crate::lexer::{ContextKind, LexemeKind, Lexer};
use crate::parser::parser_error::ParserError;

mod parser_helpers;

lalrpop_mod!(
    // TODO(v2): Remove and allow lints individually if needed
    #[allow(clippy::all)]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(warnings)]
    #[rustfmt::skip]
    pub(crate) grammar, "/parser/grammar.generated.rs"); // synthesized by LALRPOP

pub mod parser_error;

/// A Solidity Parser
///
/// TODO(v2): Error recovery, for now we just fail
/// TODO(v2): Support multiple versions, for now only 0.8.30 is supported
pub trait Parser {
    /// The type of the non-terminal that this parser produces
    type NonTerminal;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, ParserError>;

    // TODO(v2): This is temporary, once the language definition is restricted to only supported versions
    // it won't be needed
    fn check_version(version: LanguageVersion) {
        assert!(
            version == LanguageVersion::V0_8_30,
            "Only version 0.8.30 is currently supported by the V2 parser"
        );
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

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, ParserError> {
        Self::check_version(version);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::SourceUnitParser::new();
        parser.parse(input, lexer).map_err(|e| e.into())
    }
}

impl Parser for ExpressionParser {
    type NonTerminal = Expression;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, ParserError> {
        Self::check_version(version);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ExpressionParser::new();
        parser.parse(input, lexer).map_err(|e| e.into())
    }
}

impl Parser for ContractDefinitionParser {
    type NonTerminal = ContractDefinition;

    fn parse(input: &str, version: LanguageVersion) -> Result<Self::NonTerminal, ParserError> {
        Self::check_version(version);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ContractDefinitionParser::new();
        parser.parse(input, lexer).map_err(|e| e.into())
    }
}

/// Iterate over the lexemes and their offsets
///
/// TODO(v2): This iterator skips all trivia, we'll want to include it in
/// future versions
impl Iterator for Lexer<'_> {
    type Item = Result<(usize, LexemeKind, usize), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(lexeme) = self.next_lexeme() {
            match lexeme.kind {
                LexemeKind::Whitespace
                | LexemeKind::EndOfLine
                | LexemeKind::SingleLineComment
                | LexemeKind::MultiLineComment
                | LexemeKind::SingleLineNatSpecComment
                | LexemeKind::MultiLineNatSpecComment => {}
                _ => return Some(Ok((lexeme.range.start, lexeme.kind, lexeme.range.end))),
            }
        }
        None
    }
}
