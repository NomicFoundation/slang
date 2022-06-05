use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::combinator_tree::{CombinatorTree, CombinatorTreeNodeData};

pub struct TypeTree {
    pub root: TypeTreeNode,
}

pub type TypeTreeNode = Box<TypeTreeNodeData>;

pub enum TypeTreeNodeData {
    Tuple(Vec<TypeTreeNode>),
    Choice(String, Vec<TypeTreeNode>),
    Repetition(TypeTreeNode),
    Option(TypeTreeNode),
    Named(String),
    Length,
    Char,
    Unit,
}

impl TypeTree {
    pub fn tuple_size(&self) -> usize {
        self.root.tuple_size()
    }

    pub fn to_type_definition_code(&self) -> TokenStream {
        let mut subtypes = Vec::new();
        let node_type = self.root.collect_type_definition_code(true, &mut subtypes);
        quote!(
            #[allow(unused_imports)]
            use super::*;
            #node_type
            #(#subtypes)*
        )
    }
}

impl TypeTreeNodeData {
    fn tuple_size(&self) -> usize {
        if let Self::Tuple(members) = self {
            members.len()
        } else {
            0
        }
    }

    fn collect_type_definition_code(
        &self,
        is_top_level: bool,
        accum: &mut Vec<TokenStream>,
    ) -> TokenStream {
        match self {
            Self::Tuple(members) => {
                let elements = members
                    .iter()
                    .map(|e| e.collect_type_definition_code(false, accum));
                if is_top_level {
                    quote!( pub struct N(#(pub #elements),*); )
                } else {
                    quote!( (#(#elements),*) )
                }
            }
            Self::Choice(name, choices) => {
                let mut tags = vec![];
                for x in choices.iter().enumerate() {
                    let (i, c): (usize, &TypeTreeNode) = x;
                    let tag = format_ident!("_{}", i);
                    let tipe = c.collect_type_definition_code(false, accum);
                    tags.push(if c.tuple_size() > 0 {
                        quote!( #tag(#tipe), )
                    } else {
                        quote!( #tag(#tipe), )
                    })
                }
                let name = format_ident!("{}", name);
                accum.push(quote!( pub enum #name { #(#tags)* } ));
                if is_top_level {
                    quote! ( pub type N = #name; )
                } else {
                    quote!( #name )
                }
            }
            Self::Repetition(child) => {
                let child = child.collect_type_definition_code(false, accum);
                if is_top_level {
                    quote!( pub type N = Vec<#child>; )
                } else {
                    quote!( Vec<#child> )
                }
            }
            Self::Option(child) => {
                let child = child.collect_type_definition_code(false, accum);
                if is_top_level {
                    quote!( pub type N = Option<#child>; )
                } else {
                    quote!( Option<#child> )
                }
            }
            Self::Named(name) => {
                let name = format_ident!("{}", name.to_case(Case::Snake));
                if is_top_level {
                    quote!( pub type N = #name::N; )
                } else {
                    quote!(#name::N)
                }
            }
            Self::Length => {
                if is_top_level {
                    quote!(
                        pub type N = usize;
                    )
                } else {
                    quote!(usize)
                }
            }
            Self::Char => {
                if is_top_level {
                    quote!(
                        pub type N = char;
                    )
                } else {
                    quote!(char)
                }
            }
            Self::Unit => {
                if is_top_level {
                    quote!(
                        pub type N = ();
                    )
                } else {
                    quote!(())
                }
            }
        }
    }
}

fn tt_tuple(children: Vec<TypeTreeNode>) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Tuple(children))
}

fn tt_choice(name: String, children: Vec<TypeTreeNode>) -> TypeTreeNode {
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
                choices.iter().map(|c| c.to_type_tree_node()).collect(),
            ),
            Self::Sequence { elements } => {
                tt_tuple(elements.iter().map(|e| e.to_type_tree_node()).collect())
            }
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
                expr,
                min: 0,
                separator: Some(separator),
                ..
            } => tt_option(tt_tuple(vec![
                expr.to_type_tree_node(),
                tt_repetition(tt_tuple(vec![
                    separator.to_type_tree_node(),
                    expr.to_type_tree_node(),
                ])),
            ])),
            Self::Repeat {
                expr,
                separator: Some(separator),
                ..
            } => tt_tuple(vec![
                expr.to_type_tree_node(),
                tt_repetition(tt_tuple(vec![
                    separator.to_type_tree_node(),
                    expr.to_type_tree_node(),
                ])),
            ]),
            Self::Reference { name } => tt_named(name.clone()),
            Self::TerminalTrie { .. } => tt_length(),
            Self::CharacterFilter { .. } => tt_char(),
            Self::End => tt_unit(),
        }
    }
}
