use std::collections::BTreeSet;

use codegen_schema::types::production::Production;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    char_set::CharSet, code_generator::CodeGenerator, combinator_node::CombinatorNode,
    trie::TerminalTrie,
};

impl<'context> CombinatorNode<'context> {
    pub fn to_scanner_code(&self, code: &mut CodeGenerator) -> TokenStream {
        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => match tree.production.as_ref() {
                Production::Scanner { name, .. } => {
                    let scanner_function_name = format_ident!("scan_{}", name.to_snake_case());
                    quote! { self.#scanner_function_name(stream) }
                }
                Production::TriviaParser { .. }
                | Production::Parser { .. }
                | Production::PrecedenceParser { .. } => {
                    unreachable!("Token productions can only reference other token productions")
                }
            },

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_scanner_code(code))
                    .collect::<Vec<_>>();
                quote! { scan_sequence!(#(#elements),*) }
            }

            Self::Choice { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_scanner_code(code))
                    .collect::<Vec<_>>();
                quote! { scan_choice!(stream, #(#elements),*) }
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                let expr = expr.to_scanner_code(code);
                quote! { scan_optional!(stream, #expr) }
            }

            Self::ZeroOrMore { expr, .. } => {
                let expr = expr.to_scanner_code(code);
                quote! { scan_zero_or_more!(stream, #expr) }
            }
            Self::OneOrMore { expr, .. } => {
                let expr = expr.to_scanner_code(code);
                quote! { scan_one_or_more!(stream, #expr) }
            }

            Self::Repeated { expr, min, max, .. } => {
                let expr = expr.to_scanner_code(code);
                quote! { scan_repeated!(stream, #expr, #min, #max) }
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open, expr, close, ..
            } => {
                let open = open.chars().collect::<Vec<_>>();
                let expr = expr.to_scanner_code(code);
                let close = close.chars().collect::<Vec<_>>();
                quote! { scan_delimited_by!(stream, [#(#open),*], #expr, [#(#close),*]) }
            }

            Self::SeparatedBy {
                expr, separator, ..
            } => {
                let expr = expr.to_scanner_code(code);
                let separator = separator.chars();
                quote! { scan_separated_by!(stream, #expr, [#(#separator),*]) }
            }

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceExpressionRule { .. } => unreachable!(),

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { filter, .. } => filter.to_scanner_code(),

            Self::TerminalTrie { trie } => trie.to_scanner_code(),

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_scanner_code(code);
                let subtrahend = subtrahend.to_scanner_code(code);
                quote! { scan_difference!(stream, #minuend, #subtrahend) }
            }

            Self::Lookahead { .. } => todo!(),
        }
    }
}

impl CharSet {
    pub fn to_scanner_code(&self) -> TokenStream {
        if let Some(char) = self.single_char() {
            quote! { scan_chars!(stream, #char) }
        } else {
            let predicate = self.to_predicate();
            quote! { scan_predicate!(stream, |c| #predicate) }
        }
    }
}

impl TerminalTrie {
    pub fn to_scanner_code(&self) -> TokenStream {
        let (path, trie) = self.next_interesting_node(None);

        let terminals = trie
            .subtries
            .iter()
            .filter_map(|(c, t)| {
                if t.subtries.is_empty() {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();

        let subtries = trie
            .subtries
            .iter()
            .filter_map(|(c, subtrie)| {
                if terminals.contains(c) {
                    None
                } else {
                    let child_code = subtrie.to_scanner_code();
                    Some(quote! { #c + #child_code })
                }
            })
            .collect::<Vec<_>>();

        let chars = path.iter();
        let prefix = quote! { scan_chars!(stream, #(#chars),*) };
        if terminals.is_empty() && subtries.is_empty() {
            prefix
        } else {
            let empty = trie.payload.clone().map(|_| quote! { ,EMPTY });
            // just to make the code more aesthetically pleasing
            let trie = if terminals.is_empty() {
                quote! { scan_trie!(stream #empty #(,#subtries)*) }
            } else {
                quote! { scan_trie!(stream #empty ,[#(#terminals)|*] #(,#subtries)*) }
            };
            if path.is_empty() {
                trie
            } else {
                quote! { scan_sequence!(#prefix, #trie) }
            }
        }
    }
}
