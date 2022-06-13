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
    FixedTerminal(usize),
    VariableTerminal,
    Unit,
}

impl TypeTree {
    pub fn to_type_definition_code(
        &self,
        module_name: &Ident,
        grammar: &Grammar,
    ) -> (TokenStream, String) {
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
            subtype_implementations
                .iter()
                .map(|q| q.to_string())
                .collect::<Vec<_>>()
                .join("\n\n"),
        )
    }
}

impl TypeTreeNodeData {
    fn to_serde_annotation(&self, grammar: &Grammar) -> TokenStream {
        if self.is_defaultable(grammar) {
            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
        } else {
            quote!()
        }
    }

    fn is_defaultable(&self, grammar: &Grammar) -> bool {
        match self {
            TypeTreeNodeData::Choice(_, _) => false,
            TypeTreeNodeData::Tuple(_, members) => {
                members.iter().all(|(_, t)| t.is_defaultable(grammar))
            }
            TypeTreeNodeData::Named(name) => grammar
                .get_production(name.as_str())
                .map(|p| {
                    p.combinator_tree()
                        .to_type_tree()
                        .root
                        .is_defaultable(grammar)
                })
                .unwrap_or_default(),
            TypeTreeNodeData::Repetition(_)
            | TypeTreeNodeData::Option(_)
            | TypeTreeNodeData::FixedTerminal(_)
            | TypeTreeNodeData::VariableTerminal
            | TypeTreeNodeData::Unit => true,
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
                let is_defaultable = self.is_defaultable(grammar);
                let name = name.to_type_name_ident();
                accum.0.push(if is_defaultable {
                    quote!(
                        #[derive(Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
                        pub struct #name { #(#serde_annotations pub #tags: #types),* }
                    )
                } else {
                    quote!(
                        #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                        pub struct #name { #(#serde_annotations pub #tags: #types),* }
                    )
                });
                accum.1.push(if is_defaultable {
                    quote!(
                        impl DefaultTest for #module_name::#name {
                            fn is_default(&self) -> bool {
                                #(self.#tags.is_default())&&*
                            }
                        }
                    )
                } else {
                    quote!( impl DefaultTest for #module_name::#name {})
                });
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
                accum.1.push(quote!(
                    impl DefaultTest for #module_name::#name {}
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
            Self::VariableTerminal => {
                quote!(usize)
            }
            Self::FixedTerminal(size) => {
                quote!( FixedTerminal<#size> )
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

fn tt_variable_terminal() -> TypeTreeNode {
    Box::new(TypeTreeNodeData::VariableTerminal)
}

fn tt_fixed_terminal(size: usize) -> TypeTreeNode {
    Box::new(TypeTreeNodeData::FixedTerminal(size))
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
                    tt_variable_terminal()
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
                if let Some(size) = trie.common_terminal_length() {
                    tt_fixed_terminal(size)
                } else {
                    tt_variable_terminal()
                }
            }
            Self::CharacterFilter { .. } => tt_fixed_terminal(1),
            Self::End => tt_unit(),
        }
    }
}
