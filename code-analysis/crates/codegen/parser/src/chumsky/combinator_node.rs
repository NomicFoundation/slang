use codegen_schema::*;

use super::{
    character_filter::CharacterFilter, combinator_tree::CombinatorTree, naming,
    terminal_trie::TerminalTrie,
};

pub enum CombinatorNode<'context> {
    CharacterFilter {
        name: Option<String>,
        filter: &'context CharacterFilter<'context>,
    },
    Choice {
        name: Option<String>,
        elements: Vec<&'context CombinatorNode<'context>>,
    },
    DelimitedBy {
        name: Option<String>,
        open: String,
        expr: &'context CombinatorNode<'context>,
        close: String,
    },
    Difference {
        minuend: &'context CombinatorNode<'context>,
        subtrahend: &'context CombinatorNode<'context>,
    },
    PrecedenceRule {
        tree: &'context CombinatorTree<'context>,
        members: Vec<&'context CombinatorTree<'context>>,
    },
    PrecedenceRuleMember {
        tree: &'context CombinatorTree<'context>,
        parent: &'context CombinatorTree<'context>,
        next_sibling: Option<&'context CombinatorTree<'context>>,
        operator: &'context CombinatorNode<'context>,
        operator_model: OperatorModel,
    },
    #[allow(dead_code)]
    Lookahead {
        expr: &'context CombinatorNode<'context>,
        lookahead: &'context CombinatorNode<'context>,
    },
    OneOrMore {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
    },
    Optional {
        expr: &'context CombinatorNode<'context>,
    },
    Reference {
        tree: &'context CombinatorTree<'context>,
    },
    Repeated {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
        min: usize,
        max: usize,
    },
    SeparatedBy {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
        separator: String,
    },
    Sequence {
        name: Option<String>,
        elements: Vec<&'context CombinatorNode<'context>>,
    },
    TerminalTrie {
        name: Option<String>,
        trie: TerminalTrie,
    },
    ZeroOrMore {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
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

impl<'context> CombinatorNode<'context> {
    pub fn new(
        tree: &'context CombinatorTree<'context>,
        expression: &ExpressionRef,
        inherited_name: Option<String>,
    ) -> &'context CombinatorNode<'context> {
        let name = expression.config.name.clone().or(inherited_name);

        if tree.production.kind == ProductionKind::Token {
            if let Some(filter) = CharacterFilter::new(tree, expression) {
                return tree.context.alloc_node(Self::CharacterFilter {
                    name: name.or_else(|| filter.default_name()),
                    filter,
                });
            }

            if let Some(trie) = TerminalTrie::new(tree, expression) {
                return tree.context.alloc_node(Self::TerminalTrie {
                    name: name.or_else(|| trie.default_name()),
                    trie,
                });
            }
        }

        if tree.production.kind == ProductionKind::Rule
            && expression.config.parser_type == ParserType::Precedence
        {
            if let EBNF::Choice(exprs) = &expression.ebnf {
                let members = exprs.iter().map(|e| {
                    if let EBNF::Reference(prod_name) = &e.ebnf {
                       tree.context.get_tree_by_name(prod_name)
                    } else {
                        unreachable!("Validation should have checked this: The Expression pattern is only applicable to a choice of references")
                    }
                }).collect();
                return tree
                    .context
                    .alloc_node(Self::PrecedenceRule { tree, members });
            } else {
                unreachable!("Validation should have checked this: The Expression pattern is only applicable to a choice of references")
            }
        }

        tree.context.alloc_node(match &expression.ebnf {
            EBNF::Choice(exprs) => {
                if tree.production.kind != ProductionKind::Token {
                    Self::Choice {
                        name,
                        elements: exprs.iter().map(|e| Self::new(tree, e, None)).collect(),
                    }
                } else {
                    // Merge runs of TerminalTrees and CharacterFilters
                    let mut elements: Vec<&'context CombinatorNode<'context>> = vec![];

                    let mut current_terminal_tree: Option<TerminalTrie> = None;
                    let mut current_character_filter: Option<&'context CharacterFilter<'context>> =
                        None;
                    for e in exprs {
                        // Sub-expressions with a user-specified name aren't merged
                        if e.config.name.is_none() {
                            if let Some(cf) = CharacterFilter::new(tree, e) {
                                if let Some(trie) = current_terminal_tree {
                                    elements.push(tree.context.alloc_node(Self::TerminalTrie {
                                        name: trie.default_name(),
                                        trie,
                                    }));
                                    current_terminal_tree = None
                                };
                                if let Some(filter) = current_character_filter {
                                    current_character_filter = Some(filter.merged_with(cf, tree))
                                } else {
                                    current_character_filter = Some(cf)
                                }
                                continue;
                            }
                            if let Some(tt) = TerminalTrie::new(tree, e) {
                                if let Some(filter) = current_character_filter {
                                    elements.push(tree.context.alloc_node(Self::CharacterFilter {
                                        name: filter.default_name(),
                                        filter,
                                    }));
                                    current_character_filter = None
                                };
                                if let Some(trie) = current_terminal_tree.as_mut() {
                                    trie.merge_with(tt)
                                } else {
                                    current_terminal_tree = Some(tt)
                                }
                                continue;
                            }
                        }

                        if let Some(filter) = current_character_filter {
                            elements.push(tree.context.alloc_node(Self::CharacterFilter {
                                name: filter.default_name(),
                                filter,
                            }));
                            current_character_filter = None
                        };

                        if let Some(trie) = current_terminal_tree {
                            elements.push(tree.context.alloc_node(Self::TerminalTrie {
                                name: trie.default_name(),
                                trie,
                            }));
                            current_terminal_tree = None
                        };

                        elements.push(Self::new(tree, e, None))
                    }

                    if let Some(filter) = current_character_filter {
                        elements.push(tree.context.alloc_node(Self::CharacterFilter {
                            name: filter.default_name(),
                            filter,
                        }));
                    };

                    if let Some(trie) = current_terminal_tree {
                        elements.push(tree.context.alloc_node(Self::TerminalTrie {
                            name: trie.default_name(),
                            trie,
                        }));
                    };

                    Self::Choice { name, elements }
                }
            }

            EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                let expr = Self::new(tree, expr, None);
                let name = name.or_else(|| {
                    expr.name().map(|expr_name| {
                        format!(
                            "{}_and_{}_and_{}",
                            naming::name_of_terminal_string(open),
                            expr_name,
                            naming::name_of_terminal_string(close)
                        )
                    })
                });
                let open = open.clone();
                let close = close.clone();
                Self::DelimitedBy {
                    name,
                    open,
                    expr,
                    close,
                }
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                let minuend = Self::new(tree, minuend, None);
                let subtrahend = Self::new(tree, subtrahend, None);
                Self::Difference {
                    minuend,
                    subtrahend,
                }
            }

            EBNF::Not(_) => {
                if let Some(filter) = CharacterFilter::new(tree, expression) {
                    let name = name.or_else(|| filter.default_name());
                    Self::CharacterFilter { name, filter }
                } else {
                    unimplemented!("¬ is only supported on characters or sets thereof")
                }
            }

            EBNF::OneOrMore(expr) => {
                let expr = Self::new(tree, expr, None);
                let name =
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name)));
                Self::OneOrMore { name, expr }
            }

            EBNF::Optional(expr) => {
                let expr = Self::new(tree, expr, name);
                Self::Optional { expr }
            }

            EBNF::Range(_) => {
                let filter = CharacterFilter::new(tree, expression).unwrap();
                let name = name.or_else(|| filter.default_name());
                Self::CharacterFilter { name, filter }
            }

            EBNF::Reference(name) => Self::Reference {
                tree: tree
                    .context
                    .trees_by_name
                    .borrow()
                    .get(name)
                    .expect("Production not found"),
            },

            EBNF::Repeat(EBNFRepeat { expr, min, max }) => {
                let expr = Self::new(tree, expr, None);
                let name =
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name)));
                let min = *min;
                let max = *max;
                Self::Repeated {
                    name,
                    expr,
                    min,
                    max,
                }
            }

            EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
                let expr = Self::new(tree, expr, None);
                let name = name.or_else(|| {
                    expr.name().map(|expr_name| {
                        format!(
                            "{}_and_{}",
                            naming::pluralize(&expr_name),
                            naming::pluralize(&naming::name_of_terminal_string(&separator))
                        )
                    })
                });
                let separator = separator.clone();
                Self::SeparatedBy {
                    name,
                    expr,
                    separator,
                }
            }

            EBNF::Sequence(exprs) => {
                let elements = exprs.iter().map(|e| Self::new(tree, e, None)).collect();
                Self::Sequence { name, elements }
            }

            EBNF::Terminal(_) => {
                let trie = TerminalTrie::new(tree, expression).unwrap();
                let name = name.or_else(|| trie.default_name());
                Self::TerminalTrie { name, trie }
            }

            EBNF::ZeroOrMore(expr) => {
                let expr = Self::new(tree, expr, None);
                let name =
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name)));
                Self::ZeroOrMore { name, expr }
            }
        })
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::Reference { tree }
            | Self::PrecedenceRule { tree, .. }
            | Self::PrecedenceRuleMember { tree, .. } => Some(tree.production.name.clone()),

            Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::DelimitedBy { name, .. }
            | Self::OneOrMore { name, .. }
            | Self::Repeated { name, .. }
            | Self::SeparatedBy { name, .. }
            | Self::Sequence { name, .. }
            | Self::TerminalTrie { name, .. }
            | Self::ZeroOrMore { name, .. } => name.clone(),

            Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::Optional { expr } => expr.name(),
        }
    }
}
