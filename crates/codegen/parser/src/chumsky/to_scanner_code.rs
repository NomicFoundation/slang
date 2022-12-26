use proc_macro2::TokenStream;
use quote::quote;

use codegen_schema::*;

use super::{
    character_filter::CharacterFilter, code_generator::CodeGenerator,
    combinator_node::CombinatorNode, terminal_trie::TerminalTrie,
};

impl<'context> CharacterFilter<'context> {
    pub fn to_scanner_code(&self) -> TokenStream {
        if let CharacterFilter::Char { char } = self {
            quote!(scan_terminal!(#char))
        } else {
            let predicate = self.to_predicate(false);
            quote!(scan_terminal!(|&c: &char| #predicate))
        }
    }
}

impl TerminalTrie {
    pub fn to_scanner_code(&self) -> TokenStream {
        let scanners = self.generate(
            &|label, children| quote!(scan_trieprefix!(#label, [ #(#children),* ])),
            &|_, label| {
                if label.is_empty() {
                    quote!(scan_trieleaf!())
                } else {
                    quote!(scan_trieleaf!(#label))
                }
            },
        );
        quote!(scan_trie!(#(#scanners),*))
    }
}

impl<'context> CombinatorNode<'context> {
    pub fn to_scanner_code(&self, code: &mut CodeGenerator) -> TokenStream {
        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => match tree.production.kind {
                ProductionKind::Rule | ProductionKind::Trivia => {
                    unreachable!("Token productions can only reference other token productions")
                }
                ProductionKind::Token => tree.root_node.get().unwrap().to_scanner_code(code),
            },

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_scanner_code(code))
                    .collect::<Vec<_>>();
                quote!(scan_seq!( #(#elements),* ))
            }

            Self::Choice { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_scanner_code(code))
                    .collect::<Vec<_>>();
                quote!(scan_choice!( #(#elements),* ))
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_optional!(#expr))
            }

            Self::ZeroOrMore { expr, .. } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_zero_or_more!(#expr))
            }
            Self::OneOrMore { expr, .. } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_one_or_more!(#expr))
            }

            Self::Repeated { expr, min, max, .. } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_repeated!(#expr, #min, #max))
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open, expr, close, ..
            } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_seq!(scan_terminal!(#open), #expr, scan_terminal!(#close)))
            }

            Self::SeparatedBy {
                expr, separator, ..
            } => {
                let expr = expr.to_scanner_code(code);
                quote!(scan_separated_by!(#expr, scan_terminal!(#separator)))
            }

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceRule { .. } => unreachable!(),

            Self::PrecedenceRuleMember { .. } => unreachable!(),

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { filter, .. } => filter.to_scanner_code(),

            Self::TerminalTrie { trie, .. } => trie.to_scanner_code(),

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_scanner_code(code);
                let subtrahend = subtrahend.to_scanner_code(code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_scanner_code(code);
                let lookahead = lookahead.to_scanner_code(code);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
        }
    }
}

pub(crate) fn parse_macros() -> TokenStream {
    quote!(
        #[allow(unused_macros)]
        macro_rules! scan_terminal {
            ($literal:literal) => {
                just($literal).ignored()
            };
            ($filter:expr) => {
                filter($filter).ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_choice {
            ($($expr:expr),*) => {
                choice::<_, ErrorType>(($($expr),*))
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_seq {
            ( $head:expr , $($tail:expr),+ ) => {
                $head.then_ignore(scan_seq!($($tail),+ ))
            };

            ( $head:expr ) => {
                $head.ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_zero_or_more {
            ($expr:expr) => {
                $expr.repeated().ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_one_or_more {
            ($expr:expr) => {
                $expr.repeated().at_least(1).ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_repeated {
            ($expr:expr, $min:literal, $max:literal) => {
                $expr.repeated().at_least($min).at_most($max).ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_optional {
            ($expr:expr) => {
                $expr.or_not().ignored()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_separated_by {
            ($expr:expr, $separator:expr) => {
                $expr.then_ignore($separator.then_ignore($expr).repeated())
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_trie {
            ($expr:expr) => {
                $expr
            };
            ($($expr:expr),+ ) => {
                choice::<_, ErrorType>(($($expr),+))
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_trieleaf {
            ( $string:literal ) => {
                just($string).ignored()
            };
            () => {
                empty()
            };
        }

        #[allow(unused_macros)]
        macro_rules! scan_trieprefix {
            ($string:literal , [ $($expr:expr),+ ] ) => (
                just($string).ignore_then(choice::<_, ErrorType>(($($expr),+)))
            )
        }

        #[allow(unused_macros)]
        macro_rules! scan_make_node {
            ($expr:expr) => {
                $expr.map_with_span(|_, span: SpanType| lex::Node::chars(span))
            };
        }
    )
}
