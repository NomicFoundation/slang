use proc_macro2::{Ident, TokenStream};
use quote::quote;

use codegen_schema::*;

use super::{
    character_filter::CharacterFilter, code_generator::CodeGenerator,
    combinator_node::CombinatorNode, naming, terminal_trie::TerminalTrie,
};

impl<'context> CharacterFilter<'context> {
    pub fn to_lexer_code(&self, name: Option<&String>, code: &mut CodeGenerator) -> TokenStream {
        self.to_code(name, code, "lex_")
    }
}

impl TerminalTrie {
    pub fn to_lexer_code(&self, kind: Option<Ident>, code: &mut CodeGenerator) -> TokenStream {
        self.to_code(kind, code, "lex_")
    }
}

impl<'context> CombinatorNode<'context> {
    pub fn to_lexer_code(&self, code: &mut CodeGenerator) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeGenerator) -> Option<Ident> {
            name.as_ref().map(|name| code.add_token_kind(name.clone()))
        }

        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => {
                let production_parser_name = naming::to_parser_name_ident(&tree.production.name);
                match tree.production.kind {
                    ProductionKind::Rule | ProductionKind::Trivia => {
                        unreachable!("Token productions can only reference other token productions")
                    }
                    ProductionKind::Token => quote!(lex_rule!(#production_parser_name)),
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
            Self::PrecedenceRule { .. } => unreachable!(),

            Self::PrecedenceRuleMember { .. } => unreachable!(),

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { filter, name } => filter.to_lexer_code(name.as_ref(), code),

            Self::TerminalTrie { trie, name } => {
                trie.to_lexer_code(name.clone().map(|n| code.add_token_kind(n)), code)
            }

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
