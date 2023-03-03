use codegen_schema::types::{
    parser::{ParserDefinition, ParserRef},
    precedence_parser::{OperatorModel, PrecedenceParserRef},
    scanner::{ScannerDefinition, ScannerRef},
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
        open: String,
        expr: &'context CombinatorNode<'context>,
        close: String,
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
        primary_expressions: Vec<&'context CombinatorTree<'context>>,
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
        trie: TerminalTrie,
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
                // Terminals in choices are merged, and represented as a trie

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

            ScannerDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => Self::DelimitedBy {
                name: None,
                open: open.clone(),
                expr: Self::from_scanner(tree, expression),
                close: close.clone(),
            },

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
                    unimplemented!("¬ is only supported on characters or sets thereof")
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

            ScannerDefinition::SeparatedBy {
                expression,
                separator,
            } => Self::SeparatedBy {
                name: None,
                expr: Self::from_scanner(tree, expression),
                separator: separator.clone(),
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
        let primary_expressions: Vec<&CombinatorTree> = parser
            .definition
            .primary_expressions
            .iter()
            .map(|r| tree.context.get_tree_by_name(&r.reference))
            .collect();
        let operators: Vec<PrecedenceRuleOperator> = parser
            .definition
            .operators
            .iter()
            .map(|operator| -> PrecedenceRuleOperator {
                PrecedenceRuleOperator {
                    name: operator.name.clone(),
                    model: operator.model,
                    operator: Self::from_parser_definition(tree, None, &operator.definition),
                }
            })
            .collect();
        return tree.context.alloc_node(Self::PrecedenceExpressionRule {
            tree,
            operators,
            primary_expressions,
        });
    }

    fn from_parser_definition(
        tree: &'context CombinatorTree<'context>,
        name: Option<String>,
        parser_definition: &ParserDefinition,
    ) -> &'context CombinatorNode<'context> {
        tree.context.alloc_node(match &parser_definition {
            ParserDefinition::Choice(exprs) => {
                // Terminals in choices are merged, and represented as a trie

                enum TN<'c> {
                    Trie(trie::TerminalTrie),
                    Node(&'c CombinatorNode<'c>),
                }
                let mut elements = exprs
                    .iter()
                    .map(|expr| {
                        match trie::from_parser_definition(
                            tree,
                            expr.name.clone(),
                            &expr.definition,
                        ) {
                            None => TN::Node(Self::from_parser(tree, expr)),
                            Some(trie) => TN::Trie(trie),
                        }
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
                    Self::Choice { name, elements }
                }
            }

            ParserDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => Self::DelimitedBy {
                name,
                open: open.clone(),
                expr: Self::from_parser(tree, expression),
                close: close.clone(),
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

            ParserDefinition::Repeat {
                expression,
                min,
                max,
            } => Self::Repeated {
                name,
                expr: Self::from_parser(tree, expression),
                min: *min,
                max: *max,
            },

            ParserDefinition::SeparatedBy {
                expression,
                separator,
            } => Self::SeparatedBy {
                name,
                expr: Self::from_parser(tree, expression),
                separator: separator.clone(),
            },

            ParserDefinition::Sequence(exprs) => Self::Sequence {
                name,
                elements: exprs.iter().map(|e| Self::from_parser(tree, e)).collect(),
            },

            ParserDefinition::Terminal(_) => Self::TerminalTrie {
                trie: trie::from_parser_definition(tree, name, parser_definition).unwrap(),
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

            Self::DelimitedBy { open, .. } => FirstSet::single(open.chars().next().unwrap()),

            Self::PrecedenceExpressionRule {
                operators,
                primary_expressions,
                ..
            } => primary_expressions.iter().fold(
                operators
                    .iter()
                    .filter(|op| op.model == OperatorModel::UnaryPrefix)
                    .fold(FirstSet::new(), |accum, expr| {
                        accum.union_with(expr.operator.first_set())
                    }),
                |accum, expr| accum.union_with(expr.root_node.get().unwrap().first_set()),
            ),

            Self::Optional { expr }
            | Self::ZeroOrMore { expr, .. }
            | Self::Repeated { min: 0, expr, .. } => expr.first_set().with_epsilon(),

            Self::SeparatedBy {
                expr, separator, ..
            } => expr
                .first_set()
                .follow_by(FirstSet::single(separator.chars().next().unwrap())),

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
