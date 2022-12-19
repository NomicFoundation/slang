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
            EBNF::Choice(exprs) => Self::Choice {
                name,
                elements: exprs
                    .iter()
                    .map(|expr| Self::new(tree, expr, None))
                    .collect(),
            },

            EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                let expr = Self::new(tree, expr, None);
                let name = name.or_else(|| {
                    expr.name()
                        .map(|expr_name| format!("Delimited{}", expr_name,))
                });
                Self::DelimitedBy {
                    name,
                    open: open.clone(),
                    expr,
                    close: close.clone(),
                }
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => Self::Difference {
                minuend: Self::new(tree, minuend, None),
                subtrahend: Self::new(tree, subtrahend, None),
            },

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

            EBNF::Optional(expr) => Self::Optional {
                expr: Self::new(tree, expr, name),
            },

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
                Self::Repeated {
                    name,
                    expr,
                    min: *min,
                    max: *max,
                }
            }

            EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
                let expr = Self::new(tree, expr, None);
                let name = name.or_else(|| {
                    expr.name()
                        .map(|expr_name| format!("Separated{}", naming::pluralize(&expr_name),))
                });
                let separator = separator.clone();
                Self::SeparatedBy {
                    name,
                    expr,
                    separator,
                }
            }

            EBNF::Sequence(exprs) => Self::Sequence {
                name,
                elements: exprs.iter().map(|e| Self::new(tree, e, None)).collect(),
            },

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
