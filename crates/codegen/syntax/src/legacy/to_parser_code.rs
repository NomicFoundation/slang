use codegen_schema::types::{OperatorModel, ProductionDefinition};
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
    code_generator::CodeGenerator,
    combinator_node::{CombinatorNode, OperatorExpression},
    combinator_tree::CombinatorTree,
};

impl<'context> CombinatorNode<'context> {
    pub fn to_parser_code(
        &self,
        code_generator: &mut CodeGenerator,
        is_trivia: bool,
    ) -> TokenStream {
        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => reference_to_parser_code(tree, is_trivia),

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { nodes } => {
                let parsers = nodes
                    .iter()
                    .map(|node| node.to_parser_code(code_generator, is_trivia))
                    .collect();
                sequence_to_parser_code(parsers)
            }

            Self::Choice { nodes } => {
                let parsers = nodes
                    .iter()
                    .map(|node| node.to_parser_code(code_generator, is_trivia))
                    .collect();
                choice_to_parser_code(parsers)
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { node } => {
                option_to_parser_code(node.to_parser_code(code_generator, is_trivia))
            }

            Self::ZeroOrMore { node } => {
                let parser = node.to_parser_code(code_generator, is_trivia);
                zero_or_more_to_parser_code(parser)
            }

            Self::OneOrMore { node } => {
                let parser = node.to_parser_code(code_generator, is_trivia);
                one_or_more_to_parser_code(parser)
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy { open, node, close } => Self::Sequence {
                nodes: vec![
                    open.context.alloc_node(Self::Reference { tree: open }),
                    node,
                    close.context.alloc_node(Self::Reference { tree: close }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            Self::SeparatedBy { node, separator } => Self::Sequence {
                nodes: vec![
                    node,
                    separator.context.alloc_node(Self::ZeroOrMore {
                        node: separator.context.alloc_node(Self::Sequence {
                            nodes: vec![
                                separator
                                    .context
                                    .alloc_node(Self::Reference { tree: separator }),
                                node,
                            ],
                        }),
                    }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            Self::TerminatedBy { node, terminator } => Self::Sequence {
                nodes: vec![
                    node,
                    terminator
                        .context
                        .alloc_node(Self::Reference { tree: terminator }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceParser {
                tree,
                primary_expression,
                operator_expressions,
            } => {
                if is_trivia {
                    unreachable!("Precedence operators cannot be used in trivia productions")
                }

                let expression_ident = if tree.production.inlined {
                    None
                } else {
                    Some(format_ident!("{}", tree.production.name))
                };

                precedence_expression_to_parser_code(
                    code_generator,
                    expression_ident,
                    primary_expression,
                    operator_expressions,
                )
            }

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { .. } => {
                unreachable!("CharacterFilter cannot be generated from a parser")
            }

            Self::TerminalTrie { .. } => {
                unreachable!("TerminalTrie cannot be generated from a parser")
            }

            Self::TrailingContext { .. } => {
                unreachable!("TrailingContext cannot be generated from a parser")
            }

            Self::Difference { .. } => unreachable!("Difference cannot be generated from a parser"),
        }
    }
}

fn reference_to_parser_code(tree: &CombinatorTree, is_trivia: bool) -> TokenStream {
    let name = &tree.production.name;
    let function_name = format_ident!("{snake_case}", snake_case = name.to_snake_case());

    match tree.production.definition {
        ProductionDefinition::Scanner { .. } => {
            let kind = format_ident!("{name}");
            if is_trivia {
                quote! {
                    self.parse_token(stream, &Self::#function_name, TokenKind::#kind)
                }
            } else {
                quote! {
                    self.parse_token_with_trivia(stream, &Self::#function_name, TokenKind::#kind)
                }
            }
        }
        ProductionDefinition::TriviaParser { .. } => {
            quote! { self.#function_name(stream) }
        }
        ProductionDefinition::Parser { .. } | ProductionDefinition::PrecedenceParser { .. } => {
            if !is_trivia {
                quote! { self.#function_name(stream) }
            } else {
                unreachable!("Trivia productions can only reference trivia or token productions")
            }
        }
    }
}

fn sequence_to_parser_code(mut parsers: Vec<TokenStream>) -> TokenStream {
    if parsers.len() == 1 {
        let parser = &parsers[0];
        quote! { #parser  }
    } else {
        let last_parser = parsers.pop().unwrap();
        quote! {
            {
                let mut running_result = ParserResult::r#match(vec![], vec![]);
                loop {
                    #(
                        if !running_result.incorporate_sequence_result(#parsers) {
                            break;
                        }
                    )*
                    running_result.incorporate_sequence_result(#last_parser);
                    break;
                }
                running_result
            }
        }
    }
}

fn choice_to_parser_code(parsers: Vec<TokenStream>) -> TokenStream {
    quote! {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                #(
                    if running_result.incorporate_choice_result(#parsers) {
                        break;
                    }
                    stream.set_position(start_position);
                )*
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }
}

fn option_to_parser_code(parser: TokenStream) -> TokenStream {
    quote! {
        transform_option_result(#parser)
    }
}

fn zero_or_more_to_parser_code(parser: TokenStream) -> TokenStream {
    quote! {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_zero_or_more_result(#parser) {}
            running_result
        }
    }
}

fn one_or_more_to_parser_code(parser: TokenStream) -> TokenStream {
    quote! {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(#parser) {}
            running_result
        }
    }
}

fn precedence_expression_to_parser_code(
    code: &mut CodeGenerator,
    expression_kind: Option<Ident>,
    primary_expression: &&CombinatorNode,
    operator_expressions: &Vec<OperatorExpression>,
) -> TokenStream {
    let mut prefix_operator_parsers = Vec::new();
    let mut postfix_operator_parsers = Vec::new();
    let mut binary_operator_parsers = Vec::new();
    let mut binding_power = 1u8;

    for expression in operator_expressions.iter() {
        let rule_kind = format_ident!("{}", expression.name.clone());
        let operator_code = expression.operator.to_parser_code(code, false);

        match expression.model {
                OperatorModel::BinaryLeftAssociative => binary_operator_parsers.push(
                    quote! { #operator_code.to_pratt_element_operator(RuleKind::#rule_kind, #binding_power, #binding_power + 1) }
                ),
                OperatorModel::BinaryRightAssociative => binary_operator_parsers.push(
                    quote! { #operator_code.to_pratt_element_operator(RuleKind::#rule_kind, #binding_power + 1, #binding_power) }
                ),
                OperatorModel::UnaryPrefix => prefix_operator_parsers.push(
                    quote! { #operator_code.to_pratt_element_operator(RuleKind::#rule_kind, 255, #binding_power) }
                ),
                OperatorModel::UnaryPostfix => postfix_operator_parsers.push(
                    quote! { #operator_code.to_pratt_element_operator(RuleKind::#rule_kind, #binding_power, 255) }
                )
            }

        binding_power += 2;
    }

    fn combine_operator_parsers(mut parsers: Vec<TokenStream>) -> Option<TokenStream> {
        if parsers.is_empty() {
            None
        } else if parsers.len() == 1 {
            parsers.pop()
        } else {
            Some(quote! {
                loop {
                    let start_position = stream.position();
                    #(
                        stream.set_position(start_position);
                        let next_result = #parsers;
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) =>
                                break next_result,
                            ParserResult::Match(_) =>
                                unreachable!("ParserResult::Match isn't constructed when parsing operators"),
                            ParserResult::IncompleteMatch(_) |
                            ParserResult::NoMatch(_) =>
                                {}
                        }
                    )*
                    stream.set_position(start_position);
                    break ParserResult::no_match(vec![]);
                }
            })
        }
    }

    let add_zero_or_more_prefix_operators_to_elements_or_break =
        combine_operator_parsers(prefix_operator_parsers).map(|combined_operator_parser| {
            quote! {
                let result = loop {
                    let result = #combined_operator_parser;
                    match result {
                        ParserResult::PrattOperatorMatch(_) =>
                            results.push(result),
                        // ParserResult::Match is handled in the combine_operator_parsers function
                        _ =>
                            break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => { break result; }
                }
            }
        });

    let add_a_primary_expression_to_elements_or_break = {
        let parser = primary_expression.to_parser_code(code, false);
        quote! {
            {
                let result = #parser;
                if result.is_match() {
                    results.push(result);
                } else {
                    break result;
                }
            }
        }
    };

    let add_zero_or_more_postfix_operators_to_elements_or_break = combine_operator_parsers(
        postfix_operator_parsers,
    )
    .map(|combined_operator_parser| -> TokenStream {
        quote! {
            let result = loop {
                let result = #combined_operator_parser;
                match result {
                    ParserResult::PrattOperatorMatch(_) =>
                        results.push(result),
                    // ParserResult::Match is handled in the combine_operator_parsers function
                    _ =>
                        break result,
                }
            };
            match result {
                ParserResult::NoMatch(_) => {}
                _ => { break result; }
            }
        }
    });

    let add_a_binary_operator_to_elements_or_break =
        combine_operator_parsers(binary_operator_parsers)
            .map(|combined_operator_parser| {
                quote! {
                    let result = #combined_operator_parser;
                    match result {
                        ParserResult::PrattOperatorMatch(_) =>
                            results.push(result),
                        // ParserResult::Match is handled in the combine_operator_parsers function
                        _ =>
                            break result,
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    // This grammar has no binary operators, so we can't match one, and we need to break out of the outer loop
                    break ParserResult::no_match(vec![]);
                }
            });

    let operator_argument_transformer = expression_kind
        .map(|kind| quote! { |children | vec![cst::Node::rule(RuleKind::#kind, children)] })
        .unwrap_or_else(|| quote! { |children| children });

    quote! {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();

            let initial_result = loop {
                #add_zero_or_more_prefix_operators_to_elements_or_break
                #add_a_primary_expression_to_elements_or_break
                #add_zero_or_more_postfix_operators_to_elements_or_break
                #add_a_binary_operator_to_elements_or_break
            };

            if results.is_empty() {
                break initial_result;
            }

            reduce_pratt_elements(#operator_argument_transformer, &mut results);

            if results.len() != 1 {
                unreachable!("Pratt parser failed to reduce to a single result: {:?}", results);
            }

            match results.remove(0) {
                ParserResult::Match(r#match) =>
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(r#match.nodes, r#match.tokens_that_would_have_allowed_more_progress);
                    }
                ParserResult::IncompleteMatch(incomplete_match) =>
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(nodes, initial_incomplete_match.tokens_that_would_have_allowed_more_progress);
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match)
                    }
                _ => unreachable!("Pratt parser produced an invalid result")
            }
        }
    }
}
