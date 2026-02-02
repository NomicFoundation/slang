use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{ContractDefinition, Expression, SourceUnit};

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

pub(crate) mod nodes;

lalrpop_mod!(
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
pub struct Parser;

impl Parser {
    pub fn parse(input: &str, version: LanguageVersion) -> Result<SourceUnit, String> {
        assert!(
            version >= LanguageVersion::V0_4_11 && version <= LanguageVersion::V0_8_33,
            "Only versions [0.4.11, 0.9.0) are currently supported by the V2 parser"
        );

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::SourceUnitParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }

    pub fn parse_expression(input: &str, version: LanguageVersion) -> Result<Expression, String> {
        assert!(
            version >= LanguageVersion::V0_4_11 && version <= LanguageVersion::V0_8_33,
            "Only versions [0.4.11, 0.9.0) are currently supported by the V2 parser"
        );

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }

    pub fn parse_contract_definition(
        input: &str,
        version: LanguageVersion,
    ) -> Result<ContractDefinition, String> {
        assert!(
            version >= LanguageVersion::V0_4_11 && version <= LanguageVersion::V0_8_33,
            "Only versions [0.4.11, 0.9.0) are currently supported by the V2 parser"
        );

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        let parser = grammar::ContractDefinitionParser::new();
        parser
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}
