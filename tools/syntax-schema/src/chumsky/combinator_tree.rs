use std::cell::Cell;

use itertools::Itertools;
use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::{
    character_filter::CharacterFilter, slang_name::SlangName, terminal_trie::TerminalTrie,
};

#[derive(Clone, Debug)]
pub struct CombinatorTree {
    pub root: CombinatorTreeNode,
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
    // Passthrough {
    //     // -> E
    //     expr: CombinatorTreeNode,
    // },
    // PassthroughOrPair {
    //     // -> E || N(E, O) || N(O, E)
    //     name: SlangName,
    //     expr: CombinatorTreeNode,
    //     optional: CombinatorTreeNode,
    //     is_prefix: bool,
    // },
    // PassthroughOrFold {
    //     // -> X := E || N(E, S, X) || N(X, S, E)
    //     name: SlangName,
    //     expr: CombinatorTreeNode,
    //     min: usize,
    //     max: Option<usize>,
    //     separator: CombinatorTreeNode,
    //     is_left_fold: bool,
    // },
    Optional {
        // -> Option<E>
        expr: CombinatorTreeNode,
    },
    SeparatedBy {
        // -> (Vec<E>, Vec<S>)
        name: SlangName, // Assigned when created
        expr: CombinatorTreeNode,
        min: usize,
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
        production: ProductionRef,
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

fn ct_reference(production: ProductionRef) -> CombinatorTreeNode {
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
        self.root.to_parser_combinator_code(self)
    }
}

impl Default for CombinatorTree {
    fn default() -> Self {
        Self {
            root: ct_end(),
            module_name: SlangName::from_string("__uninitialized__"),
        }
    }
}

impl CombinatorTreeNodeData {
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
            CombinatorTreeNodeData::Reference { production } => ct_reference(production),
            CombinatorTreeNodeData::TerminalTrie { .. } => Box::new(self),
            CombinatorTreeNodeData::CharacterFilter { .. } => Box::new(self),
            CombinatorTreeNodeData::End => ct_end(),
        }
    }

    pub fn with_interspersed(self, production: &ProductionRef) -> CombinatorTreeNode {
        match self {
            CombinatorTreeNodeData::Difference { .. }
            | CombinatorTreeNodeData::Lookahead { .. }
            | CombinatorTreeNodeData::Choice { .. }
            | CombinatorTreeNodeData::Optional { .. }
            | CombinatorTreeNodeData::SeparatedBy { .. }
            | CombinatorTreeNodeData::Repeat { .. }
            | CombinatorTreeNodeData::Reference { .. }
            | CombinatorTreeNodeData::TerminalTrie { .. }
            | CombinatorTreeNodeData::CharacterFilter { .. }
            | CombinatorTreeNodeData::End => Box::new(self),
            CombinatorTreeNodeData::Sequence { name, elements } => ct_sequence(
                name,
                #[allow(unstable_name_collisions)]
                elements
                    .into_iter()
                    .intersperse_with(|| {
                        (production.slang_name(), ct_reference(production.clone()))
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
            CombinatorTreeNodeData::SeparatedBy { expr, .. }
            | CombinatorTreeNodeData::Repeat { expr, .. } => expr.name().plural(),
            CombinatorTreeNodeData::Reference { production } => production.slang_name(),
            CombinatorTreeNodeData::End => SlangName::from_string("end_marker"),
        }
    }

    fn to_parser_combinator_code(&self, tree: &CombinatorTree) -> TokenStream {
        match self {
            CombinatorTreeNodeData::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_parser_combinator_code(tree);
                let subtrahend = subtrahend.to_parser_combinator_code(tree);
                quote! ( difference(#minuend, #subtrahend) )
            }
            CombinatorTreeNodeData::Lookahead { expr, lookahead } => {
                let expr = expr.to_parser_combinator_code(tree);
                let lookahead = lookahead.to_parser_combinator_code(tree);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
            CombinatorTreeNodeData::Choice { choices, name } => {
                let module_name = tree.module_name.to_module_name_ident();
                let choice_name = name.to_type_name_ident();
                let choices = choices.iter().map(|(n, c)| {
                    let constructor = n.to_enum_tag_ident();
                    let expr = c.to_parser_combinator_code(tree);
                    quote!( #expr.map(|v| Box::new(#module_name::#choice_name::#constructor(v))) )
                });
                quote!( choice(( #(#choices),* )) )
            }
            CombinatorTreeNodeData::Sequence { elements, name } => {
                let struct_name = name.to_type_name_ident();
                let mut elements = elements
                    .iter()
                    .map(|(_, e)| e.to_parser_combinator_code(tree));
                let first = elements.next().unwrap();
                let rest = elements.map(|e| quote!( .then(#e) ));
                let module_name = tree.module_name.to_module_name_ident();
                quote!( #first #(#rest)* .map(|v| Box::new(#module_name::#struct_name::new(v))) )
            }
            CombinatorTreeNodeData::Optional { expr } => {
                let expr = expr.to_parser_combinator_code(tree);
                quote!( #expr.or_not() )
            }
            CombinatorTreeNodeData::Repeat { expr, min, max, .. } => {
                let vec_to_length = if let Self::CharacterFilter { .. } = **expr {
                    // Vec<()>
                    quote!( .map(|v| v.len()) )
                } else {
                    quote!()
                };

                let expr = expr.to_parser_combinator_code(tree);

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
            CombinatorTreeNodeData::SeparatedBy {
                name,
                expr,
                min,
                max,
                separator,
            } => {
                let expr = expr.to_parser_combinator_code(tree);
                let separator = separator.to_parser_combinator_code(tree);

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
                let name = production.slang_name().to_parser_name_ident();
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

    pub fn initialize_combinator_tree(&self, grammar: &Grammar) {
        let root = self
            .expression_to_generate()
            .to_combinator_tree_node(self, grammar);
        let root = if self.is_token {
            root
        } else {
            let ignore = grammar.get_production("IGNORE");
            root.with_interspersed(&ignore)
        };
        let mut index = Cell::new(0);
        let root = root.with_unambiguous_named_types(&mut index);
        *self.combinator_tree.borrow_mut() = CombinatorTree {
            root,
            module_name: self.slang_name(),
        }
    }
}

impl Expression {
    fn to_combinator_tree_node(
        &self,
        production: &Production,
        grammar: &Grammar,
    ) -> CombinatorTreeNode {
        if let Some(pattern) = &self.config.pattern {
            match pattern {
                ExpressionPattern::Passthrough => match &self.ebnf {
                    EBNF::Reference(name) => {
                        let production = grammar.get_production(&name);
                        return production
                            .expression_to_generate()
                            .to_combinator_tree_node(production.as_ref(), grammar);
                    }
                    EBNF::Sequence(elements) if elements.len() == 2 => {
                        // match (&elements[0].ebnf, &elements[1].ebnf) {
                        //     (
                        //         EBNF::Repeat(EBNFRepeat {
                        //             min: 0,
                        //             max: Some(1),
                        //             ..
                        //         }),
                        //         _,
                        //     ) => {}
                        //     (
                        //         _,
                        //         EBNF::Repeat(EBNFRepeat {
                        //             min: 0,
                        //             max: Some(1),
                        //             ..
                        //         }),
                        //     ) => {
                        //         let name = self
                        //             .config
                        //             .name
                        //             .as_ref()
                        //             .map(|s| SlangName::from_string(s))
                        //             .unwrap_or_else(|| SlangName::anonymous_type(subtype_index));
                        //         ct_passthrough(
                        //             name,
                        //             0,
                        //             elements[0].to_combinator_tree_node(
                        //                 subtype_index,
                        //                 production,
                        //                 grammar,
                        //             ),
                        //             elements[1].to_combinator_tree_node(
                        //                 subtype_index,
                        //                 production,
                        //                 grammar,
                        //             ),
                        //         )
                        //     }
                        //     _ => (),
                        // }
                    }
                    _ => {}
                },
                ExpressionPattern::FoldLeftOrPassthrough
                | ExpressionPattern::FoldRightOrPassthrough => match &self.ebnf {
                    EBNF::Repeat(EBNFRepeat {
                        min: _,
                        max: _,
                        expr: _,
                        separator: _,
                    }) => {}
                    _ => {}
                },
                ExpressionPattern::Inline => {}
            }
        }

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

                    // Merge adjacent TerminalTrees

                    let mut choices: Vec<(SlangName, CombinatorTreeNode)> = vec![];
                    {
                        let mut current_terminal_tree: Option<TerminalTrie> = None;
                        for e in exprs {
                            if let Some(tt) = e.to_terminal_trie(grammar) {
                                if let Some(ctt) = current_terminal_tree.as_mut() {
                                    ctt.merge_with(tt)
                                } else {
                                    current_terminal_tree = Some(tt)
                                }
                            } else {
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
                        }
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
                EBNF::Reference(name) => ct_reference(grammar.get_production(name)),
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
