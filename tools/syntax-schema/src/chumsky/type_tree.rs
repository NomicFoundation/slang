use convert_case::{Case, Casing};
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::combinator_tree::{CombinatorTree, CombinatorTreeNodeData};

pub struct TypeTree {
    pub root: TypeTreeNode,
}

pub type TypeTreeNode = Box<TypeTreeNodeData>;

pub enum TypeTreeNodeData {
    Tuple(String, Vec<(String, TypeTreeNode)>),
    Choice(String, Vec<(String, TypeTreeNode)>),
    Repetition(TypeTreeNode),
    Option(TypeTreeNode),
    Named(String),
    Length,
    Char,
    Unit,
}

impl TypeTree {
    pub fn to_type_definition_code(&self) -> TokenStream {
        let mut subtypes = Vec::new();
        let node_type = self.root.collect_type_definition_code(&mut subtypes);
        subtypes.reverse();
        quote!(
            #[allow(unused_imports)]
            use super::*;
            pub type N = #node_type;
            #(#subtypes)*
        )
    }
}

impl TypeTreeNodeData {
    fn collect_type_definition_code(&self, accum: &mut Vec<TokenStream>) -> TokenStream {
        match self {
            Self::Tuple(name, members) => {
                let tags: Vec<Ident> = members
                    .iter()
                    .map(|(n, _)| format_ident!("{}", n))
                    .collect();
                let types: Vec<TokenStream> = members
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(accum))
                    .collect();
                let nested_tags: TokenStream = tags
                    .iter()
                    .map(|t| quote!(#t))
                    .reduce(|accum, tag| quote!((#accum, #tag)))
                    .unwrap();
                let nested_types: TokenStream = types
                    .iter()
                    .cloned()
                    .reduce(|accum, tipe| quote!((#accum, #tipe)))
                    .unwrap();
                let name = format_ident!("{}", name);
                accum.push(quote!(
                    pub struct #name { #(pub #tags: #types),* }
                    impl #name {
                        pub fn new(#nested_tags: #nested_types) -> Self {
                            Self { #(#tags),* }
                        }
                    }
                ));
                quote!( #name )
            }
            Self::Choice(name, choices) => {
                let tags: Vec<Ident> = choices
                    .iter()
                    .map(|(n, _)| format_ident!("{}", n))
                    .collect();
                let types: Vec<TokenStream> = choices
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(accum))
                    .collect();
                let name = format_ident!("{}", name);
                accum.push(quote!( pub enum #name { #(#tags(#types)),* } ));
                quote!( #name )
            }
            Self::Repetition(child) => {
                let child = child.collect_type_definition_code(accum);
                quote!( Vec<#child> )
            }
            Self::Option(child) => {
                let child = child.collect_type_definition_code(accum);
                quote!( Option<#child> )
            }
            Self::Named(name) => {
                let name = format_ident!("{}", name.to_case(Case::Snake));
                quote!(#name::N)
            }
            Self::Length => {
                quote!(usize)
            }
            Self::Char => {
                quote!(char)
            }
            Self::Unit => {
                quote!(())
            }
        }
    }
}

fn tt_tuple(name: String, children: Vec<(String, TypeTreeNode)>) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Tuple(name, children))
}

fn tt_choice(name: String, children: Vec<(String, TypeTreeNode)>) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Choice(name, children))
}

fn tt_repetition(child: TypeTreeNode) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Repetition(child))
}

fn tt_option(child: TypeTreeNode) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Option(child))
}

fn tt_named(name: String) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Named(name))
}

fn tt_length() -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Length)
}

fn tt_char() -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Char)
}

fn tt_unit() -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Unit)
}

impl CombinatorTree {
    pub fn to_type_tree(&self) -> TypeTree {
        TypeTree {
            root: self.root.to_type_tree_node(),
        }
    }
}

impl CombinatorTreeNodeData {
    fn to_type_tree_node(&self) -> TypeTreeNode {
        match self {
            Self::Difference { minuend, .. } => minuend.to_type_tree_node(),
            Self::Lookahead { expr, .. } => expr.to_type_tree_node(),
            Self::Choice { choices, name } => tt_choice(
                name.clone(),
                choices
                    .iter()
                    .map(|(n, c)| (n.clone(), c.to_type_tree_node()))
                    .collect(),
            ),
            Self::Sequence { elements, name } => tt_tuple(
                name.clone(),
                elements
                    .iter()
                    .map(|(n, c)| (n.clone(), c.to_type_tree_node()))
                    .collect(),
            ),
            Self::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => tt_option(expr.to_type_tree_node()),
            Self::Repeat {
                expr,
                separator: None,
                ..
            } => tt_repetition(expr.to_type_tree_node()),
            Self::Repeat {
                name,
                expr,
                min,
                separator: Some(separator),
                ..
            } => {
                let inner = tt_tuple(
                    format!("{}", name),
                    vec![
                        (
                            expr.name().map_or_else(
                                || "expressions".to_owned(),
                                |n| n.to_case(Case::Snake).to_plural(),
                            ),
                            tt_repetition(expr.to_type_tree_node()),
                        ),
                        (
                            separator.name().map_or_else(
                                || "separators".to_owned(),
                                |n| n.to_case(Case::Snake).to_plural(),
                            ),
                            tt_repetition(separator.to_type_tree_node()),
                        ),
                    ],
                );
                if *min == 0 {
                    tt_option(inner)
                } else {
                    inner
                }
            }
            Self::Reference { name } => tt_named(name.clone()),
            Self::TerminalTrie { .. } => tt_length(),
            Self::CharacterFilter { .. } => tt_char(),
            Self::End => tt_unit(),
        }
    }
}
