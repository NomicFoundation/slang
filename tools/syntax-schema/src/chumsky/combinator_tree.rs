use std::cell::Cell;

use convert_case::{Case, Casing};
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
        choices: Vec<CombinatorTreeNode>,
    },
    Sequence {
        elements: Vec<CombinatorTreeNode>,
    },
    Repeat {
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
                let choice_name = format_ident!("{}", name);

                let choices = choices.iter().enumerate().map(|(i, c)| {
                    let member = format_ident!("_{}", i);
                    let expr = c.to_parser_combinator_code(tree);
                    let module_name = format_ident!("{}", tree.module_name.clone());
                    quote!( #expr.map(#module_name::#choice_name::#member) )
                });

                quote!( choice(( #(#choices),* )) )
            }
            CombinatorTreeNodeData::Sequence { elements } => {
                let mut elements = elements.iter().map(|e| e.to_parser_combinator_code(tree));

                let flattener = if elements.len() > 2 {
                    let nested_fields = (1..elements.len()).map(|i| format_ident!("_{}", i)).fold(
                        {
                            let ident = format_ident!("_0");
                            quote!( #ident )
                        },
                        |accum, ident| quote!( (#accum, #ident) ),
                    );
                    let flattened_fields = (0..elements.len()).map(|i| format_ident!("_{}", i));

                    quote!(.map(|#nested_fields| (#(#flattened_fields),*)) )
                } else {
                    quote!()
                };

                let first = elements.next().unwrap();
                let rest = elements.map(|e| quote!( .then(#e) ));

                quote!( #first #(#rest)* #flattener )
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
                expr,
                min,
                max,
                separator: Some(separator),
            } => {
                let expr = expr.to_parser_combinator_code(tree);
                let separator = separator.to_parser_combinator_code(tree);

                match (min, max) {
                    (0, None) => quote!( #expr.then(#separator.then(#expr).repeated()).or_not() ),
                    (0, Some(max)) => {
                        quote!( #expr.then(#separator.then(#expr).repeated().at_most(#max - 1)).or_not() )
                    }
                    (1, None) => {
                        quote!( #expr.then(#separator.then(#expr).repeated()) )
                    }
                    (1, Some(max)) => {
                        quote!( #expr.then(#separator.then(#expr).repeated().at_most(#max - 1)) )
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

fn ct_choice(name: String, choices: Vec<CombinatorTreeNode>) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Choice { name, choices })
}

fn ct_sequence(elements: Vec<CombinatorTreeNode>) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Sequence { elements })
}

fn ct_repeat(
    expr: CombinatorTreeNode,
    min: usize,
    max: Option<usize>,
    separator: Option<CombinatorTreeNode>,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Repeat {
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
                }) => ct_repeat(
                    expr.to_combinator_tree_node(subtype_index, grammar),
                    *min,
                    *max,
                    separator
                        .clone()
                        .map(|s| s.to_combinator_tree_node(subtype_index, grammar)),
                ),
                EBNF::Choice(exprs) => {
                    // merge (mixed) TerminalTrie ?
                    // merge (mixed) CharacterFilter ?
                    let index = subtype_index.get();
                    subtype_index.set(index + 1);
                    let name = format!("C{}", index);
                    ct_choice(
                        name,
                        exprs
                            .iter()
                            .map(|e| e.to_combinator_tree_node(subtype_index, grammar))
                            .collect(),
                    )
                }
                EBNF::Sequence(exprs) => ct_sequence(
                    exprs
                        .iter()
                        .map(|e| e.to_combinator_tree_node(subtype_index, grammar))
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
