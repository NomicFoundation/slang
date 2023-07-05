use codegen_schema::types::{OperatorModel, ProductionDefinition};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    code_generator::CodeGenerator,
    combinator_node::{CombinatorNode, PrecedenceRuleOperator},
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
            Self::Sequence { elements, name } => {
                let parsers = elements
                    .iter()
                    .map(|element| element.to_parser_code(code_generator, is_trivia))
                    .collect();
                sequence_to_parser_code(code_generator, name, parsers)
            }

            Self::Choice { elements, name } => {
                let parsers = elements
                    .iter()
                    .map(|element| element.to_parser_code(code_generator, is_trivia))
                    .collect();
                choice_to_parser_code(code_generator, name, parsers)
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                option_to_parser_code(expr.to_parser_code(code_generator, is_trivia))
            }

            Self::ZeroOrMore { expr, name } => {
                let parser = expr.to_parser_code(code_generator, is_trivia);
                zero_or_more_to_parser_code(code_generator, name, parser)
            }

            Self::OneOrMore { expr, name } => {
                let parser = expr.to_parser_code(code_generator, is_trivia);
                one_or_more_to_parser_code(code_generator, name, parser)
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => Self::Sequence {
                name: name.clone(),
                elements: vec![
                    open.context.alloc_node(Self::Reference { tree: open }),
                    expr,
                    close.context.alloc_node(Self::Reference { tree: close }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => Self::Sequence {
                name: name.clone(),
                elements: vec![
                    expr,
                    separator.context.alloc_node(Self::ZeroOrMore {
                        name: None,
                        expr: separator.context.alloc_node(Self::Sequence {
                            name: None,
                            elements: vec![
                                separator
                                    .context
                                    .alloc_node(Self::Reference { tree: separator }),
                                expr,
                            ],
                        }),
                    }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            Self::TerminatedBy {
                expr,
                terminator,
                name,
            } => Self::Sequence {
                name: name.clone(),
                elements: vec![
                    expr,
                    terminator
                        .context
                        .alloc_node(Self::Reference { tree: terminator }),
                ],
            }
            .to_parser_code(code_generator, is_trivia),

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceExpressionRule {
                primary_expression,
                operators,
                ..
            } => {
                if is_trivia {
                    unreachable!("Precedence expressions cannot be used in trivia productions")
                }

                precedence_expression_to_parser_code(code_generator, primary_expression, operators)
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

fn kind_wrapper(code: &mut CodeGenerator, name: Option<String>) -> Option<TokenStream> {
    name.clone().map(|name| {
        let kind = code.add_rule_kind(name.to_owned());
        quote! { .with_kind(RuleKind::#kind) }
    })
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

fn sequence_to_parser_code(
    code: &mut CodeGenerator,
    name: &Option<String>,
    mut parsers: Vec<TokenStream>,
) -> TokenStream {
    let kind_wrapper = kind_wrapper(code, name.clone());
    if parsers.len() == 1 {
        let parser = &parsers[0];
        quote! { #parser #kind_wrapper }
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
                running_result #kind_wrapper
            }
        }
    }
}

fn choice_to_parser_code(
    code: &mut CodeGenerator,
    name: &Option<String>,
    mut parsers: Vec<TokenStream>,
) -> TokenStream {
    let kind_wrapper = kind_wrapper(code, name.clone());
    let last_parser = parsers.pop().unwrap();
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
                running_result.incorporate_choice_result(#last_parser);
                break;
            }
            running_result #kind_wrapper
        }
    }
}

fn option_to_parser_code(parser: TokenStream) -> TokenStream {
    quote! {
        transform_option_result(#parser)
    }
}

fn zero_or_more_to_parser_code(
    code_generator: &mut CodeGenerator,
    name: &Option<String>,
    parser: TokenStream,
) -> TokenStream {
    let kind_wrapper = kind_wrapper(code_generator, name.clone());
    quote! {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_zero_or_more_result(#parser) {}
            running_result #kind_wrapper
        }
    }
}

fn one_or_more_to_parser_code(
    code_generator: &mut CodeGenerator,
    name: &Option<String>,
    parser: TokenStream,
) -> TokenStream {
    let kind_wrapper = kind_wrapper(code_generator, name.clone());
    quote! {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(#parser) {}
            running_result #kind_wrapper
        }
    }
}

fn precedence_expression_to_parser_code(
    code: &mut CodeGenerator,
    primary_expression: &&CombinatorNode,
    operators: &Vec<PrecedenceRuleOperator>,
) -> TokenStream {
    let mut prefix_operator_parsers = Vec::new();
    let mut postfix_operator_parsers = Vec::new();
    let mut binary_operator_parsers = Vec::new();
    let mut binding_power = 1u8;

    for operator in operators.iter() {
        let rule_kind = code.add_rule_kind(operator.name.clone());
        let operator_code = operator.operator.to_parser_code(code, false);

        match operator.model {
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
                            elements.push(result),
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
                    elements.push(result);
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
                        elements.push(result),
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
                            elements.push(result),
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

    quote! {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();

            let result = loop {
                #add_zero_or_more_prefix_operators_to_elements_or_break
                #add_a_primary_expression_to_elements_or_break
                #add_zero_or_more_postfix_operators_to_elements_or_break
                #add_a_binary_operator_to_elements_or_break
            };

            if elements.is_empty() {
                break result;
            }

            reduce_pratt_elements(&mut elements);

            if elements.len() != 1 {
                unreachable!("Pratt parser failed to reduce to a single result: {:?}", elements);
            }

            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(r#match.nodes, r#match.tokens_that_would_have_allowed_more_progress);
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
            }
        }
    }
}
