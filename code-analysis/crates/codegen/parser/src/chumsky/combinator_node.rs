use std::rc::Rc;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use codegen_schema::*;

use super::{
    character_filter::{CharacterFilter, CharacterFilterRef},
    code_fragments::CodeFragments,
    combinator_forest::CombinatorForest,
    combinator_tree::CombinatorTree,
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
    Expression {
        name: String,
        members: Vec<ProductionRef>,
    },
    ExpressionMember {
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

        if production.kind == ProductionKind::ExpressionRule {
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
        Rc::new(Self::Expression { name, members })
    }

    pub fn expression_member(
        name: String,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
        operator: CombinatorNodeRef,
        operator_model: OperatorModel,
    ) -> CombinatorNodeRef {
        Rc::new(Self::ExpressionMember {
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
            Self::Expression { name, .. } | Self::ExpressionMember { name, .. } => {
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

    pub fn to_lexer_code(&self, code: &mut CodeFragments) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeFragments) -> Option<Ident> {
            name.as_ref().map(|name| code.add_token_kind(name.clone()))
        }

        match self {
            Self::CharacterFilter { filter, name } => filter.to_lexer_code(name.as_ref(), code),

            Self::Choice { elements, name } => {
                let choices = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_choice!( #kind, #(#choices),* ))
                } else {
                    quote!(lex_choice!( #(#choices),* ))
                }
            }

            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let expr = expr.to_lexer_code(code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        lex_seq!(#kind, lex_terminal!(#open_kind, #open), #expr, lex_terminal!(#close_kind, #close))
                    )
                } else {
                    quote!(
                        lex_seq!(lex_terminal!(#open_kind, #open), #expr, lex_terminal!(#close_kind, #close))
                    )
                }
            }

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_lexer_code(code);
                let subtrahend = subtrahend.to_lexer_code(code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Expression { .. } => unreachable!(),

            Self::ExpressionMember { .. } => unreachable!(),

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_lexer_code(code);
                let lookahead = lookahead.to_lexer_code(code);
                quote!( #expr.then_ignore( #lookahead.rewind() ))
            }

            Self::OneOrMore { expr, name } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_one_or_more!(#kind, #expr))
                } else {
                    quote!(lex_one_or_more!(#expr))
                }
            }

            Self::Optional { expr, .. } => {
                let expr = expr.to_lexer_code(code);
                quote!(lex_optional!(#expr))
            }

            Self::Reference { production } => {
                let production_parser_name = naming::to_parser_name_ident(&production.name);
                match production.kind {
                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule
                    | ProductionKind::Trivia => {
                        unreachable!("Token productions can only reference other token productions")
                    }
                    ProductionKind::Token => quote!(lex_rule!(#production_parser_name)),
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_repeated!(#kind, #expr, #min, #max))
                } else {
                    quote!(lex_repeated!(#expr, #min, #max))
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let expr = expr.to_lexer_code(code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        lex_separated_by!(#kind, #expr, lex_terminal!(#separator_kind, #separator))
                    )
                } else {
                    quote!(lex_separated_by!(#expr, lex_terminal!(#separator_kind, #separator)))
                }
            }

            Self::Sequence { elements, name } => {
                let expr = elements
                    .iter()
                    .map(|e| e.to_lexer_code(code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_seq![ #kind, #(#expr),* ])
                } else {
                    quote!(lex_seq![ #(#expr),* ])
                }
            }

            Self::TerminalTrie { trie, .. } => trie.to_lexer_code(code),

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_lexer_code(code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(lex_zero_or_more!(#kind, #expr))
                } else {
                    quote!(lex_zero_or_more!(#expr))
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_trivia_code(
        &self,
        forest: &CombinatorForest,
        tree: &CombinatorTree,
        code: &mut CodeFragments,
    ) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeFragments) -> Option<Ident> {
            name.as_ref().map(|name| code.add_rule_kind(name.clone()))
        }

        match self {
            Self::CharacterFilter { filter, name } => filter.to_trivia_code(name.as_ref(), code),

            Self::Choice { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_trivia_code(forest, tree, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(choice!( #kind, #(#elements),* ))
                } else {
                    quote!(choice!( #(#elements),* ))
                }
            }

            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let expr = expr.to_trivia_code(forest, tree, code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        delimited_by!(#kind, trivia_terminal!(#open_kind, #open), #expr, trivia_terminal!(#close_kind, #close))
                    )
                } else {
                    quote!(
                        delimited_by!(trivia_terminal!(#open_kind, #open), #expr, trivia_terminal!(#close_kind, #close))
                    )
                }
            }

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_trivia_code(forest, tree, code);
                let subtrahend = subtrahend.to_trivia_code(forest, tree, code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Expression { .. } => unreachable!("Expressions are not allowed in trivia"),

            Self::ExpressionMember { .. } => {
                unreachable!("ExpressionMembers are not allowed in trivia")
            }

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                let lookahead = lookahead.to_trivia_code(forest, tree, code);
                quote!( #expr.then_ignore(#lookahead.rewind()) )
            }

            Self::OneOrMore { expr, name } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(one_or_more!(#kind, #expr))
                } else {
                    quote!(one_or_more!(#expr))
                }
            }

            Self::Optional { expr, .. } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                quote!(optional!(#expr))
            }

            Self::Reference { production } => {
                let production_parser_name = naming::to_parser_name_ident(&production.name);
                match production.kind {
                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule => unreachable!(
                        "Trivia productions can only reference trivia or token productions"
                    ),
                    ProductionKind::Trivia => quote!(rule!(#production_parser_name)),
                    ProductionKind::Token => quote!(trivia_token!(#production_parser_name)),
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(repeated!(#kind, #expr, #min, #max))
                } else {
                    quote!(repeated!(#expr, #min, #max))
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        separated_by!(#kind, #expr, trivia_terminal!(#separator_kind, #separator))
                    )
                } else {
                    quote!(separated_by!(#expr, trivia_terminal!(#separator_kind, #separator)))
                }
            }

            Self::Sequence { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_trivia_code(forest, tree, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(seq!( #kind, #(#elements),* ))
                } else {
                    quote!(seq!( #(#elements),* ))
                }
            }

            Self::TerminalTrie { trie, .. } => trie.to_trivia_code(code),

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_trivia_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(zero_or_more!(#kind, #expr))
                } else {
                    quote!(zero_or_more!(#expr))
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_parser_code(
        &self,
        forest: &CombinatorForest,
        tree: &CombinatorTree,
        code: &mut CodeFragments,
    ) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeFragments) -> Option<Ident> {
            name.as_ref().map(|name| code.add_rule_kind(name.clone()))
        }

        match self {
            Self::CharacterFilter { filter, name } => filter.to_parser_code(name.as_ref(), code),

            Self::Choice { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_parser_code(forest, tree, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(choice!( #kind, #(#elements),* ))
                } else {
                    quote!(choice!( #(#elements),* ))
                }
            }

            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let expr = expr.to_parser_code(forest, tree, code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        delimited_by!(#kind, terminal!(#open_kind, #open), #expr, terminal!(#close_kind, #close))
                    )
                } else {
                    quote!(
                        delimited_by!(terminal!(#open_kind, #open), #expr, terminal!(#close_kind, #close))
                    )
                }
            }

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_parser_code(forest, tree, code);
                let subtrahend = subtrahend.to_parser_code(forest, tree, code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Expression { members, .. } => {
                let first_parser_name = naming::to_parser_name_ident(&members[0].name);
                quote!(rule!(#first_parser_name))
            }

            Self::ExpressionMember {
                next_sibling,
                operator,
                operator_model,
                name,
                ..
            } => {
                let kind = code.add_rule_kind(name.clone());
                let operator = operator.to_parser_code(forest, tree, code);
                let next_sibling = next_sibling
                    .clone()
                    .map(|next| naming::to_parser_name_ident(&next.name));

                match operator_model {
                    OperatorModel::None => match next_sibling {
                        Some(next_sibling) => quote!( choice((#operator, #next_sibling.clone())) ),
                        None => operator,
                    },

                    OperatorModel::BinaryLeftAssociative => {
                        let next_sibling = next_sibling
                            .expect("Cannot have binary operator as last expression member");
                        quote!(left_associative_binary_expression!(#kind, #next_sibling, #operator))
                    }

                    OperatorModel::BinaryRightAssociative => {
                        let next_sibling = next_sibling
                            .expect("Cannot have binary operator as last expression member");
                        quote!(
                            right_associative_binary_expression!(#kind, #next_sibling, #operator)
                        )
                    }

                    OperatorModel::UnaryPrefix => {
                        let next_sibling = next_sibling
                            .expect("Cannot have unary operator as last expression member");
                        quote!(unary_prefix_expression!(#kind, #next_sibling, #operator))
                    }

                    OperatorModel::UnarySuffix => {
                        let next_sibling = next_sibling
                            .expect("Cannot have unary operator as last expression member");
                        quote!(unary_suffix_expression!(#kind, #next_sibling, #operator))
                    }
                }
            }

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_parser_code(forest, tree, code);
                let lookahead = lookahead.to_parser_code(forest, tree, code);
                quote!( #expr.then_ignore(#lookahead.rewind()) )
            }

            Self::OneOrMore { expr, name } => {
                let expr = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(one_or_more!(#kind, #expr))
                } else {
                    quote!(one_or_more!(#expr))
                }
            }

            Self::Optional { expr, .. } => {
                let expr = expr.to_parser_code(forest, tree, code);
                quote!(optional!(#expr))
            }

            Self::Reference { production } => {
                let production_parser_name = naming::to_parser_name_ident(&production.name);
                match production.kind {
                    ProductionKind::Rule
                    | ProductionKind::ExpressionRule
                    | ProductionKind::ExpressionMemberRule
                    | ProductionKind::Trivia => quote!(rule!(#production_parser_name)),
                    ProductionKind::Token => quote!(token!(#production_parser_name)),
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(repeated!(#kind, #expr, #min, #max))
                } else {
                    quote!(repeated!(#expr, #min, #max))
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let expr = expr.to_parser_code(forest, tree, code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(separated_by!(#kind, #expr, terminal!(#separator_kind, #separator)))
                } else {
                    quote!(separated_by!(#expr, terminal!(#separator_kind, #separator)))
                }
            }

            Self::Sequence { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_parser_code(forest, tree, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(seq!( #kind, #(#elements),* ))
                } else {
                    quote!(seq!( #(#elements),* ))
                }
            }

            Self::TerminalTrie { trie, .. } => trie.to_parser_code(code),

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_parser_code(forest, tree, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(zero_or_more!(#kind, #expr))
                } else {
                    quote!(zero_or_more!(#expr))
                }
            }
        }
    }
}
