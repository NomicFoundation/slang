use std::cell::Cell;

use convert_case::{Case, Casing};
use inflector::Inflector;
// use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::*;

use super::{character_filter::CharacterFilter, terminal_trie::TerminalTrie};

pub struct CombinatorTree {
    pub root: CombinatorTreeNode,
    pub module_name: String,
}

type CombinatorTreeNode = Box<CombinatorTreeNodeData>;

pub enum CombinatorTreeNodeData {
    Difference {
        minuend: CombinatorTreeNode,
        subtrahend: CombinatorTreeNode,
    },
    Lookahead {
        expr: CombinatorTreeNode,
        lookahead: CombinatorTreeNode,
    },
    Choice {
        name: String,
        choices: Vec<(String, CombinatorTreeNode)>,
    },
    Sequence {
        name: String,
        elements: Vec<(String, CombinatorTreeNode)>,
    },
    Repeat {
        name: String,
        expr: CombinatorTreeNode,
        min: usize,
        max: Option<usize>,
        separator: Option<CombinatorTreeNode>,
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

impl CombinatorTree {
    pub fn to_parser_combinator_code(&self) -> TokenStream {
        self.root.to_parser_combinator_code(self)
    }
}

impl CombinatorTreeNodeData {
    pub fn name(&self) -> Option<String> {
        match self {
            CombinatorTreeNodeData::Difference { minuend, .. } => minuend.name(),
            CombinatorTreeNodeData::Lookahead { expr, .. } => expr.name(),
            CombinatorTreeNodeData::Choice { name, .. } => Some(name.clone()),
            CombinatorTreeNodeData::Sequence { name, .. } => Some(name.clone()),
            CombinatorTreeNodeData::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => expr.name(),
            CombinatorTreeNodeData::Repeat { expr, .. } => {
                expr.name().and_then(|n| {
                    if n.starts_with('_') {
                        Some(n.to_ascii_lowercase().to_plural())
                    } else {
                        // Convert to snake and then pluralize to avoid e.g. BARs -> ba_rs later on
                        Some(n.to_case(Case::Snake).to_plural())
                    }
                })
            }
            CombinatorTreeNodeData::Reference { name } => Some(name.clone()),
            CombinatorTreeNodeData::TerminalTrie { .. } => None,
            CombinatorTreeNodeData::CharacterFilter { filter } => {
                filter.name().map(|n| format!("{}_char", n))
            }
            CombinatorTreeNodeData::End => None,
        }
    }

    fn to_parser_combinator_code(&self, tree: &CombinatorTree) -> TokenStream {
        match self {
            CombinatorTreeNodeData::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_parser_combinator_code(tree);
                let subtrahend = subtrahend.to_parser_combinator_code(tree);
                quote! ( #minuend.excluding(#subtrahend) )
            }
            CombinatorTreeNodeData::Lookahead { expr, lookahead } => {
                let expr = expr.to_parser_combinator_code(tree);
                let lookahead = lookahead.to_parser_combinator_code(tree);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
            CombinatorTreeNodeData::Choice { choices, name } => {
                let module_name = format_ident!("{}", tree.module_name.clone());
                let choice_name = format_ident!("{}", name);
                let choices = choices.iter().map(|(n, c)| {
                    let constructor = format_ident!("{}", n);
                    let expr = c.to_parser_combinator_code(tree);
                    quote!( #expr.map(#module_name::#choice_name::#constructor) )
                });
                quote!( choice(( #(#choices),* )) )
            }
            CombinatorTreeNodeData::Sequence { elements, name } => {
                let struct_name = format_ident!("{}", name);
                let mut elements = elements
                    .iter()
                    .map(|(_, e)| e.to_parser_combinator_code(tree));
                let first = elements.next().unwrap();
                let rest = elements.map(|e| quote!( .then(#e) ));
                let module_name = format_ident!("{}", tree.module_name.clone());
                quote!( #first #(#rest)* .map(#module_name::#struct_name::new) )
            }
            CombinatorTreeNodeData::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => {
                let expr = expr.to_parser_combinator_code(tree);
                quote!( #expr.or_not() )
            }
            CombinatorTreeNodeData::Repeat {
                expr,
                min,
                max,
                separator: None,
                ..
            } => {
                let expr = expr.to_parser_combinator_code(tree);

                match (min, max) {
                    (0, None) => quote!( #expr.repeated() ),
                    (0, Some(max)) => quote!( #expr.repeated().at_most(#max) ),
                    (min, None) => quote!( #expr.repeated().at_least(#min) ),
                    (min, Some(max)) if min == max => quote!( #expr.repeated().exactly(#min) ),
                    (min, Some(max)) => quote!( #expr.repeated().at_least(#min).at_most(#max) ),
                }
            }
            CombinatorTreeNodeData::Repeat {
                name,
                expr,
                min,
                max,
                separator: Some(separator),
            } => {
                let expr = expr.to_parser_combinator_code(tree);
                let separator = separator.to_parser_combinator_code(tree);

                let mapping = {
                    let module_name = format_ident!("{}", tree.module_name.clone());
                    let struct_name = format_ident!("{}", name);
                    quote!( .map(repetition_mapper).map( #module_name::#struct_name::new) )
                };

                let repetition = quote!(#separator.then(#expr).repeated());

                match (min, max) {
                    (0, None) => {
                        quote!( #expr.then()#mapping.or_not() )
                    }
                    (0, Some(max)) => {
                        quote!( #expr.then(#repetition.at_most(#max - 1))#mapping.or_not() )
                    }
                    (1, None) => {
                        quote!( #expr.then(#repetition)#mapping )
                    }
                    (1, Some(max)) => {
                        quote!( #expr.then(#repetition.at_most(#max - 1))#mapping )
                    }
                    (min, None) => {
                        quote!( #expr.then(#repetition.at_least(#min - 1))#mapping )
                    }
                    (min, Some(max)) if min == max => {
                        quote!( #expr.then(#repetition.exactly(#min - 1))#mapping )
                    }
                    (min, Some(max)) => {
                        quote!( #expr.then(#repetition.at_least(#min - 1).at_most(#max - 1))#mapping )
                    }
                }
            }
            CombinatorTreeNodeData::Reference { name } => {
                let name = format_ident!("{}_parser", name.to_case(Case::Snake));
                quote!( #name.clone() )
            }
            CombinatorTreeNodeData::TerminalTrie { trie } => trie.to_parser_expression(),
            CombinatorTreeNodeData::CharacterFilter { filter } => filter.to_parser_expression(),
            CombinatorTreeNodeData::End => quote!(end()),
        }
    }
}

pub fn ct_difference(
    minuend: CombinatorTreeNode,
    subtrahend: CombinatorTreeNode,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Difference {
        minuend,
        subtrahend,
    })
}

#[allow(dead_code)]
fn ct_lookahead(expr: CombinatorTreeNode, lookahead: CombinatorTreeNode) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Lookahead { expr, lookahead })
}

fn ct_choice(name: String, choices: Vec<(String, CombinatorTreeNode)>) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Choice { name, choices })
}

fn ct_sequence(name: String, elements: Vec<(String, CombinatorTreeNode)>) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Sequence { name, elements })
}

fn ct_repeat(
    name: String,
    expr: CombinatorTreeNode,
    min: usize,
    max: Option<usize>,
    separator: Option<CombinatorTreeNode>,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Repeat {
        name,
        expr,
        min,
        max,
        separator,
    })
}

fn ct_reference(name: String) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Reference { name })
}

fn ct_terminal_trie(trie: TerminalTrie) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::TerminalTrie { trie })
}

fn ct_character_filter(filter: CharacterFilter) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::CharacterFilter { filter })
}

fn ct_end() -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::End)
}

impl Production {
    pub fn to_combinator_tree(&self, grammar: &Grammar) -> CombinatorTree {
        CombinatorTree {
            root: self
                .expression_to_generate()
                .to_combinator_tree_node(&mut Cell::new(0), grammar),
            module_name: self.name.to_case(Case::Snake),
        }
    }
}

impl Expression {
    fn to_combinator_tree_node(
        &self,
        subtype_index: &mut Cell<usize>,
        grammar: &Grammar,
    ) -> CombinatorTreeNode {
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
                    minuend.to_combinator_tree_node(subtype_index, grammar),
                    subtrahend.to_combinator_tree_node(subtype_index, grammar),
                ),
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max,
                    separator,
                }) => {
                    let index = subtype_index.get();
                    subtype_index.set(index + 1);
                    let name = format!("_S{}", index);
                    ct_repeat(
                        name,
                        expr.to_combinator_tree_node(subtype_index, grammar),
                        *min,
                        *max,
                        separator
                            .clone()
                            .map(|s| s.to_combinator_tree_node(subtype_index, grammar)),
                    )
                }
                EBNF::Choice(exprs) => {
                    let index = subtype_index.get();
                    subtype_index.set(index + 1);
                    let name = format!("_C{}", index);
                    ct_choice(
                        name,
                        exprs
                            .iter()
                            .enumerate()
                            .map(|(i, e)| {
                                let e = e.to_combinator_tree_node(subtype_index, grammar);
                                let name = e.name().map_or_else(
                                    || format!("_{}", i),
                                    |n| n.to_case(Case::UpperCamel),
                                );
                                (name, e)
                            })
                            .collect(),
                    )

                    // TODO: check for duplicated names

                    // let mut pairs = elements
                    //     .iter()
                    //     .enumerate()
                    //     .map(|(i, c)| {
                    //         let ttn = c.to_type_tree_node();
                    //         (
                    //             ttn.name()
                    //                 .map_or_else(|| format!("_{}", i), |n| n.to_case(Case::Snake)),
                    //             ttn,
                    //         )
                    //     })
                    //     .collect::<Vec<_>>();
                    // // Find all the duplicated names, with the count of their occurance
                    // let mut names = MultiSet::<String>::from_iter(pairs.iter().map(|(n, _)| n.clone()));
                    // names.retain(|_, count| count > 1);
                    // // Reverse so that the suffix goes from _0 .. _n when we re-reverse the list
                    // pairs.reverse();
                    // let mut pairs: Vec<_> = pairs
                    //     .into_iter()
                    //     .map(|(n, t)| {
                    //         if let Some(count) = names.get(&n) {
                    //             // Remove the element to decrement the occurance occount
                    //             names.remove(&n);
                    //             (format!("{}_{}", n, count - 1), t)
                    //         } else {
                    //             (n, t)
                    //         }
                    //     })
                    //     .collect();
                    // pairs.reverse();
                    // pairs
                }
                EBNF::Sequence(exprs) => {
                    let index = subtype_index.get();
                    subtype_index.set(index + 1);
                    let name = format!("_S{}", index);
                    ct_sequence(
                        name,
                        exprs
                            .iter()
                            .enumerate()
                            .map(|(i, e)| {
                                let e = e.to_combinator_tree_node(subtype_index, grammar);
                                let name = e.name().map_or_else(
                                    || format!("_{}", i),
                                    |n| {
                                        if n.starts_with('_') {
                                            n.to_ascii_lowercase()
                                        } else {
                                            n.to_case(Case::Snake)
                                        }
                                    },
                                );
                                (name, e)
                            })
                            .collect(),
                    )
                }
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
