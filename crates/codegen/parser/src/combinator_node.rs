use codegen_schema::types::productions::{
    ExpressionParser, ExpressionRef, ParserType, ProductionKind,
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
    PrecedenceExpressionRule {
        tree: &'context CombinatorTree<'context>,
        operators: Vec<PrecedenceRuleOperator<'context>>,
        primary_expressions: Vec<&'context CombinatorTree<'context>>,
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
        trie: TerminalTrie,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorModel {
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

impl<'context> CombinatorNode<'context> {
    pub fn new(
        tree: &'context CombinatorTree<'context>,
        expression: &ExpressionRef,
    ) -> &'context CombinatorNode<'context> {
        if let Some(ParserType::Precedence) = expression.config.parser_type {
            if tree.production.kind == ProductionKind::Rule {
                if let ExpressionParser::Choice(exprs) = &expression.parser {
                    let mut primary_expressions = Vec::new();
                    let mut operators = Vec::new();
                    for expr in exprs {
                        if let ExpressionParser::Reference(prod_name) = &expr.parser {
                            let operator_tree = tree.context.get_tree_by_name(prod_name);
                            if let Some(operator) = operator_tree.to_precedence_rule_operator(tree)
                            {
                                operators.push(operator);
                            } else {
                                primary_expressions.push(operator_tree);
                            }
                        } else {
                            unreachable!("Validation should have checked this: The precedence parser type is only applicable to a choice of references")
                        }
                    }
                    return tree.context.alloc_node(Self::PrecedenceExpressionRule {
                        tree,
                        operators,
                        primary_expressions,
                    });
                } else {
                    unreachable!("Validation should have checked this: The precedence parser type is only applicable to a choice of references")
                }
            } else {
                unreachable!("Validation should have checked this: The precendence parser type is only applicable to rules")
            }
        }

        let name = expression.config.name.clone();

        if tree.production.kind == ProductionKind::Token {
            if let Some(filter) = CharSet::from_expression(tree, expression, true) {
                return tree
                    .context
                    .alloc_node(Self::CharacterFilter { name, filter });
            }

            if let Some(trie) = trie::from_expression(tree, expression) {
                return tree.context.alloc_node(Self::TerminalTrie { trie });
            }
        }

        tree.context.alloc_node(match &expression.parser {
            ExpressionParser::Choice(exprs) => {
                // Terminals in choices are merged, and represented as a trie

                enum TN<'c> {
                    Trie(trie::TerminalTrie),
                    Node(&'c CombinatorNode<'c>),
                }
                let mut elements = exprs
                    .iter()
                    .map(|expr| match trie::from_expression(tree, expr) {
                        None => TN::Node(Self::new(tree, expr)),
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
                    Self::Choice { name, elements }
                }
            }

            ExpressionParser::DelimitedBy {
                open,
                expression,
                close,
            } => Self::DelimitedBy {
                name,
                open: open.clone(),
                expr: Self::new(tree, expression),
                close: close.clone(),
            },

            ExpressionParser::Difference {
                minuend,
                subtrahend,
            } => Self::Difference {
                minuend: Self::new(tree, minuend),
                subtrahend: Self::new(tree, subtrahend),
            },

            ExpressionParser::Not(_) => {
                if let Some(filter) = CharSet::from_expression(tree, expression, true) {
                    Self::CharacterFilter { name, filter }
                } else {
                    unimplemented!("¬ is only supported on characters or sets thereof")
                }
            }

            ExpressionParser::OneOrMore(expr) => Self::OneOrMore {
                name,
                expr: Self::new(tree, expr),
            },

            ExpressionParser::Optional(expr) => Self::Optional {
                expr: Self::new(tree, expr),
            },

            ExpressionParser::Range { .. } => Self::CharacterFilter {
                name,
                filter: CharSet::from_expression(tree, expression, true).unwrap(),
            },

            ExpressionParser::Reference(name) => Self::Reference {
                tree: tree
                    .context
                    .trees_by_name
                    .borrow()
                    .get(name)
                    .expect("Production not found"),
            },

            ExpressionParser::Repeat {
                expression,
                min,
                max,
            } => Self::Repeated {
                name,
                expr: Self::new(tree, expression),
                min: *min,
                max: *max,
            },

            ExpressionParser::SeparatedBy {
                expression,
                separator,
            } => Self::SeparatedBy {
                name,
                expr: Self::new(tree, expression),
                separator: separator.clone(),
            },

            ExpressionParser::Sequence(exprs) => Self::Sequence {
                name,
                elements: exprs.iter().map(|e| Self::new(tree, e)).collect(),
            },

            ExpressionParser::Terminal(_) => {
                if let Some(filter) = CharSet::from_expression(tree, expression, true) {
                    Self::CharacterFilter { name, filter }
                } else {
                    Self::TerminalTrie {
                        trie: trie::from_expression(tree, expression).unwrap(),
                    }
                }
            }

            ExpressionParser::ZeroOrMore(expr) => Self::ZeroOrMore {
                name,
                expr: Self::new(tree, expr),
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
            | Self::OneOrMore { expr, .. }
            | Self::Lookahead { expr, .. }
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
