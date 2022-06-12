use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::schema::Grammar;

use super::{
    combinator_tree::{CombinatorTree, CombinatorTreeNodeData},
    slang_name::SlangName,
};

pub struct TypeTree {
    pub root: TypeTreeNode,
}

pub type TypeTreeNode = Box<TypeTreeNodeData>;

pub enum TypeTreeNodeData {
    Tuple(SlangName, Vec<(SlangName, TypeTreeNode)>),
    Choice(SlangName, Vec<(SlangName, TypeTreeNode)>),
    Repetition(TypeTreeNode),
    Option(TypeTreeNode),
    Named(SlangName),
    Length,
    Char,
    Unit,
}

impl TypeTree {
    pub fn to_type_definition_code(
        &self,
        module_name: &Ident,
        grammar: &Grammar,
    ) -> (TokenStream, TokenStream) {
        let mut subtypes = (vec![], vec![]);
        let node_type =
            self.root
                .collect_type_definition_code(&module_name, grammar, &mut subtypes);
        subtypes.0.reverse();
        subtypes.1.reverse();
        let (subtype_definitions, subtype_implementations) = subtypes;
        (
            quote!(
                pub mod #module_name {
                    #[allow(unused_imports)]
                    use super::*;
                    pub type N = #node_type;
                    #(#subtype_definitions)*
                }
            ),
            quote!( #(#subtype_implementations)* ),
        )
    }

    fn to_serde_annotation(&self, grammar: &Grammar) -> TokenStream {
        self.root.to_serde_annotation(grammar)
    }
}

impl TypeTreeNodeData {
    fn to_serde_annotation(&self, grammar: &Grammar) -> TokenStream {
        match self {
            TypeTreeNodeData::Repetition(_) => quote!(
                #[serde(default, skip_serializing_if="Vec::is_empty")]
            ),
            TypeTreeNodeData::Option(_) => quote!(
                #[serde(default, skip_serializing_if="Option::is_none")]
            ),
            TypeTreeNodeData::Named(name) => {
                if let Some(production) = grammar.get_production(name.as_str()) {
                    production
                        .combinator_tree()
                        .to_type_tree()
                        .to_serde_annotation(grammar)
                } else {
                    quote!()
                }
            }
            TypeTreeNodeData::Char | TypeTreeNodeData::Unit => {
                quote!( #[serde(skip)])
            }
            TypeTreeNodeData::Tuple(_, _) | TypeTreeNodeData::Choice(_, _) => quote!(),
            TypeTreeNodeData::Length => quote!(
                #[serde(default, skip_serializing_if="usize_is_zero")]
            ),
        }
    }

    fn collect_type_definition_code(
        &self,
        module_name: &Ident,
        grammar: &Grammar,
        accum: &mut (Vec<TokenStream>, Vec<TokenStream>),
    ) -> TokenStream {
        match self {
            Self::Tuple(name, members) => {
                let tags: Vec<Ident> = members
                    .iter()
                    .map(|(n, _)| n.to_field_name_ident())
                    .collect();
                let types: Vec<TokenStream> = members
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(module_name, grammar, accum))
                    .collect();
                let serde_annotations: Vec<TokenStream> = members
                    .iter()
                    .map(|(_, t)| t.to_serde_annotation(grammar))
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
                let name = name.to_type_name_ident();
                accum.0.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                    pub struct #name { #(#serde_annotations pub #tags: #types),* }
                ));
                accum.1.push(quote!(
                    impl #module_name::#name {
                        pub fn new(#nested_tags: #nested_types) -> Self {
                            Self { #(#tags),* }
                        }
                    }
                ));
                quote!( Box<#module_name::#name> )
            }
            Self::Choice(name, choices) => {
                let tags: Vec<Ident> = choices.iter().map(|(n, _)| n.to_enum_tag_ident()).collect();
                let types: Vec<TokenStream> = choices
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(module_name, grammar, accum))
                    .collect();
                let name = name.to_type_name_ident();
                accum.0.push(quote!(
                  #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                   pub enum #name { #(#tags(#types)),* }
                ));
                quote!( Box<#module_name::#name> )
            }
            Self::Repetition(child) => {
                let child = child.collect_type_definition_code(module_name, grammar, accum);
                quote!( Vec<#child> )
            }
            Self::Option(child) => {
                let child = child.collect_type_definition_code(module_name, grammar, accum);
                quote!( Option<#child> )
            }
            Self::Named(name) => {
                let name = name.to_module_name_ident();
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

fn tt_tuple(name: SlangName, children: Vec<(SlangName, TypeTreeNode)>) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Tuple(name, children))
}

fn tt_choice(name: SlangName, children: Vec<(SlangName, TypeTreeNode)>) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Choice(name, children))
}

fn tt_repetition(child: TypeTreeNode) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Repetition(child))
}

fn tt_option(child: TypeTreeNode) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Option(child))
}

fn tt_named(name: SlangName) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Named(name))
}

fn tt_length() -> TypeTreeNode {
    Box::new(TypeTreeNodeData::Length)
}

#[allow(dead_code)]
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
    pub fn to_type_tree_node(&self) -> TypeTreeNode {
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
            } => {
                if let Self::CharacterFilter { .. } = **expr {
                    tt_length()
                } else {
                    tt_repetition(expr.to_type_tree_node())
                }
            }
            Self::Repeat {
                name,
                expr,
                min,
                separator: Some(separator),
                ..
            } => {
                let inner = tt_tuple(
                    name.clone(),
                    vec![
                        (
                            expr.name().map_or_else(
                                || SlangName::from_string("expressions"),
                                |n| n.plural(),
                            ),
                            tt_repetition(expr.to_type_tree_node()),
                        ),
                        (
                            separator.name().map_or_else(
                                || SlangName::from_string("separators"),
                                |n| n.plural(),
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
            Self::TerminalTrie { trie, .. } => {
                if trie.common_terminal_length().is_some() {
                    tt_unit()
                } else {
                    tt_length()
                }
            }
            Self::CharacterFilter { .. } => tt_unit(),
            Self::End => tt_unit(),
        }
    }
}
