use codegen_language_definition::model::Identifier;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::parser::codegen::parser_definition::{
    make_choice, make_sequence, ParserDefinitionNodeCodegen,
};
use crate::parser::grammar::{
    PrecedenceOperatorModel, PrecedenceParserDefinitionNode, PrecedenceParserDefinitionRef,
};

pub trait PrecedenceParserDefinitionCodegen {
    fn to_parser_code(&self) -> TokenStream;
    /// Emit a helper parser function for each precedence expression that ensures the main parser
    /// identifies a single node of the expected type, with a child node being the expected
    /// precedence expression.
    fn to_precedence_expression_parser_code(&self) -> Vec<(Identifier, TokenStream)>;
}

impl PrecedenceParserDefinitionCodegen for PrecedenceParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        let code = self
            .node()
            .to_parser_code(self.context(), format_ident!("{}", self.name()));

        let nonterminal_kind = format_ident!("{}", self.name());
        quote! { #code.with_kind(NonterminalKind::#nonterminal_kind) }
    }

    fn to_precedence_expression_parser_code(&self) -> Vec<(Identifier, TokenStream)> {
        let parser_name = format_ident!("{}", self.name().to_snake_case());
        let nonterminal_name = format_ident!("{}", self.name());

        self.node().precedence_expression_names.iter().map(|name| {
            let op_nonterminal_name = format_ident!("{name}");

            // Ensure that the parser correctly identifies a single node of the expected type,
            // which contains a single child node representing the expected precedence operator.
            let code = quote! {
                let result = self.#parser_name(input);
                let ParserResult::Match(r#match) = &result else { return result; };

                // If the result won't match exactly, we return a dummy `ParserResult::no_match`, since
                // can't precisely determine the expected terminals or completeness of the match otherwise.
                match &r#match.nodes[..] {
                    [cst::Edge { node: cst::Node::Nonterminal(node), .. }] if node.kind == NonterminalKind::#nonterminal_name => match &node.children[..] {
                        [inner @ cst::Edge { node: cst::Node::Nonterminal(node), .. }] if node.kind == NonterminalKind::#op_nonterminal_name => {
                            ParserResult::r#match(vec![inner.clone()], r#match.expected_terminals.clone())
                        }
                        _ => ParserResult::default(),
                    }
                    _ => ParserResult::default(),
                }
            };


            (name.clone(), code)
        }).collect()
    }
}

pub(super) trait PrecedenceParserDefinitionNodeCodegen {
    fn to_parser_code(&self, context_name: &Identifier, expression_kind: Ident) -> TokenStream;
}

impl PrecedenceParserDefinitionNodeCodegen for PrecedenceParserDefinitionNode {
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
    // precedence parser root e.g. `NonterminalKind::Expression`.
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
    fn to_parser_code(&self, context_name: &Identifier, expression_kind: Ident) -> TokenStream {
        let mut prefix_operator_parsers: Vec<TokenStream> = Vec::new();
        let mut postfix_operator_parsers: Vec<TokenStream> = Vec::new();
        let mut binary_operator_parsers: Vec<TokenStream> = Vec::new();

        // Closures rather than local functions because they
        // need to access the `Language` instance as `self`.
        let mut operator_closures = Vec::new();

        let mut binding_power = 1u8;
        for (model, name, operator_definition) in &self.operators {
            let operator_code = operator_definition.to_parser_code(context_name, false);
            let nonterminal_kind = format_ident!("{}", name);
            let model_name = match model {
                PrecedenceOperatorModel::BinaryLeftAssociative => "left",
                PrecedenceOperatorModel::BinaryRightAssociative => "right",
                PrecedenceOperatorModel::Prefix => "prefix",
                PrecedenceOperatorModel::Postfix => "postfix",
            };
            let closure_name =
                format_ident!("parse_{model_name}_{name}", name = name.to_snake_case());

            let parser = quote! { #closure_name(input) };

            match model {
                PrecedenceOperatorModel::BinaryLeftAssociative => {
                    operator_closures.push(quote! {
                        let #closure_name = |input: &mut ParserContext<'_>|
                            PrecedenceHelper::to_binary_operator(
                                NonterminalKind::#nonterminal_kind,
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
                                NonterminalKind::#nonterminal_kind,
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
                                NonterminalKind::#nonterminal_kind,
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
                                NonterminalKind::#nonterminal_kind,
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

        // First, establish the binary operand parser `BinaryOperand ::= PrefixOperator* PrimaryExpression PostfixOperator*`
        if !prefix_operator_parsers.is_empty() {
            let prefix_operator_parser = make_choice(prefix_operator_parsers);
            operator_closures.push(quote! { let prefix_operator_parser = |input: &mut ParserContext<'_>| #prefix_operator_parser; });
            binary_operand_terms
                .push(quote! { ZeroOrMoreHelper::run(input, prefix_operator_parser) });
        }

        let primary_expression_parser = self.primary_expression.to_parser_code(context_name, false);
        operator_closures.push(quote! { let primary_expression_parser = |input: &mut ParserContext<'_>| #primary_expression_parser; });
        binary_operand_terms.push(quote! {  primary_expression_parser(input) });

        if !postfix_operator_parsers.is_empty() {
            let postfix_operator_parser = make_choice(postfix_operator_parsers);
            operator_closures.push(quote! { let postfix_operator_parser = |input: &mut ParserContext<'_>| #postfix_operator_parser; });
            binary_operand_terms
                .push(quote! { ZeroOrMoreHelper::run(input, postfix_operator_parser) });
        }

        let binary_operand_parser = make_sequence(binary_operand_terms);

        // Now, establish the linear expression parser `Expression ::= BinaryOperand ( BinaryOperator BinaryOperand )*`
        let linear_expression_parser = if binary_operator_parsers.is_empty() {
            // No binary operators, so the expression is simply `BinaryOperand`
            binary_operand_parser
        } else {
            operator_closures.push(quote! { let binary_operand_parser = |input: &mut ParserContext<'_>| #binary_operand_parser; });

            let binary_operator_parser = make_choice(binary_operator_parsers);
            operator_closures.push(quote! { let binary_operator_parser = |input: &mut ParserContext<'_>| #binary_operator_parser; });

            // `BinaryOperand ( BinaryOperator BinaryOperand )*`
            make_sequence(vec![quote! { binary_operand_parser(input) }, {
                let pairs = make_sequence(vec![
                    quote! { binary_operator_parser(input) },
                    quote! { binary_operand_parser(input)  },
                ]);
                quote! { ZeroOrMoreHelper::run(input, |input| #pairs) }
            }])
        };

        operator_closures
                .push(quote! { let linear_expression_parser = |input: &mut ParserContext<'_>| #linear_expression_parser; });

        quote! {
            #(
                #operator_closures
            )*

            PrecedenceHelper::reduce_precedence_result(NonterminalKind::#expression_kind, linear_expression_parser(input))
        }
    }
}
