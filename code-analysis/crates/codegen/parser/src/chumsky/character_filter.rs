// use std::cell::Cell;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use codegen_schema::*;

use super::{
    // combinator_node::CombinatorNode,
    combinator_tree::CombinatorTree,
    generated_code::GeneratedCode,
    naming,
};

// pub enum CharacterFilterParent<'context> {
//     CharacterFilter(&'context CharacterFilter<'context>),
//     CombinatorNode(&'context CombinatorNode<'context>),
// }

// pub struct CharFilter<'context> {
//     pub parent: CharacterFilterParent<'context>,
//     pub variant: CharacterFilter<'context>,
// }

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
    ) -> Option<&'context CharacterFilter<'context>> {
        match &expression.ebnf {
            EBNF::Not(child) => Self::new(tree, child).map(|child| {
                tree.context
                    .alloc_character_filter(Self::Negation { child })
                    as &CharacterFilter<'context>
            }),

            EBNF::Choice(children) => {
                let nodes = children
                    .iter()
                    .filter_map(|c| Self::new(tree, c))
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

            EBNF::Reference(name) => {
                Self::new(tree, &tree.context.get_tree_by_name(name).expression())
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                if let (Some(minuend), Some(subtrahend)) =
                    (Self::new(tree, minuend), Self::new(tree, subtrahend))
                {
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

            EBNF::DelimitedBy(_)
            | EBNF::OneOrMore(_)
            | EBNF::Optional(_)
            | EBNF::Repeat(_)
            | EBNF::SeparatedBy(_)
            | EBNF::Sequence(_)
            | EBNF::ZeroOrMore(_) => None,
        }
    }

    pub fn merged_with(
        &'context self,
        other: &'context CharacterFilter<'context>,
        tree: &CombinatorTree<'context>,
    ) -> &'context CharacterFilter<'context> {
        tree.context.alloc_character_filter(Self::Disjunction {
            nodes: vec![self, other],
        })
    }

    pub fn default_name(&self) -> Option<String> {
        if let Self::Negation { child } = self {
            child.default_name().map(|n| format!("not_{}", n))
        } else if let Self::Char { char, .. } = self {
            Some(naming::name_of_terminal_char(*char))
        } else {
            None
        }
    }

    pub(super) fn to_code(
        &self,
        name: Option<&String>,
        code: &mut GeneratedCode,
        macro_prefix: &str,
    ) -> TokenStream {
        let macro_name = format_ident!("{}terminal", macro_prefix);
        if let CharacterFilter::Char { char } = self {
            let kind = code.add_terminal_kind(char.to_string());
            quote!(#macro_name!(#kind, #char))
        } else {
            let predicate = self.to_predicate(false);
            if let Some(kind) = name.map(|n| code.add_token_kind(n.clone())) {
                quote!(#macro_name!(#kind, |&c: &char| #predicate))
            } else {
                quote!(#macro_name!(|&c: &char| #predicate))
            }
        }
    }

    fn to_predicate(&self, negated: bool) -> TokenStream {
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
