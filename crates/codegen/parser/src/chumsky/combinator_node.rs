use codegen_schema::types::productions::{
    EBNFDelimitedBy, EBNFDifference, EBNFRepeat, EBNFSeparatedBy, ExpressionRef, ParserType,
    ProductionKind, EBNF,
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
        operators: Vec<PrecedenceRuleOperator<'context>>,
        trailing_rules: Vec<&'context CombinatorTree<'context>>,
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
                if let EBNF::Choice(exprs) = &expression.ebnf {
                    let mut trailing_rules = Vec::new();
                    let mut operators = Vec::new();
                    for expr in exprs {
                        if let EBNF::Reference(prod_name) = &expr.ebnf {
                            let operator_tree = tree.context.get_tree_by_name(prod_name);
                            if let Some(operator) = operator_tree.to_precedence_rule_operator(tree)
                            {
                                operators.push(operator);
                            } else {
                                trailing_rules.push(operator_tree);
                            }
                        } else {
                            unreachable!("Validation should have checked this: The precedence parser type is only applicable to a choice of references")
                        }
                    }
                    return tree.context.alloc_node(Self::PrecedenceRule {
                        tree,
                        operators,
                        trailing_rules,
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
            if let Some(filter) = CharacterFilter::new(tree, expression, true) {
                return tree
                    .context
                    .alloc_node(Self::CharacterFilter { name, filter });
            }
        }

        if let Some(trie) = TerminalTrie::new(tree, expression) {
            return tree.context.alloc_node(Self::TerminalTrie { trie });
        }

        tree.context.alloc_node(match &expression.ebnf {
            EBNF::Choice(exprs) => {
                // Terminals in choices are merged, and represented as a trie

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

            EBNF::DelimitedBy(EBNFDelimitedBy {
                open,
                expression,
                close,
            }) => Self::DelimitedBy {
                name,
                open: open.clone(),
                expr: Self::new(tree, expression),
                close: close.clone(),
            },

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => Self::Difference {
                minuend: Self::new(tree, minuend),
                subtrahend: Self::new(tree, subtrahend),
            },

            EBNF::Not(_) => {
                if let Some(filter) = CharacterFilter::new(tree, expression, true) {
                    Self::CharacterFilter { name, filter }
                } else {
                    unimplemented!("¬ is only supported on characters or sets thereof")
                }
            }

            EBNF::OneOrMore(expr) => Self::OneOrMore {
                name,
                expr: Self::new(tree, expr),
            },

            EBNF::Optional(expr) => Self::Optional {
                expr: Self::new(tree, expr),
            },

            EBNF::Range(_) => Self::CharacterFilter {
                name,
                filter: CharacterFilter::new(tree, expression, true).unwrap(),
            },

            EBNF::Reference(name) => Self::Reference {
                tree: tree
                    .context
                    .trees_by_name
                    .borrow()
                    .get(name)
                    .expect("Production not found"),
            },

            EBNF::Repeat(EBNFRepeat {
                expression,
                min,
                max,
            }) => Self::Repeated {
                name,
                expr: Self::new(tree, expression),
                min: *min,
                max: *max,
            },

            EBNF::SeparatedBy(EBNFSeparatedBy {
                expression,
                separator,
            }) => Self::SeparatedBy {
                name,
                expr: Self::new(tree, expression),
                separator: separator.clone(),
            },

            EBNF::Sequence(exprs) => Self::Sequence {
                name,
                elements: exprs.iter().map(|e| Self::new(tree, e)).collect(),
            },

            EBNF::Terminal(_) => Self::TerminalTrie {
                trie: TerminalTrie::new(tree, expression).unwrap(),
            },

            EBNF::ZeroOrMore(expr) => Self::ZeroOrMore {
                name,
                expr: Self::new(tree, expr),
            },
        })
    }

    pub fn has_named_structure(&self) -> bool {
        match self {
            Self::Reference { .. } | Self::PrecedenceRule { .. } => false,

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

            Self::PrecedenceRule { .. } => false,

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
