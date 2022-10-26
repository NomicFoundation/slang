use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use codegen_schema::*;

use super::{code_fragments::CodeFragments, naming, ProductionChumskyExtensions};

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
            EBNF::Not(child) => Self::from_expression(grammar, child).map(Self::negation),

            EBNF::Choice(children) => {
                let child_filters = children
                    .iter()
                    .filter_map(|c| Self::from_expression(grammar, c))
                    .collect::<Vec<_>>();
                if child_filters.len() == children.len() {
                    Some(CharacterFilter::disjunction(child_filters))
                } else {
                    None
                }
            }

            EBNF::Terminal(string) => {
                if string.chars().count() == 1 {
                    Some({
                        let char = string.chars().next().unwrap();
                        CharacterFilter::char(char)
                    })
                } else {
                    None
                }
            }

            EBNF::Range(EBNFRange { from, to }) => Some({
                let from = *from;
                let to = *to;
                CharacterFilter::range(from, to)
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
                        let nodes = vec![minuend, Self::negation(subtrahend)];
                        CharacterFilter::conjunction(nodes)
                    })
                } else {
                    None
                }
            }

            EBNF::DelimitedBy(_)
            | EBNF::OneOrMore(_)
            | EBNF::Optional(_)
            | EBNF::Repeat(_)
            | EBNF::SeparatedBy(_)
            | EBNF::Sequence(_)
            | EBNF::ZeroOrMore(_) => None,
        }
    }

    fn negation(child: CharacterFilterRef) -> CharacterFilterRef {
        Box::new(Self::Negation { child })
    }

    fn range(from: char, to: char) -> CharacterFilterRef {
        Box::new(Self::Range { from, to })
    }

    fn char(char: char) -> CharacterFilterRef {
        Box::new(Self::Char { char })
    }

    fn disjunction(nodes: Vec<CharacterFilterRef>) -> CharacterFilterRef {
        Box::new(Self::Disjunction { nodes })
    }

    fn conjunction(nodes: Vec<CharacterFilterRef>) -> CharacterFilterRef {
        Box::new(Self::Conjunction { nodes })
    }

    pub fn merged_with(self, other: CharacterFilterRef) -> CharacterFilterRef {
        let nodes = vec![Box::new(self), other];
        CharacterFilter::disjunction(nodes)
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

    pub fn to_lexer_code(&self, name: Option<&String>, code: &mut CodeFragments) -> TokenStream {
        self.to_code(name, code, "lex_")
    }

    pub fn to_trivia_code(&self, name: Option<&String>, code: &mut CodeFragments) -> TokenStream {
        self.to_code(name, code, "trivia_")
    }

    pub fn to_parser_code(&self, name: Option<&String>, code: &mut CodeFragments) -> TokenStream {
        self.to_code(name, code, "")
    }

    fn to_code(
        &self,
        name: Option<&String>,
        code: &mut CodeFragments,
        macro_prefix: &str,
    ) -> TokenStream {
        let macro_name = format_ident!("{}terminal", macro_prefix);
        if let CharacterFilter::Char { char } = self {
            let kind = code.add_terminal_kind(char.to_string());
            quote!(#macro_name!(#kind, #char))
        } else {
            let predicate = self.to_predicate(false);
            if let Some(kind) = name.map(|n| code.add_token_kind(n.clone())) {
                quote!(#macro_name!(#kind, |&c: &char| #predicate))
            } else {
                quote!(#macro_name!(|&c: &char| #predicate))
            }
        }
    }

    fn to_predicate(&self, negated: bool) -> TokenStream {
        match self {
            CharacterFilter::Negation { child } => child.to_predicate(!negated),

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
                let nodes = nodes.iter().map(|n| n.to_predicate(negated));
                if negated {
                    quote! ( #(#nodes)||* )
                } else {
                    quote! ( #(#nodes)&&* )
                }
            }

            CharacterFilter::Disjunction { nodes } => {
                let nodes = nodes.iter().map(|n| n.to_predicate(negated));
                if negated {
                    quote! ( #(#nodes)&&* )
                } else {
                    quote! ( #(#nodes)||* )
                }
            }
        }
    }
}
