use std::cell::RefCell;

use codegen_schema::types::productions::ProductionKind;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

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
        let parser = self.generate(
            &|label, children| quote!( just(#label).ignore_then(choice!( #(#children),* )) ),
            &|entry, label| {
                let kind = code
                    .borrow_mut()
                    .add_token_kind(entry.name.clone().unwrap());
                if label.is_empty() {
                    quote!( empty().to(TokenKind::#kind) )
                } else {
                    quote!( just(#label).to(TokenKind::#kind) )
                }
            },
        );
        if is_trivia {
            quote!(
                #parser.map_with_span(|kind, span: SpanType| {
                    cst::Node::trivia_token(kind, lex::Node::chars_unwrapped(span))
                })
            )
        } else {
            quote!(with_trivia!(
                #parser.map_with_span(|kind, span: SpanType| (kind, span))
            )
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }))
        }
    }

    pub fn to_operator_parser_code(&self, code: &mut CodeGenerator) -> TokenStream {
        let code = RefCell::new(code);
        let parser = self.generate(
            &|label, children| quote!( just(#label).ignore_then(choice!( #(#children),* )) ),
            &|entry, label| {
                let kind = code
                    .borrow_mut()
                    .add_token_kind(entry.name.clone().unwrap());
                let payload = &entry.payload;
                if label.is_empty() {
                    quote!( empty().to((TokenKind::#kind, #payload)) )
                } else {
                    quote!( just(#label).to((TokenKind::#kind, #payload)) )
                }
            },
        );
        quote!(with_trivia!(
            #parser.map_with_span(|payload, span: SpanType| (payload, span))
        )
        .map(
            |(
                (
                    leading_trivia,
                    ((token_kind, left_binding_power, right_binding_power, kind), range),
                ),
                trailing_trivia,
            )| Pratt::Operator {
                node: cst::Node::token(
                    token_kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia
                ),
                kind,
                left_binding_power,
                right_binding_power
            }
        ))
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
            Self::PrecedenceExpressionRule {
                primary_expressions,
                operators,
                ..
            } => {
                if is_trivia {
                    unreachable!("Precedence expressions cannot be used in trivia productions")
                }

                let mut prefix_operators = Vec::new();
                let mut prefix_operator_trie = TerminalTrie::new();
                let mut suffix_operators = Vec::new();
                let mut suffix_operator_trie = TerminalTrie::new();
                let mut binary_operators = Vec::new();
                let mut binary_operator_trie = TerminalTrie::new();
                for (index, operator) in operators.iter().enumerate() {
                    let binding_power = (1 + index * 2) as u8;
                    let name = code.add_rule_kind(operator.name.clone());
                    match operator.model {
                        OperatorModel::BinaryLeftAssociative => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                let mut trie = trie.clone();
                                trie.set_payloads(
                                    quote!(#binding_power, #binding_power + 1, RuleKind::#name),
                                );
                                binary_operator_trie =
                                    binary_operator_trie.merged_with(trie.clone());
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                binary_operators.push(quote!(
                                #operator_code.map(|node| Pratt::Operator {
                                    node, kind: RuleKind::#name, left_binding_power: #binding_power, right_binding_power: #binding_power + 1
                                })
                            ))
                            }
                        }
                        OperatorModel::BinaryRightAssociative => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                let mut trie = trie.clone();
                                trie.set_payloads(
                                    quote!(#binding_power + 1, #binding_power, RuleKind::#name),
                                );
                                binary_operator_trie =
                                    binary_operator_trie.merged_with(trie.clone());
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                binary_operators.push(quote!(
                                #operator_code.map(|node| Pratt::Operator {
                                    node, kind: RuleKind::#name, left_binding_power: #binding_power + 1, right_binding_power: #binding_power
                                })
                            ))
                            }
                        }
                        OperatorModel::UnaryPrefix => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                let mut trie = trie.clone();
                                trie.set_payloads(quote!(255, #binding_power, RuleKind::#name));
                                prefix_operator_trie =
                                    prefix_operator_trie.merged_with(trie.clone());
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                prefix_operators.push(quote!(
                                #operator_code.map(|node| Pratt::Operator {
                                    node, kind: RuleKind::#name, left_binding_power: 255, right_binding_power: #binding_power
                                })
                            ))
                            }
                        }
                        OperatorModel::UnarySuffix => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                let mut trie = trie.clone();
                                trie.set_payloads(quote!(#binding_power, 255, RuleKind::#name));
                                suffix_operator_trie =
                                    suffix_operator_trie.merged_with(trie.clone());
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                suffix_operators.push(quote!(
                                #operator_code.map(|node| Pratt::Operator {
                                    node, kind: RuleKind::#name, left_binding_power: #binding_power, right_binding_power: 255
                                })
                            ))
                            }
                        }
                    }
                }

                binary_operators.insert(0, binary_operator_trie.to_operator_parser_code(code));
                prefix_operators.insert(0, prefix_operator_trie.to_operator_parser_code(code));
                suffix_operators.insert(0, suffix_operator_trie.to_operator_parser_code(code));

                fn maybe_choice(mut operators: Vec<TokenStream>) -> TokenStream {
                    if operators.len() == 1 {
                        operators.pop().unwrap()
                    } else {
                        quote!(choice!( #(#operators),* ))
                    }
                }
                let binary_operators = maybe_choice(binary_operators);
                let prefix_operators = maybe_choice(prefix_operators);
                let suffix_operators = maybe_choice(suffix_operators);

                let primary_expressions = maybe_choice(
                    primary_expressions
                        .iter()
                        .map(|tree| {
                            let rule_name = format_ident!("{}", tree.production.name);
                            quote!(rule!(#rule_name))
                        })
                        .collect::<Vec<_>>(),
                );

                quote!({
                    enum Pratt {
                        Operator {
                            kind: RuleKind,
                            node: Option<Rc<cst::Node>>,
                            left_binding_power: u8,
                            right_binding_power: u8,
                        },
                        Node(Option<Rc<cst::Node>>),
                    }
                    let prefix_operator = #prefix_operators;
                    let suffix_operator = #suffix_operators;
                    let primary_expression = #primary_expressions;
                    let prefixes_primary_suffixes = prefix_operator.repeated().then(primary_expression).then(suffix_operator.repeated());
                    type PPS = ((Vec<Pratt>, Option<Rc<cst::Node>>), Vec<Pratt>);
                    let binary_operator = #binary_operators;
                    prefixes_primary_suffixes.clone().then(binary_operator.then(prefixes_primary_suffixes).repeated()).map(
                        |(pps, tail): (PPS, Vec<(Pratt, PPS)>)| {
                            let mut elements = Vec::new();
                            let ((prefixes, expr), suffixes) = pps;
                            elements.extend(prefixes.into_iter());
                            elements.push(Pratt::Node(expr));
                            elements.extend(suffixes.into_iter());
                            for (binary_operator, pps) in tail.into_iter() {
                                elements.push(binary_operator);
                                let ((prefixes, expr), suffixes) = pps;
                                elements.extend(prefixes.into_iter());
                                elements.push(Pratt::Node(expr));
                                elements.extend(suffixes.into_iter());
                            }

                            let mut i = 0;
                            while elements.len() > 1 {
                                if let Pratt::Operator { right_binding_power, left_binding_power, .. } = &elements[i] {
                                    let next_left_binding_power = if elements.len() == i + 1 {
                                        0
                                    } else if let Pratt::Operator { left_binding_power, .. } = &elements[i + 1] {
                                        *left_binding_power
                                    } else if elements.len() == i + 2 {
                                        0
                                    } else if let Pratt::Operator { left_binding_power, .. } = &elements[i + 2] {
                                        *left_binding_power
                                    } else {
                                        0
                                    };
                                    if *right_binding_power <= next_left_binding_power {
                                        i += 1;
                                        continue;
                                    }
                                    if *right_binding_power == 255 {
                                        let left = elements.remove(i - 1);
                                        let op = elements.remove(i - 1);
                                        if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }) = (left, op) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![left, op],
                                            );
                                            elements.insert(i - 1, Pratt::Node(node));
                                            i = i.saturating_sub(2);
                                        } else {
                                            unreachable!()
                                        }
                                    } else  if *left_binding_power == 255 {
                                        let op = elements.remove(i);
                                        let right = elements.remove(i);
                                        if let (Pratt::Operator { node: op, kind, .. }, Pratt::Node(right)) = (op, right) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![op, right],
                                            );
                                            elements.insert(i, Pratt::Node(node));
                                            i = i.saturating_sub(1);
                                        } else {
                                            unreachable!()
                                        }
                                    } else {
                                        let left = elements.remove(i - 1);
                                        let op = elements.remove(i - 1);
                                        let right = elements.remove(i - 1);
                                        if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }, Pratt::Node(right)) = (left, op, right) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![left, op, right],
                                            );
                                            elements.insert(i - 1, Pratt::Node(node));
                                            i = i.saturating_sub(2);
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                } else {
                                    i += 1;
                                }
                            }

                            if let Pratt::Node(node) = elements.pop().unwrap() {
                                node
                            } else {
                                unreachable!()
                            }
                        }
                    )
                })
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
        macro_rules! with_trivia {
            ($expr:expr) => {
                LeadingTrivia
                    .clone()
                    .then($expr)
                    .then(TrailingTrivia.clone())
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(TokenKind::$kind, lex::Node::chars_unwrapped(span))
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(TokenKind::$kind, lex::Node::chars_unwrapped(span))
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! terminal {
            ($kind:ident, $literal:literal) => {
                with_trivia!(just($literal).map_with_span(|_, span: SpanType| span)).map(
                    |((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    },
                )
            };
            ($kind:ident, $filter:expr) => {
                with_trivia!(filter($filter).map_with_span(|_, span: SpanType| span)).map(
                    |((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    },
                )
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                rule!($token_rule).map(|token: Option<Rc<lex::Node>>| {
                    let token = token.unwrap(); // token rule should always return a token
                    if let lex::NodeContents::Named(kind, element) = &token.contents {
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
                with_trivia!($token_rule.clone())
                    .map(|((leading_trivia, token), trailing_trivia): ((_, Option<Rc<lex::Node>>), _)| {
                        let token = token.unwrap(); // token rule should always return a token
                        if let lex::NodeContents::Named(kind, element) = &token.contents {
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
            ( $kind:ident, $($expr:expr),* ) => {
                choice::<_, ErrorType>(($($expr),* ))
            };
            ( $head:expr, $($tail:expr),+ ) => {
                choice::<_, ErrorType>(( $head, $($tail),+ ))
            };
            ( $expr:expr ) => {
                $expr
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
        macro_rules! delimited_by {
            ($kind:ident, $open:expr, $expr:expr, $close:expr) => {
                seq!($kind, $open, $expr, $close)
            };
            ($open:expr, $expr:expr, $close:expr) => {
                seq!($open, $expr, $close)
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
    )
}
