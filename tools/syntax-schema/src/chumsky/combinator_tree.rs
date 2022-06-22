use std::{cell::Cell, rc::Rc};

use itertools::Itertools;
use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::*;

use super::{
    character_filter::CharacterFilter, slang_name::SlangName, terminal_trie::TerminalTrie,
};

#[derive(Clone, Debug)]
pub struct CombinatorTree {
    pub root: CombinatorTreeNode,
    pub production: ProductionWeakRef,
    pub module_name: SlangName,
}

type CombinatorTreeNode = Box<CombinatorTreeNodeData>;

#[derive(Clone, Debug)]
pub enum CombinatorTreeNodeData {
    Difference {
        // -> M
        minuend: CombinatorTreeNode,
        subtrahend: CombinatorTreeNode,
    },
    Lookahead {
        // -> E
        expr: CombinatorTreeNode,
        lookahead: CombinatorTreeNode,
    },
    Choice {
        // N::N0(E0) | N:N1(E1) ..
        name: SlangName, // Assigned when created
        choices: Vec<(SlangName, CombinatorTreeNode)>,
    },
    Sequence {
        // -> Vec<E>
        name: SlangName, // Assigned when created
        elements: Vec<(SlangName, CombinatorTreeNode)>,
    },
    PairOrPassthrough {
        // -> E || N(E, O) || N(O, E)
        name: SlangName,
        expr: CombinatorTreeNode,
        optional: CombinatorTreeNode,
        direction: Direction,
    },
    FoldOrPassthrough {
        // -> E | N(E, S, N) RIGHT | N(N, S, E) LEFT
        name: SlangName,
        expr: CombinatorTreeNode,
        separator: CombinatorTreeNode,
        direction: Direction,
    },
    Optional {
        // -> Option<E>
        expr: CombinatorTreeNode,
    },
    SeparatedBy {
        // -> (Vec<E>, Vec<S>)
        name: SlangName, // Assigned when created
        expr: CombinatorTreeNode,
        min: usize, // > 0
        max: Option<usize>,
        separator: CombinatorTreeNode,
    },
    Repeat {
        // -> Vec<E>
        name: SlangName, // Assigned when created
        expr: CombinatorTreeNode,
        min: usize,
        max: Option<usize>,
    },
    Reference {
        production: ProductionWeakRef,
    },
    TerminalTrie {
        // -> Fixed<n> || usize
        name: SlangName,
        trie: TerminalTrie,
    },
    CharacterFilter {
        // -> Fixed<1>
        name: SlangName,
        filter: CharacterFilter,
    },
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
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

fn ct_lookahead(expr: CombinatorTreeNode, lookahead: CombinatorTreeNode) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Lookahead { expr, lookahead })
}

fn ct_choice(name: SlangName, choices: Vec<(SlangName, CombinatorTreeNode)>) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Choice { name, choices })
}

fn ct_sequence(
    name: SlangName,
    elements: Vec<(SlangName, CombinatorTreeNode)>,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Sequence { name, elements })
}

fn ct_optional(expr: CombinatorTreeNode) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Optional { expr })
}

fn ct_pair_or_passthrough(
    name: SlangName,
    expr: CombinatorTreeNode,
    optional: CombinatorTreeNode,
    direction: Direction,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::PairOrPassthrough {
        name,
        expr,
        optional,
        direction,
    })
}

fn ct_fold_or_passthrough(
    name: SlangName,
    expr: CombinatorTreeNode,
    separator: CombinatorTreeNode,
    direction: Direction,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::FoldOrPassthrough {
        name,
        expr,
        separator,
        direction,
    })
}

fn ct_separated_by(
    name: SlangName,
    expr: CombinatorTreeNode,
    min: usize,
    max: Option<usize>,
    separator: CombinatorTreeNode,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::SeparatedBy {
        name,
        expr,
        min,
        max,
        separator,
    })
}

fn ct_repeat(
    name: SlangName,
    expr: CombinatorTreeNode,
    min: usize,
    max: Option<usize>,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Repeat {
        name,
        expr,
        min,
        max,
    })
}

fn ct_reference(production: ProductionWeakRef) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Reference { production })
}

fn ct_terminal_trie(name: SlangName, trie: TerminalTrie) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::TerminalTrie { name, trie })
}

fn ct_character_filter(name: SlangName, filter: CharacterFilter) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::CharacterFilter { name, filter })
}

fn ct_end() -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::End)
}

impl CombinatorTree {
    pub fn to_parser_combinator_code(&self) -> TokenStream {
        let production = self.production.upgrade().unwrap();
        if production.pattern == Some(ProductionPattern::Expression) {
            self.root.to_expression_parser_combinator_code(self)
        } else {
            self.root.to_default_parser_combinator_code(self)
        }
    }
}

impl Default for CombinatorTree {
    fn default() -> Self {
        Self {
            root: ct_end(),
            production: Default::default(),
            module_name: SlangName::from_string("__uninitialized__"),
        }
    }
}

impl CombinatorTreeNodeData {
    // TODO: generic tree transformer?
    fn with_unambiguous_named_types(self, index: &mut Cell<usize>) -> CombinatorTreeNode {
        match self {
            CombinatorTreeNodeData::Difference {
                minuend,
                subtrahend,
            } => ct_difference(
                minuend.with_unambiguous_named_types(index),
                subtrahend.with_unambiguous_named_types(index),
            ),
            CombinatorTreeNodeData::Lookahead { expr, lookahead } => ct_lookahead(
                expr.with_unambiguous_named_types(index),
                lookahead.with_unambiguous_named_types(index),
            ),
            CombinatorTreeNodeData::Choice { name, choices } => {
                let name = name.self_or_numbered(index);
                let choices = disambiguate_structure_names(
                    choices
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect(),
                );
                ct_choice(name, choices)
            }
            CombinatorTreeNodeData::Sequence { name, elements } => {
                let name = name.self_or_numbered(index);
                let elements = disambiguate_structure_names(
                    elements
                        .into_iter()
                        .enumerate()
                        .map(|(i, (n, e))| {
                            let e = e.with_unambiguous_named_types(index);
                            let n = n.self_or_else(|| e.name()).self_or_positional(i);
                            (n, e)
                        })
                        .collect(),
                );
                ct_sequence(name, elements)
            }
            CombinatorTreeNodeData::Optional { expr } => {
                ct_optional(expr.with_unambiguous_named_types(index))
            }
            CombinatorTreeNodeData::PairOrPassthrough {
                name,
                expr,
                optional,
                direction,
            } => {
                let name = name.self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                let optional = optional.with_unambiguous_named_types(index);
                ct_pair_or_passthrough(name, expr, optional, direction)
            }
            CombinatorTreeNodeData::FoldOrPassthrough {
                name,
                expr,
                separator,
                direction,
            } => {
                let name = name.self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                let separator = separator.with_unambiguous_named_types(index);
                ct_fold_or_passthrough(name, expr, separator, direction)
            }
            CombinatorTreeNodeData::SeparatedBy {
                name,
                expr,
                min,
                max,
                separator,
            } => {
                let name = name.self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                let separator = separator.with_unambiguous_named_types(index);
                ct_separated_by(name, expr, min, max, separator)
            }
            CombinatorTreeNodeData::Repeat {
                name,
                expr,
                min,
                max,
            } => {
                let name = name.self_or_numbered(index);
                let expr = expr.with_unambiguous_named_types(index);
                ct_repeat(name, expr, min, max)
            }
            CombinatorTreeNodeData::Reference { production } => ct_reference(production.clone()),
            CombinatorTreeNodeData::TerminalTrie { .. } => Box::new(self),
            CombinatorTreeNodeData::CharacterFilter { .. } => Box::new(self),
            CombinatorTreeNodeData::End => ct_end(),
        }
    }

    // TODO: generic tree transformer?
    fn with_interspersed(self, production: &ProductionRef) -> CombinatorTreeNode {
        match self {
            CombinatorTreeNodeData::Difference { .. }
            | CombinatorTreeNodeData::Lookahead { .. }
            | CombinatorTreeNodeData::Choice { .. }
            | CombinatorTreeNodeData::Optional { .. }
            | CombinatorTreeNodeData::PairOrPassthrough { .. }
            | CombinatorTreeNodeData::FoldOrPassthrough { .. }
            | CombinatorTreeNodeData::SeparatedBy { .. }
            | CombinatorTreeNodeData::Repeat { .. }
            | CombinatorTreeNodeData::Reference { .. }
            | CombinatorTreeNodeData::TerminalTrie { .. }
            | CombinatorTreeNodeData::CharacterFilter { .. }
            | CombinatorTreeNodeData::End => Box::new(self),
            // TODO: inject the production into the sequence elements
            CombinatorTreeNodeData::Sequence { name, elements } => ct_sequence(
                name,
                #[allow(unstable_name_collisions)]
                elements
                    .into_iter()
                    .intersperse_with(|| {
                        (
                            production.slang_name(),
                            ct_reference(Rc::downgrade(production)),
                        )
                    })
                    .collect(),
            ),
        }
    }

    pub fn name(&self) -> SlangName {
        match self {
            CombinatorTreeNodeData::TerminalTrie { name, .. }
            | CombinatorTreeNodeData::CharacterFilter { name, .. }
            | CombinatorTreeNodeData::Choice { name, .. }
            | CombinatorTreeNodeData::Sequence { name, .. } => name.clone(),
            CombinatorTreeNodeData::Difference { minuend: expr, .. }
            | CombinatorTreeNodeData::Lookahead { expr, .. }
            | CombinatorTreeNodeData::Optional { expr } => expr.name(),
            CombinatorTreeNodeData::PairOrPassthrough { expr, .. }
            | CombinatorTreeNodeData::FoldOrPassthrough { expr, .. }
            | CombinatorTreeNodeData::SeparatedBy { expr, .. }
            | CombinatorTreeNodeData::Repeat { expr, .. } => expr.name().pluralize(),
            CombinatorTreeNodeData::Reference { production } => {
                production.upgrade().unwrap().slang_name()
            }
            CombinatorTreeNodeData::End => SlangName::from_string("end_marker"),
        }
    }

    fn to_expression_parser_combinator_code(&self, tree: &CombinatorTree) -> TokenStream {
        if let Self::Choice { name: _, choices } = self {
            // Each choice is it's own production, with the ref(root) changing to
            // reference the next choice.

            let mut names = (0..choices.len())
                .map(|i| format_ident!("choice_{}", i))
                .collect::<Vec<_>>();
            let mut exprs = choices
                .iter()
                .enumerate()
                .map(|(i, (_, e))| match e.as_ref() {
                    CombinatorTreeNodeData::Sequence { name: _, elements } if elements.len() == 2 => {
                        match (elements[0].1.as_ref(), elements[1].1.as_ref()) {
                            (
                                Self::Optional {
                                    expr: optional,
                                },
                                Self::Reference{ production },
                            )  if production.upgrade().unwrap().name == tree.production.upgrade().unwrap().name => {
                                let optional = optional.to_default_parser_combinator_code(tree);
                                let previous_choice_name = format_ident!("choice_{}", i + 1);
                                quote!( #optional.repeated().then(#previous_choice_name) )
                            }
                            (
                                Self::Reference{production},
                                Self::Optional {
                                    expr: optional,
                                }
                            )  if production.upgrade().unwrap().name == tree.production.upgrade().unwrap().name => {
                                let optional = optional.to_default_parser_combinator_code(tree);
                                let previous_choice_name = format_ident!("choice_{}", i + 1);
                                quote!( #previous_choice_name.then(#optional.repeated()) )
                            }
                            _ => e.to_default_parser_combinator_code(tree)
                        }
                    }
                    CombinatorTreeNodeData::SeparatedBy {
                        name: _,
                        expr,
                        min: 1,
                        max: None,
                        separator,
                    } => match expr.as_ref() {
                        Self::Reference { production }
                            if production.upgrade().unwrap().name
                                == tree.production.upgrade().unwrap().name =>
                        {
                            let separator = separator.to_default_parser_combinator_code(tree);
                            let previous_choice_name = format_ident!("choice_{}", i + 1);
                            quote!( #previous_choice_name.then(#separator.then(#previous_choice_name).repeated()) )
                        }
                        _ => e.to_default_parser_combinator_code(tree),
                    },
                    _ => e.to_default_parser_combinator_code(tree),
                }).collect::<Vec<_>>();

            names.reverse();
            exprs.reverse();
            let first_name = format_ident!("choice_0");

            quote!(
                {
                    #(let #names = #exprs.boxed();)*
                    #first_name
                }
            )

            // ForAll choices:
            // [ prefix... ref(root) ] -> X ( + X::N(prefix..., ref) ) NB: prefix can be repeated
            // [ ref(root) suffix... ] -> X ( + X::N(ref, ...suffix) ) NB: suffix can be repeated
            // 1…*{ ref(root) / S } -> X ( + X::N(left: ref, ...S, right: ref) )
            // other -> X::N(other)
        } else {
            panic!("The 'Expression' pattern can only be used with a choice production")
        }
    }

    fn to_default_parser_combinator_code(&self, tree: &CombinatorTree) -> TokenStream {
        self.to_parser_combinator_code(tree, |node, tree| {
            node.to_default_parser_combinator_code(tree)
        })
    }

    // This is a generic tree walker
    fn to_parser_combinator_code<F>(&self, tree: &CombinatorTree, visitor: F) -> TokenStream
    where
        F: Fn(&CombinatorTreeNode, &CombinatorTree) -> TokenStream,
    {
        match self {
            CombinatorTreeNodeData::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = visitor(minuend, tree);
                let subtrahend = visitor(subtrahend, tree);
                quote! ( difference(#minuend, #subtrahend) )
            }
            CombinatorTreeNodeData::Lookahead { expr, lookahead } => {
                let expr = visitor(expr, tree);
                let lookahead = visitor(lookahead, tree);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
            CombinatorTreeNodeData::Choice { choices, name } => {
                let module_name = tree.module_name.to_module_name_ident();
                let choice_name = name.to_type_name_ident();
                let choices = choices.iter().map(|(n, c)| {
                    let constructor = n.to_enum_tag_ident();
                    let expr = visitor(c, tree);
                    quote!( #expr.map(|v| Box::new(#module_name::#choice_name::#constructor(v))) )
                });
                quote!( choice(( #(#choices),* )) )
            }
            CombinatorTreeNodeData::Sequence { elements, name } => {
                let struct_name = name.to_type_name_ident();
                let mut elements = elements.iter().map(|(_, e)| visitor(e, tree));
                let first = elements.next().unwrap();
                let rest = elements.map(|e| quote!( .then(#e) ));
                let module_name = tree.module_name.to_module_name_ident();
                quote!( #first #(#rest)* .map(|v| Box::new(#module_name::#struct_name::new(v))) )
            }
            CombinatorTreeNodeData::Optional { expr } => {
                let expr = visitor(expr, tree);
                quote!( #expr.or_not() )
            }
            CombinatorTreeNodeData::Repeat { expr, min, max, .. } => {
                let vec_to_length = if let Self::CharacterFilter { .. } = **expr {
                    // Vec<()>
                    quote!( .map(|v| v.len()) )
                } else {
                    quote!()
                };

                let expr = visitor(expr, tree);

                match (min, max) {
                    (0, None) => quote!( #expr.repeated()#vec_to_length ),
                    (0, Some(max)) => quote!( #expr.repeated().at_most(#max)#vec_to_length),
                    (min, None) => quote!( #expr.repeated().at_least(#min)#vec_to_length ),
                    (min, Some(max)) if min == max => {
                        quote!( #expr.repeated().exactly(#min)#vec_to_length )
                    }
                    (min, Some(max)) => {
                        quote!( #expr.repeated().at_least(#min).at_most(#max)#vec_to_length )
                    }
                }
            }
            CombinatorTreeNodeData::PairOrPassthrough {
                name,
                expr,
                optional,
                direction,
            } => {
                let expr = visitor(expr, tree);
                let optional = visitor(optional, tree);

                let module_name = tree.module_name.to_module_name_ident();
                let struct_name = name.to_type_name_ident();

                let map = quote!( .map(|v| Box::new(#module_name::#struct_name::new(v))) );
                if *direction == Direction::Left {
                    quote!( #expr.then(#optional.or_not())#map )
                } else {
                    quote!( #optional.or_not.then(#expr)#map )
                }
            }
            CombinatorTreeNodeData::FoldOrPassthrough {
                name,
                expr,
                separator,
                direction: _,
            } => {
                let expr = visitor(expr, tree);
                let separator = visitor(separator, tree);

                let module_name = tree.module_name.to_module_name_ident();
                let struct_name = name.to_type_name_ident();

                quote!( #expr.then(#separator.then(#expr).repeated()).map(repetition_mapper).map(|v| Box::new(#module_name::#struct_name::new(v)))  )
            }
            CombinatorTreeNodeData::SeparatedBy {
                name,
                expr,
                min,
                max,
                separator,
            } => {
                let expr = visitor(expr, tree);
                let separator = visitor(separator, tree);

                let mapping = {
                    let module_name = tree.module_name.to_module_name_ident();
                    let struct_name = name.to_type_name_ident();
                    quote!( .map(repetition_mapper).map(|v| Box::new(#module_name::#struct_name::new(v))) )
                };

                let repetition = quote!(#separator.then(#expr).repeated());

                match (min, max) {
                    (0, None) => {
                        quote!( #expr.then(#repetition)#mapping.or_not() )
                    }
                    (0, Some(max)) => {
                        quote!( #expr.then(#repetition.at_most(#max - 1))#mapping.or_not() )
                    }
                    (1, None) => {
                        quote!( #expr.then(#repetition)#mapping )
                    }
                    (1, Some(max)) => {
                        quote!( #expr.then(#repetition.at_most(#max - 1))#mapping )
                    }
                    (min, None) => {
                        quote!( #expr.then(#repetition.at_least(#min - 1))#mapping )
                    }
                    (min, Some(max)) if min == max => {
                        quote!( #expr.then(#repetition.exactly(#min - 1))#mapping )
                    }
                    (min, Some(max)) => {
                        quote!( #expr.then(#repetition.at_least(#min - 1).at_most(#max - 1))#mapping )
                    }
                }
            }
            CombinatorTreeNodeData::Reference { production } => {
                let name = production
                    .upgrade()
                    .unwrap()
                    .slang_name()
                    .to_parser_name_ident();
                quote!( #name.clone() )
            }
            CombinatorTreeNodeData::TerminalTrie { trie, .. } => trie.to_parser_combinator_code(),
            CombinatorTreeNodeData::CharacterFilter { filter, .. } => {
                filter.to_parser_combinator_code()
            }
            CombinatorTreeNodeData::End => quote!(end()),
        }
    }
}

impl Production {
    pub fn slang_name(&self) -> SlangName {
        SlangName::from_string(&self.name)
    }

    pub fn combinator_tree(&self) -> std::cell::Ref<'_, CombinatorTree> {
        self.combinator_tree.borrow()
    }

    pub fn initialize_combinator_tree(production: &ProductionRef, grammar: &Grammar) {
        let root = production
            .expression_to_generate()
            .to_combinator_tree_node(production.as_ref(), grammar);

        let root = if production.is_token() {
            root
        } else {
            let ignore = grammar.get_production("IGNORE");
            root.with_interspersed(&ignore)
        };
        let mut index = Cell::new(0);
        let root = root.with_unambiguous_named_types(&mut index);
        *production.combinator_tree.borrow_mut() = CombinatorTree {
            root,
            production: Rc::downgrade(production),
            module_name: production.slang_name(),
        }
    }
}

impl Expression {
    fn to_combinator_tree_node(
        &self,
        production: &Production,
        grammar: &Grammar,
    ) -> CombinatorTreeNode {
        // if let Some(pattern) = &self.config.pattern {
        //     match pattern {
        //         ExpressionPattern::PairOrPassthrough => match &self.ebnf {
        //             // TODO: Ensure validation is done earlier
        //             // 1. must be sequence of: option + reference or reference + option
        //             EBNF::Sequence(elements) if elements.len() == 2 => {
        //                 match (&elements[0].ebnf, &elements[1].ebnf) {
        //                     (
        //                         EBNF::Repeat(EBNFRepeat {
        //                             min: 0,
        //                             max: Some(1),
        //                             expr: optional,
        //                             ..
        //                         }),
        //                         EBNF::Reference(_),
        //                     ) => {
        //                         return ct_pair_or_passthrough(
        //                             self.config.slang_name(),
        //                             elements[1].to_combinator_tree_node(production, grammar),
        //                             optional.to_combinator_tree_node(production, grammar),
        //                             Direction::Left,
        //                         );
        //                     }
        //                     (
        //                         EBNF::Reference(_),
        //                         EBNF::Repeat(EBNFRepeat {
        //                             min: 0,
        //                             max: Some(1),
        //                             expr: optional,
        //                             ..
        //                         }),
        //                     ) => {
        //                         return ct_pair_or_passthrough(
        //                             self.config.slang_name(),
        //                             elements[0].to_combinator_tree_node(production, grammar),
        //                             optional.to_combinator_tree_node(production, grammar),
        //                             Direction::Right,
        //                         );
        //                     }
        //                     _ => (),
        //                 }
        //             }
        //             _ => {}
        //         },

        //         // TODO: Ensure validation is done earlier
        //         // 1. must be 1…*{ reference / Some(..) }
        //         ExpressionPattern::FoldLeftOrPassthrough => match &self.ebnf {
        //             EBNF::Repeat(EBNFRepeat {
        //                 min: 1,
        //                 max: None,
        //                 expr,
        //                 separator: Some(separator),
        //             }) => {
        //                 if let EBNF::Reference(_) = expr.ebnf {
        //                     return ct_fold_or_passthrough(
        //                         self.config.slang_name(),
        //                         expr.to_combinator_tree_node(production, grammar),
        //                         separator.to_combinator_tree_node(production, grammar),
        //                         Direction::Left,
        //                     );
        //                 }
        //             }
        //             _ => {}
        //         },

        //         // TODO: Ensure validation is done earlier
        //         // 1. repeat must be 1…*
        //         // 2. max must be None
        //         // 3. separator must be Some - otherwise fold doesn't make sense
        //         // 4. expr must be reference
        //         ExpressionPattern::FoldRightOrPassthrough => match &self.ebnf {
        //             EBNF::Repeat(EBNFRepeat {
        //                 min: 1,
        //                 max: None,
        //                 expr,
        //                 separator: Some(separator),
        //             }) => {
        //                 if let EBNF::Reference(_) = expr.ebnf {
        //                     return ct_fold_or_passthrough(
        //                         self.config.slang_name(),
        //                         expr.to_combinator_tree_node(production, grammar),
        //                         separator.to_combinator_tree_node(production, grammar),
        //                         Direction::Right,
        //                     );
        //                 }
        //             }
        //             _ => {}
        //         },
        //     }
        // }

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
                    let name = self.config.slang_name();

                    // Merge runs of TerminalTrees and CharacterFilters

                    let mut choices: Vec<(SlangName, CombinatorTreeNode)> = vec![];
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
    mut members: Vec<(SlangName, CombinatorTreeNode)>,
) -> Vec<(SlangName, CombinatorTreeNode)> {
    // Find all the duplicated names, with the count of their occurance
    let mut names = MultiSet::<SlangName>::from_iter(members.iter().map(|(n, _)| n.clone()));
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
