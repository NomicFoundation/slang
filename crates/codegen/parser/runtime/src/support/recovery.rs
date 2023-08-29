use crate::cst;
use crate::kinds::TokenKind;
use crate::parse_error::ParseError;
use crate::support::ParserResult;
use crate::text_index::{TextRange, TextRangeExtensions as _};

use super::Stream;

impl ParserResult {
    pub fn try_recover_with(
        self,
        stream: &mut Stream,
        skip_tokens_for_recovery: impl Fn(&mut Stream) -> Option<TextRange>,
    ) -> ParserResult {
        match self {
            ParserResult::IncompleteMatch(mut result) => {
                if let Some(skipped) = skip_tokens_for_recovery(stream) {
                    result.nodes.push(cst::Node::token(
                        TokenKind::SKIPPED,
                        stream.content(skipped.utf8()),
                    ));

                    stream.emit(ParseError {
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
