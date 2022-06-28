use std::rc::Rc;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::schema::ProductionWeakRef;

use super::{
    combinator_tree::{CombinatorTree, CombinatorTreeRoot},
    name::Name,
};

pub type TypeTreeRef = Rc<TypeTree>;

pub type NamedTypeTreeRef = (Name, TypeTreeRef);

pub enum TypeTree {
    Product(Name, Vec<NamedTypeTreeRef>),
    Sum(Name, Vec<NamedTypeTreeRef>),
    Repetition(TypeTreeRef),
    Option(TypeTreeRef),
    Expression(Name, Vec<Name>),
    ExpressionMember(ProductionWeakRef, TypeTreeRef),
    Reference(ProductionWeakRef),
    FixedTerminal(usize),
    VariableTerminal,
    Unit,
}

pub trait TypeTreeRefTrait {
    fn to_type_definition_code(&self, module_name: &Ident) -> (TokenStream, String);
}

impl TypeTreeRefTrait for TypeTreeRef {
    fn to_type_definition_code(&self, module_name: &Ident) -> (TokenStream, String) {
        let mut subtypes = (vec![], vec![]);
        let node_type = self.collect_type_definition_code(&module_name, &mut subtypes);
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

impl TypeTree {
    fn to_serde_annotation(&self) -> TokenStream {
        if self.is_defaultable() {
            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
        } else {
            quote!()
        }
    }

    fn is_defaultable(&self) -> bool {
        match self {
            Self::Expression(_, _) | Self::ExpressionMember(_, _) | Self::Sum(_, _) => false,
            Self::Product(_, members) => members.iter().all(|(_, t)| t.is_defaultable()),
            Self::Reference(production) => production
                .upgrade()
                .unwrap()
                .combinator_tree()
                .to_type_tree()
                .is_defaultable(),
            Self::Repetition(_)
            | Self::Option(_)
            | Self::FixedTerminal(_)
            | Self::VariableTerminal
            | Self::Unit => true,
        }
    }

    fn collect_type_definition_code(
        &self,
        module_name: &Ident,
        accum: &mut (Vec<TokenStream>, Vec<TokenStream>),
    ) -> TokenStream {
        match self {
            Self::Product(name, members) => {
                let tags: Vec<Ident> = members
                    .iter()
                    .map(|(n, _)| n.to_field_name_ident())
                    .collect();
                let types: Vec<TokenStream> = members
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(module_name, accum))
                    .collect();
                let serde_annotations: Vec<TokenStream> = members
                    .iter()
                    .map(|(_, t)| t.to_serde_annotation())
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
                let is_defaultable = self.is_defaultable();
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
            Self::Sum(name, choices) => {
                let tags: Vec<Ident> = choices.iter().map(|(n, _)| n.to_enum_tag_ident()).collect();
                let types: Vec<TokenStream> = choices
                    .iter()
                    .map(|(_, t)| t.collect_type_definition_code(module_name, accum))
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
                let child = child.collect_type_definition_code(module_name, accum);
                quote!( Vec<#child> )
            }
            Self::Option(child) => {
                let child = child.collect_type_definition_code(module_name, accum);
                quote!( Option<#child> )
            }
            Self::Expression(name, members) => {
                let tags: Vec<Ident> = members.iter().map(|n| n.to_enum_tag_ident()).collect();
                let types: Vec<Ident> = members.iter().map(|n| n.to_module_name_ident()).collect();
                let name = name.to_type_name_ident();
                accum.0.push(quote!(
                  #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                   pub enum #name { #(#tags(#types::E)),* }
                ));
                accum.1.push(quote!(
                    impl DefaultTest for #module_name::#name {}
                ));
                quote!( Box<#module_name::#name> )
            }
            Self::ExpressionMember(expression, inner_type) => {
                let inner_type = inner_type.collect_type_definition_code(module_name, accum);
                accum.0.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                    pub type E = #inner_type;
                ));
                let name = expression
                    .upgrade()
                    .unwrap()
                    .slang_name()
                    .to_module_name_ident();
                quote!(#name::N)
            }
            Self::Reference(production) => {
                let name = production
                    .upgrade()
                    .unwrap()
                    .slang_name()
                    .to_module_name_ident();
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

fn tt_product(name: Name, children: Vec<NamedTypeTreeRef>) -> TypeTreeRef {
    Rc::new(TypeTree::Product(name, children))
}

fn tt_sum(name: Name, children: Vec<NamedTypeTreeRef>) -> TypeTreeRef {
    Rc::new(TypeTree::Sum(name, children))
}

fn tt_repetition(child: TypeTreeRef) -> TypeTreeRef {
    Rc::new(TypeTree::Repetition(child))
}

fn tt_option(child: TypeTreeRef) -> TypeTreeRef {
    Rc::new(TypeTree::Option(child))
}

fn tt_expression(name: Name, member_names: Vec<Name>) -> TypeTreeRef {
    Rc::new(TypeTree::Expression(name, member_names))
}

fn tt_expression_member(expression: ProductionWeakRef, inner_type: TypeTreeRef) -> TypeTreeRef {
    Rc::new(TypeTree::ExpressionMember(expression, inner_type))
}

fn tt_reference(production: ProductionWeakRef) -> TypeTreeRef {
    Rc::new(TypeTree::Reference(production))
}

fn tt_variable_terminal() -> TypeTreeRef {
    Rc::new(TypeTree::VariableTerminal)
}

fn tt_fixed_terminal(size: usize) -> TypeTreeRef {
    Rc::new(TypeTree::FixedTerminal(size))
}

fn tt_unit() -> TypeTreeRef {
    Rc::new(TypeTree::Unit)
}

impl CombinatorTreeRoot {
    pub fn to_type_tree(&self) -> TypeTreeRef {
        self.root.to_type_tree()
    }
}

impl CombinatorTree {
    fn to_type_tree(&self) -> TypeTreeRef {
        match self {
            Self::Difference { minuend, .. } => minuend.to_type_tree(),
            Self::Lookahead { expr, .. } => expr.to_type_tree(),
            Self::Choice { choices, name } => tt_sum(
                name.clone(),
                choices
                    .iter()
                    .map(|(n, c)| (n.clone(), c.to_type_tree()))
                    .collect(),
            ),
            Self::Sequence { elements, name } => tt_product(
                name.clone(),
                elements
                    .iter()
                    .map(|(n, c)| (n.clone(), c.to_type_tree()))
                    .collect(),
            ),
            Self::Optional { expr } => tt_option(expr.to_type_tree()),
            Self::Repeated { expr, .. } => {
                if let Self::CharacterFilter { .. } = **expr {
                    tt_variable_terminal()
                } else {
                    tt_repetition(expr.to_type_tree())
                }
            }
            Self::SeparatedBy {
                name,
                expr,
                min,
                separator,
                ..
            } => {
                let inner = tt_product(
                    name.clone(),
                    vec![
                        (
                            expr.name().pluralize().self_or_positional(0),
                            tt_repetition(expr.to_type_tree()),
                        ),
                        (
                            separator.name().pluralize().self_or_positional(1),
                            tt_repetition(separator.to_type_tree()),
                        ),
                    ],
                );
                if *min == 0 {
                    tt_option(inner)
                } else {
                    inner
                }
            }
            Self::Expression { name, members } => tt_expression(
                name.clone(),
                members
                    .iter()
                    .map(|m| m.upgrade().unwrap().slang_name())
                    .collect(),
            ),
            Self::ExpressionMember { parent, expr, .. } => {
                tt_expression_member(parent.clone(), expr.to_type_tree())
            }
            Self::Reference { production } => tt_reference(production.clone()),
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
