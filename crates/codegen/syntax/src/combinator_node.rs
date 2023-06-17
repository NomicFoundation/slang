use codegen_schema::types::{
    OperatorModel, ParserDefinition, ParserRef, PrecedenceParserRef, ScannerDefinition, ScannerRef,
};
use itertools::Itertools;

use super::{
    char_set::CharSet,
    combinator_tree::CombinatorTree,
    first_set::FirstSet,
    trie::{self, TerminalTrie},
};

pub enum CombinatorNode<'context> {
    CharacterFilter {
        name: Option<String>,
        filter: CharSet,
    },
    Choice {
        name: Option<String>,
        elements: Vec<&'context CombinatorNode<'context>>,
    },
    DelimitedBy {
        name: Option<String>,
        open: &'context CombinatorTree<'context>,
        expr: &'context CombinatorNode<'context>,
        close: &'context CombinatorTree<'context>,
    },
    Difference {
        minuend: &'context CombinatorNode<'context>,
        subtrahend: &'context CombinatorNode<'context>,
    },
    OneOrMore {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
    },
    Optional {
        expr: &'context CombinatorNode<'context>,
    },
    PrecedenceExpressionRule {
        tree: &'context CombinatorTree<'context>,
        operators: Vec<PrecedenceRuleOperator<'context>>,
        primary_expression: &'context CombinatorNode<'context>,
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
        separator: &'context CombinatorTree<'context>,
    },
    Sequence {
        name: Option<String>,
        elements: Vec<&'context CombinatorNode<'context>>,
    },
    TerminalTrie {
        trie: TerminalTrie,
    },
    TerminatedBy {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
        terminator: &'context CombinatorTree<'context>,
    },
    TrailingContext {
        expression: &'context CombinatorNode<'context>,
        not_followed_by: &'context CombinatorNode<'context>,
    },
    ZeroOrMore {
        name: Option<String>,
        expr: &'context CombinatorNode<'context>,
    },
}

pub struct PrecedenceRuleOperator<'context> {
    pub name: String,
    pub model: OperatorModel,
    pub operator: &'context CombinatorNode<'context>,
}

impl<'context> CombinatorNode<'context> {
    pub fn from_scanner(
        tree: &'context CombinatorTree<'context>,
        scanner: &ScannerRef,
    ) -> &'context CombinatorNode<'context> {
        if let Some(filter) = CharSet::from_scanner(tree, scanner.clone()) {
            return tree
                .context
                .alloc_node(Self::CharacterFilter { name: None, filter });
        }

        if let Some(trie) = trie::from_scanner(tree, scanner.clone()) {
            return tree.context.alloc_node(Self::TerminalTrie { trie });
        }

        tree.context.alloc_node(match &scanner.definition {
            ScannerDefinition::Choice(exprs) => {
                // (unversioned) terminals in choices are merged, and represented as a trie

                enum TN<'c> {
                    Trie(trie::TerminalTrie),
                    Node(&'c CombinatorNode<'c>),
                }
                let mut elements = exprs
                    .iter()
                    .map(|expr| match trie::from_scanner(tree, expr.clone()) {
                        None => TN::Node(Self::from_scanner(tree, expr)),
                        Some(trie) => TN::Trie(trie),
                    })
                    .coalesce(|prev, curr| match (prev, curr) {
                        (TN::Trie(prev), TN::Trie(curr)) => Ok(TN::Trie({
                            let mut n = prev.clone();
                            n.extend(curr);
                            n
                        })),
                        pair => Err(pair),
                    })
                    .map(|either| match either {
                        TN::Node(node) => node,
                        TN::Trie(trie) => tree.context.alloc_node(Self::TerminalTrie { trie }),
                    })
                    .collect::<Vec<_>>();
                if elements.len() == 1 {
                    return elements.pop().unwrap();
                } else {
                    Self::Choice {
                        name: None,
                        elements,
                    }
                }
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => Self::Difference {
                minuend: Self::from_scanner(tree, minuend),
                subtrahend: Self::from_scanner(tree, subtrahend),
            },

            ScannerDefinition::Not(_) => {
                if let Some(filter) = CharSet::from_scanner(tree, scanner.clone()) {
                    Self::CharacterFilter { name: None, filter }
                } else {
                    unimplemented!("'Not' is only supported on characters or sets thereof")
                }
            }

            ScannerDefinition::OneOrMore(expr) => Self::OneOrMore {
                name: None,
                expr: Self::from_scanner(tree, expr),
            },

            ScannerDefinition::Optional(expr) => Self::Optional {
                expr: Self::from_scanner(tree, expr),
            },

            ScannerDefinition::Range { .. } => Self::CharacterFilter {
                name: None,
                filter: CharSet::from_scanner(tree, scanner.clone()).unwrap(),
            },

            ScannerDefinition::Reference(name) => Self::Reference {
                tree: tree.context.get_tree_by_name(name),
            },

            ScannerDefinition::Repeat {
                expression,
                min,
                max,
            } => Self::Repeated {
                name: None,
                expr: Self::from_scanner(tree, expression),
                min: *min,
                max: *max,
            },

            ScannerDefinition::Sequence(exprs) => Self::Sequence {
                name: None,
                elements: exprs.iter().map(|e| Self::from_scanner(tree, e)).collect(),
            },

            ScannerDefinition::Terminal(_) => {
                if let Some(filter) = CharSet::from_scanner(tree, scanner.clone()) {
                    Self::CharacterFilter { name: None, filter }
                } else {
                    Self::TerminalTrie {
                        trie: trie::from_scanner(tree, scanner.clone()).unwrap(),
                    }
                }
            }

            ScannerDefinition::TrailingContext {
                expression,
                not_followed_by,
            } => Self::TrailingContext {
                expression: Self::from_scanner(tree, expression),
                not_followed_by: Self::from_scanner(tree, not_followed_by),
            },

            ScannerDefinition::ZeroOrMore(expr) => Self::ZeroOrMore {
                name: None,
                expr: Self::from_scanner(tree, expr),
            },
        })
    }

    pub fn from_parser(
        tree: &'context CombinatorTree<'context>,
        parser: &ParserRef,
    ) -> &'context CombinatorNode<'context> {
        Self::from_parser_definition(tree, parser.name.clone(), &parser.definition)
    }

    pub fn from_precedence_parser(
        tree: &'context CombinatorTree<'context>,
        parser: &PrecedenceParserRef,
    ) -> &'context CombinatorNode<'context> {
        let operators: Vec<PrecedenceRuleOperator> = parser
            .operators
            .iter()
            .map(|definition| -> PrecedenceRuleOperator {
                PrecedenceRuleOperator {
                    name: definition.name.clone(),
                    model: definition.model,
                    operator: Self::from_parser(tree, &definition.operator),
                }
            })
            .collect();

        let primary_expression = Self::from_parser(tree, &parser.primary_expression);

        return tree.context.alloc_node(Self::PrecedenceExpressionRule {
            tree,
            operators,
            primary_expression,
        });
    }

    fn from_parser_definition(
        tree: &'context CombinatorTree<'context>,
        name: Option<String>,
        parser_definition: &ParserDefinition,
    ) -> &'context CombinatorNode<'context> {
        tree.context.alloc_node(match &parser_definition {
            ParserDefinition::Choice(exprs) => {
                let mut elements = exprs
                    .iter()
                    .map(|expr| Self::from_parser(tree, expr))
                    .collect::<Vec<_>>();

                if name.is_some() || elements.len() > 1 {
                    Self::Choice { name, elements }
                } else {
                    return elements.pop().unwrap();
                }
            }

            ParserDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => Self::DelimitedBy {
                name,
                open: tree.context.get_tree_by_name(&open.reference),
                expr: Self::from_parser(tree, expression),
                close: tree.context.get_tree_by_name(&close.reference),
            },

            ParserDefinition::OneOrMore(expr) => Self::OneOrMore {
                name,
                expr: Self::from_parser(tree, expr),
            },

            ParserDefinition::Optional(expr) => Self::Optional {
                expr: Self::from_parser(tree, expr),
            },

            ParserDefinition::Reference(name) => Self::Reference {
                tree: tree.context.get_tree_by_name(name),
            },

            ParserDefinition::SeparatedBy {
                expression,
                separator,
            } => Self::SeparatedBy {
                name,
                expr: Self::from_parser(tree, expression),
                separator: tree.context.get_tree_by_name(&separator.reference),
            },

            ParserDefinition::Sequence(exprs) => Self::Sequence {
                name,
                elements: exprs.iter().map(|e| Self::from_parser(tree, e)).collect(),
            },

            ParserDefinition::TerminatedBy {
                expression,
                terminator,
            } => Self::TerminatedBy {
                name,
                expr: Self::from_parser(tree, expression),
                terminator: tree.context.get_tree_by_name(&terminator.reference),
            },

            ParserDefinition::ZeroOrMore(expr) => Self::ZeroOrMore {
                name,
                expr: Self::from_parser(tree, expr),
            },
        })
    }

    pub fn first_set(&self) -> FirstSet {
        match self {
            Self::CharacterFilter { filter, .. } => FirstSet::from_char_set(filter.clone()),

            Self::TerminalTrie {
                trie: TerminalTrie { subtries, .. },
                ..
            } => FirstSet::multiple(subtries.keys().copied()),

            Self::DelimitedBy { open, .. } => open.first_set(),

            Self::PrecedenceExpressionRule {
                operators,
                primary_expression,
                ..
            } => operators
                .iter()
                .filter(|op| op.model == OperatorModel::UnaryPrefix)
                .fold(FirstSet::new(), |accum, expr| {
                    accum.union_with(expr.operator.first_set())
                })
                .union_with(primary_expression.first_set()),

            Self::Optional { expr }
            | Self::ZeroOrMore { expr, .. }
            | Self::Repeated { min: 0, expr, .. } => expr.first_set().with_epsilon(),

            Self::SeparatedBy {
                expr, separator, ..
            } => expr.first_set().follow_by(separator.first_set()),

            Self::TerminatedBy {
                expr, terminator, ..
            } => expr.first_set().follow_by(terminator.first_set()),

            Self::Reference { tree } => tree.first_set(),

            Self::Repeated { expr, .. }
            | Self::TrailingContext {
                expression: expr, ..
            }
            | Self::OneOrMore { expr, .. }
            | Self::Difference { minuend: expr, .. } => expr.first_set(),

            Self::Choice { elements, .. } => {
                elements.iter().fold(FirstSet::new(), |accum, expr| {
                    accum.union_with(expr.first_set())
                })
            }

            Self::Sequence { elements, .. } => {
                elements.iter().fold(FirstSet::epsilon(), |accum, expr| {
                    // have to do this check here to avoid infinite recursion
                    if accum.includes_epsilon {
                        accum.follow_by(expr.first_set())
                    } else {
                        accum
                    }
                })
            }
        }
    }
}
