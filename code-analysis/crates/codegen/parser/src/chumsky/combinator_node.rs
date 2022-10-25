use std::rc::Rc;

use codegen_schema::*;

use super::{
    character_filter::{CharacterFilter, CharacterFilterRef},
    naming,
    terminal_trie::TerminalTrie,
};

#[derive(Clone, Debug)]
pub enum CombinatorNode {
    CharacterFilter {
        name: Option<String>,
        filter: CharacterFilterRef,
    },
    Choice {
        name: Option<String>,
        elements: Vec<CombinatorNodeRef>,
    },
    DelimitedBy {
        name: Option<String>,
        open: String,
        expr: CombinatorNodeRef,
        close: String,
    },
    Difference {
        minuend: CombinatorNodeRef,
        subtrahend: CombinatorNodeRef,
    },
    PrecedenceRule {
        name: String,
        members: Vec<ProductionRef>,
    },
    PrecedenceRuleMember {
        name: String,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
        operator: CombinatorNodeRef,
        operator_model: OperatorModel,
    },
    #[allow(dead_code)]
    Lookahead {
        expr: CombinatorNodeRef,
        lookahead: CombinatorNodeRef,
    },
    OneOrMore {
        name: Option<String>,
        expr: CombinatorNodeRef,
    },
    Optional {
        name: Option<String>,
        expr: CombinatorNodeRef,
    },
    Reference {
        production: ProductionRef,
    },
    Repeated {
        name: Option<String>,
        expr: CombinatorNodeRef,
        min: usize,
        max: usize,
    },
    SeparatedBy {
        name: Option<String>,
        expr: CombinatorNodeRef,
        separator: String,
    },
    Sequence {
        name: Option<String>,
        elements: Vec<CombinatorNodeRef>,
    },
    TerminalTrie {
        name: Option<String>,
        trie: TerminalTrie,
    },
    ZeroOrMore {
        name: Option<String>,
        expr: CombinatorNodeRef,
    },
}

pub type CombinatorNodeRef = Rc<CombinatorNode>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorModel {
    None,
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

impl CombinatorNode {
    pub fn from_expression(
        grammar: &GrammarRef,
        production: &ProductionRef,
        expression: &ExpressionRef,
        inherited_name: Option<String>,
    ) -> CombinatorNodeRef {
        let name = expression.config.name.clone().or(inherited_name);

        if production.kind == ProductionKind::Token {
            if let Some(filter) = CharacterFilter::from_expression(grammar, expression) {
                return Self::character_filter(name.or_else(|| filter.default_name()), filter);
            }

            if let Some(terminal_trie) = TerminalTrie::from_expression(grammar, expression) {
                return Self::terminal_trie(
                    name.or_else(|| terminal_trie.default_name()),
                    terminal_trie,
                );
            }
        }

        if production.kind == ProductionKind::Rule
            && expression.config.parser_type == ParserType::Precedence
        {
            if let EBNF::Choice(exprs) = &expression.ebnf {
                let choices = exprs.iter().map(|e| {
                    if let EBNF::Reference(prod_name) = &e.ebnf {
                       grammar.get_production(prod_name)
                    } else {
                        unreachable!("Validation should have checked this: The Expression pattern is only applicable to a choice of references")
                    }
                }).collect();
                return Self::expression(production.name.clone(), choices);
            } else {
                unreachable!("Validation should have checked this: The Expression pattern is only applicable to a choice of references")
            }
        }

        match &expression.ebnf {
            EBNF::Choice(exprs) => {
                if production.kind != ProductionKind::Token {
                    Self::choice(
                        name,
                        exprs
                            .iter()
                            .map(|e| Self::from_expression(grammar, production, e, None))
                            .collect(),
                    )
                } else {
                    // Merge runs of TerminalTrees and CharacterFilters
                    let mut choices: Vec<CombinatorNodeRef> = vec![];
                    {
                        let mut current_terminal_tree: Option<TerminalTrie> = None;
                        let mut current_character_filter: Option<CharacterFilterRef> = None;
                        for e in exprs {
                            // Sub-expressions with a user-specified name aren't merged
                            if e.config.name.is_none() {
                                if let Some(cf) = CharacterFilter::from_expression(grammar, e) {
                                    if let Some(ctt) = current_terminal_tree {
                                        choices.push({
                                            let name = ctt.default_name();
                                            Self::terminal_trie(name, ctt)
                                        });
                                        current_terminal_tree = None
                                    };
                                    if let Some(ccf) = current_character_filter {
                                        current_character_filter = Some(ccf.merged_with(cf))
                                    } else {
                                        current_character_filter = Some(cf)
                                    }
                                    continue;
                                }
                                if let Some(tt) = TerminalTrie::from_expression(grammar, e) {
                                    if let Some(ccf) = current_character_filter {
                                        choices.push({
                                            let name = ccf.default_name();
                                            Self::character_filter(name, ccf)
                                        });
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
                                choices.push({
                                    let name = ccf.default_name();
                                    Self::character_filter(name, ccf)
                                });
                                current_character_filter = None
                            };

                            if let Some(ctt) = current_terminal_tree {
                                choices.push({
                                    let name = ctt.default_name();
                                    Self::terminal_trie(name, ctt)
                                });
                                current_terminal_tree = None
                            };

                            choices.push(Self::from_expression(grammar, production, e, None))
                        }

                        if let Some(ccf) = current_character_filter {
                            choices.push({
                                let name = ccf.default_name();
                                Self::character_filter(name, ccf)
                            });
                        };

                        if let Some(ctt) = current_terminal_tree {
                            choices.push({
                                let name = ctt.default_name();
                                Self::terminal_trie(name, ctt)
                            });
                        };
                    }
                    Self::choice(name, choices)
                }
            }

            EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::delimited_by(
                    name.or_else(|| {
                        expr.name().map(|expr_name| {
                            format!(
                                "{}_and_{}_and_{}",
                                naming::name_of_terminal_string(open),
                                expr_name,
                                naming::name_of_terminal_string(close)
                            )
                        })
                    }),
                    open.clone(),
                    expr,
                    close.clone(),
                )
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                let minuend = Self::from_expression(grammar, production, minuend, None);
                let subtrahend = Self::from_expression(grammar, production, subtrahend, None);
                Self::difference(minuend, subtrahend)
            }

            EBNF::Not(_) => {
                if let Some(filter) = CharacterFilter::from_expression(grammar, expression) {
                    Self::character_filter(name.or_else(|| filter.default_name()), filter)
                } else {
                    unimplemented!("¬ is only supported on characters or sets thereof")
                }
            }

            EBNF::OneOrMore(expr) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::one_or_more(
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name))),
                    expr,
                )
            }

            EBNF::Optional(expr) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::optional(name.or_else(|| expr.name()), expr)
            }

            EBNF::Range(_) => {
                let filter = CharacterFilter::from_expression(grammar, expression).unwrap();
                Self::character_filter(name.or_else(|| filter.default_name()), filter)
            }

            EBNF::Reference(name) => Self::reference(grammar.get_production(&name)),

            EBNF::Repeat(EBNFRepeat { expr, min, max }) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::repeat(
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name))),
                    expr,
                    *min,
                    *max,
                )
            }

            EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::separated_by(
                    name.or_else(|| {
                        expr.name().map(|expr_name| {
                            format!(
                                "{}_and_{}",
                                naming::pluralize(&expr_name),
                                naming::pluralize(&naming::name_of_terminal_string(&separator))
                            )
                        })
                    }),
                    expr,
                    separator.clone(),
                )
            }

            EBNF::Sequence(exprs) => {
                let exprs = exprs
                    .iter()
                    .map(|e| Self::from_expression(grammar, production, e, None))
                    .collect();
                Self::sequence(name, exprs)
            }

            EBNF::Terminal(_) => {
                let terminal_trie = TerminalTrie::from_expression(grammar, expression).unwrap();
                Self::terminal_trie(name.or_else(|| terminal_trie.default_name()), terminal_trie)
            }

            EBNF::ZeroOrMore(expr) => {
                let expr = Self::from_expression(grammar, production, expr, None);
                Self::zero_or_more(
                    name.or_else(|| expr.name().map(|expr_name| naming::pluralize(&expr_name))),
                    expr,
                )
            }
        }
    }

    pub fn character_filter(name: Option<String>, filter: CharacterFilterRef) -> CombinatorNodeRef {
        Rc::new(Self::CharacterFilter { name, filter })
    }

    pub fn choice(name: Option<String>, elements: Vec<CombinatorNodeRef>) -> CombinatorNodeRef {
        Rc::new(Self::Choice { name, elements })
    }

    pub fn delimited_by(
        name: Option<String>,
        open: String,
        expr: CombinatorNodeRef,
        close: String,
    ) -> CombinatorNodeRef {
        Rc::new(Self::DelimitedBy {
            name,
            open,
            expr,
            close,
        })
    }

    pub fn difference(
        minuend: CombinatorNodeRef,
        subtrahend: CombinatorNodeRef,
    ) -> CombinatorNodeRef {
        Rc::new(Self::Difference {
            minuend,
            subtrahend,
        })
    }

    pub fn expression(name: String, members: Vec<ProductionRef>) -> CombinatorNodeRef {
        Rc::new(Self::PrecedenceRule { name, members })
    }

    pub fn expression_member(
        name: String,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
        operator: CombinatorNodeRef,
        operator_model: OperatorModel,
    ) -> CombinatorNodeRef {
        Rc::new(Self::PrecedenceRuleMember {
            name,
            parent,
            next_sibling,
            operator,
            operator_model,
        })
    }

    #[allow(dead_code)]
    pub fn lookahead(expr: CombinatorNodeRef, lookahead: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::Lookahead { expr, lookahead })
    }

    pub fn one_or_more(name: Option<String>, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::OneOrMore { name, expr })
    }

    pub fn optional(name: Option<String>, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::Optional { name, expr })
    }

    pub fn reference(production: ProductionRef) -> CombinatorNodeRef {
        Rc::new(Self::Reference { production })
    }

    pub fn repeat(
        name: Option<String>,
        expr: CombinatorNodeRef,
        min: usize,
        max: usize,
    ) -> CombinatorNodeRef {
        Rc::new(Self::Repeated {
            name,
            expr,
            min,
            max,
        })
    }

    pub fn separated_by(
        name: Option<String>,
        expr: CombinatorNodeRef,
        separator: String,
    ) -> CombinatorNodeRef {
        Rc::new(Self::SeparatedBy {
            name,
            expr,
            separator,
        })
    }

    pub fn sequence(name: Option<String>, elements: Vec<CombinatorNodeRef>) -> CombinatorNodeRef {
        Rc::new(Self::Sequence { name, elements })
    }

    pub fn terminal_trie(name: Option<String>, trie: TerminalTrie) -> CombinatorNodeRef {
        Rc::new(Self::TerminalTrie { name, trie })
    }

    pub fn zero_or_more(name: Option<String>, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::ZeroOrMore { name, expr })
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::PrecedenceRule { name, .. } | Self::PrecedenceRuleMember { name, .. } => {
                Some(name.clone())
            }

            Self::Reference { production } => Some(production.name.clone()),

            Self::Difference { minuend, .. } => minuend.name(),
            Self::Lookahead { expr, .. } => expr.name(),

            Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::DelimitedBy { name, .. }
            | Self::OneOrMore { name, .. }
            | Self::Optional { name, .. }
            | Self::Repeated { name, .. }
            | Self::SeparatedBy { name, .. }
            | Self::Sequence { name, .. }
            | Self::TerminalTrie { name, .. }
            | Self::ZeroOrMore { name, .. } => name.clone(),
        }
    }
}
