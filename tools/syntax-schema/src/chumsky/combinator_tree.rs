use std::{
    cell::Cell,
    collections::BTreeSet,
    rc::{Rc, Weak},
};

use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::{character_filter::CharacterFilter, name::Name, terminal_trie::TerminalTrie};

#[derive(Clone, Debug)]
pub struct CombinatorTreeRoot {
    pub root: CombinatorTreeRef,
    production: ProductionWeakRef,
}

type CombinatorTreeRef = Rc<CombinatorTree>;
type NamedCombinatorTreeRef = (Name, CombinatorTreeRef);

#[derive(Clone, Debug)]
pub enum CombinatorTree {
    CharacterFilter {
        name: Name,
        filter: CharacterFilter,
    },
    Choice {
        name: Name,
        elements: Vec<NamedCombinatorTreeRef>,
    },
    DelimitedBy {
        name: Name,
        open: String,
        expr: CombinatorTreeRef,
        close: String,
    },
    Difference {
        minuend: CombinatorTreeRef,
        subtrahend: CombinatorTreeRef,
    },
    End,
    Expression {
        name: Name,
        members: Vec<ProductionWeakRef>,
    },
    ExpressionMember {
        name: Name,
        parent_name: Name,
        next_sibling_name: Option<Name>,
        operator: CombinatorTreeRef,
        operator_model: OperatorModel,
    },
    Lookahead {
        expr: CombinatorTreeRef,
        lookahead: CombinatorTreeRef,
    },
    OneOrMore {
        name: Name,
        expr: CombinatorTreeRef,
    },
    Optional {
        expr: CombinatorTreeRef,
    },
    Reference {
        production: ProductionWeakRef,
    },
    Repeated {
        name: Name,
        expr: CombinatorTreeRef,
        min: usize,
        max: usize,
    },
    SeparatedBy {
        name: Name,
        expr: CombinatorTreeRef,
        separator: String,
    },
    Sequence {
        name: Name,
        elements: Vec<NamedCombinatorTreeRef>,
    },
    TerminalTrie {
        name: Name,
        trie: TerminalTrie,
    },
    ZeroOrMore {
        name: Name,
        expr: CombinatorTreeRef,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorModel {
    None,
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

fn ct_character_filter(name: Name, filter: CharacterFilter) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::CharacterFilter { name, filter })
}

fn ct_choice(name: Name, elements: Vec<(Name, CombinatorTreeRef)>) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Choice { name, elements })
}

fn ct_delimited_by(
    name: Name,
    open: String,
    expr: CombinatorTreeRef,
    close: String,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::DelimitedBy {
        name,
        open,
        expr,
        close,
    })
}

fn ct_difference(minuend: CombinatorTreeRef, subtrahend: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Difference {
        minuend,
        subtrahend,
    })
}

fn ct_end() -> CombinatorTreeRef {
    Rc::new(CombinatorTree::End)
}

fn ct_expression(name: Name, members: Vec<ProductionWeakRef>) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Expression { name, members })
}

fn ct_expression_member(
    name: Name,
    parent_name: Name,
    next_sibling_name: Option<Name>,
    operator: CombinatorTreeRef,
    operator_model: OperatorModel,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::ExpressionMember {
        name,
        parent_name,
        next_sibling_name,
        operator,
        operator_model,
    })
}

fn ct_lookahead(expr: CombinatorTreeRef, lookahead: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Lookahead { expr, lookahead })
}

fn ct_one_or_more(name: Name, expr: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::OneOrMore { name, expr })
}

fn ct_optional(expr: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Optional { expr })
}

fn ct_reference(production: ProductionWeakRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Reference { production })
}

fn ct_repeat(name: Name, expr: CombinatorTreeRef, min: usize, max: usize) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Repeated {
        name,
        expr,
        min,
        max,
    })
}

fn ct_separated_by(name: Name, expr: CombinatorTreeRef, separator: String) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::SeparatedBy {
        name,
        expr,
        separator,
    })
}

fn ct_sequence(name: Name, elements: Vec<(Name, CombinatorTreeRef)>) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Sequence { name, elements })
}

fn ct_terminal_trie(name: Name, trie: TerminalTrie) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::TerminalTrie { name, trie })
}

fn ct_zero_or_more(name: Name, expr: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::ZeroOrMore { name, expr })
}

pub struct ProductionGeneratedCode {
    pub parser_field_definition: TokenStream,
    pub parser_field_initialization: TokenStream,

    pub parser_implementation_predeclaration: TokenStream,
    pub parser_implementation: TokenStream,

    pub tree_interface: TokenStream,
    pub tree_implementation: TokenStream,
}

impl CombinatorTreeRoot {
    pub fn production(&self) -> ProductionRef {
        self.production.upgrade().unwrap()
    }

    pub fn slang_name(&self) -> Name {
        self.production().slang_name()
    }

    pub fn referenced_identifiers(&self) -> BTreeSet<String> {
        let mut accum = BTreeSet::new();
        self.root.collect_identifiers(&mut accum);
        if !self.production().is_token() {
            accum.insert("LeadingTrivia".to_owned());
            accum.insert("TrailingTrivia".to_owned());
        }
        accum
    }

    pub fn to_generated_code(&self, is_recursive: bool) -> ProductionGeneratedCode {
        let GeneratedCode {
            parser_type,
            parser,
            mut tree_interface,
            mut tree_implementation,
        } = self.root.to_generated_code(self);

        let module_name = self.slang_name().to_module_name_ident();
        let type_name = self.slang_name().to_type_name_ident();
        let parser_name = self.slang_name().to_parser_name_ident();
        let field_name = self.slang_name().to_field_name_ident();

        if self.production().is_token() {
            let parser_type = parser_type.clone();
            if self.root.has_default() {
                tree_interface.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
                    pub struct WithTrivia {
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub leading: LeadingTrivia,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub content: #parser_type,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub trailing: TrailingTrivia,
                    }
                ));
                tree_implementation.push(quote!(
                    impl DefaultTest for #module_name::WithTrivia {
                        fn is_default(&self) -> bool {
                            self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
                        }
                    }
                ))
            } else {
                tree_interface.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct WithTrivia {
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub leading: LeadingTrivia,
                        pub content: #parser_type,
                        #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                        pub trailing: TrailingTrivia,
                    }
                ));
            }
        }

        ProductionGeneratedCode {
            tree_interface: {
                let module = if tree_interface.is_empty() {
                    quote!()
                } else {
                    quote!(
                        pub mod #module_name {
                        #[allow(unused_imports)]
                        use super::*;
                        #(#tree_interface)*
                    })
                };
                quote!(
                    pub type #type_name = #parser_type;
                    #module
                )
            },

            parser_field_definition: quote!( pub #field_name: ParserType<#type_name> ),

            parser_field_initialization: if is_recursive {
                quote!( #field_name: #parser_name.boxed() )
            } else {
                quote!( #field_name: #parser_name )
            },

            parser_implementation_predeclaration: if is_recursive {
                quote!( let mut #parser_name = Recursive::declare(); )
            } else {
                quote!()
            },

            parser_implementation: if is_recursive {
                quote!( #parser_name.define(#parser.boxed()); )
            } else {
                quote!( let #parser_name = #parser.boxed(); )
            },

            tree_implementation: quote!( #(#tree_implementation)* ),
        }
    }
}

impl Default for CombinatorTreeRoot {
    fn default() -> Self {
        Self {
            root: ct_end(),
            production: Default::default(),
        }
    }
}

trait CombinatorTreeNodeTrait {
    fn with_unambiguous_named_types(&self, index: &mut Cell<usize>) -> CombinatorTreeRef;
}

impl CombinatorTreeNodeTrait for CombinatorTreeRef {
    fn with_unambiguous_named_types(&self, index: &mut Cell<usize>) -> CombinatorTreeRef {
        match self.as_ref() {
            CombinatorTree::CharacterFilter { .. }
            | CombinatorTree::End
            | CombinatorTree::Expression { .. }
            | CombinatorTree::Reference { .. }
            | CombinatorTree::TerminalTrie { .. } => self.clone(),

            CombinatorTree::Choice { name, elements } => {
                ct_choice(name.clone().self_or_numbered(index), {
                    let elements = elements
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.clone().self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect::<Vec<_>>();
                    let names = disambiguate_structure_names(
                        elements.iter().map(|(n, _)| n.clone()).collect(),
                    );
                    elements
                        .into_iter()
                        .zip(names.into_iter())
                        .map(|((_, e), n)| (n, e))
                        .collect()
                })
            }

            CombinatorTree::DelimitedBy {
                name,
                open,
                expr,
                close,
            } => ct_delimited_by(
                name.clone().self_or_numbered(index),
                open.clone(),
                expr.with_unambiguous_named_types(index),
                close.clone(),
            ),

            CombinatorTree::Difference {
                minuend,
                subtrahend,
            } => ct_difference(
                minuend.with_unambiguous_named_types(index),
                subtrahend.with_unambiguous_named_types(index),
            ),

            CombinatorTree::ExpressionMember {
                name,
                parent_name,
                next_sibling_name,
                operator,
                operator_model,
            } => ct_expression_member(
                name.clone(),
                parent_name.clone(),
                next_sibling_name.clone(),
                operator.with_unambiguous_named_types(index),
                *operator_model,
            ),

            CombinatorTree::Lookahead { expr, lookahead } => ct_lookahead(
                expr.with_unambiguous_named_types(index),
                lookahead.with_unambiguous_named_types(index),
            ),

            CombinatorTree::OneOrMore { name, expr } => ct_one_or_more(
                name.clone().self_or_numbered(index),
                expr.with_unambiguous_named_types(index),
            ),

            CombinatorTree::Optional { expr } => {
                ct_optional(expr.with_unambiguous_named_types(index))
            }

            CombinatorTree::Repeated {
                name,
                expr,
                min,
                max,
            } => ct_repeat(
                name.clone().self_or_numbered(index),
                expr.with_unambiguous_named_types(index),
                *min,
                *max,
            ),

            CombinatorTree::SeparatedBy {
                name,
                expr,
                separator,
            } => ct_separated_by(
                name.clone().self_or_numbered(index),
                expr.with_unambiguous_named_types(index),
                separator.clone(),
            ),

            CombinatorTree::Sequence { name, elements } => {
                ct_sequence(name.clone().self_or_numbered(index), {
                    let elements = elements
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.clone().self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect::<Vec<_>>();
                    let names = disambiguate_structure_names(
                        elements.iter().map(|(n, _)| n.clone()).collect(),
                    );
                    elements
                        .into_iter()
                        .zip(names.into_iter())
                        .map(|((_, e), n)| (n, e))
                        .collect()
                })
            }

            CombinatorTree::ZeroOrMore { name, expr } => ct_zero_or_more(
                name.clone().self_or_numbered(index),
                expr.with_unambiguous_named_types(index),
            ),
        }
    }
}

#[derive(Default)]
pub struct GeneratedCode {
    pub parser_type: TokenStream,
    pub parser: TokenStream,
    pub tree_interface: Vec<TokenStream>,
    pub tree_implementation: Vec<TokenStream>,
}

impl GeneratedCode {
    pub fn merge(&mut self, other: Self) {
        self.parser_type = other.parser_type;
        self.parser = other.parser;
        self.tree_interface.extend(other.tree_interface);
        self.tree_implementation.extend(other.tree_implementation);
    }
}

impl CombinatorTree {
    pub fn name(&self) -> Name {
        match self {
            Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::DelimitedBy { name, .. }
            | Self::Expression { name, .. }
            | Self::ExpressionMember { name, .. }
            | Self::Sequence { name, .. }
            | Self::TerminalTrie { name, .. } => name.clone(),

            Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::Optional { expr } => expr.name(),

            Self::End => Name::from_string("end_marker"),

            Self::OneOrMore { expr, .. }
            | Self::Repeated { expr, .. }
            | Self::SeparatedBy { expr, .. }
            | Self::ZeroOrMore { expr, .. } => expr.name().pluralize(),

            Self::Reference { production } => production.upgrade().unwrap().slang_name(),
        }
    }

    fn has_default(&self) -> bool {
        match self {
            Self::CharacterFilter { .. }
            | Self::End
            | Self::OneOrMore { .. }
            | Self::Optional { .. }
            | Self::Repeated { .. }
            | Self::SeparatedBy { .. }
            | Self::TerminalTrie { .. }
            | Self::ZeroOrMore { .. } => true,

            Self::Choice { .. } | Self::Expression { .. } => false,

            Self::DelimitedBy { expr, .. }
            | Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. } => expr.has_default(),

            Self::ExpressionMember {
                operator,
                operator_model,
                ..
            } => *operator_model == OperatorModel::None && operator.has_default(),

            Self::Reference { production } => production
                .upgrade()
                .unwrap()
                .combinator_tree()
                .root
                .has_default(),

            Self::Sequence { elements, .. } => elements.iter().all(|(_, e)| e.has_default()),
        }
    }

    fn generated_code_for_string(str: &str, with_trivia: bool) -> (TokenStream, TokenStream, Name) {
        let size = str.chars().count();
        let parser = if size == 1 {
            let char = str.chars().next().unwrap();
            quote!( just(#char).map(|_| FixedSizeTerminal::<#size>()) )
        } else {
            quote!( just(#str).map(|_| FixedSizeTerminal::<#size>()) )
        };

        if with_trivia {
            (
                quote!(
                    leading_trivia_parser.clone().then(#parser).then(trailing_trivia_parser.clone())
                    .map(|((leading, content), trailing)| FixedSizeTerminalWithTrivia { leading, content, trailing })
                ),
                quote!(FixedSizeTerminalWithTrivia<#size>),
                Name::from_terminal(str),
            )
        } else {
            (
                parser,
                quote!(FixedSizeTerminal<#size>),
                Name::from_terminal(str),
            )
        }
    }

    fn to_generated_code(&self, tree: &CombinatorTreeRoot) -> GeneratedCode {
        match self {
            Self::CharacterFilter { filter, .. } => {
                filter.to_generated_code(!tree.production().is_token())
            }

            Self::Choice { name, elements } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let mut fields = vec![];
                let mut parsers = vec![];
                for (name, element) in elements {
                    result.merge(element.to_generated_code(tree));

                    let name = name.to_enum_tag_ident();
                    let parser_type = result.parser_type.clone();
                    fields.push(quote!( #name(#parser_type) ));

                    let parser = result.parser.clone();
                    parsers.push(
                        quote!( #parser.map(|v| Box::new(#module_name::#type_name::#name(v))) ),
                    );
                }

                result.tree_interface.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub enum #type_name { #(#fields),* }
                ));

                result.parser = quote!( choice(( #(#parsers),* )) );
                result.parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            Self::DelimitedBy {
                name,
                open,
                expr,
                close,
            } => {
                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let (open_parser, open_type, open_name) =
                    Self::generated_code_for_string(open, !tree.production().is_token());
                let (close_parser, close_type, close_name) =
                    Self::generated_code_for_string(close, !tree.production().is_token());

                let new_names =
                    disambiguate_structure_names(vec![open_name, expr.name(), close_name]);
                let mut new_names = new_names.iter().map(Name::to_field_name_ident);
                let open_name = new_names.next().unwrap();
                let expr_name = new_names.next().unwrap();
                let close_name = new_names.next().unwrap();

                let mut result = expr.to_generated_code(tree);
                let expr_annotation = if self.has_default() {
                    result.tree_implementation.push(quote!(
                        impl Default for #module_name::#type_name {
                            fn default() -> Self {
                                Self {
                                    #open_name: Default::default(),
                                    #expr_name: Default::default(),
                                    #close_name: Default::default(),
                                }
                            }
                        }
                        impl DefaultTest for #module_name::#type_name {
                            fn is_default(&self) -> bool {
                                self.#open_name.is_default()
                                && self.#expr_name.is_default()
                                && self.#close_name.is_default()
                            }
                        }
                    ));

                    quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                } else {
                    quote!()
                };

                let expr_type = result.parser_type;

                result.tree_interface.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct #type_name {
                        #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                        pub #open_name: #open_type,
                        #expr_annotation
                        pub #expr_name: #expr_type,
                        #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                        pub #close_name: #close_type,
                    }
                ));

                result.tree_implementation.push(quote!(
                    impl #module_name::#type_name {
                        pub fn from_parse(((#open_name, #expr_name), #close_name): ((#open_type, #expr_type), #close_type)) -> Self {
                            Self { #open_name, #expr_name, #close_name }
                        }
                    }
                ));

                let expr_parser = result.parser;
                result.parser = quote!( #open_parser.then(#expr_parser).then(#close_parser).map(|v| #module_name::#type_name::from_parse(v)) );
                result.parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let mut minuend = minuend.to_generated_code(tree);
                let subtrahend = subtrahend.to_generated_code(tree);

                let minuend_parser = minuend.parser;
                let subtrahend_parser = subtrahend.parser;
                minuend.parser = quote! ( difference(#minuend_parser, #subtrahend_parser) );

                minuend
            }

            Self::End => {
                let mut result: GeneratedCode = Default::default();
                result.parser = quote!(end());
                result.parser_type = quote!(());
                result
            }

            Self::Expression { name, members } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let names = members
                    .iter()
                    .map(|p| p.upgrade().unwrap().slang_name())
                    .collect::<Vec<_>>();

                let fields = names
                    .iter()
                    .map(|name| {
                        let tag_name = name.to_enum_tag_ident();
                        let module_name = name.to_module_name_ident();
                        quote!( #tag_name(#module_name::E) )
                    })
                    .collect::<Vec<_>>();
                result.tree_interface.push(quote!(
                   #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                   pub enum #type_name { #(#fields),* }
                ));

                let first_parser_name = names[0].to_parser_name_ident();
                result.parser = quote!( #first_parser_name.clone() );
                result.parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            Self::ExpressionMember {
                name,
                parent_name,
                next_sibling_name,
                operator,
                operator_model,
            } => {
                let mut result = operator.to_generated_code(tree);

                let operator_type = result.parser_type;
                let operator_parser = result.parser;

                let tag_name = name.to_enum_tag_ident();
                let module_name = name.to_module_name_ident();
                let parent_type_name = parent_name.to_type_name_ident();
                let parent_module_name = parent_name.to_module_name_ident();

                match operator_model {
                    OperatorModel::None => {
                        result.parser = match next_sibling_name {
                            Some(next_sibling_name) => {
                                let next_sibling_parser_name =
                                    next_sibling_name.to_parser_name_ident();
                                quote!(
                                    choice((
                                        #operator_parser.map(|v| Box::new(#parent_module_name::#parent_type_name::#tag_name(v))),
                                        #next_sibling_parser_name.clone()
                                    ))
                                )
                            }
                            None => {
                                quote!( #operator_parser.map(|v| Box::new(#parent_module_name::#parent_type_name::#tag_name(v))) )
                            }
                        };
                        result
                            .tree_interface
                            .push(quote!( pub type E = #operator_type; ));
                    }

                    OperatorModel::BinaryLeftAssociative => {
                        let next_sibling_parser_name = next_sibling_name
                            .clone()
                            .expect("Cannot have binary operator as last expression member")
                            .to_parser_name_ident();
                        let annotation = if operator.has_default() {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.tree_interface.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        result.parser = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                                    operator_operand_pairs.into_iter().fold(first_operand, |left_operand, (operator, right_operand)|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator, right_operand }))
                                    )
                                }
                            )
                        );
                    }

                    OperatorModel::BinaryRightAssociative => {
                        let next_sibling_parser_name = next_sibling_name
                            .clone()
                            .expect("Cannot have binary operator as last expression member")
                            .to_parser_name_ident();
                        let annotation = if operator.has_default() {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.tree_interface.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        result.parser = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => [ (a X) (b Y) (c Z) ] d
                                    let mut last_operand = first_operand;
                                    let mut operand_operator_pairs = vec![];
                                    for (operator, right_operand) in operator_operand_pairs.into_iter() {
                                        let left_operand = std::mem::replace(&mut last_operand, right_operand);
                                        operand_operator_pairs.push((left_operand, operator))
                                    }
                                    // [ (a X) (b Y) (c Z) ] d => { a X { b Y { c Z d } } }
                                    operand_operator_pairs.into_iter().rfold(last_operand, |right_operand, (left_operand, operator)|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator, right_operand }))
                                    )
                                }
                            )
                        );
                    }

                    OperatorModel::UnaryPrefix => {
                        let next_sibling_parser_name = next_sibling_name
                            .clone()
                            .expect("Cannot have unary prefix operator as last expression member")
                            .to_parser_name_ident();
                        let annotation = if operator.has_default() {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.tree_interface.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                #annotation
                                pub operator: #operator_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        result.parser = quote!(
                            #operator_parser.repeated()
                            .then(#next_sibling_parser_name.clone())
                            .map(|(mut operators, operand)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.reverse();
                                    operators.into_iter().fold(operand, |right_operand, operator|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { operator, right_operand }))
                                    )
                                }
                            )
                        )
                    }

                    OperatorModel::UnarySuffix => {
                        let next_sibling_parser_name = next_sibling_name
                            .clone()
                            .expect("Cannot have unary suffix operator as last expression member")
                            .to_parser_name_ident();
                        let annotation = if operator.has_default() {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.tree_interface.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_type,
                            }
                        ));

                        result.parser = quote!(
                            #next_sibling_parser_name.clone()
                            .then(#operator_parser.repeated())
                            .map(|(operand, operators)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.into_iter().fold(operand, |left_operand, operator|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator }))
                                    )
                                }
                            )
                        );
                    }
                };

                result.parser_type = quote!( #parent_type_name );

                result
            }

            Self::Lookahead { expr, lookahead } => {
                let mut expr = expr.to_generated_code(tree);
                let lookahead = lookahead.to_generated_code(tree);

                let expr_parser = expr.parser;
                let lookahead_parser = lookahead.parser;
                expr.parser = quote!( #expr_parser.then_ignore( #lookahead_parser.rewind() ));

                expr
            }

            Self::OneOrMore { expr, .. } => {
                let mut result = expr.to_generated_code(tree);

                let mut parser = result.parser;
                parser = quote!( #parser.repeated().at_least(1usize) );

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    parser = quote!( #parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.parser_type = quote!(VariableSizeTerminal);
                } else {
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( Vec<#parser_type> );
                };

                result.parser = parser;

                result
            }

            Self::Optional { expr } => {
                let mut result = expr.to_generated_code(tree);

                let parser = result.parser;
                result.parser = quote!( #parser.or_not() );

                let parser_type = result.parser_type;
                result.parser_type = quote!( Option<#parser_type> );

                result
            }

            Self::Reference { production } => {
                let mut result: GeneratedCode = Default::default();
                let production = production.upgrade().unwrap();

                let name = production.slang_name();
                let production_parser_name = name.to_parser_name_ident();
                let production_type_name = name.to_type_name_ident();

                if !tree.production().is_token() && production.is_token() {
                    let production_module_name = name.to_module_name_ident();
                    result.parser = quote!(
                        leading_trivia_parser.clone()
                        .then(#production_parser_name.clone())
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading, content), trailing)|
                            #production_module_name::WithTrivia { leading, content, trailing })
                    );
                    result.parser_type = quote!( #production_module_name::WithTrivia );
                } else {
                    result.parser = quote!( #production_parser_name.clone() );
                    result.parser_type = quote!( #production_type_name );
                }

                result
            }

            Self::Repeated { expr, min, max, .. } => {
                let mut result = expr.to_generated_code(tree);

                let mut parser = result.parser;
                parser = if min == max {
                    quote!( #parser.repeated().exactly(#min) )
                } else {
                    quote!( #parser.repeated().at_least(#min).at_most(#max) )
                };

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    parser = quote!( #parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.parser_type = quote!(VariableSizeTerminal);
                } else {
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( Vec<#parser_type> );
                };

                result.parser = parser;

                result
            }

            Self::SeparatedBy {
                name,
                expr,
                separator,
            } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let expr = expr.to_generated_code(tree);
                let (separator_parser, separator_type, _separator_name) =
                    Self::generated_code_for_string(separator, !tree.production().is_token());

                let expr_parser_type = expr.parser_type.clone();
                let separator_parser_type = separator_type.clone();
                let expr_parser = expr.parser.clone();
                let separator_parser = separator_parser.clone();

                result.merge(expr);

                let repetition = quote!(#separator_parser.then(#expr_parser).repeated());
                let mapping = quote!(
                    .map(repetition_mapper)
                    .map(|(elements, separators)| #module_name::#type_name { elements, separators })
                );
                result.parser = quote!( #expr_parser.then(#repetition) #mapping );

                result.tree_interface.push({
                    quote!(
                        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                        pub struct #type_name {
                            #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                            pub elements: Vec<#expr_parser_type>,
                            #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                            pub separators: Vec<#separator_parser_type>
                        }
                    )
                });

                result.tree_implementation.push(quote!(
                    impl Default for #module_name::#type_name {
                        fn default() -> Self {
                            Self {
                                elements: Default::default(),
                                separators: Default::default(),
                            }
                        }
                    }
                    impl DefaultTest for #module_name::#type_name {
                        fn is_default(&self) -> bool {
                            self.elements.is_default() && self.separators.is_default()
                        }
                    }
                ));

                result.parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::Sequence { name, elements } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let mut field_annotations = vec![];
                let mut field_names = vec![];
                let mut field_types = vec![];
                let mut parser_chain = None;
                for (name, element) in elements {
                    result.merge(element.to_generated_code(tree));

                    field_annotations.push(if element.has_default() {
                        quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                    } else {
                        quote!()
                    });

                    let name = name.to_field_name_ident();
                    field_names.push(quote!( #name ));

                    let parser_type = result.parser_type.clone();
                    field_types.push(quote!( #parser_type ));

                    let next_parser = result.parser.clone();
                    parser_chain = parser_chain
                        .and_then(|p| Some(quote!( #p.then(#next_parser) )))
                        .or_else(|| Some(next_parser))
                }

                result.tree_interface.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct #type_name {
                        #(
                            #field_annotations
                            pub #field_names: #field_types
                        ),*
                    }
                ));

                let folded_field_names = field_names
                    .clone()
                    .into_iter()
                    .reduce(|accum, next| quote!( (#accum, #next) ));
                let folded_field_types = field_types
                    .clone()
                    .into_iter()
                    .reduce(|accum, next| quote!( (#accum, #next) ));
                result.tree_implementation.push(quote!(
                    impl #module_name::#type_name {
                        pub fn from_parse(#folded_field_names: #folded_field_types) -> Self {
                            Self { #(#field_names),* }
                        }
                    }
                ));

                if self.has_default() {
                    result.tree_implementation.push(quote!(
                        impl Default for #module_name::#type_name {
                            fn default() -> Self {
                                Self {
                                    #(#field_names: Default::default()),*
                                }
                            }
                        }
                        impl DefaultTest for #module_name::#type_name {
                            fn is_default(&self) -> bool {
                                #(self. #field_names .is_default())&&*
                            }
                        }
                    ));
                }

                let parser = parser_chain.unwrap();
                result.parser = quote!( #parser.map(|v| #module_name::#type_name::from_parse(v)) );
                result.parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::TerminalTrie { trie, .. } => {
                trie.to_generated_code(!tree.production().is_token())
            }

            Self::ZeroOrMore { expr, .. } => {
                let mut result = expr.to_generated_code(tree);

                let mut parser = result.parser;
                parser = quote!( #parser.repeated() );

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    parser = quote!( #parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.parser_type = quote!(VariableSizeTerminal);
                } else {
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( Vec<#parser_type> );
                };

                result.parser = parser;

                result
            }
        }
    }

    fn collect_identifiers(&self, accum: &mut BTreeSet<String>) {
        match self {
            Self::CharacterFilter { .. } | Self::End | Self::TerminalTrie { .. } => {}

            Self::Choice { elements, .. } | Self::Sequence { elements, .. } => {
                for (_, member) in elements {
                    member.collect_identifiers(accum);
                }
            }

            Self::DelimitedBy { expr, .. }
            | Self::OneOrMore { expr, .. }
            | Self::Optional { expr }
            | Self::Repeated { expr, .. }
            | Self::SeparatedBy { expr, .. }
            | Self::ZeroOrMore { expr, .. } => expr.collect_identifiers(accum),

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                minuend.collect_identifiers(accum);
                subtrahend.collect_identifiers(accum)
            }

            Self::Expression { members, .. } => {
                for pr in members {
                    let p = pr.upgrade().unwrap();
                    accum.insert(p.name.clone());
                }
            }

            Self::ExpressionMember {
                parent_name,
                next_sibling_name,
                operator,
                ..
            } => {
                accum.insert(parent_name.raw());
                if let Some(n) = next_sibling_name {
                    accum.insert(n.raw());
                }
                operator.collect_identifiers(accum);
            }

            Self::Lookahead { expr, lookahead } => {
                expr.collect_identifiers(accum);
                lookahead.collect_identifiers(accum);
            }

            Self::Reference { production } => {
                accum.insert(production.upgrade().unwrap().name.clone());
            }
        }
    }
}

impl Grammar {
    pub fn create_combinator_trees(&mut self) {
        let all_productions = self
            .productions
            .iter()
            .map(|(_, v)| v)
            .flatten()
            .collect::<Vec<_>>();
        for production in &all_productions {
            production.create_combinator_tree(self);
        }
        for production in &all_productions {
            if production.kind == Some(ProductionKind::ExpressionRule) {
                production.transform_expression_members();
            }
        }
    }
}

trait ProductionRefTrait {
    fn create_combinator_tree(&self, grammar: &Grammar);
    fn transform_expression_members(&self);
}

impl ProductionRefTrait for ProductionRef {
    fn create_combinator_tree(&self, grammar: &Grammar) {
        let expression = self.expression_to_generate();
        let root = expression.to_combinator_tree_node(self.as_ref(), grammar);
        let mut index = Cell::new(0);
        let root = root.with_unambiguous_named_types(&mut index);
        *self.combinator_tree.borrow_mut() = CombinatorTreeRoot {
            root,
            production: Rc::downgrade(self),
        };
    }

    fn transform_expression_members(&self) {
        let ct = self.combinator_tree();
        if let CombinatorTree::Expression {
            members: choices, ..
        } = ct.root.as_ref()
        {
            let mut choices = choices.clone();
            choices.reverse();
            let mut next_sibling: Weak<Production> = Weak::new();
            for choice in choices {
                let choice = choice.upgrade().unwrap();
                let name = choice.slang_name();
                let parent_name = self.slang_name();
                let next_sibling_name = next_sibling.upgrade().map(|p| p.slang_name());

                let mut tree = choice.combinator_tree.borrow_mut();

                if let CombinatorTree::Sequence { elements, .. } = tree.root.as_ref() {
                    let left =
                        if let CombinatorTree::Reference { production } = elements[0].1.as_ref() {
                            production.upgrade().unwrap().name == self.name
                        } else {
                            false
                        };

                    let right = if let CombinatorTree::Reference { production } =
                        elements[elements.len() - 1].1.as_ref()
                    {
                        production.upgrade().unwrap().name == self.name
                    } else {
                        false
                    };

                    let (operator, model) = match (&left, &right) {
                        (false, false) => (&elements[..], OperatorModel::None),
                        (false, true) => {
                            (&elements[..elements.len() - 1], OperatorModel::UnaryPrefix)
                        }
                        (true, false) => (&elements[1..], OperatorModel::UnarySuffix),
                        (true, true) => (
                            &elements[1..elements.len() - 1],
                            if choice.expression_to_generate().config.associativity
                                == Some(ExpressionAssociativity::Right)
                            {
                                OperatorModel::BinaryRightAssociative
                            } else {
                                OperatorModel::BinaryLeftAssociative
                            },
                        ),
                    };
                    let operator = if operator.len() == 1 {
                        operator[0].1.clone()
                    } else {
                        ct_sequence(Name::from_string("operator"), operator.into())
                    };

                    tree.root =
                        ct_expression_member(name, parent_name, next_sibling_name, operator, model);
                } else {
                    tree.root = ct_expression_member(
                        name,
                        parent_name,
                        next_sibling_name,
                        tree.root.clone(),
                        OperatorModel::None,
                    );
                }

                next_sibling = Rc::downgrade(&choice);
            }
        }
    }
}

impl Production {
    pub fn slang_name(&self) -> Name {
        Name::from_string(&self.name)
    }

    pub fn combinator_tree(&self) -> std::cell::Ref<'_, CombinatorTreeRoot> {
        self.combinator_tree.borrow()
    }
}

impl Expression {
    fn to_combinator_tree_node(
        &self,
        production: &Production,
        grammar: &Grammar,
    ) -> CombinatorTreeRef {
        if let Some(filter) = self.to_character_filter(grammar) {
            let name = self
                .config
                .slang_name()
                .self_or_else(|| filter.slang_name());
            return ct_character_filter(name, filter);
        } else if let Some(terminal_trie) = self.to_terminal_trie(grammar) {
            let name = self
                .config
                .slang_name()
                .self_or_else(|| terminal_trie.slang_name());
            return ct_terminal_trie(name, terminal_trie);
        } else {
            match &self.ebnf {
                EBNF::Choice(exprs) => {
                    let name = self.config.slang_name();

                    if production.kind == Some(ProductionKind::ExpressionRule) {
                        let choices = exprs.iter().map(|e| {
                            if let EBNF::Reference(prod_name) = &e.ebnf {
                               Rc::downgrade(&grammar.get_production(prod_name))
                            } else {
                                unreachable!("Validation should have checked that pattern: Expression is only aplpied to a choice between references")
                            }
                        }).collect();
                        return ct_expression(production.slang_name(), choices);
                    }

                    // Merge runs of TerminalTrees and CharacterFilters

                    let mut choices: Vec<(Name, CombinatorTreeRef)> = vec![];
                    {
                        let mut current_terminal_tree: Option<TerminalTrie> = None;
                        let mut current_character_filter: Option<CharacterFilter> = None;
                        for e in exprs {
                            // Sub-expressions with a user-specified name aren't merged
                            if e.config.name.is_none() {
                                if let Some(cf) = e.to_character_filter(grammar) {
                                    if let Some(ctt) = current_terminal_tree {
                                        choices.push((
                                            ctt.slang_name(),
                                            ct_terminal_trie(ctt.slang_name(), ctt),
                                        ));
                                        current_terminal_tree = None
                                    };
                                    if let Some(ccf) = current_character_filter {
                                        current_character_filter = Some(ccf.merged_with(cf))
                                    } else {
                                        current_character_filter = Some(cf)
                                    }
                                    continue;
                                }
                                if let Some(tt) = e.to_terminal_trie(grammar) {
                                    if let Some(ccf) = current_character_filter {
                                        choices.push((
                                            ccf.slang_name(),
                                            ct_character_filter(ccf.slang_name(), ccf),
                                        ));
                                        current_character_filter = None
                                    };
                                    if let Some(ctt) = current_terminal_tree.as_mut() {
                                        ctt.merge_with(tt)
                                    } else {
                                        current_terminal_tree = Some(tt)
                                    }
                                    continue;
                                }
                            }

                            if let Some(ccf) = current_character_filter {
                                choices.push((
                                    ccf.slang_name(),
                                    ct_character_filter(ccf.slang_name(), ccf),
                                ));
                                current_character_filter = None
                            };

                            if let Some(ctt) = current_terminal_tree {
                                choices.push((
                                    ctt.slang_name(),
                                    ct_terminal_trie(ctt.slang_name(), ctt),
                                ));
                                current_terminal_tree = None
                            };

                            choices.push({
                                let e = e.to_combinator_tree_node(production, grammar);
                                (e.name(), e)
                            })
                        }

                        if let Some(ccf) = current_character_filter {
                            choices.push((
                                ccf.slang_name(),
                                ct_character_filter(ccf.slang_name(), ccf),
                            ));
                        };

                        if let Some(ctt) = current_terminal_tree {
                            choices
                                .push((ctt.slang_name(), ct_terminal_trie(ctt.slang_name(), ctt)));
                        };
                    }

                    ct_choice(name, choices)
                }

                EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => ct_delimited_by(
                    self.config.slang_name(),
                    open.clone(),
                    expr.to_combinator_tree_node(production, grammar),
                    close.clone(),
                ),

                EBNF::Difference(EBNFDifference {
                    minuend,
                    subtrahend,
                }) => ct_difference(
                    minuend.to_combinator_tree_node(production, grammar),
                    subtrahend.to_combinator_tree_node(production, grammar),
                ),

                EBNF::End => ct_end(),

                EBNF::Not(_) => unimplemented!("¬ is only supported on characters or sets thereof"),

                EBNF::OneOrMore(expr) => ct_one_or_more(
                    self.config.slang_name(),
                    expr.to_combinator_tree_node(production, grammar),
                ),

                EBNF::Optional(expr) => {
                    ct_optional(expr.to_combinator_tree_node(production, grammar))
                }

                EBNF::Range(_) => unreachable!("Ranges are always character filters"),

                EBNF::Reference(name) => ct_reference(Rc::downgrade(&grammar.get_production(name))),

                EBNF::Repeat(EBNFRepeat { expr, min, max }) => ct_repeat(
                    self.config.slang_name(),
                    expr.to_combinator_tree_node(production, grammar),
                    *min,
                    *max,
                ),

                EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => ct_separated_by(
                    self.config.slang_name(),
                    expr.to_combinator_tree_node(production, grammar),
                    separator.clone(),
                ),

                EBNF::Sequence(exprs) => ct_sequence(
                    self.config.slang_name(),
                    exprs
                        .iter()
                        .map(|e| {
                            let e = e.to_combinator_tree_node(production, grammar);
                            (e.name(), e)
                        })
                        .collect(),
                ),

                EBNF::Terminal(_) => {
                    unreachable!("Terminals are either character filters or terminal tries")
                }

                EBNF::ZeroOrMore(expr) => ct_zero_or_more(
                    self.config.slang_name(),
                    expr.to_combinator_tree_node(production, grammar),
                ),
            }
        }
    }
}

// TODO: this should remove disambiguation suffixes *before* checking
// for repeated identifiers.
fn disambiguate_structure_names(mut members: Vec<Name>) -> Vec<Name> {
    // Find all the duplicated names, with the count of their occurance
    let mut names = MultiSet::<Name>::from_iter(members.iter().cloned());
    names.retain(|_, count| count > 1);
    if names.is_empty() {
        members
    } else {
        // Reverse so that the suffix goes from _0 .. _n when we re-reverse the list
        members.reverse();
        members = members
            .into_iter()
            .map(|n| {
                if let Some(count) = names.get(&n) {
                    // Remove the element to decrement the occurance occount
                    names.remove(&n);
                    n.with_disambiguating_suffix(count - 1)
                } else {
                    n
                }
            })
            .collect::<Vec<_>>();
        members.reverse();
        members
    }
}
