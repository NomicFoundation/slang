use proc_macro2::TokenStream;
use quote::quote;

use codegen_schema::*;

use super::combinator_tree::CombinatorTree;

pub enum CharacterFilter<'context> {
    Negation {
        child: &'context CharacterFilter<'context>,
    },
    Range {
        from: char,
        to: char,
    },
    Char {
        char: char,
    },
    Disjunction {
        nodes: Vec<&'context CharacterFilter<'context>>,
    },
    Conjunction {
        nodes: Vec<&'context CharacterFilter<'context>>,
    },
}

impl<'context> CharacterFilter<'context> {
    pub fn new(
        tree: &CombinatorTree<'context>,
        expression: &ExpressionRef,
        inline_references: bool,
    ) -> Option<&'context CharacterFilter<'context>> {
        match &expression.ebnf {
            EBNF::Not(child) => Self::new(tree, child, inline_references).map(|child| {
                tree.context
                    .alloc_character_filter(Self::Negation { child })
                    as &CharacterFilter<'context> // removes the mutability
            }),

            EBNF::Choice(children) => {
                let nodes = children
                    .iter()
                    .filter_map(|c| Self::new(tree, c, inline_references))
                    .collect::<Vec<_>>();
                if nodes.len() == children.len() {
                    Some(
                        tree.context
                            .alloc_character_filter(Self::Disjunction { nodes }),
                    )
                } else {
                    None
                }
            }

            EBNF::Terminal(string) => {
                if string.chars().count() == 1 {
                    Some({
                        let char = string.chars().next().unwrap();
                        tree.context.alloc_character_filter(Self::Char { char })
                    })
                } else {
                    None
                }
            }

            EBNF::Range(EBNFRange { from, to }) => Some({
                let from = *from;
                let to = *to;
                tree.context
                    .alloc_character_filter(Self::Range { from, to })
            }),

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                if let (Some(minuend), Some(subtrahend)) = (
                    Self::new(tree, minuend, inline_references),
                    Self::new(tree, subtrahend, inline_references),
                ) {
                    Some({
                        let nodes = vec![
                            minuend,
                            tree.context
                                .alloc_character_filter(Self::Negation { child: subtrahend }),
                        ];
                        tree.context
                            .alloc_character_filter(Self::Conjunction { nodes })
                    })
                } else {
                    None
                }
            }

            EBNF::Reference(name) => {
                if inline_references {
                    Self::new(
                        tree,
                        &tree.context.get_tree_by_name(name).expression(),
                        inline_references,
                    )
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub(crate) fn to_predicate(&self, negated: bool) -> TokenStream {
        match self {
            CharacterFilter::Negation { child } => child.to_predicate(!negated),

            CharacterFilter::Range { from, to } => {
                if negated {
                    quote!( (c < #from || #to < c) )
                } else {
                    quote!( (#from <= c && c <= #to) )
                }
            }

            CharacterFilter::Char { char } => {
                if negated {
                    quote!( c != #char )
                } else {
                    quote!( c == #char )
                }
            }

            CharacterFilter::Conjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_predicate(negated));
                if negated {
                    quote! ( #(#nodes)||* )
                } else {
                    quote! ( #(#nodes)&&* )
                }
            }

            CharacterFilter::Disjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_predicate(negated));
                if negated {
                    quote! ( #(#nodes)&&* )
                } else {
                    quote! ( #(#nodes)||* )
                }
            }
        }
    }
}
