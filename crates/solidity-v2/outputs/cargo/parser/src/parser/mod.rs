use std::rc::Rc;

use lalrpop_util::lalrpop_mod;
use slang_solidity_v2_ast::ast::nodes::{Expression, SourceUnitStruct};
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

// TODO: How do I get rid of the squiggly line here?
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
pub struct Parser {}

impl Parser {
    pub fn parse(input: &str, version: LanguageVersion) -> Result<Rc<SourceUnitStruct>, String> {
        assert!(version == LanguageVersion::V0_8_30);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        grammar::SourceUnitParser::new()
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }

    pub fn parse_expression(input: &str, version: LanguageVersion) -> Result<Expression, String> {
        assert!(version == LanguageVersion::V0_8_30);

        let lexer = Lexer::new(ContextKind::Solidity, input, version);
        grammar::ExpressionParser::new()
            .parse(input, lexer)
            // TODO(v2): Improve on showing the error
            .map_err(|e| format!("{e:?}"))
    }
}
