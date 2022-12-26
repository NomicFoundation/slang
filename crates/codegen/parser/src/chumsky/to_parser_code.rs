use std::cell::RefCell;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use codegen_schema::*;

use crate::chumsky::combinator_node::OperatorModel;

use super::{
    character_filter::CharacterFilter, code_generator::CodeGenerator,
    combinator_node::CombinatorNode, terminal_trie::TerminalTrie,
};

impl<'context> CharacterFilter<'context> {
    pub fn to_parser_code(
        &self,
        name: String,
        is_trivia: bool,
        code: &mut CodeGenerator,
    ) -> TokenStream {
        let kind = code.add_token_kind(name);
        let macro_name = if is_trivia {
            "trivia_terminal"
        } else {
            "terminal"
        };
        if let CharacterFilter::Char { char } = self {
            quote!(#macro_name!(#kind, #char))
        } else {
            let predicate = self.to_predicate(false);
            quote!(#macro_name!(#kind, |&c: &char| #predicate))
        }
    }
}

impl TerminalTrie {
    pub fn to_parser_code(&self, is_trivia: bool, code: &mut CodeGenerator) -> TokenStream {
        let code = RefCell::new(code);
        let parsers = self.generate(
            &|label, children| quote!(trieprefix!(#label, [ #(#children),* ])),
            &|name, label| {
                let kind = code.borrow_mut().add_token_kind(name.unwrap().clone());
                if label.is_empty() {
                    quote!(trieleaf!(#kind))
                } else {
                    quote!(trieleaf!(#kind, #label))
                }
            },
        );
        if is_trivia {
            quote!(trivia_trie!(#(#parsers),*))
        } else {
            quote!(trie!(#(#parsers),*))
        }
    }
}

impl<'context> CombinatorNode<'context> {
    pub fn to_parser_code(&self, is_trivia: bool, code: &mut CodeGenerator) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeGenerator) -> Option<Ident> {
            name.as_ref().map(|name| code.add_rule_kind(name.clone()))
        }

        let token_macro = if is_trivia {
            quote!(trivia_token!)
        } else {
            quote!(token!)
        };
        let terminal_macro = if is_trivia {
            quote!(trivia_terminal!)
        } else {
            quote!(terminal!)
        };

        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => {
                let production_name = format_ident!("{}", tree.production.name);
                match tree.production.kind {
                    ProductionKind::Rule if is_trivia => unreachable!(
                        "Trivia productions can only reference trivia or token productions"
                    ),
                    ProductionKind::Rule => quote!(rule!(#production_name)),
                    ProductionKind::Trivia => quote!(rule!(#production_name)),
                    ProductionKind::Token => quote!(#token_macro(#production_name)),
                }
            }

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_parser_code(is_trivia, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(seq!( #kind, #(#elements),* ))
                } else {
                    quote!(seq!( #(#elements),* ))
                }
            }

            Self::Choice { elements, name } => {
                let elements = elements
                    .iter()
                    .map(|element| element.to_parser_code(is_trivia, code))
                    .collect::<Vec<_>>();
                if let Some(kind) = create_kind(name, code) {
                    quote!(choice!( #kind, #(#elements),* ))
                } else {
                    quote!(choice!( #(#elements),* ))
                }
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                let expr = expr.to_parser_code(is_trivia, code);
                quote!(optional!(#expr))
            }

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_parser_code(is_trivia, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(zero_or_more!(#kind, #expr))
                } else {
                    quote!(zero_or_more!(#expr))
                }
            }
            Self::OneOrMore { expr, name } => {
                let expr = expr.to_parser_code(is_trivia, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(one_or_more!(#kind, #expr))
                } else {
                    quote!(one_or_more!(#expr))
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_parser_code(is_trivia, code);
                if let Some(kind) = create_kind(name, code) {
                    quote!(repeated!(#kind, #expr, #min, #max))
                } else {
                    quote!(repeated!(#expr, #min, #max))
                }
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open_kind = code.add_terminal_kind(open.clone());
                let expr = expr.to_parser_code(is_trivia, code);
                let close_kind = code.add_terminal_kind(close.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        delimited_by!(#kind, #terminal_macro(#open_kind, #open), #expr, #terminal_macro(#close_kind, #close))
                    )
                } else {
                    quote!(
                        delimited_by!(#terminal_macro(#open_kind, #open), #expr, #terminal_macro(#close_kind, #close))
                    )
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let separator_kind = code.add_terminal_kind(separator.clone());
                if let Some(kind) = create_kind(name, code) {
                    quote!(
                        separated_by!(#kind, #expr, #terminal_macro(#separator_kind, #separator))
                    )
                } else {
                    quote!(separated_by!(#expr, #terminal_macro(#separator_kind, #separator)))
                }
            }

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceRule { members, .. } => {
                let first_parser_name = format_ident!("{}", members[0].production.name);
                quote!(rule!(#first_parser_name))
            }

            Self::PrecedenceRuleMember {
                tree,
                next_sibling,
                operator,
                operator_model,
                ..
            } => {
                let kind = code.add_rule_kind(tree.production.name.clone());
                let operator = operator.to_parser_code(is_trivia, code);
                let next_sibling = next_sibling
                    .clone()
                    .map(|next| format_ident!("{}", next.production.name));

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

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { filter, name } => filter.to_parser_code(
                name.clone().expect("Rule character filters must be named"),
                is_trivia,
                code,
            ),

            Self::TerminalTrie { trie, .. } => trie.to_parser_code(is_trivia, code),

            Self::Difference {
                minuend,
                subtrahend,
            } => {
                let minuend = minuend.to_parser_code(is_trivia, code);
                let subtrahend = subtrahend.to_parser_code(is_trivia, code);
                quote! ( difference(#minuend, #subtrahend) )
            }

            Self::Lookahead { expr, lookahead } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let lookahead = lookahead.to_parser_code(is_trivia, code);
                quote!( #expr.then_ignore(#lookahead.rewind()) )
            }
        }
    }
}
pub fn parse_macros() -> TokenStream {
    quote!(
        #[allow(unused_macros)]
        macro_rules! trivia_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(span.start()..span.end()),
                    )
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(span.start()..span.end()),
                    )
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! terminal {
            ($kind:ident, $literal:literal) => {
                LeadingTrivia
                    .clone()
                    .then(
                        just($literal).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(TrailingTrivia.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
            ($kind:ident, $filter:expr) => {
                LeadingTrivia
                    .clone()
                    .then(
                        filter($filter).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(TrailingTrivia.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                $token_rule.clone().map(|token: Option<Rc<lex::Node>>| {
                    let token = token.unwrap(); // token rule should always return a token
                    if let lex::Node::Named(kind, element) = token.as_ref() {
                        cst::Node::trivia_token(*kind, element.clone())
                    } else {
                        unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                    }
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! token {
            ($token_rule:ident) => {
                LeadingTrivia.clone()
                    .then($token_rule.clone())
                    .then(TrailingTrivia.clone())
                    .map(|((leading_trivia, token), trailing_trivia): ((_, Option<Rc<lex::Node>>), _)| {
                        let token = token.unwrap(); // token rule should always return a token
                        if let lex::Node::Named(kind, element) = token.as_ref() {
                            cst::Node::token(*kind, element.clone(), leading_trivia, trailing_trivia)
                        } else {
                            unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! rule {
            ($rule:ident) => {
                $rule.clone()
            };
        }

        #[allow(unused_macros)]
        macro_rules! choice {
            ($kind:ident, $($expr:expr),*) => {
                choice::<_, ErrorType>(($($expr),* ))
            };
            ($($expr:expr),* ) => {
                choice::<_, ErrorType>(($($expr),* ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! seq {
            // HELPERS -------------------------------------------------------------------------------

            /*
                (@exp a, b, c, d)
                => a.then(@exp b, c, d)
                => a.then(b.then(@exp c, d))
                => a.then(b.then(c.then(@exp d)))
                => a.then(b.then(c.then(d)))
            */

            (@exp $head:expr , $($tail:expr),+ ) => {
                $head.then(seq!(@exp $($tail),+ ))
            };

            (@exp $head:expr ) => {
                $head
            };

            /*
                (@args [], v, a, b, c, d)
                => (@args [v.0,], v.1, b, c, d)
                => (@args [v.0, v.1.0,], v.1.1, c, d)
                => (@args [v.0, v.1.0, v.1.1.0,], v.1.1.1, d)
                => vec![v.0, v.1.0, v.1.1.0, v.1.1.0, v1.1.1, ]
            */

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr , $($tail:expr),+ ) => {
                seq!(@args [ $($accum,)* $current.0, ] , $current.1 , $($tail),+ )
            };

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr ) => {
                vec![ $($accum,)* $current ]
            };

            //----------------------------------------------------------------------------------------

            ($kind:ident, $($expr:expr),+ ) => {
                seq!(@exp $($expr),+ )
                    .map(|v| cst::Node::rule(RuleKind::$kind, seq!(@args [] , v , $($expr),+ )))
            };

            ($($expr:expr),+ ) => {
                seq!(@exp $($expr),+ )
                    .map(|v| cst::Node::group(seq!(@args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr) => {
                $expr.repeated().map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! one_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! optional {
            ($expr:expr) => {
                $expr.or_not().map(|opt| opt.flatten())
            };
        }

        #[allow(unused_macros)]
        macro_rules! separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::rule(RuleKind::$kind, v)
                    })
            };
            ($expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::group(v)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! left_associative_binary_expression {
            ($kind:ident, $next_sibling:ident, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                            rest.into_iter().fold(
                                first,
                                |left_operand, (operator, right_operand)| {
                                    cst::Node::rule(
                                        RuleKind::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! right_associative_binary_expression {
            ($kind:ident, $next_sibling:ident, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            // a [ (X b) (Y c) (Z d) ] => [ (a X) (b Y) (c Z) ] d
                            let mut last_operand = first;
                            let mut operand_operator_pairs = vec![];
                            for (operator, right_operand) in rest.into_iter() {
                                let left_operand =
                                    std::mem::replace(&mut last_operand, right_operand);
                                operand_operator_pairs.push((left_operand, operator))
                            }
                            // [ (a X) (b Y) (c Z) ] d => { a X { b Y { c Z d } } }
                            operand_operator_pairs.into_iter().rfold(
                                last_operand,
                                |right_operand, (left_operand, operator)| {
                                    cst::Node::rule(
                                        RuleKind::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! unary_prefix_expression {
            ($kind:ident, $next_sibling:ident, $operator:expr) => {
                $operator
                    .repeated()
                    .then($next_sibling.clone())
                    .map(|(mut operators, operand)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators.reverse();
                            operators
                                .into_iter()
                                .fold(operand, |right_operand, operator| {
                                    cst::Node::rule(RuleKind::$kind, vec![operator, right_operand])
                                })
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! unary_suffix_expression {
            ($kind:ident, $next_sibling:ident, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.repeated())
                    .map(|(operand, operators)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators
                                .into_iter()
                                .fold(operand, |left_operand, operator| {
                                    cst::Node::rule(RuleKind::$kind, vec![left_operand, operator])
                                })
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! delimited_by {
            ($kind:ident, $open:expr, $expr:expr, $close:expr) => {
                seq!($kind, $open, $expr, $close)
            };
            ($open:expr, $expr:expr, $close:expr) => {
                seq!($open, $expr, $close)
            };
        }

        #[allow(unused_macros)]
        macro_rules! trie {
            ($kind:ident, $($expr:expr),* ) => {
                trie!($($expr),*).map(|child| cst::Node::rule(RuleKind::$kind, vec![child]))
            };
            ( $expr:expr ) => {
                LeadingTrivia.clone()
                    .then($expr.map_with_span(|kind, span: SpanType| (kind, span)))
                    .then(TrailingTrivia.clone())
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        cst::Node::token(kind, lex::Node::chars_unwrapped(range), leading_trivia, trailing_trivia)
                    })
            };
            ( $($expr:expr),+ ) => {
                LeadingTrivia.clone()
                    .then(choice::<_, ErrorType>(($($expr),+)).map_with_span(|kind, span: SpanType| (kind, span)))
                    .then(TrailingTrivia.clone())
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        cst::Node::token(kind, lex::Node::chars_unwrapped(range), leading_trivia, trailing_trivia)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_trie {
            ( $expr:expr ) => {
                $expr.map_with_span(|kind, span: SpanType| cst::Node::trivia_token(kind, lex::Node::chars_unwrapped(span)))
            };
            ( $($expr:expr),+ ) => {
                choice::<_, ErrorType>(($($expr),+))
                    .map_with_span(|kind, span: SpanType| cst::Node::trivia_token(kind, lex::Node::chars_unwrapped(span)))
            };
        }

        #[allow(unused_macros)]
        macro_rules! trieprefix {
            ($string:literal , [ $($expr:expr),+ ] ) => (
                just($string).ignore_then(choice::<_, ErrorType>(($($expr),+)))
            )
        }

        #[allow(unused_macros)]
        macro_rules! trieleaf {
            ( $kind:ident, $string:literal ) => {
                just($string).to(TokenKind::$kind)
            };
            ( $kind:ident ) => {
                empty().to(TokenKind::$kind)
            };
        }

        #[allow(unused_macros)]
        macro_rules! define_rule {
            ($kind:ident, $expr:expr) => {
                $kind.define($expr.map(|node| cst::Node::rule(RuleKind::$kind, vec![node])));
                parsers.insert(
                    ProductionKind::$kind,
                    Parser::new(
                        $kind
                            .clone()
                            .map(|node| cst::Node::top_level_rule(RuleKind::$kind, node))
                            .then_ignore(end())
                            .boxed(),
                    ),
                );
            };
        }

        #[allow(unused_macros)]
        macro_rules! define_precedence_rule_member {
            ($kind:ident, $expr:expr) => {
                $kind.define($expr);
                parsers.insert(
                    ProductionKind::$kind,
                    Parser::new(
                        $kind
                            .clone()
                            .map(|node| cst::Node::top_level_rule(RuleKind::$kind, node))
                            .then_ignore(end())
                            .boxed(),
                    ),
                );
            };
        }
    )
}
