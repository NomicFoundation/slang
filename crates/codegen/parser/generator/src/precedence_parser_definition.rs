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
            format_ident!("{name}", name = self.name().to_pascal_case()),
        )
    }
}

pub trait PrecedenceParserDefinitionNodeExtensions {
    fn to_parser_code(&self, context_name: &'static str, expression_kind: Ident) -> TokenStream;
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
    // `ParserResult` and the helpers such as `ChoiceHelper` handle `PrattOperatorMatch`
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

    #[allow(clippy::too_many_lines)] // Repetition-heavy with 4 kinds of precedence operators
    fn to_parser_code(&self, context_name: &'static str, expression_kind: Ident) -> TokenStream {
        let mut prefix_operator_parsers: Vec<OperatorParser> = Vec::new();
        let mut postfix_operator_parsers: Vec<OperatorParser> = Vec::new();
        let mut binary_operator_parsers: Vec<OperatorParser> = Vec::new();

        // Closures rather than local functions because they
        // need to access the `Language` instance as `self`.
        let mut operator_closures = Vec::new();

        let mut binding_power = 1u8;
        for (version_quality_ranges, model, name, operator_definition) in &self.operators {
            let operator_code = operator_definition
                .node()
                .to_parser_code(context_name, false);
            let rule_kind = format_ident!("{}", name);
            let closure_name = format_ident!(
                // Make a name that won't conflict with the parsers we define below
                "parse_{name}{version_tag}",
                version_tag = disambiguating_name_suffix(version_quality_ranges),
                name = operator_definition.name().to_snake_case()
            );

            let parser = (
                quote! { #closure_name(input) },
                version_quality_ranges.clone(),
            );

            match model {
                PrecedenceOperatorModel::BinaryLeftAssociative => {
                    operator_closures.push(quote! {
                        let #closure_name = |input: &mut ParserContext<'_>|
                            PrecedenceHelper::to_binary_operator(
                                RuleKind::#rule_kind,
                                #binding_power,
                                #binding_power + 1,
                                #operator_code
                            );
                    });
                    binary_operator_parsers.push(parser);
                }
                PrecedenceOperatorModel::BinaryRightAssociative => {
                    operator_closures.push(quote! {
                        let #closure_name = |input: &mut ParserContext<'_>|
                            PrecedenceHelper::to_binary_operator(
                                RuleKind::#rule_kind,
                                #binding_power + 1,
                                #binding_power,
                                #operator_code
                            );
                    });
                    binary_operator_parsers.push(parser);
                }
                PrecedenceOperatorModel::Prefix => {
                    operator_closures.push(quote! {
                        let #closure_name = |input: &mut ParserContext<'_>|
                            PrecedenceHelper::to_prefix_operator(
                                RuleKind::#rule_kind,
                                #binding_power,
                                #operator_code
                            );
                    });
                    prefix_operator_parsers.push(parser);
                }
                PrecedenceOperatorModel::Postfix => {
                    operator_closures.push(quote! {
                        let #closure_name = |input: &mut ParserContext<'_>|
                            PrecedenceHelper::to_postfix_operator(
                                RuleKind::#rule_kind,
                                #binding_power,
                                #operator_code
                            );
                    });
                    postfix_operator_parsers.push(parser);
                }
            }

            binding_power += 2;
        }

        let mut binary_operand_terms = vec![];

        if !prefix_operator_parsers.is_empty() {
            let prefix_operator_parser = make_choice(prefix_operator_parsers);
            operator_closures.push(quote! { let prefix_operator_parser = |input: &mut ParserContext<'_>| #prefix_operator_parser; });
            binary_operand_terms.push(
                quote! { ZeroOrMoreHelper::run(input, |input| prefix_operator_parser(input)) },
            );
        }

        let primary_expression_parser = self.primary_expression.to_parser_code(context_name, false);
        operator_closures.push(quote! { let primary_expression_parser = |input: &mut ParserContext<'_>| #primary_expression_parser; });
        binary_operand_terms.push(quote! {  primary_expression_parser(input) });

        if !postfix_operator_parsers.is_empty() {
            let postfix_operator_parser = make_choice(postfix_operator_parsers);
            operator_closures.push(quote! { let postfix_operator_parser = |input: &mut ParserContext<'_>| #postfix_operator_parser; });
            binary_operand_terms.push(
                quote! { ZeroOrMoreHelper::run(input, |input| postfix_operator_parser(input)) },
            );
        }

        let binary_operand_parser = make_sequence(binary_operand_terms);

        if binary_operator_parsers.is_empty() {
            operator_closures.push(quote! { let linear_expression_parser = |input: &mut ParserContext<'_>| #binary_operand_parser; });
        } else {
            operator_closures.push(quote! { let binary_operand_parser = |input: &mut ParserContext<'_>| #binary_operand_parser; });

            let binary_operator_parser = make_choice(binary_operator_parsers);
            operator_closures.push(quote! { let binary_operator_parser = |input: &mut ParserContext<'_>| #binary_operator_parser; });

            let linear_expression_parser =
                make_sequence(vec![quote! { binary_operand_parser(input) }, {
                    let pairs = make_sequence(vec![
                        quote! { binary_operator_parser(input) },
                        quote! { binary_operand_parser(input)  },
                    ]);
                    quote! { ZeroOrMoreHelper::run(input, |input| #pairs) }
                }]);
            operator_closures
                .push(quote! { let linear_expression_parser = |input: &mut ParserContext<'_>| #linear_expression_parser; });
        }

        quote! {
            #(
                // TODO(#638): remove duplicates once we use DSL v2 versioning schema
                #[allow(unused_variables)]
                #operator_closures
            )*

            PrecedenceHelper::reduce_precedence_result(RuleKind::#expression_kind, linear_expression_parser(input))
        }
    }
}

fn disambiguating_name_suffix(ranges: &[VersionQualityRange]) -> String {
    let mut suffix = String::new();
    for vqr in ranges {
        suffix.push('_');
        suffix.push_str(&vqr.quality.to_string().to_lowercase());
        suffix.push_str("_from_");
        suffix.push_str(&vqr.from.to_string().replace('.', "_"));
    }
    suffix
}

// TODO: merge these three functions into parse_definition by changing
// `to_parser_code` to use `(TokenStream, Vec<VersionQualityRange>)` as
// the core type i.e. the `OperatorParser` type above
type OperatorParser = (TokenStream, Vec<VersionQualityRange>);

fn make_sequence(parsers: Vec<TokenStream>) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|parser| quote! { seq.elem(#parser)?; })
        .collect::<Vec<_>>();
    quote! {
        SequenceHelper::run(|mut seq| {
            #(#parsers)*
            seq.finish()
        })
    }
}

fn make_choice(parsers: Vec<OperatorParser>) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|(parser, version_quality_ranges)| {
            version_quality_ranges.wrap_code(
                quote! {
                    let result = #parser;
                    choice.consider(input, result)?;
                },
                None,
            )
        })
        .collect::<Vec<_>>();
    quote! {
        ChoiceHelper::run(input, |mut choice, input| {
            #(#parsers)*
            choice.finish(input)
        })
    }
}
