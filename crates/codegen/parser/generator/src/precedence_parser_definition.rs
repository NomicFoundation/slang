use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use codegen_grammar::{
    PrecedenceOperatorModel, PrecedenceParserDefinitionNode, PrecedenceParserDefinitionRef,
    VersionQualityRange,
};

use super::parser_definition::{ParserDefinitionNodeExtensions, VersionQualityRangeVecExtensions};

pub trait PrecedenceParserDefinitionExtensions {
    fn to_parser_code(&self) -> TokenStream;
}

impl PrecedenceParserDefinitionExtensions for PrecedenceParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        self.node().to_parser_code(
            self.context(),
            Some(format_ident!("{name}", name = self.name().to_pascal_case())),
        )
    }
}

pub trait PrecedenceParserDefinitionNodeExtensions {
    fn to_parser_code(
        &self,
        context_name: &'static str,
        expression_kind: Option<Ident>,
    ) -> TokenStream;
}

impl PrecedenceParserDefinitionNodeExtensions for PrecedenceParserDefinitionNode {
    // A Pratt parser can be implemented as two simple passes,
    // only the first of which can generate parse errors.
    //
    // The first pass is to parse the `Expression` in this grammar into a linear sequence:
    //
    //     ```
    //     Expression ::= BinaryOperand ( BinaryOperator BinaryOperand )*
    //     BinaryOperand ::= PrefixOperator* PrimaryExpression PostfixOperator*
    //     PrefixOperator ::= PrefixOperator_1 | PrefixOperator_2 | ...
    //     PostfixOperator ::= PostfixOperator_1 | PostfixOperator_2 | ...
    //     BinaryOperator ::= BinaryOperator_1 | BinaryOperator_2 | ...
    //     ```
    //
    // ... and that has nothing to do with precedence parsing.
    //
    // The only complication is that each Operator must be annotated with the left/right
    // binding strength. Hence `ParserResult::PrattOperatorMatch`.
    //
    // Parse errors can only occur in this pass, so we can reuse
    // the exact same code as the normal parser, just making sure that the helpers in
    // `ParserResult` and functions such as `incorporate_choice_result` treat `PrattOperatorMatch`
    // similarly to `Match`. The algebra is not complicated.
    //
    // This means that any parse error will return a CST in terms of this linear structure,
    // rather that the tree structure that the precedence parser implies. But it’s an error,
    // so I think that doesn’t matter.
    //
    // The second pass is to use the binding strengths to resolve the linear structure into a
    // tree, and maybe wrap each child of each Operator as a node with the kind of the overall
    // precedence parser root e.g. `RuleKind::Expression`.
    //
    // Given the result of step one, this second pass cannot fail to correctly resolve to a
    // single node. So all the panics disappear and Pratt parsing becomes “… so simple that
    // there are obviously no deficiencies”
    //
    // The code below is the first pass.
    //
    // The second pass is in the method `PrecedenceHelper::reduce_precedence_result` because it
    // is independent of the grammar.

    fn to_parser_code(
        &self,
        context_name: &'static str,
        expression_kind: Option<Ident>,
    ) -> TokenStream {
        type OperatorParser = (TokenStream, Vec<VersionQualityRange>);
        let mut prefix_operator_parsers: Vec<OperatorParser> = Vec::new();
        let mut postfix_operator_parsers: Vec<OperatorParser> = Vec::new();
        let mut binary_operator_parsers: Vec<OperatorParser> = Vec::new();

        // Closures rather than local functions because they
        // need to access the `Language` instance as `self`.
        let mut operator_closures = Vec::new();

        let mut binding_power = 1u8;
        for (version_quality_ranges, model, name, operator_definition) in self.operators.iter() {
            let mut mop = |left_binding_power, right_binding_power| {
                let operator_code = operator_definition
                    .node()
                    .to_parser_code(context_name, false);
                let rule_kind = format_ident!("{}", name);
                let closure_name = format_ident!(
                    // Make a name that won't conflict with the parsers we define below
                    "parse_{name}{version_tag}",
                    version_tag = version_quality_ranges.disambiguating_name_suffix(),
                    name = operator_definition.name().to_snake_case()
                );
                if left_binding_power == 255 {
                    operator_closures.push(quote! {
                        let #closure_name = |stream: &mut Stream|
                            PrecedenceHelper::to_prefix_operator(
                                RuleKind::#rule_kind,
                                #right_binding_power,
                                #operator_code
                            );
                    });
                } else if right_binding_power == 255 {
                    operator_closures.push(quote! {
                        let #closure_name = |stream: &mut Stream|
                            PrecedenceHelper::to_postfix_operator(
                                RuleKind::#rule_kind,
                                #left_binding_power,
                                #operator_code
                            );
                    });
                } else {
                    operator_closures.push(quote! {
                        let #closure_name = |stream: &mut Stream|
                            PrecedenceHelper::to_binary_operator(
                                RuleKind::#rule_kind,
                                #left_binding_power,
                                #right_binding_power,
                                #operator_code
                            );
                    });
                }
                (
                    quote! { #closure_name(stream) },
                    version_quality_ranges.clone(),
                )
            };
            match model {
                PrecedenceOperatorModel::BinaryLeftAssociative => {
                    binary_operator_parsers.push(mop(binding_power, binding_power + 1))
                }
                PrecedenceOperatorModel::BinaryRightAssociative => {
                    binary_operator_parsers.push(mop(binding_power + 1, binding_power))
                }
                PrecedenceOperatorModel::Prefix => {
                    prefix_operator_parsers.push(mop(255, binding_power))
                }
                PrecedenceOperatorModel::Postfix => {
                    postfix_operator_parsers.push(mop(binding_power, 255))
                }
            }

            binding_power += 2;
        }

        // TODO: merge these three functions into parse_definition by changing
        // `to_parser_code` to use `(TokenStream, Vec<VersionQualityRange>)` as
        // the core type i.e. the `OperatorParser` type above
        fn make_sequence(parsers: Vec<TokenStream>) -> TokenStream {
            let parsers = parsers
                .iter()
                .map(|parser| {
                    quote! {
                        if helper.handle_next_result(#parser) {
                            break;
                        }
                    }
                })
                .collect::<Vec<_>>();
            quote! {
                {
                    let mut helper = SequenceHelper::new();
                    loop {
                        #(#parsers)*
                        break;
                    }
                    helper.result()
                }
            }
        }

        fn make_choice(parsers: Vec<OperatorParser>) -> TokenStream {
            let parsers = parsers
                .iter()
                .map(|(parser, version_quality_ranges)| {
                    version_quality_ranges.wrap_code(
                        quote! {
                            let result = #parser;
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        },
                        None,
                    )
                })
                .collect::<Vec<_>>();
            quote! {
                {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        #(#parsers)*
                        break;
                    }
                    helper.result(stream)
                }
            }
        }

        let mut binary_operand_terms = vec![];

        if !prefix_operator_parsers.is_empty() {
            let prefix_operator_parser = make_choice(prefix_operator_parsers);
            operator_closures.push(quote! { let prefix_operator_parser = |stream: &mut Stream| #prefix_operator_parser; });
            binary_operand_terms.push(
                quote! { ZeroOrMoreHelper::run(stream, |stream| prefix_operator_parser(stream)) },
            );
        }

        let primary_expression_parser = self.primary_expression.to_parser_code(context_name, false);
        operator_closures.push(quote! { let primary_expression_parser = |stream: &mut Stream| #primary_expression_parser; });
        binary_operand_terms.push(quote! {  primary_expression_parser(stream) });

        if !postfix_operator_parsers.is_empty() {
            let postfix_operator_parser = make_choice(postfix_operator_parsers);
            operator_closures.push(quote! { let postfix_operator_parser = |stream: &mut Stream| #postfix_operator_parser; });
            binary_operand_terms.push(
                quote! { ZeroOrMoreHelper::run(stream, |stream| postfix_operator_parser(stream)) },
            );
        }

        let binary_operand_parser = make_sequence(binary_operand_terms);

        if binary_operator_parsers.is_empty() {
            operator_closures.push(quote! { let linear_expression_parser = |stream: &mut Stream| #binary_operand_parser; });
        } else {
            operator_closures.push(quote! { let binary_operand_parser = |stream: &mut Stream| #binary_operand_parser; });

            let binary_operator_parser = make_choice(binary_operator_parsers);
            operator_closures.push(quote! { let binary_operator_parser = |stream: &mut Stream| #binary_operator_parser; });

            let linear_expression_parser =
                make_sequence(vec![quote! { binary_operand_parser(stream) }, {
                    let pairs = make_sequence(vec![
                        quote! { binary_operator_parser(stream) },
                        quote! { binary_operand_parser(stream)  },
                    ]);
                    quote! { ZeroOrMoreHelper::run(stream, |stream| #pairs) }
                }]);
            operator_closures
                .push(quote! { let linear_expression_parser = |stream: &mut Stream| #linear_expression_parser; });
        }

        let expression_kind_literal = if let Some(kind) = expression_kind {
            quote! { Some(RuleKind::#kind) }
        } else {
            quote! { None }
        };

        quote! {
            #(#operator_closures)*
            PrecedenceHelper::reduce_precedence_result(#expression_kind_literal, linear_expression_parser(stream))
        }
    }
}
