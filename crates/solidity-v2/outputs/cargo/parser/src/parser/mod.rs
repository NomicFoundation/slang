use bumpalo::Bump;
use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_ast::ast::nodes::{Expression, SourceUnit};
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

pub mod nodes;

lalrpop_mod!(
    #[allow(non_snake_case)]
    pub grammar, "/parser/grammar.generated.rs"); // synthesized by LALRPOP

#[cfg(test)]
mod tests;

pub mod temp_cst_output;

/// A Solidity Parser
///
/// TODO(v2): Error recovery, for now we just fail
/// TODO(v2): Support multiple versions, for now only 0.8.30 is supported
pub struct Parser {
    arena: Bump,
}

impl Parser {
    pub fn new() -> Self {
        Self { arena: Bump::new() }
    }

    pub fn parse<'arena>(
        &'arena self,
        input: &str,
        version: LanguageVersion,
    ) -> Result<SourceUnit<'arena>, String> {
        assert!(version == LanguageVersion::V0_8_30);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::SourceUnitParser::new();
        parser
            .parse(&self.arena, input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }

    pub fn parse_expression<'arena>(
        &'arena self,
        input: &str,
        version: LanguageVersion,
    ) -> Result<Expression<'arena>, String> {
        assert!(version == LanguageVersion::V0_8_30);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(&self.arena, input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}
