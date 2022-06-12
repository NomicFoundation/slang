use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::slang_name::SlangName;

#[derive(Clone, Debug)]
pub enum CharacterFilterNode {
    Range { from: char, to: char, negated: bool },
    Char { char: char, negated: bool },
    Disjunction { nodes: Vec<CharacterFilter> },
    Conjunction { nodes: Vec<CharacterFilter> },
}

impl CharacterFilterNode {
    pub fn negated(self) -> CharacterFilter {
        match self {
            Self::Range { from, to, negated } => cf_range(from, to, !negated),
            Self::Char { char, negated } => cf_char(char, !negated),
            Self::Disjunction { nodes } => {
                cf_conjunction(nodes.into_iter().map(|n| n.negated()).collect())
            }
            Self::Conjunction { nodes } => {
                cf_disjunction(nodes.into_iter().map(|n| n.negated()).collect())
            }
        }
    }
}

pub type CharacterFilter = Box<CharacterFilterNode>;

fn cf_range(from: char, to: char, negated: bool) -> CharacterFilter {
    Box::new(CharacterFilterNode::Range { from, to, negated })
}

fn cf_char(char: char, negated: bool) -> CharacterFilter {
    Box::new(CharacterFilterNode::Char { char, negated })
}
fn cf_disjunction(nodes: Vec<CharacterFilter>) -> CharacterFilter {
    Box::new(CharacterFilterNode::Disjunction { nodes })
}

fn cf_conjunction(nodes: Vec<CharacterFilter>) -> CharacterFilter {
    Box::new(CharacterFilterNode::Conjunction { nodes })
}

impl CharacterFilterNode {
    pub fn slang_name(&self) -> Option<SlangName> {
        if let Self::Char { char, .. } = self {
            SlangName::from_terminal_char(*char)
        } else {
            None
        }
    }

    pub fn to_parser_combinator_code(&self) -> TokenStream {
        if let CharacterFilterNode::Char {
            char,
            negated: false,
        } = self
        {
            quote!(just(#char).ignored() )
        } else {
            let predicate = self.to_parser_predicate();
            quote!( filter(|&c: &char| #predicate).ignored() )
        }
    }

    fn to_parser_predicate(&self) -> TokenStream {
        match self {
            CharacterFilterNode::Range {
                from,
                to,
                negated: false,
            } => quote!( (#from <= c && c <= #to) ),
            CharacterFilterNode::Range {
                from,
                to,
                negated: true,
            } => quote!( (c < #from || #to < c) ),
            CharacterFilterNode::Char {
                char,
                negated: false,
            } => quote!( c == #char ),
            CharacterFilterNode::Char {
                char,
                negated: true,
            } => quote!( c != #char ),
            CharacterFilterNode::Disjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate());
                quote! ( #(#nodes)||* )
            }
            CharacterFilterNode::Conjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate());
                quote! ( #(#nodes)&&* )
            }
        }
    }
}

impl Expression {
    pub fn to_character_filter(&self, grammar: &Grammar) -> Option<CharacterFilter> {
        match &self.ebnf {
            EBNF::Not(child) => child.to_character_filter(grammar).map(|c| c.negated()),
            EBNF::Choice(children) => {
                let child_filters = children
                    .iter()
                    .filter_map(|c| c.to_character_filter(grammar))
                    .collect::<Vec<_>>();
                if child_filters.len() == children.len() {
                    Some(cf_disjunction(child_filters))
                } else {
                    None
                }
            }
            EBNF::Terminal(string) => {
                if string.chars().count() == 1 {
                    Some(cf_char(string.chars().next().unwrap(), false))
                } else {
                    None
                }
            }
            EBNF::Range(EBNFRange { from, to }) => Some(cf_range(*from, *to, false)),
            EBNF::Reference(name) => {
                if let Some(production) = grammar.get_production(name) {
                    production
                        .expression_to_generate()
                        .to_character_filter(grammar)
                } else {
                    None
                }
            }
            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                if let (Some(minuend), Some(subtrahend)) = (
                    minuend.to_character_filter(grammar),
                    subtrahend.to_character_filter(grammar),
                ) {
                    Some(cf_conjunction(vec![minuend, subtrahend.negated()]))
                } else {
                    None
                }
            }
            EBNF::Sequence(_) | EBNF::End | EBNF::Repeat(_) => None,
        }
    }
}
