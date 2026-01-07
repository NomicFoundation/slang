use bumpalo::Bump;
use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{ContractDefinition, Expression, SourceUnit};

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

pub(crate) mod nodes;

lalrpop_mod!(
    #[allow(non_snake_case)]
    pub(crate) grammar, "/parser/grammar.generated.rs"); // synthesized by LALRPOP

#[cfg(test)]
mod tests;

// TODO(v2): This is temporary and it's used to compare against V1
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
        assert!(
            version == LanguageVersion::V0_8_30,
            "Only 0.8.30 is currently supported by the V2 parser"
        );

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
        assert!(
            version == LanguageVersion::V0_8_30,
            "Only 0.8.30 is currently supported by the V2 parser"
        );

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(&self.arena, input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }

    pub fn parse_contract_definition<'arena>(
        &'arena self,
        input: &str,
        version: LanguageVersion,
    ) -> Result<ContractDefinition<'arena>, String> {
        assert!(
            version == LanguageVersion::V0_8_30,
            "Only 0.8.30 is currently supported by the V2 parser"
        );

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ContractDefinitionParser::new();
        parser
            .parse(&self.arena, input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}
