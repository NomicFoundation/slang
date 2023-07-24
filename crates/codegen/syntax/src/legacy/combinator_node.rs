use codegen_schema::types::{
    OperatorModel, ParserDefinition, ParserRef, PrecedenceParserRef, ScannerDefinition, ScannerRef,
};
use itertools::Itertools;

use super::{
    char_first_set::CharFirstSet,
    char_set::CharSet,
    combinator_tree::CombinatorTree,
    trie::{self, TerminalTrie},
};

pub enum CombinatorNode<'context> {
    CharacterFilter {
        filter: CharSet,
    },
    Choice {
        nodes: Vec<&'context CombinatorNode<'context>>,
    },
    DelimitedBy {
        open: &'context CombinatorTree<'context>,
        node: &'context CombinatorNode<'context>,
        close: &'context CombinatorTree<'context>,
    },
    Difference {
        minuend: &'context CombinatorNode<'context>,
        subtrahend: &'context CombinatorNode<'context>,
    },
    OneOrMore {
        node: &'context CombinatorNode<'context>,
    },
    Optional {
        node: &'context CombinatorNode<'context>,
    },
    PrecedenceParser {
        tree: &'context CombinatorTree<'context>,
        operator_expressions: Vec<OperatorExpression<'context>>,
        primary_expression: &'context CombinatorNode<'context>,
    },
    Reference {
        tree: &'context CombinatorTree<'context>,
    },
    SeparatedBy {
        node: &'context CombinatorNode<'context>,
        separator: &'context CombinatorTree<'context>,
    },
    Sequence {
        nodes: Vec<&'context CombinatorNode<'context>>,
    },
    TerminalTrie {
        trie: TerminalTrie,
    },
    TerminatedBy {
        node: &'context CombinatorNode<'context>,
        terminator: &'context CombinatorTree<'context>,
    },
    TrailingContext {
        node: &'context CombinatorNode<'context>,
        not_followed_by: &'context CombinatorNode<'context>,
    },
    ZeroOrMore {
        node: &'context CombinatorNode<'context>,
    },
}

pub struct OperatorExpression<'context> {
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
            return tree.context.alloc_node(Self::CharacterFilter { filter });
        }

        if let Some(trie) = trie::from_scanner(tree, scanner.clone()) {
            return tree.context.alloc_node(Self::TerminalTrie { trie });
        }

        tree.context.alloc_node(match &scanner.definition {
            ScannerDefinition::Choice(scanners) => {
                // (unversioned) terminals in choices are merged, and represented as a trie

                enum TN<'c> {
                    Trie(trie::TerminalTrie),
                    Node(&'c CombinatorNode<'c>),
                }
                let mut nodes = scanners
                    .iter()
                    .map(|scanner| match trie::from_scanner(tree, scanner.clone()) {
                        None => TN::Node(Self::from_scanner(tree, scanner)),
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
                if nodes.len() == 1 {
                    return nodes.pop().unwrap();
                } else {
                    Self::Choice { nodes }
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
                    Self::CharacterFilter { filter }
                } else {
                    unimplemented!("'Not' is only supported on characters or sets thereof")
                }
            }

            ScannerDefinition::OneOrMore(scanner) => Self::OneOrMore {
                node: Self::from_scanner(tree, scanner),
            },

            ScannerDefinition::Optional(scanner) => Self::Optional {
                node: Self::from_scanner(tree, scanner),
            },

            ScannerDefinition::Range { .. } => Self::CharacterFilter {
                filter: CharSet::from_scanner(tree, scanner.clone()).unwrap(),
            },

            ScannerDefinition::Reference(name) => Self::Reference {
                tree: tree.context.get_tree_by_name(name),
            },

            ScannerDefinition::Sequence(scanners) => Self::Sequence {
                nodes: scanners
                    .iter()
                    .map(|scanner| Self::from_scanner(tree, scanner))
                    .collect(),
            },

            ScannerDefinition::Terminal(_) => {
                if let Some(filter) = CharSet::from_scanner(tree, scanner.clone()) {
                    Self::CharacterFilter { filter }
                } else {
                    Self::TerminalTrie {
                        trie: trie::from_scanner(tree, scanner.clone()).unwrap(),
                    }
                }
            }

            ScannerDefinition::TrailingContext {
                scanner,
                not_followed_by,
            } => Self::TrailingContext {
                node: Self::from_scanner(tree, scanner),
                not_followed_by: Self::from_scanner(tree, not_followed_by),
            },

            ScannerDefinition::ZeroOrMore(scanner) => Self::ZeroOrMore {
                node: Self::from_scanner(tree, scanner),
            },
        })
    }

    pub fn from_parser(
        tree: &'context CombinatorTree<'context>,
        parser: &ParserRef,
    ) -> &'context CombinatorNode<'context> {
        Self::from_parser_definition(tree, &parser.definition)
    }

    pub fn from_precedence_parser(
        tree: &'context CombinatorTree<'context>,
        parser: &PrecedenceParserRef,
    ) -> &'context CombinatorNode<'context> {
        let operator_expressions: Vec<OperatorExpression> = parser
            .operator_expressions
            .iter()
            .map(|expression| -> OperatorExpression {
                OperatorExpression {
                    name: expression.name.clone(),
                    model: expression.model,
                    operator: Self::from_parser(tree, &expression.operator),
                }
            })
            .collect();

        let primary_expression = Self::from_parser(tree, &parser.primary_expression);

        return tree.context.alloc_node(Self::PrecedenceParser {
            tree,
            operator_expressions,
            primary_expression,
        });
    }

    fn from_parser_definition(
        tree: &'context CombinatorTree<'context>,
        parser_definition: &ParserDefinition,
    ) -> &'context CombinatorNode<'context> {
        tree.context.alloc_node(match &parser_definition {
            ParserDefinition::Choice(parsers) => {
                let mut nodes = parsers
                    .iter()
                    .map(|parser| Self::from_parser(tree, parser))
                    .collect::<Vec<_>>();

                if nodes.len() > 1 {
                    Self::Choice { nodes }
                } else {
                    return nodes.pop().unwrap();
                }
            }

            ParserDefinition::DelimitedBy {
                open,
                parser,
                close,
            } => Self::DelimitedBy {
                open: tree.context.get_tree_by_name(&open.reference),
                node: Self::from_parser(tree, parser),
                close: tree.context.get_tree_by_name(&close.reference),
            },

            ParserDefinition::OneOrMore(parser) => Self::OneOrMore {
                node: Self::from_parser(tree, parser),
            },

            ParserDefinition::Optional(parser) => Self::Optional {
                node: Self::from_parser(tree, parser),
            },

            ParserDefinition::Reference(name) => Self::Reference {
                tree: tree.context.get_tree_by_name(name),
            },

            ParserDefinition::SeparatedBy { parser, separator } => Self::SeparatedBy {
                node: Self::from_parser(tree, parser),
                separator: tree.context.get_tree_by_name(&separator.reference),
            },

            ParserDefinition::Sequence(parsers) => Self::Sequence {
                nodes: parsers
                    .iter()
                    .map(|parser| Self::from_parser(tree, parser))
                    .collect(),
            },

            ParserDefinition::TerminatedBy { parser, terminator } => Self::TerminatedBy {
                node: Self::from_parser(tree, parser),
                terminator: tree.context.get_tree_by_name(&terminator.reference),
            },

            ParserDefinition::ZeroOrMore(parser) => Self::ZeroOrMore {
                node: Self::from_parser(tree, parser),
            },
        })
    }

    pub fn char_first_set(&self) -> CharFirstSet {
        match self {
            Self::CharacterFilter { filter, .. } => CharFirstSet::from_char_set(filter.clone()),

            Self::TerminalTrie {
                trie: TerminalTrie { subtries, .. },
                ..
            } => CharFirstSet::multiple(subtries.keys().copied()),

            Self::DelimitedBy { open, .. } => open.char_first_set(),

            Self::PrecedenceParser {
                operator_expressions,
                primary_expression,
                ..
            } => operator_expressions
                .iter()
                .filter(|expression| expression.model == OperatorModel::UnaryPrefix)
                .fold(CharFirstSet::new(), |accum, expression| {
                    accum.union_with(expression.operator.char_first_set())
                })
                .union_with(primary_expression.char_first_set()),

            Self::Optional { node } | Self::ZeroOrMore { node, .. } => {
                node.char_first_set().with_epsilon()
            }

            Self::SeparatedBy {
                node, separator, ..
            } => node.char_first_set().follow_by(separator.char_first_set()),

            Self::TerminatedBy {
                node, terminator, ..
            } => node.char_first_set().follow_by(terminator.char_first_set()),

            Self::Reference { tree } => tree.char_first_set(),

            Self::TrailingContext { node, .. }
            | Self::OneOrMore { node, .. }
            | Self::Difference { minuend: node, .. } => node.char_first_set(),

            Self::Choice { nodes, .. } => nodes.iter().fold(CharFirstSet::new(), |accum, node| {
                accum.union_with(node.char_first_set())
            }),

            Self::Sequence { nodes, .. } => {
                nodes.iter().fold(CharFirstSet::epsilon(), |accum, node| {
                    // have to do this check here to avoid infinite recursion
                    if accum.includes_epsilon {
                        accum.follow_by(node.char_first_set())
                    } else {
                        accum
                    }
                })
            }
        }
    }
}
