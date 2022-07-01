use std::{
    cell::Cell,
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

impl CombinatorTreeRoot {
    pub fn production(&self) -> ProductionRef {
        self.production.upgrade().unwrap()
    }

    pub fn slang_name(&self) -> Name {
        self.production().slang_name()
    }
}

#[derive(Clone, Debug)]
pub enum CombinatorTree {
    Difference {
        minuend: CombinatorTreeRef,
        subtrahend: CombinatorTreeRef,
    },
    Lookahead {
        expr: CombinatorTreeRef,
        lookahead: CombinatorTreeRef,
    },
    Sequence {
        name: Name,
        elements: Vec<NamedCombinatorTreeRef>,
    },
    Choice {
        name: Name,
        choices: Vec<NamedCombinatorTreeRef>,
    },
    Optional {
        expr: CombinatorTreeRef,
    },
    SeparatedBy {
        name: Name,
        expr: CombinatorTreeRef,
        min: usize, // > 0
        max: Option<usize>,
        separator: CombinatorTreeRef,
    },
    Repeated {
        name: Name,
        expr: CombinatorTreeRef,
        min: usize,
        max: Option<usize>,
    },
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
    Reference {
        production: ProductionWeakRef,
    },
    TerminalTrie {
        name: Name,
        trie: TerminalTrie,
        with_noise: bool,
    },
    CharacterFilter {
        name: Name,
        filter: CharacterFilter,
        with_noise: bool,
    },
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorModel {
    None,
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub fn ct_difference(
    minuend: CombinatorTreeRef,
    subtrahend: CombinatorTreeRef,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Difference {
        minuend,
        subtrahend,
    })
}

fn ct_lookahead(expr: CombinatorTreeRef, lookahead: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Lookahead { expr, lookahead })
}

fn ct_choice(name: Name, choices: Vec<(Name, CombinatorTreeRef)>) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Choice { name, choices })
}

fn ct_sequence(name: Name, elements: Vec<(Name, CombinatorTreeRef)>) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Sequence { name, elements })
}

fn ct_optional(expr: CombinatorTreeRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Optional { expr })
}

fn ct_separated_by(
    name: Name,
    expr: CombinatorTreeRef,
    min: usize,
    max: Option<usize>,
    separator: CombinatorTreeRef,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::SeparatedBy {
        name,
        expr,
        min,
        max,
        separator,
    })
}

fn ct_repeat(
    name: Name,
    expr: CombinatorTreeRef,
    min: usize,
    max: Option<usize>,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Repeated {
        name,
        expr,
        min,
        max,
    })
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

fn ct_reference(production: ProductionWeakRef) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Reference { production })
}

fn ct_terminal_trie(name: Name, trie: TerminalTrie, with_noise: bool) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::TerminalTrie {
        name,
        trie,
        with_noise,
    })
}

fn ct_character_filter(name: Name, filter: CharacterFilter, with_noise: bool) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::CharacterFilter {
        name,
        filter,
        with_noise,
    })
}

fn ct_end() -> CombinatorTreeRef {
    Rc::new(CombinatorTree::End)
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
    pub fn to_generated_code(&self, is_recursive: bool) -> ProductionGeneratedCode {
        let GeneratedCode {
            parser_type,
            parser,
            tree_interface,
            tree_implementation,
        } = self.root.to_generated_code(self);

        let module_name = self.slang_name().to_module_name_ident();
        let type_name = self.slang_name().to_type_name_ident();
        let parser_name = self.slang_name().to_parser_name_ident();
        let field_name = self.slang_name().to_field_name_ident();

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
            parser_field_initialization: quote!( #field_name: #parser_name ),
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
            CombinatorTree::Difference {
                minuend,
                subtrahend,
            } => ct_difference(
                minuend.with_unambiguous_named_types(index),
                subtrahend.with_unambiguous_named_types(index),
            ),
            CombinatorTree::Lookahead { expr, lookahead } => ct_lookahead(
                expr.with_unambiguous_named_types(index),
                lookahead.with_unambiguous_named_types(index),
            ),
            CombinatorTree::Choice { name, choices } => {
                let name = name.clone().self_or_numbered(index);
                let choices = disambiguate_structure_names(
                    choices
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.clone().self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect(),
                );
                ct_choice(name, choices)
            }
            CombinatorTree::Sequence { name, elements } => {
                let name = name.clone().self_or_numbered(index);
                let elements = disambiguate_structure_names(
                    elements
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.clone().self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect(),
                );
                ct_sequence(name, elements)
            }
            CombinatorTree::Optional { expr } => {
                ct_optional(expr.with_unambiguous_named_types(index))
            }
            CombinatorTree::SeparatedBy {
                name,
                expr,
                min,
                max,
                separator,
            } => {
                let name = name.clone().self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                let separator = separator.with_unambiguous_named_types(index);
                ct_separated_by(name, expr, *min, *max, separator)
            }
            CombinatorTree::Repeated {
                name,
                expr,
                min,
                max,
            } => {
                let name = name.clone().self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                ct_repeat(name, expr, *min, *max)
            }
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
            CombinatorTree::Expression { .. }
            | CombinatorTree::Reference { .. }
            | CombinatorTree::TerminalTrie { .. }
            | CombinatorTree::CharacterFilter { .. } => self.clone(),
            CombinatorTree::End => ct_end(),
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
            Self::TerminalTrie { name, .. }
            | Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::Expression { name, .. }
            | Self::ExpressionMember { name, .. }
            | Self::Sequence { name, .. } => name.clone(),
            Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::Optional { expr } => expr.name(),
            Self::SeparatedBy { expr, .. } | Self::Repeated { expr, .. } => expr.name().pluralize(),
            Self::Reference { production } => production.upgrade().unwrap().slang_name(),
            Self::End => Name::from_string("end_marker"),
        }
    }

    fn to_generated_code(&self, tree: &CombinatorTreeRoot) -> GeneratedCode {
        match self {
            CombinatorTree::Difference {
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

            CombinatorTree::Lookahead { expr, lookahead } => {
                let mut expr = expr.to_generated_code(tree);
                let lookahead = lookahead.to_generated_code(tree);

                let expr_parser = expr.parser;
                let lookahead_parser = lookahead.parser;
                expr.parser = quote!( #expr_parser.then_ignore( #lookahead_parser.rewind() ));

                expr
            }

            CombinatorTree::Sequence { name, elements } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let mut field_names = vec![];
                let mut field_types = vec![];
                let mut parser_chain = None;
                for (name, element) in elements {
                    result.merge(element.to_generated_code(tree));

                    let name = name.to_field_name_ident();
                    field_names.push(quote!( #name ));

                    let parser_type = result.parser_type.clone();
                    field_types.push(quote!( #parser_type ));

                    let next_parser = result.parser.clone();
                    parser_chain = parser_chain
                        .and_then(|p| Some(quote!( #p.then(#next_parser) )))
                        .or_else(|| Some(next_parser))
                }

                result
                    .tree_interface
                    .push(quote!( pub struct #type_name { #(pub #field_names: #field_types),* } ));

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
                        pub fn new(#folded_field_names: #folded_field_types) -> Self {
                            Self { #(#field_names),* }
                        }
                    }
                ));
                // TODO: serde
                // TODO: default

                let parser = parser_chain.unwrap();
                result.parser =
                    quote!( #parser.map(|v| Box::new(#module_name::#type_name::new(v))) );
                result.parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            CombinatorTree::Choice { name, choices } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let mut fields = vec![];
                let mut parsers = vec![];
                for (name, element) in choices {
                    result.merge(element.to_generated_code(tree));

                    let name = name.to_enum_tag_ident();
                    let parser_type = result.parser_type.clone();
                    fields.push(quote!( #name(#parser_type) ));

                    let parser = result.parser.clone();
                    parsers.push(
                        quote!( #parser.map(|v| Box::new(#module_name::#type_name::#name(v))) ),
                    );
                }

                result
                    .tree_interface
                    .push(quote!( pub enum #type_name { #(#fields),* } ));

                result.parser = quote!( choice(( #(#parsers),* )) );
                result.parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            CombinatorTree::Optional { expr } => {
                let mut result = expr.to_generated_code(tree);

                let parser = result.parser;
                result.parser = quote!( #parser.or_not() );

                let parser_type = result.parser_type;
                result.parser_type = quote!( Option<#parser_type> );

                result
            }

            CombinatorTree::SeparatedBy {
                name,
                expr,
                separator,
                min,
                max,
            } => {
                let mut result: GeneratedCode = Default::default();

                let module_name = tree.slang_name().to_module_name_ident();
                let type_name = name.to_type_name_ident();

                let expr = expr.to_generated_code(tree);
                let separator = separator.to_generated_code(tree);

                let expr_parser_type = expr.parser_type.clone();
                let separator_parser_type = separator.parser_type.clone();
                let expr_parser = expr.parser.clone();
                let separator_parser = separator.parser.clone();

                result.merge(expr);
                result.merge(separator);

                let repetition = quote!(#separator_parser.then(#expr_parser).repeated());
                let mapping =
                    quote!( .map(repetition_mapper).map(|v| #module_name::#type_name::new(v)) );
                result.parser = match (min, max) {
                    (0, None) => {
                        quote!( #expr_parser.then(#repetition)#mapping.or_not() )
                    }
                    (0, Some(max)) => {
                        quote!( #expr_parser.then(#repetition.at_most(#max - 1))#mapping.or_not() )
                    }
                    (1, None) => {
                        quote!( #expr_parser.then(#repetition)#mapping )
                    }
                    (1, Some(max)) => {
                        quote!( #expr_parser.then(#repetition.at_most(#max - 1))#mapping )
                    }
                    (min, None) => {
                        quote!( #expr_parser.then(#repetition.at_least(#min - 1))#mapping )
                    }
                    (min, Some(max)) if min == max => {
                        quote!( #expr_parser.then(#repetition.exactly(#min - 1))#mapping )
                    }
                    (min, Some(max)) => {
                        quote!( #expr_parser.then(#repetition.at_least(#min - 1).at_most(#max - 1))#mapping )
                    }
                };

                result.tree_interface.push({
                    quote!(
                        pub struct #type_name {
                            pub elements: Vec<#expr_parser_type>,
                            pub separators: Vec<#separator_parser_type>
                    })
                });

                result.parser_type = if *min == 0 {
                    quote!( Option<#module_name::#type_name> )
                } else {
                    quote!( #module_name::#type_name )
                };

                result
            }

            CombinatorTree::Repeated { expr, min, max, .. } => {
                let mut result = expr.to_generated_code(tree);

                let mut parser = result.parser;
                parser = quote!( #parser.repeated() );
                match (min, max) {
                    (0, None) => {}
                    (0, Some(max)) => parser = quote!( #parser.at_most(#max) ),
                    (min, None) => parser = quote!( #parser.at_least(#min) ),
                    (min, Some(max)) if min == max => parser = quote!( #parser.exactly(#min) ),
                    (min, Some(max)) => parser = quote!( #parser.at_least(#min).at_most(#max) ),
                };

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> usize
                    parser = quote!( #parser.map(|v| v.len()) );
                    result.parser_type = quote!(usize);
                } else {
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( Vec<#parser_type> );
                };

                result.parser = parser;

                result
            }

            CombinatorTree::Expression { name, members } => {
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
                result
                    .tree_interface
                    .push(quote!( pub enum #type_name { #(#fields),* } ));

                let first_parser_name = names[0].to_parser_name_ident();
                result.parser = quote!( #first_parser_name );
                result.parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            CombinatorTree::ExpressionMember {
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

                match operator_model {
                    OperatorModel::None => {
                        result.parser = match next_sibling_name {
                            Some(next_sibling_name) => {
                                let next_sibling_parser_name =
                                    next_sibling_name.to_parser_name_ident();
                                quote!(
                                    choice((
                                        #operator_parser.map(#parent_type_name::#tag_name),
                                        #next_sibling_parser_name
                                    ))
                                )
                            }
                            None => quote!( #operator_parser.map(#parent_type_name::#tag_name) ),
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
                        result.tree_interface.push(quote!(
                            pub struct E {
                                pub left: #parent_type_name,
                                pub operator: #operator_type,
                                pub right: #parent_type_name,
                            }
                        ));

                        result.parser = quote!(
                           #next_sibling_parser_name
                            .then(#operator_parser.then(#next_sibling_parser_name).repeated())
                            .map(|(next, pairs)|
                                if pairs.is_empty {
                                    next
                                } else {
                                    pairs.iter().fold(next, |left, (operator, right)|
                                        #parent_type_name::#tag_name(#module_name::E { left, operator, right })
                                    )
                                }
                            )
                        );
                    }

                    OperatorModel::BinaryRightAssociative => todo!(),

                    OperatorModel::UnaryPrefix => {
                        let next_sibling_parser_name = next_sibling_name
                            .clone()
                            .expect("Cannot have unary prefix operator as last expression member")
                            .to_parser_name_ident();
                        result.tree_interface.push(quote!(
                            pub struct E {
                                pub operator: #operator_type,
                                pub right: #parent_type_name,
                            }
                        ));

                        result.parser = quote!(
                            #operator_parser.repeated()
                            .then(#next_sibling_parser_name)
                            .map(|(mut operators, operand)|
                                if operators.is_empty {
                                    operand
                                } else {
                                    operators.reverse();
                                    operators.iter().fold(operand, |right, operator|
                                        #parent_type_name::#tag_name(#module_name::E { operator, right })
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
                        result.tree_interface.push(quote!(
                            pub struct E {
                                pub left: #parent_type_name,
                                pub operator: #operator_type,
                            }
                        ));

                        result.parser = quote!(
                            #next_sibling_parser_name
                            .then(#operator_parser.repeated())
                            .map(|(operand, operators)|
                                if operators.is_empty {
                                    operand
                                } else {
                                    operators.iter().fold(operand, |left, operator|
                                        #parent_type_name::#tag_name(#module_name::E { left, operator })
                                    )
                                }
                            )
                        );
                    }
                };

                result.parser_type = quote!( #parent_type_name );

                result
            }

            CombinatorTree::Reference { production } => {
                let mut result: GeneratedCode = Default::default();

                let name = production.upgrade().unwrap().slang_name();

                let parser_name = name.to_parser_name_ident();
                result.parser = quote!( #parser_name.clone() );

                let type_name = name.to_type_name_ident();
                result.parser_type = quote!( #type_name );

                result
            }

            CombinatorTree::TerminalTrie {
                trie, with_noise, ..
            } => {
                let mut result = trie.to_generated_code();

                if *with_noise {
                    let parser = result.parser;
                    result.parser = quote!( with_noise(#parser) );
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( WithNoise<#parser_type> )
                }

                result
            }

            CombinatorTree::CharacterFilter {
                filter, with_noise, ..
            } => {
                let mut result = filter.to_generated_code();

                if *with_noise {
                    let parser = result.parser;
                    result.parser = quote!( with_noise(#parser) );
                    let parser_type = result.parser_type;
                    result.parser_type = quote!( WithNoise<#parser_type> )
                }

                result
            }

            CombinatorTree::End => {
                let mut result: GeneratedCode = Default::default();
                result.parser = quote!(end());
                result.parser_type = quote!(());
                result
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
            if production.pattern == Some(ProductionPattern::Expression) {
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
                            OperatorModel::BinaryLeftAssociative,
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
            return ct_character_filter(name, filter, !production.is_token());
        } else if let Some(terminal_trie) = self.to_terminal_trie(grammar) {
            let name = self
                .config
                .slang_name()
                .self_or_else(|| terminal_trie.slang_name());
            return ct_terminal_trie(name, terminal_trie, !production.is_token());
        } else {
            match &self.ebnf {
                EBNF::End => ct_end(),
                EBNF::Difference(EBNFDifference {
                    minuend,
                    subtrahend,
                }) => ct_difference(
                    minuend.to_combinator_tree_node(production, grammar),
                    subtrahend.to_combinator_tree_node(production, grammar),
                ),
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min: 0,
                    max: Some(1),
                    ..
                }) => {
                    let et = expr.to_combinator_tree_node(production, grammar);
                    ct_optional(et)
                }
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max,
                    separator: None,
                }) => {
                    let name = self.config.slang_name();
                    let et = expr.to_combinator_tree_node(production, grammar);
                    ct_repeat(name, et, *min, *max)
                }
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max,
                    separator: Some(separator),
                }) => {
                    let name = self.config.slang_name();
                    let et = expr.to_combinator_tree_node(production, grammar);
                    let st = separator.to_combinator_tree_node(production, grammar);
                    ct_separated_by(name, et, *min, *max, st)
                }
                EBNF::Choice(exprs) => {
                    let name = production.slang_name();

                    if production.pattern == Some(ProductionPattern::Expression) {
                        let choices = exprs.iter().map(|e| {
                            if let EBNF::Reference(prod_name) = &e.ebnf {
                               Rc::downgrade(&grammar.get_production(prod_name))
                            } else {
                                unreachable!("Validation should have checked that pattern: Expression is only aplpied to a choice between references")
                            }
                        }).collect();
                        return ct_expression(name, choices);
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
                                            ct_terminal_trie(
                                                ctt.slang_name(),
                                                ctt,
                                                !production.is_token(),
                                            ),
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
                                            ct_character_filter(
                                                ccf.slang_name(),
                                                ccf,
                                                !production.is_token(),
                                            ),
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
                                    ct_character_filter(
                                        ccf.slang_name(),
                                        ccf,
                                        !production.is_token(),
                                    ),
                                ));
                                current_character_filter = None
                            };

                            if let Some(ctt) = current_terminal_tree {
                                choices.push((
                                    ctt.slang_name(),
                                    ct_terminal_trie(ctt.slang_name(), ctt, !production.is_token()),
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
                                ct_character_filter(ccf.slang_name(), ccf, !production.is_token()),
                            ));
                        };

                        if let Some(ctt) = current_terminal_tree {
                            choices.push((
                                ctt.slang_name(),
                                ct_terminal_trie(ctt.slang_name(), ctt, !production.is_token()),
                            ));
                        };
                    }

                    ct_choice(name, choices)
                }
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
                EBNF::Reference(name) => ct_reference(Rc::downgrade(&grammar.get_production(name))),

                EBNF::Not(_) => unimplemented!("¬ is only supported on characters or sets thereof"),
                EBNF::Terminal(_) => {
                    unreachable!("Terminals are either character filters or terminal tries")
                }
                EBNF::Range(_) => unreachable!("Ranges are always character filters"),
            }
        }
    }
}

// TODO: this should remove disambiguation suffixes *before* checking
// for repeated identifiers.
fn disambiguate_structure_names(
    mut members: Vec<(Name, CombinatorTreeRef)>,
) -> Vec<(Name, CombinatorTreeRef)> {
    // Find all the duplicated names, with the count of their occurance
    let mut names = MultiSet::<Name>::from_iter(members.iter().map(|(n, _)| n.clone()));
    names.retain(|_, count| count > 1);
    if names.is_empty() {
        members
    } else {
        // Reverse so that the suffix goes from _0 .. _n when we re-reverse the list
        members.reverse();
        members = members
            .into_iter()
            .map(|(n, t)| {
                if let Some(count) = names.get(&n) {
                    // Remove the element to decrement the occurance occount
                    names.remove(&n);
                    (n.with_disambiguating_suffix(count - 1), t)
                } else {
                    (n, t)
                }
            })
            .collect::<Vec<_>>();
        members.reverse();
        members
    }
}
