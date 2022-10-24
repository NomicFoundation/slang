use codegen_schema::ProductionKind;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{
    character_filter::CharacterFilter, code_fragments::CodeFragments,
    combinator_node::CombinatorNode, naming, terminal_trie::TerminalTrie,
};

impl TerminalTrie {
    pub fn to_lexer_code(&self, code: &mut CodeFragments) -> TokenStream {
        self.to_code(code, "lex_")
    }
}

impl CharacterFilter {
    pub fn to_lexer_code(&self, name: Option<&String>, code: &mut CodeFragments) -> TokenStream {
        self.to_code(name, code, "lex_")
    }
}

impl CombinatorNode {
    pub fn to_lexer_code(&self, code: &mut CodeFragments) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeFragments) -> Option<Ident> {
            name.as_ref().map(|name| code.add_token_kind(name.clone()))
        }

        match self {
            // -----------------------------------------------------------------------------------------------
            // Simple References
            //
            Self::Reference { production } => {
                let production_parser_name = naming::to_parser_name_ident(&production.name);
                match production.kind {
                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule
                    | ProductionKind::Trivia => {
                        unreachable!("Token productions can only reference other token productions")
                    }
                    ProductionKind::Token => quote!(lex_rule!(#production_parser_name)),
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Sequence and Choice
            //
            Self::Sequence { elements, name } => {
                let expr = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_seq![ #kind, #(#expr),* ])
                } else {
                    quote!(lex_seq![ #(#expr),* ])
                }
            }

            Self::Choice { elements, name } => {
                let choices = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_choice!( #kind, #(#choices),* ))
                } else {
                    quote!(lex_choice!( #(#choices),* ))
                }
            }

            // -----------------------------------------------------------------------------------------------
            // Numeric Quantifiers
            //
            Self::Optional { expr, .. } => {
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

            // -----------------------------------------------------------------------------------------------
            // Stereotypes
            //
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

            // -----------------------------------------------------------------------------------------------
            // Expressions
            //
            Self::Expression { .. } => unreachable!("Expressions not allowed in tokens"),

            Self::ExpressionMember { .. } => {
                unreachable!("ExpressionMembers not allowed in tokens")
            }

            // -----------------------------------------------------------------------------------------------
            // Terminals and Utilities
            //
            Self::CharacterFilter { filter, name } => filter.to_lexer_code(name.as_ref(), code),

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
