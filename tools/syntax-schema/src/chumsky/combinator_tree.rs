use std::{
    cell::Cell,
    rc::{Rc, Weak}
};

use mset::MultiSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::*;

use super::{
    character_filter::CharacterFilter, name::Name, terminal_trie::TerminalTrie,
};

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
        // -> M
        minuend: CombinatorTreeRef,
        subtrahend: CombinatorTreeRef,
    },
    Lookahead {
        // -> E
        expr: CombinatorTreeRef,
        lookahead: CombinatorTreeRef,
    },
    Sequence {
        // -> struct N { E ... }
        name: Name, // Assigned when created
        elements: Vec<NamedCombinatorTreeRef>,
    },
    Choice {
        // -> enum N { C(E) .. }
        name: Name, // Assigned when created
        choices: Vec<NamedCombinatorTreeRef>,
    },
    Optional {
        // -> Option<E>
        expr: CombinatorTreeRef,
    },
    SeparatedBy {
        // -> (Vec<E>, Vec<S>)
        name: Name, // Assigned when created
        expr: CombinatorTreeRef,
        min: usize, // > 0
        max: Option<usize>,
        separator: CombinatorTreeRef,
    },
    Repeated {
        // -> Vec<E>
        name: Name, // Assigned when created
        expr: CombinatorTreeRef,
        min: usize,
        max: Option<usize>,
    },
    Expression {
        // -> enum N { C(P::E) .. }
        name: Name, // Assigned when created
        members: Vec<ProductionWeakRef>,
    },
    ExpressionMember {
        // P::N(E)
        parent: ProductionWeakRef,
        next_sibling: ProductionWeakRef, // Empty ref means no next sibling
        expr: CombinatorTreeRef,
        direction: Direction,
    },
    Reference {
        production: ProductionWeakRef,
    },
    TerminalTrie {
        // -> Fixed<n> || usize
        // -> { leading: ignore::N,  len: Fixed<n> || usize, trailing: ignore::N }
        name: Name,
        trie: TerminalTrie,
        with_noise: bool,
    },
    CharacterFilter {
        // -> Fixed<1>
        // -> { leading: ignore::N, len: Fixed<1>, trailing: ignore::N }
        name: Name,
        filter: CharacterFilter,
        with_noise: bool,
    },
    End,
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

fn ct_sequence(
    name: Name,
    elements: Vec<(Name, CombinatorTreeRef)>,
) -> CombinatorTreeRef {
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

fn ct_expression(
    name: Name,
    members: Vec<ProductionWeakRef>
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::Expression {
        name, members
    })
}

fn ct_expression_member(
    parent: ProductionWeakRef,
    next_sibling: ProductionWeakRef,
    expr: CombinatorTreeRef,
    direction: Direction,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::ExpressionMember {
        parent,
        next_sibling,
        expr,
        direction,
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

fn ct_character_filter(
    name: Name,
    filter: CharacterFilter,
    with_noise: bool,
) -> CombinatorTreeRef {
    Rc::new(CombinatorTree::CharacterFilter {
        name,
        filter,
        with_noise,
    })
}

fn ct_end() -> CombinatorTreeRef {
    Rc::new(CombinatorTree::End)
}

impl CombinatorTreeRoot {
    pub fn to_parser_combinator_code(&self) -> TokenStream {
        self.root.to_default_parser_combinator_code(self)
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
    // TODO: generic tree transformer?
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
                parent,
                next_sibling,
                expr,
                direction,
            } => ct_expression_member(
                parent.clone(),
                next_sibling.clone(),
                expr.with_unambiguous_named_types(index),
                *direction,
            ),
            CombinatorTree::Expression { .. } |
            CombinatorTree::Reference { .. } |
            CombinatorTree::TerminalTrie { .. } |
            CombinatorTree::CharacterFilter { .. } => self.clone(),
            CombinatorTree::End => ct_end(),
        }
    }
}

impl CombinatorTree {
    pub fn name(&self) -> Name {
        match self {
            Self::TerminalTrie { name, .. }
            | Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::Expression { name, .. }
            | Self::Sequence { name, .. } => name.clone(),
            Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::ExpressionMember { expr, .. }
            | Self::Optional { expr } => expr.name(),
            Self::SeparatedBy { expr, .. }
            | Self::Repeated { expr, .. } => expr.name().pluralize(),
            Self::Reference { production } => {
                production.upgrade().unwrap().slang_name()
            }
            Self::End => Name::from_string("end_marker"),
        }
    }

    fn to_default_parser_combinator_code(&self, tree: &CombinatorTreeRoot) -> TokenStream {
        self.to_parser_combinator_code(tree, |node, tree| {
            node.to_default_parser_combinator_code(tree)
        })
    }

    // This is a generic tree walker
    fn to_parser_combinator_code<F>(&self, tree: &CombinatorTreeRoot, visitor: F) -> TokenStream
    where
        F: Fn(&CombinatorTreeRef, &CombinatorTreeRoot) -> TokenStream,
    {
        match self {
            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = visitor(minuend, tree);
                let subtrahend = visitor(subtrahend, tree);
                quote! ( difference(#minuend, #subtrahend) )
            }
            Self::Lookahead { expr, lookahead } => {
                let expr = visitor(expr, tree);
                let lookahead = visitor(lookahead, tree);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }
            Self::Choice { choices, name } => {
                let module_name = tree.slang_name().to_module_name_ident();
                let choice_name = name.to_type_name_ident();
                let choices = choices.iter().map(|(n, c)| {
                    let constructor = n.to_enum_tag_ident();
                    let expr = visitor(c, tree);
                    quote!( #expr.map(|v| Box::new(#module_name::#choice_name::#constructor(v))) )
                });
                quote!( choice(( #(#choices),* )) )
            }
            Self::Sequence { elements, name } => {
                // TODO: pattern: Expression

                let struct_name = name.to_type_name_ident();
                let elements = elements
                    .iter()
                    .map(|(_, e)| visitor(e, tree))
                    .reduce(|accum, next| quote!( #accum.then(#next) ));
                let module_name = tree.slang_name().to_module_name_ident();
                quote!( #elements .map(|v| Box::new(#module_name::#struct_name::new(v))) )
            }
            Self::Optional { expr } => {
                let expr = visitor(expr, tree);
                quote!( #expr.or_not() )
            }
            Self::Repeated { expr, min, max, .. } => {
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
            Self::SeparatedBy {
                name,
                expr,
                min,
                max,
                separator,
            } => {
                let expr = visitor(expr, tree);
                let separator = visitor(separator, tree);

                let mapping = {
                    let module_name = tree.slang_name().to_module_name_ident();
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
            Self::Expression {
                name: _, members
            } => {
                let parser_name = members[0].upgrade().unwrap().slang_name().to_parser_name_ident();
                quote!( #parser_name )
            }
            Self::ExpressionMember {
                parent,
                next_sibling,
                expr,
                direction,
            } => {
                let enum_name = {
                    let p = parent.upgrade().unwrap();
                    let module_name = p.slang_name().to_module_name_ident();
                    let choice_name = p.combinator_tree().root.name().to_type_name_ident();
                    let constructor = tree.slang_name().to_enum_tag_ident();
                    quote!( #module_name::#choice_name::#constructor )
                };

                let next_parser = next_sibling
                    .upgrade()
                    .map(|n| n.slang_name().to_parser_name_ident());
                if let Self::Sequence { elements, .. } = expr.as_ref() {
                    if let Self::Reference { production } = elements[0].1.as_ref() {
                        if production.as_ptr() == parent.as_ptr() {
                            let is_right_recursive = if let Self::Reference { production } =
                                elements[elements.len() - 1].1.as_ref()
                            {
                                production.as_ptr() == parent.as_ptr()
                            } else {
                                false
                            };

                            if is_right_recursive {
                                let operator = elements[1..elements.len() - 1]
                                    .iter()
                                    .map(|(_, e)| visitor(e, tree))
                                    .reduce(|prev, next| quote!(#prev.then(#next)));
                                if *direction == Direction::Left {
                                    return quote!(
                                        #next_parser.then(#operator.then(#next_parser).repeated()).map(|first, rest| rest.reduce(first, #enum_name))
                                    );
                                } else {
                                    let parent = parent
                                        .upgrade()
                                        .unwrap()
                                        .slang_name()
                                        .to_parser_name_ident();
                                    return quote!(
                                        #next_parser.then(#operator).then(#parent).map(#enum_name)
                                    );
                                }
                            } else {
                                let operator = elements[1..elements.len()]
                                    .iter()
                                    .map(|(_, e)| visitor(e, tree))
                                    .reduce(|prev, next| quote!(#prev.then(#next)));
                                return quote!(
                                    #next_parser.then(#operator.repeated()).map(|first, rest| rest.reduce(first, #enum_name))
                                );
                            }
                        }
                    }
                }

                let operator = visitor(expr, tree);
                if let Some(next_parser) = next_parser {
                    quote!( choice((#operator.map(#enum_name), #next_parser)) )
                } else {
                    quote!( #operator.map(#enum_name) )
                }
            }
            Self::Reference { production } => {
                let name = production
                    .upgrade()
                    .unwrap()
                    .slang_name()
                    .to_parser_name_ident();
                quote!( #name.clone() )
            }
            Self::TerminalTrie {
                trie, with_noise, ..
            } => {
                let expr = trie.to_parser_combinator_code();
                if *with_noise {
                    quote!( wrap_with_noise(#expr) )
                } else {
                    expr
                }
            }
            Self::CharacterFilter {
                filter, with_noise, ..
            } => {
                let expr = filter.to_parser_combinator_code();
                if *with_noise {
                    quote!( wrap_with_noise(#expr) )
                } else {
                    expr
                }
            }
            Self::End => quote!(end()),
        }
    }
}

impl Grammar {
    pub fn initialize_combinator_trees(&mut self) {
        let all_productions = self
            .productions
            .iter()
            .map(|(_, v)| v)
            .flatten()
            .collect::<Vec<_>>();
        for production in &all_productions {
            production.initialize_combinator_tree(self);
        }
        for production in &all_productions {
            if production.pattern == Some(ProductionPattern::Expression) {
                production.apply_expression_pattern();
            }
        }
    }
}

trait ProductionRefTrait {
    fn initialize_combinator_tree(&self, grammar: &Grammar);
    fn apply_expression_pattern(&self);
}

impl ProductionRefTrait for ProductionRef {

    // Takes a &ProductionRef, not &self, so not a 'method'
    fn initialize_combinator_tree(&self, grammar: &Grammar) {
        let expression = self.expression_to_generate();
        let root = expression.to_combinator_tree_node(self.as_ref(), grammar);
        let mut index = Cell::new(0);
        let root = root.with_unambiguous_named_types(&mut index);
        *self.combinator_tree.borrow_mut() = CombinatorTreeRoot {
            root,
            production: Rc::downgrade(self),
        };
    }

    fn apply_expression_pattern(&self) {
        let ct = self.combinator_tree();
        if let CombinatorTree::Expression { members: choices, .. } = ct.root.as_ref() {
            let mut choices = choices.clone();
            choices.reverse();
            let mut next_sibling = Weak::new();
            for choice in choices {
                let choice = choice.upgrade().unwrap();
                let mut tree = choice.combinator_tree.borrow_mut();
                tree.root = ct_expression_member(
                    Rc::downgrade(&self),
                    next_sibling.clone(),
                    tree.root.clone(),
                    Direction::Left,
                );
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
                        let choices = exprs.iter().map(|e| {if let EBNF::Reference(prod_name) = &e.ebnf {
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
