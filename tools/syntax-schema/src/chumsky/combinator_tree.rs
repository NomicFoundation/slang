use std::cell::Cell;

use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::{
    character_filter::CharacterFilter, slang_name::SlangName, terminal_trie::TerminalTrie,
};

pub struct CombinatorTree {
    pub root: CombinatorTreeNode,
    pub module_name: SlangName,
}

type CombinatorTreeNode = Box<CombinatorTreeNodeData>;

pub enum CombinatorTreeNodeData {
    Difference {
        minuend: CombinatorTreeNode,
        subtrahend: CombinatorTreeNode,
    },
    Lookahead {
        expr: CombinatorTreeNode,
        lookahead: CombinatorTreeNode,
    },
    Choice {
        name: SlangName,
        choices: Vec<(SlangName, CombinatorTreeNode)>,
    },
    Sequence {
        name: SlangName,
        elements: Vec<(SlangName, CombinatorTreeNode)>,
    },
    Repeat {
        name: SlangName,
        expr: CombinatorTreeNode,
        min: usize,
        max: Option<usize>,
        separator: Option<CombinatorTreeNode>,
    },
    Reference {
        name: SlangName,
    },
    TerminalTrie {
        name: Option<SlangName>,
        trie: TerminalTrie,
    },
    CharacterFilter {
        name: Option<SlangName>,
        filter: CharacterFilter,
    },
    End,
}

impl CombinatorTree {
    pub fn to_parser_combinator_code(&self) -> TokenStream {
        self.root.to_parser_combinator_code(self)
    }
}

impl CombinatorTreeNodeData {
    pub fn name(&self) -> Option<SlangName> {
        match self {
            CombinatorTreeNodeData::Difference { minuend, .. } => minuend.name(),
            CombinatorTreeNodeData::Lookahead { expr, .. } => expr.name(),
            CombinatorTreeNodeData::Choice { name, .. } => Some(name.clone()),
            CombinatorTreeNodeData::Sequence { name, .. } => Some(name.clone()),
            CombinatorTreeNodeData::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => expr.name(),
            CombinatorTreeNodeData::Repeat { expr, .. } => expr.name().map(|n| n.plural()),
            CombinatorTreeNodeData::Reference { name } => Some(name.clone()),
            CombinatorTreeNodeData::TerminalTrie { name, .. } => name.clone(),
            CombinatorTreeNodeData::CharacterFilter { name, .. } => name.clone(),
            CombinatorTreeNodeData::End => Some(SlangName::from_string("end_marker")),
        }
    }

    fn append_noise(self, subtype_index: &mut Cell<usize>) -> CombinatorTreeNode {
        match self {
            CombinatorTreeNodeData::Sequence { name, mut elements } => {
                elements.push((
                    SlangName::from_string("IGNORE"),
                    ct_reference(SlangName::from_string("IGNORE")),
                ));
                elements = disambiguate_structure_names(elements);
                ct_sequence(name, elements)
            }

            CombinatorTreeNodeData::Lookahead { .. }
            | CombinatorTreeNodeData::Difference { .. }
            | CombinatorTreeNodeData::Repeat { .. }
            | CombinatorTreeNodeData::Choice { .. }
            | CombinatorTreeNodeData::Reference { .. }
            | CombinatorTreeNodeData::TerminalTrie { .. }
            | CombinatorTreeNodeData::CharacterFilter { .. } => {
                let index = subtype_index.get();
                subtype_index.set(index + 1);
                let name = SlangName::from_prefix_and_index("_S", index);
                let c0 = (
                    self.name()
                        .unwrap_or_else(|| SlangName::from_prefix_and_index("_", 0)),
                    Box::new(self),
                );
                let ignore = (
                    SlangName::from_string("IGNORE"),
                    ct_reference(SlangName::from_string("IGNORE")),
                );
                ct_sequence(name, vec![c0, ignore])
            }

            CombinatorTreeNodeData::End => Box::new(self),
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
            CombinatorTreeNodeData::Repeat {
                expr,
                min: 0,
                max: Some(1),
                ..
            } => {
                let expr = expr.to_parser_combinator_code(tree);
                quote!( #expr.or_not() )
            }
            CombinatorTreeNodeData::Repeat {
                expr,
                min,
                max,
                separator: None,
                ..
            } => {
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
            CombinatorTreeNodeData::Repeat {
                name,
                expr,
                min,
                max,
                separator: Some(separator),
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
            CombinatorTreeNodeData::Reference { name } => {
                let name = name.to_parser_name_ident();
                quote!( #name.clone() )
            }
            CombinatorTreeNodeData::TerminalTrie { trie, .. } => trie.to_parser_expression(),
            CombinatorTreeNodeData::CharacterFilter { filter, .. } => filter.to_parser_expression(),
            CombinatorTreeNodeData::End => quote!(end()),
        }
    }
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

#[allow(dead_code)]
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

fn ct_repeat(
    name: SlangName,
    expr: CombinatorTreeNode,
    min: usize,
    max: Option<usize>,
    separator: Option<CombinatorTreeNode>,
) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Repeat {
        name,
        expr,
        min,
        max,
        separator,
    })
}

fn ct_reference(name: SlangName) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::Reference { name })
}

fn ct_terminal_trie(name: Option<SlangName>, trie: TerminalTrie) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::TerminalTrie { name, trie })
}

fn ct_character_filter(name: Option<SlangName>, filter: CharacterFilter) -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::CharacterFilter { name, filter })
}

fn ct_end() -> CombinatorTreeNode {
    Box::new(CombinatorTreeNodeData::End)
}

impl Production {
    pub fn to_combinator_tree(&self, grammar: &Grammar) -> CombinatorTree {
        CombinatorTree {
            root: self.expression_to_generate().to_combinator_tree_node(
                &mut Cell::new(0),
                self,
                grammar,
            ),
            module_name: SlangName::from_string(&self.name),
        }
    }
}

impl Expression {
    fn to_combinator_tree_node(
        &self,
        subtype_index: &mut Cell<usize>,
        production: &Production,
        grammar: &Grammar,
    ) -> CombinatorTreeNode {
        if let Some(filter) = self.to_character_filter(grammar) {
            let name = self
                .config
                .name
                .as_ref()
                .map(|s| SlangName::from_string(s))
                .or_else(|| filter.slang_name());
            return ct_character_filter(name, filter);
        } else if let Some(terminal_trie) = self.to_terminal_trie(grammar) {
            let name = self
                .config
                .name
                .as_ref()
                .map(|s| SlangName::from_string(s))
                .or_else(|| terminal_trie.slang_name());
            return ct_terminal_trie(name, terminal_trie);
        } else {
            match &self.ebnf {
                EBNF::End => ct_end(),
                EBNF::Difference(EBNFDifference {
                    minuend,
                    subtrahend,
                }) => ct_difference(
                    minuend.to_combinator_tree_node(subtype_index, production, grammar),
                    subtrahend.to_combinator_tree_node(subtype_index, production, grammar),
                ),
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max: Some(1),
                    ..
                }) => {
                    let name = self
                        .config
                        .name
                        .as_ref()
                        .map(|s| SlangName::from_string(s))
                        .unwrap_or_else(|| {
                            let index = subtype_index.get();
                            subtype_index.set(index + 1);
                            SlangName::from_prefix_and_index("_S", index)
                        });
                    let et = expr.to_combinator_tree_node(subtype_index, production, grammar);
                    ct_repeat(name, et, *min, Some(1), None)
                }
                EBNF::Repeat(EBNFRepeat {
                    expr,
                    min,
                    max,
                    separator,
                }) => {
                    let name = self
                        .config
                        .name
                        .as_ref()
                        .map(|s| SlangName::from_string(s))
                        .unwrap_or_else(|| {
                            let index = subtype_index.get();
                            subtype_index.set(index + 1);
                            SlangName::from_prefix_and_index("_S", index)
                        });
                    let mut et = expr.to_combinator_tree_node(subtype_index, production, grammar);
                    let mut st = separator
                        .clone()
                        .map(|s| s.to_combinator_tree_node(subtype_index, production, grammar));
                    if !production.is_token {
                        et = et.append_noise(subtype_index);
                        st = st.map(|st| st.append_noise(subtype_index));
                    }
                    ct_repeat(name, et, *min, *max, st)
                }
                EBNF::Choice(exprs) => {
                    let name = self
                        .config
                        .name
                        .as_ref()
                        .map(|s| SlangName::from_string(s))
                        .unwrap_or_else(|| {
                            let index = subtype_index.get();
                            subtype_index.set(index + 1);
                            SlangName::from_prefix_and_index("_C", index)
                        });

                    // TODO: Merge potential terminal_trie and character_filter elements.

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
                                    let name = ctt.slang_name().unwrap_or_else(|| {
                                        SlangName::from_prefix_and_index("_", choices.len())
                                    });
                                    choices.push((name, ct_terminal_trie(ctt.slang_name(), ctt)));
                                    current_terminal_tree = None
                                };
                                choices.push({
                                    let e = e.to_combinator_tree_node(
                                        subtype_index,
                                        production,
                                        grammar,
                                    );
                                    let name = e.name().unwrap_or_else(|| {
                                        SlangName::from_prefix_and_index("_", choices.len())
                                    });
                                    (name, e)
                                })
                            }
                        }
                        if let Some(ctt) = current_terminal_tree {
                            let name = ctt.slang_name().unwrap_or_else(|| {
                                SlangName::from_prefix_and_index("_", choices.len())
                            });
                            choices.push((name, ct_terminal_trie(ctt.slang_name(), ctt)));
                        };
                    }

                    choices = disambiguate_structure_names(choices);
                    ct_choice(name, choices)
                }
                EBNF::Sequence(exprs) => {
                    let name = self
                        .config
                        .name
                        .as_ref()
                        .map(|s| SlangName::from_string(s))
                        .unwrap_or_else(|| {
                            let index = subtype_index.get();
                            subtype_index.set(index + 1);
                            SlangName::from_prefix_and_index("_S", index)
                        });

                    let mut members = exprs
                        .iter()
                        .enumerate()
                        .map(|(i, e)| {
                            let e = e.to_combinator_tree_node(subtype_index, production, grammar);
                            let name = e
                                .name()
                                .unwrap_or_else(|| SlangName::from_prefix_and_index("_", i));
                            let e = (name, e);
                            if !production.is_token && 0 < i {
                                let name = SlangName::from_string("IGNORE");
                                vec![(name.clone(), ct_reference(name)), e]
                            } else {
                                vec![e]
                            }
                        })
                        .flatten()
                        .collect::<Vec<_>>();

                    members = disambiguate_structure_names(members);
                    ct_sequence(name, members)
                }
                EBNF::Reference(name) => ct_reference(SlangName::from_string(name)),
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
