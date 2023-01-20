use codegen_schema::types::productions::{
    ExpressionParser, ExpressionRef, ParserType, ProductionKind,
};
use itertools::Itertools;

use super::{
    character_filter::CharacterFilter, combinator_tree::CombinatorTree, terminal_trie::TerminalTrie,
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
    ) -> &'context CombinatorNode<'context> {
        let name = expression.config.name.clone();

        // Terminals in choices are merged, and represented as a trie

        if tree.production.kind == ProductionKind::Token {
            if let Some(filter) = CharacterFilter::new(tree, expression, true) {
                return tree
                    .context
                    .alloc_node(Self::CharacterFilter { name, filter });
            }
        }

        if let Some(trie) = TerminalTrie::new(tree, expression) {
            return tree.context.alloc_node(Self::TerminalTrie { trie });
        }

        if tree.production.kind == ProductionKind::Rule {
            if let Some(ParserType::Precedence) = expression.config.parser_type {
                if let ExpressionParser::Choice(exprs) = &expression.parser {
                    let members = exprs.iter().map(|e| {
                    if let ExpressionParser::Reference(prod_name) = &e.parser {
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
        }

        tree.context.alloc_node(match &expression.parser {
            ExpressionParser::Choice(exprs) => {
                enum TN<'c> {
                    Trie(TerminalTrie),
                    Node(&'c CombinatorNode<'c>),
                }
                let elements = exprs
                    .iter()
                    .map(|expr| match TerminalTrie::new(tree, expr) {
                        None => TN::Node(Self::new(tree, expr)),
                        Some(trie) => TN::Trie(trie),
                    })
                    .coalesce(|prev, curr| match (prev, curr) {
                        (TN::Trie(prev), TN::Trie(curr)) => Ok(TN::Trie(prev.merged_with(curr))),
                        pair => Err(pair),
                    })
                    .map(|either| match either {
                        TN::Node(node) => node,
                        TN::Trie(trie) => tree.context.alloc_node(Self::TerminalTrie { trie }),
                    })
                    .collect();
                Self::Choice { name, elements }
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
                if let Some(filter) = CharacterFilter::new(tree, expression, true) {
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
                filter: CharacterFilter::new(tree, expression, true).unwrap(),
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

            ExpressionParser::Terminal(_) => Self::TerminalTrie {
                trie: TerminalTrie::new(tree, expression).unwrap(),
            },

            ExpressionParser::ZeroOrMore(expr) => Self::ZeroOrMore {
                name,
                expr: Self::new(tree, expr),
            },
        })
    }

    pub fn has_named_structure(&self) -> bool {
        match self {
            Self::Reference { .. }
            | Self::PrecedenceRule { .. }
            | Self::PrecedenceRuleMember { .. } => false,

            Self::CharacterFilter { name, .. } => name.is_some(),

            Self::TerminalTrie { trie } => trie.has_named_structure(),

            Self::Choice { name, elements } | Self::Sequence { name, elements } => {
                name.is_some() || elements.iter().any(|e| e.has_named_structure())
            }

            Self::DelimitedBy { name, expr, .. }
            | Self::OneOrMore { name, expr }
            | Self::Repeated { name, expr, .. }
            | Self::SeparatedBy { name, expr, .. }
            | Self::ZeroOrMore { name, expr } => name.is_some() || expr.has_named_structure(),

            Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::Optional { expr } => expr.has_named_structure(),
        }
    }

    pub fn can_be_empty(&self) -> bool {
        match self {
            Self::CharacterFilter { .. } | Self::TerminalTrie { .. } | Self::DelimitedBy { .. } => {
                false
            }

            Self::Optional { .. } | Self::ZeroOrMore { .. } => true,

            Self::Repeated { expr, min, .. } => *min == 0 || expr.can_be_empty(),

            Self::Reference { tree } => tree.can_be_empty(),

            Self::PrecedenceRule { members, .. } => members[0].can_be_empty(),

            // TODO: Maybe next_sibling shouldn't be optional?
            Self::PrecedenceRuleMember { next_sibling, .. } => {
                next_sibling.map(|ns| ns.can_be_empty()).unwrap_or(true)
            }

            // TODO: choice should limit members to those that cannot be empty
            Self::Choice { elements, .. } => elements.iter().any(|e| e.can_be_empty()),

            Self::Sequence { elements, .. } => elements.iter().all(|e| e.can_be_empty()),

            Self::OneOrMore { expr, .. }
            | Self::SeparatedBy { expr, .. }
            | Self::Lookahead { expr, .. }
            | Self::Difference { minuend: expr, .. } => expr.can_be_empty(),
        }
    }
}
