use std::cell::RefCell;

use codegen_schema::types::productions::ProductionKind;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
    character_filter::CharacterFilter, code_generator::CodeGenerator,
    combinator_node::CombinatorNode, terminal_trie::TerminalTrie,
};

impl<'context> CharacterFilter<'context> {
    pub fn to_lexer_code(&self, kind: String, code: &mut CodeGenerator) -> TokenStream {
        let kind = code.add_token_kind(kind);
        if let CharacterFilter::Char { char } = self {
            quote!(lex_terminal!(#kind, #char))
        } else {
            let predicate = self.to_predicate(false);
            quote!(lex_terminal!(#kind, |&c: &char| #predicate))
        }
    }
}

impl TerminalTrie {
    pub fn to_lexer_code(&self, code: &mut CodeGenerator) -> TokenStream {
        let code = RefCell::new(code);
        let lexer = self.generate(
            &|label, children| quote!( just(#label).ignore_then(choice!( #(#children),* )) ),
            &|entry, label| {
                let kind = code
                    .borrow_mut()
                    .add_token_kind(entry.name.clone().unwrap());
                if label.is_empty() {
                    quote!( empty().to(TokenKind::#kind) )
                } else {
                    quote!( just(#label).to(TokenKind::#kind) )
                }
            },
        );
        quote!( #lexer.map_with_span(|_, span: SpanType| lex::Node::chars(span)) )
    }
}

impl<'context> CombinatorNode<'context> {
    pub fn to_lexer_code(&self, code: &mut CodeGenerator) -> TokenStream {
        if !self.has_named_structure() {
            let scanner = self.to_scanner_code(code);
            return quote!(scan_make_node!(#scanner));
        }

        fn create_kind(name: &Option<String>, code: &mut CodeGenerator) -> Option<Ident> {
            name.as_ref().map(|name| code.add_token_kind(name.clone()))
        }

        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => {
                let production_name = format_ident!("{}", tree.production.name);
                match tree.production.kind {
                    ProductionKind::Rule | ProductionKind::Trivia => {
                        unreachable!("Token productions can only reference other token productions")
                    }
                    ProductionKind::Token => quote!(lex_rule!(#production_name)),
                }
            }

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_seq!( #kind, #(#elements),* ))
                } else {
                    quote!(lex_seq!( #(#elements),* ))
                }
            }

            Self::Choice { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_choice!( #kind, #(#elements),* ))
                } else {
                    quote!(lex_choice!( #(#elements),* ))
                }
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                let expr = expr.to_lexer_code(code);
                quote!(lex_optional!(#expr))
            }

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_zero_or_more!(#kind, #expr))
                } else {
                    quote!(lex_zero_or_more!(#expr))
                }
            }
            Self::OneOrMore { expr, name } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_one_or_more!(#kind, #expr))
                } else {
                    quote!(lex_one_or_more!(#expr))
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_repeated!(#kind, #expr, #min, #max))
                } else {
                    quote!(lex_repeated!(#expr, #min, #max))
                }
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let expr = expr.to_lexer_code(code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        lex_seq!(#kind, lex_terminal!(#open_kind, #open), #expr, lex_terminal!(#close_kind, #close))
                    )
                } else {
                    quote!(
                        lex_seq!(lex_terminal!(#open_kind, #open), #expr, lex_terminal!(#close_kind, #close))
                    )
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let expr = expr.to_lexer_code(code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        lex_separated_by!(#kind, #expr, lex_terminal!(#separator_kind, #separator))
                    )
                } else {
                    quote!(lex_separated_by!(#expr, lex_terminal!(#separator_kind, #separator)))
                }
            }

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceExpressionRule { .. } => unreachable!(),

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { filter, name } => filter.to_lexer_code(
                name.clone().expect("Lexer character filters must be named"),
                code,
            ),

            Self::TerminalTrie { trie, .. } => trie.to_lexer_code(code),

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_lexer_code(code);
                let subtrahend = subtrahend.to_lexer_code(code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_lexer_code(code);
                let lookahead = lookahead.to_lexer_code(code);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
        }
    }
}

pub fn parse_macros() -> TokenStream {
    quote!(
        #[allow(unused_macros)]
        macro_rules! lex_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    lex::Node::named(TokenKind::$kind, lex::Node::chars(span))
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    lex::Node::named(TokenKind::$kind, lex::Node::chars(span))
                })
            };
            ($literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| lex::Node::chars(span))
            };
            ($filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| lex::Node::chars(span))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_rule {
            ($rule:ident) => {
                $rule.clone()
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_choice {
            ($kind:ident, $($expr:expr),*) => {
                lex_choice!($($expr),*).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($($expr:expr),*) => {
                choice::<_, ErrorType>(($($expr),*))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_seq {
            // HELPERS -------------------------------------------------------------------------------

            /*
                (@exp a, b, c, d)
                => a.then(@exp b, c, d)
                => a.then(b.then(@exp c, d))
                => a.then(b.then(c.then(@exp d)))
                => a.then(b.then(c.then(d)))
            */

            (@exp $head:expr , $($tail:expr),+ ) => {
                $head.then(lex_seq!(@exp $($tail),+ ))
            };

            (@exp $head:expr ) => {
                $head
            };

            /*
                (@args [], v, a, b, c, d)
                => (@args [v.0,], v.1, b, c, d)
                => (@args [v.0, v.1.0,], v.1.1, c, d)
                => (@args [v.0, v.1.0, v.1.1.0,], v.1.1.1, d)
                => vec![v.0, v.1.0, v.1.1.0, v.1.1.0, v1.1.1, ]
            */

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr , $($tail:expr),+ ) => {
                lex_seq!(@args [ $($accum,)* $current.0, ] , $current.1 , $($tail),+ )
            };

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr ) => {
                vec![ $($accum,)* $current ]
            };

            //----------------------------------------------------------------------------------------

            ($kind:ident, $($expr:expr),+ ) => {
                lex_seq!(@exp $($expr),+ )
                    .map(|v| lex::Node::named(TokenKind::$kind, lex::Node::sequence(lex_seq!(@args [] , v , $($expr),+ ))))
            };

            ($($expr:expr),+ ) => {
                lex_seq!(@exp $($expr),+ )
                    .map(|v| lex::Node::sequence(lex_seq!(@args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($kind:ident, $expr:expr) => {
                lex_zero_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().map(lex::Node::sequence)
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($kind:ident, $expr:expr) => {
                lex_one_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().at_least(1).map(lex::Node::sequence)
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                lex_repeated!($expr, $min, $max)
                    .map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(lex::Node::sequence)
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_optional {
            ($expr:expr) => {
                $expr.or_not().map(|v| v.flatten())
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                lex_separated_by!($expr, $separator)
                    .map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        lex::Node::sequence(v)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! define_token {
            ($kind:ident, $expr:expr) => {
                $kind.define($expr.map(|node| lex::Node::named(TokenKind::$kind, node)));
                parsers.insert(
                    ProductionKind::$kind,
                    Parser::new(
                        $kind
                            .clone()
                            .map(cst::Node::top_level_token)
                            .then_ignore(end())
                            .boxed(),
                    ),
                );
            };
        }
    )
}
