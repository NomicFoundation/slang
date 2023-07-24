use super::{
    cst,
    parse_output::{ParseError, ParseOutput},
    stream::Stream,
};

use crate::syntax::nodes::TokenKind;

// Return type of the function has to be a type parameter of the trait
pub trait ScannerFunction<L, R>
where
    Self: Fn(&L, &mut Stream) -> R,
{
    fn scan(&self, language: &L, input: &str, token_kind: TokenKind) -> Option<ParseOutput>;
}

impl<L, F> ScannerFunction<L, bool> for F
where
    F: Fn(&L, &mut Stream) -> bool,
{
    fn scan(&self, language: &L, input: &str, token_kind: TokenKind) -> Option<ParseOutput> {
        let mut stream = Stream::new(input);
        Some(if self(language, &mut stream) && stream.peek().is_none() {
            ParseOutput {
                parse_tree: cst::Node::token(
                    token_kind,
                    input[..stream.position().utf8].to_string(),
                ),
                errors: vec![],
            }
        } else {
            ParseOutput {
                parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                errors: vec![ParseError::new_covering_range(
                    Default::default()..input.into(),
                    vec![token_kind],
                )],
            }
        })
    }
}

impl<L, F> ScannerFunction<L, Option<bool>> for F
where
    F: Fn(&L, &mut Stream) -> Option<bool>,
{
    fn scan(&self, language: &L, input: &str, token_kind: TokenKind) -> Option<ParseOutput> {
        let mut stream = Stream::new(input);
        self(language, &mut stream).map(|result| {
            if result && stream.peek().is_none() {
                ParseOutput {
                    parse_tree: cst::Node::token(
                        token_kind,
                        input[..stream.position().utf8].to_string(),
                    ),
                    errors: vec![],
                }
            } else {
                ParseOutput {
                    parse_tree: cst::Node::token(TokenKind::SKIPPED, input.to_string()),
                    errors: vec![ParseError::new_covering_range(
                        Default::default()..input.into(),
                        vec![token_kind],
                    )],
                }
            }
        })
    }
}
