use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::slang_name::SlangName;

#[derive(Clone, Debug)]
pub enum CharacterFilterNode {
    Negation { child: CharacterFilter },
    Range { from: char, to: char },
    Char { char: char },
    Disjunction { nodes: Vec<CharacterFilter> },
    Conjunction { nodes: Vec<CharacterFilter> },
}

pub type CharacterFilter = Box<CharacterFilterNode>;

fn cf_negation(child: CharacterFilter) -> CharacterFilter {
    if let CharacterFilterNode::Negation { child } = *child {
        child
    } else {
        Box::new(CharacterFilterNode::Negation { child })
    }
}

fn cf_range(from: char, to: char) -> CharacterFilter {
    Box::new(CharacterFilterNode::Range { from, to })
}

fn cf_char(char: char) -> CharacterFilter {
    Box::new(CharacterFilterNode::Char { char })
}
fn cf_disjunction(nodes: Vec<CharacterFilter>) -> CharacterFilter {
    Box::new(CharacterFilterNode::Disjunction { nodes })
}

fn cf_conjunction(nodes: Vec<CharacterFilter>) -> CharacterFilter {
    Box::new(CharacterFilterNode::Conjunction { nodes })
}

impl CharacterFilterNode {
    pub fn slang_name(&self) -> Option<SlangName> {
        if let Self::Negation { child } = self {
            child
                .slang_name()
                .map(|s| SlangName::from_string(&format!("not_{}", s.as_str())))
        } else if let Self::Char { char, .. } = self {
            SlangName::from_terminal_char(*char)
        } else {
            None
        }
    }

    pub fn to_parser_combinator_code(&self) -> TokenStream {
        let map = quote!(.map(|_| FixedTerminal::<1>()) );
        if let CharacterFilterNode::Char { char } = self {
            quote!(just(#char)#map )
        } else {
            let predicate = self.to_parser_predicate(false);
            quote!( filter(|&c: &char| #predicate)#map )
        }
    }

    fn to_parser_predicate(&self, negated: bool) -> TokenStream {
        match self {
            CharacterFilterNode::Negation { child } => child.to_parser_predicate(!negated),
            CharacterFilterNode::Range { from, to } => {
                if negated {
                    quote!( (c < #from || #to < c) )
                } else {
                    quote!( (#from <= c && c <= #to) )
                }
            }
            CharacterFilterNode::Char { char } => {
                if negated {
                    quote!( c != #char )
                } else {
                    quote!( c == #char )
                }
            }
            CharacterFilterNode::Disjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate(negated));
                if negated {
                    quote! ( #(#nodes)&&* )
                } else {
                    quote! ( #(#nodes)||* )
                }
            }
            CharacterFilterNode::Conjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate(negated));
                if negated {
                    quote! ( #(#nodes)||* )
                } else {
                    quote! ( #(#nodes)&&* )
                }
            }
        }
    }
}

impl Expression {
    pub fn to_character_filter(&self, grammar: &Grammar) -> Option<CharacterFilter> {
        match &self.ebnf {
            EBNF::Not(child) => child.to_character_filter(grammar).map(cf_negation),
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
                    Some(cf_char(string.chars().next().unwrap()))
                } else {
                    None
                }
            }
            EBNF::Range(EBNFRange { from, to }) => Some(cf_range(*from, *to)),
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
                    Some(cf_conjunction(vec![minuend, cf_negation(subtrahend)]))
                } else {
                    None
                }
            }
            EBNF::Sequence(_) | EBNF::End | EBNF::Repeat(_) => None,
        }
    }
}
