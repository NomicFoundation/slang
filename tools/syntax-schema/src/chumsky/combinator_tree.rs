use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::*;

use super::{character_filter::CharacterFilter, terminal_trie::TerminalTrie};

pub enum CombinatorTreeNode {
    Difference {
        minuend: CombinatorTree,
        subtrahend: CombinatorTree,
    },
    Lookahead {
        expr: CombinatorTree,
        lookahead: CombinatorTree,
    },
    Choice {
        name: String,
        choices: Vec<CombinatorTree>,
    },
    Sequence {
        elements: Vec<CombinatorTree>,
    },
    Repeat {
        expr: CombinatorTree,
        min: usize,
        max: Option<usize>,
        separator: Option<CombinatorTree>,
    },
    Reference {
        name: String,
    },
    TerminalTrie {
        trie: TerminalTrie,
    },
    CharacterFilter {
        filter: CharacterFilter,
    },
    End,
}

type CombinatorTree = Box<CombinatorTreeNode>;

pub fn ct_difference(minuend: CombinatorTree, subtrahend: CombinatorTree) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Difference {
        minuend,
        subtrahend,
    })
}

#[allow(dead_code)]
fn ct_lookahead(expr: CombinatorTree, lookahead: CombinatorTree) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Lookahead { expr, lookahead })
}

fn ct_choice(name: String, choices: Vec<CombinatorTree>) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Choice { name, choices })
}

fn ct_sequence(elements: Vec<CombinatorTree>) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Sequence { elements })
}

fn ct_repeat(
    expr: CombinatorTree,
    min: usize,
    max: Option<usize>,
    separator: Option<CombinatorTree>,
) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Repeat {
        expr,
        min,
        max,
        separator,
    })
}

fn ct_reference(name: String) -> CombinatorTree {
    Box::new(CombinatorTreeNode::Reference { name })
}

fn ct_terminal_trie(trie: TerminalTrie) -> CombinatorTree {
    Box::new(CombinatorTreeNode::TerminalTrie { trie })
}

fn ct_character_filter(filter: CharacterFilter) -> CombinatorTree {
    Box::new(CombinatorTreeNode::CharacterFilter { filter })
}

fn ct_end() -> CombinatorTree {
    Box::new(CombinatorTreeNode::End)
}

impl CombinatorTreeNode {
    pub fn to_parser_expression(&self) -> TokenStream {
        match self {
            CombinatorTreeNode::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_parser_expression();
                let subtrahend = subtrahend.to_parser_expression();
                quote! ( #minuend.excluding(#subtrahend) )
            }
            CombinatorTreeNode::Lookahead { expr, lookahead } => {
                let expr = expr.to_parser_expression();
                let lookahead = lookahead.to_parser_expression();
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
            CombinatorTreeNode::Choice { choices, .. } => {
                let choices = choices.iter().map(|c| c.to_parser_expression());
                quote!( choice(( #(#choices),* )) )
            }
            CombinatorTreeNode::Sequence { elements } => {
                let mut elements = elements.iter().map(|e| e.to_parser_expression());
                let first = elements.next().unwrap();
                let rest = elements.map(|e| quote!( .then(#e) ));
                quote!( #first #(#rest)* )
            }
            CombinatorTreeNode::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => {
                let expr = expr.to_parser_expression();
                quote!( #expr.or_not() )
            }
            CombinatorTreeNode::Repeat {
                expr,
                min,
                max,
                separator: None,
            } => {
                let expr = expr.to_parser_expression();
                match (min, max) {
                    (0, None) => quote!( #expr.repeated() ),
                    (0, Some(max)) => quote!( #expr.repeated().at_most(#max) ),
                    (min, None) => quote!( #expr.repeated().at_least(#min) ),
                    (min, Some(max)) if min == max => quote!( #expr.repeated().exactly(#min) ),
                    (min, Some(max)) => quote!( #expr.repeated().at_least(#min).at_most(#max) ),
                }
            }
            CombinatorTreeNode::Repeat {
                expr,
                min,
                max,
                separator: Some(separator),
            } => {
                let expr = expr.to_parser_expression();
                let separator = separator.to_parser_expression();
                match (min, max) {
                    (0, None) => quote!( #expr.then(#separator.then(#expr).repeated()).or_not() ),
                    (0, Some(max)) => {
                        quote!( #expr.then(#separator.then(#expr).repeated().at_most(#max - 1)).or_not() )
                    }
                    (min, None) => {
                        quote!( #expr.then(#separator.then(#expr).repeated().at_least(#min - 1)) )
                    }
                    (min, Some(max)) if min == max => {
                        quote!( #expr.then(#separator.then(#expr).repeated().exactly(#min - 1)) )
                    }
                    (min, Some(max)) => {
                        quote!( #expr.then(#separator.then(#expr).repeated().at_least(#min - 1).at_most(#max - 1)) )
                    }
                }
            }
            CombinatorTreeNode::Reference { name } => {
                let name = format_ident!("{}", name);
                quote!( #name )
            }
            CombinatorTreeNode::TerminalTrie { trie } => trie.to_parser_expression(),
            CombinatorTreeNode::CharacterFilter { filter } => filter.to_parser_expression(),
            CombinatorTreeNode::End => quote!(end()),
        }
    }

    pub fn to_type_expression(&self) -> TokenStream {
        match self {
            CombinatorTreeNode::Difference { minuend, .. } => minuend.to_type_expression(),
            CombinatorTreeNode::Lookahead { expr, .. } => expr.to_type_expression(),
            CombinatorTreeNode::Choice { name, .. } => {
                let name = format_ident!("{}", name);
                quote!(#name)
            }
            CombinatorTreeNode::Sequence { elements } => {
                let elements = elements.iter().map(|e| e.to_type_expression());
                quote!( (#(#elements),*) )
            }
            CombinatorTreeNode::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => {
                let expr = expr.to_type_expression();
                quote!( Option<#expr> )
            }
            CombinatorTreeNode::Repeat {
                expr,
                separator: None,
                ..
            } => {
                let expr = expr.to_type_expression();
                quote!( Vec<#expr> )
            }
            CombinatorTreeNode::Repeat {
                expr,
                min: 0,
                separator: Some(separator),
                ..
            } => {
                let expr = expr.to_type_expression();
                let separator = separator.to_type_expression();
                quote!( Option<(#expr, Vec<(#separator, #expr))> )
            }
            CombinatorTreeNode::Repeat {
                expr,
                separator: Some(separator),
                ..
            } => {
                let expr = expr.to_type_expression();
                let separator = separator.to_type_expression();
                quote!( (#expr, Vec<(#separator, #expr)) )
            }
            CombinatorTreeNode::Reference { name } => {
                let name = format_ident!("{}", name.to_case(Case::UpperCamel));
                quote!(#name)
            }
            CombinatorTreeNode::TerminalTrie { .. } => quote!(CN),
            CombinatorTreeNode::CharacterFilter { .. } => quote!(C),
            CombinatorTreeNode::End => quote!(()),
        }
    }
}

impl Expression {
    pub fn to_combinator_tree(&self, production: &Production, grammar: &Grammar) -> CombinatorTree {
        if let Some(filter) = self.to_character_filter(grammar) {
            return ct_character_filter(filter);
        } else if let Some(terminal_trie) = self.to_terminal_trie(grammar) {
            return ct_terminal_trie(terminal_trie);
        } else {
            match &self.ebnf {
                EBNF::End => ct_end(),
                EBNF::Difference(EBNFDifference {
                    minuend,
                    subtrahend,
                }) => ct_difference(
                    minuend.to_combinator_tree(production, grammar),
                    subtrahend.to_combinator_tree(production, grammar),
                ),
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max,
                    separator,
                }) => ct_repeat(
                    expr.to_combinator_tree(production, grammar),
                    *min,
                    *max,
                    separator
                        .clone()
                        .map(|s| s.to_combinator_tree(production, grammar)),
                ),
                EBNF::Choice(exprs) => {
                    // merge TerminalTrie ?
                    // merge CharacterFilter ?
                    let mut s = DefaultHasher::new();
                    exprs.hash(&mut s);
                    let name = format!("Choice{}", s.finish());
                    ct_choice(
                        name,
                        exprs
                            .iter()
                            .map(|e| e.to_combinator_tree(production, grammar))
                            .collect(),
                    )
                }
                EBNF::Sequence(exprs) => ct_sequence(
                    exprs
                        .iter()
                        .map(|e| e.to_combinator_tree(production, grammar))
                        .collect(),
                ),
                EBNF::Reference(name) => ct_reference(name.clone()),
                EBNF::Not(_) => unimplemented!("¬ is only supported on characters or sets thereof"),
                EBNF::Terminal(_) => {
                    unreachable!("Terminals are either character filters or terminal tries")
                }
                EBNF::Range(_) => unreachable!("Ranges are always character filters"),
            }
        }
    }
}
