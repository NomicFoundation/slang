use crate::cst;
use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::{TextRange, TextRangeExtensions as _};

use super::ParserContext;

impl ParserResult {
    pub fn try_recover_with(
        self,
        input: &mut ParserContext,
        skip_tokens_for_recovery: impl Fn(&mut ParserContext) -> Option<TextRange>,
    ) -> ParserResult {
        match self {
            ParserResult::IncompleteMatch(mut result) => {
                if let Some(skipped) = skip_tokens_for_recovery(input) {
                    result.nodes.push(cst::Node::token(
                        TokenKind::SKIPPED,
                        input.content(skipped.utf8()),
                    ));

                    input.emit(ParseError {
                        text_range: skipped,
                        tokens_that_would_have_allowed_more_progress: result
                            .expected_tokens
                            .clone(),
                    });
                }

                ParserResult::r#match(result.nodes, result.expected_tokens)
            }
            result => result,
        }
    }
}
