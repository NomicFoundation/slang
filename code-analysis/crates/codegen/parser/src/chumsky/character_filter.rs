use proc_macro2::TokenStream;
use quote::quote;

use codegen_schema::*;

use super::{combinator_node, naming, ProductionChumskyExtensions};

#[derive(Clone, Debug)]
pub enum CharacterFilter {
    Negation { child: CharacterFilterRef },
    Range { from: char, to: char },
    Char { char: char },
    Disjunction { nodes: Vec<CharacterFilterRef> },
    Conjunction { nodes: Vec<CharacterFilterRef> },
}

pub type CharacterFilterRef = Box<CharacterFilter>;

impl CharacterFilter {
    pub fn from_expression(
        grammar: &Grammar,
        expression: &ExpressionRef,
    ) -> Option<CharacterFilterRef> {
        match &expression.ebnf {
            EBNF::Not(child) => Self::from_expression(grammar, child).map(Self::new_negation),

            EBNF::Choice(children) => {
                if expression.config.merge == Some(false) {
                    return None;
                }
                let child_filters = children
                    .iter()
                    .filter_map(|c| Self::from_expression(grammar, c))
                    .collect::<Vec<_>>();
                if child_filters.len() == children.len() {
                    Some(CharacterFilter::new_disjunction(child_filters))
                } else {
                    None
                }
            }

            EBNF::Terminal(string) => {
                if string.chars().count() == 1 {
                    Some({
                        let char = string.chars().next().unwrap();
                        CharacterFilter::new_char(char)
                    })
                } else {
                    None
                }
            }

            EBNF::Range(EBNFRange { from, to }) => Some({
                let from = *from;
                let to = *to;
                CharacterFilter::new_range(from, to)
            }),

            EBNF::Reference(name) => {
                Self::from_expression(
                    grammar,
                    &grammar.get_production(name).expression_to_generate(),
                )
                // None
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                if let (Some(minuend), Some(subtrahend)) = (
                    Self::from_expression(grammar, minuend),
                    Self::from_expression(grammar, subtrahend),
                ) {
                    Some({
                        let nodes = vec![minuend, Self::new_negation(subtrahend)];
                        CharacterFilter::new_conjunction(nodes)
                    })
                } else {
                    None
                }
            }

            EBNF::DelimitedBy(_)
            | EBNF::End
            | EBNF::OneOrMore(_)
            | EBNF::Optional(_)
            | EBNF::Repeat(_)
            | EBNF::SeparatedBy(_)
            | EBNF::Sequence(_)
            | EBNF::ZeroOrMore(_) => None,
        }
    }

    fn new_negation(child: CharacterFilterRef) -> CharacterFilterRef {
        Box::new(Self::Negation { child })
    }

    fn new_range(from: char, to: char) -> CharacterFilterRef {
        Box::new(Self::Range { from, to })
    }

    fn new_char(char: char) -> CharacterFilterRef {
        Box::new(Self::Char { char })
    }

    fn new_disjunction(nodes: Vec<CharacterFilterRef>) -> CharacterFilterRef {
        Box::new(Self::Disjunction { nodes })
    }

    fn new_conjunction(nodes: Vec<CharacterFilterRef>) -> CharacterFilterRef {
        Box::new(Self::Conjunction { nodes })
    }

    pub fn default_name(&self) -> Option<String> {
        if let Self::Negation { child } = self {
            child.default_name().map(|n| format!("not_{}", n))
        } else if let Self::Char { char, .. } = self {
            Some(naming::name_of_terminal_char(*char))
        } else {
            None
        }
    }

    pub fn merged_with(self, other: CharacterFilterRef) -> CharacterFilterRef {
        let nodes = vec![Box::new(self), other];
        CharacterFilter::new_disjunction(nodes)
    }

    pub fn to_generated_code(
        &self,
        name: String,
        with_trivia: bool,
    ) -> combinator_node::CodeForNode {
        let mut result: combinator_node::CodeForNode = Default::default();

        let kind = naming::to_kind_ident(&name, &name);
        result.cst_token_part_kinds.insert(kind.clone());
        let (cst_parser, ast_parser) = if let CharacterFilter::Char { char } = self {
            (
                quote!(just(#char).to(Node::new_token_part(TokenPartKind::#kind, 1))),
                quote!(just(#char).to(FixedSizeTerminal::<1>()) ),
            )
        } else {
            let predicate = self.to_parser_predicate(false);
            (
                quote!( filter(|&c: &char| #predicate).to(Node::new_token_part(TokenPartKind::#kind, 1))),
                quote!( filter(|&c: &char| #predicate).to(FixedSizeTerminal::<1>())),
            )
        };

        if with_trivia {
            result.cst_parser_impl_fragment = quote!(
                leading_trivia_parser.clone().then(#cst_parser).then(trailing_trivia_parser.clone())
                .map(Node::new_with_trivia)
            );
            result.ast_parser_impl_fragment = quote!(
                leading_trivia_parser.clone().then(#ast_parser).then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia { leading_trivia, terminal, trailing_trivia })
            );
            result.ast_parser_type = quote!(FixedSizeTerminalWithTrivia<1>);
        } else {
            result.cst_parser_impl_fragment = cst_parser;
            result.ast_parser_impl_fragment = ast_parser;
            result.ast_parser_type = quote!(FixedSizeTerminal<1>);
        }

        result
    }

    fn to_parser_predicate(&self, negated: bool) -> TokenStream {
        match self {
            CharacterFilter::Negation { child } => child.to_parser_predicate(!negated),

            CharacterFilter::Range { from, to } => {
                if negated {
                    quote!( (c < #from || #to < c) )
                } else {
                    quote!( (#from <= c && c <= #to) )
                }
            }

            CharacterFilter::Char { char } => {
                if negated {
                    quote!( c != #char )
                } else {
                    quote!( c == #char )
                }
            }

            CharacterFilter::Conjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate(negated));
                if negated {
                    quote! ( #(#nodes)||* )
                } else {
                    quote! ( #(#nodes)&&* )
                }
            }

            CharacterFilter::Disjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_parser_predicate(negated));
                if negated {
                    quote! ( #(#nodes)&&* )
                } else {
                    quote! ( #(#nodes)||* )
                }
            }
        }
    }
}
