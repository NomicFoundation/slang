use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use codegen_grammar::{
    ParserDefinitionNode, ParserDefinitionRef, TriviaParserDefinitionRef, VersionQuality,
    VersionQualityRange,
};

pub trait ParserDefinitionExtensions {
    fn to_parser_code(&self) -> TokenStream;
}

impl ParserDefinitionExtensions for ParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        self.node().applicable_version_quality_ranges().wrap_code(
            self.node().to_parser_code(self.context(), false),
            Some(quote! { ParserResult::disabled() }),
        )
    }
}

impl ParserDefinitionExtensions for TriviaParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        self.node().to_parser_code(self.context(), true)
    }
}

pub trait ParserDefinitionNodeExtensions {
    fn to_parser_code(&self, context_name: &'static str, is_trivia: bool) -> TokenStream;
    fn applicable_version_quality_ranges(&self) -> Vec<VersionQualityRange>;
}

impl ParserDefinitionNodeExtensions for ParserDefinitionNode {
    fn to_parser_code(&self, context_name: &'static str, is_trivia: bool) -> TokenStream {
        match self {
            Self::Versioned(body, _, _) => body.to_parser_code(context_name, is_trivia),

            Self::Optional(node, _) => {
                let parser = node.to_parser_code(context_name, is_trivia);
                quote! {
                    OptionalHelper::transform(#parser)
                }
            }

            Self::ZeroOrMore(node, _) => {
                let parser = node.to_parser_code(context_name, is_trivia);
                quote! {
                    ZeroOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::OneOrMore(node, _) => {
                let parser = node.to_parser_code(context_name, is_trivia);
                quote! {
                    OneOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::Sequence(nodes, _) => {
                if nodes.len() == 1 {
                    nodes[0].to_parser_code(context_name, is_trivia)
                } else {
                    let parsers = nodes
                        .iter()
                        .map(|node| {
                            let parser = node.to_parser_code(context_name, is_trivia);
                            node.applicable_version_quality_ranges()
                                .wrap_code(quote! { seq.elem(#parser)?; }, None)
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        SequenceHelper::run(|mut seq| {
                            #(#parsers)*
                            seq.finish()
                        })
                    }
                }
            }

            Self::Choice(nodes, _) => {
                let parsers = nodes
                    .iter()
                    .map(|node| {
                        let parser = node.to_parser_code(context_name, is_trivia);
                        node.applicable_version_quality_ranges().wrap_code(
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

            Self::ScannerDefinition(scanner_definition, _) => {
                let kind = format_ident!("{name}", name = scanner_definition.name());
                if is_trivia {
                    let function_name =
                        format_ident!("{}_parse_token", context_name.to_snake_case());
                    quote! {
                        self.#function_name(input, TokenKind::#kind)
                    }
                } else {
                    let function_name =
                        format_ident!("{}_parse_token_with_trivia", context_name.to_snake_case());
                    quote! {
                        self.#function_name(input, TokenKind::#kind)
                    }
                }
            }

            Self::TriviaParserDefinition(trivia_parser_definition, _) => {
                let function_name = format_ident!(
                    "{snake_case}",
                    snake_case = trivia_parser_definition.name().to_snake_case()
                );
                quote! { self.#function_name(input) }
            }

            Self::ParserDefinition(parser_definition, _) => {
                if is_trivia {
                    unreachable!(
                        "Trivia productions can only reference trivia or token productions"
                    )
                }
                if parser_definition.is_inline() {
                    parser_definition.to_parser_code()
                } else {
                    let function_name = format_ident!(
                        "{snake_case}",
                        snake_case = parser_definition.name().to_snake_case()
                    );
                    quote! {
                        self.#function_name(input)
                    }
                }
            }

            Self::PrecedenceParserDefinition(precedence_parser_definition, _) => {
                if is_trivia {
                    unreachable!(
                        "Trivia productions can only reference trivia or token productions"
                    )
                }
                let function_name = format_ident!(
                    "{snake_case}",
                    snake_case = precedence_parser_definition.name().to_snake_case()
                );
                quote! { self.#function_name(input) }
            }

            Self::DelimitedBy(open, body, close, _) => {
                let [open_token, close_token] = match (open.as_ref(), close.as_ref()) {
                    (
                        ParserDefinitionNode::ScannerDefinition(open, ..),
                        ParserDefinitionNode::ScannerDefinition(close, ..),
                    ) => [open, close].map(|scanner| format_ident!("{}", scanner.name())),
                    _ => unreachable!("Only tokens are permitted as delimiters"),
                };

                let parse_token = format_ident!(
                    "{context_name}_parse_token_with_trivia",
                    context_name = context_name.to_snake_case()
                );

                let delimiters = format_ident!(
                    "{context_name}_delimiters",
                    context_name = context_name.to_snake_case()
                );

                let context = format_ident!("{context_name}");

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.applicable_version_quality_ranges().wrap_code(
                    quote! {
                        seq.elem(#parser
                            .recover_until_with_nested_delims(input,
                                |input| Lexer::next_token::<{ LexicalContext::#context as u8 }>(self, input),
                                |input| Lexer::leading_trivia(self, input),
                                TokenKind::#close_token,
                                Self::#delimiters(),
                            )
                        )?;
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::#close_token);
                        let input = delim_guard.ctx();

                        seq.elem(self.#parse_token(input, TokenKind::#open_token))?;
                        #body_parser
                        seq.elem(self.#parse_token(input, TokenKind::#close_token))?;
                        seq.finish()
                    })
                }
            }

            Self::SeparatedBy(body, separator, _) => {
                let separator_scanner = match separator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => scanner,
                    _ => unreachable!("Only tokens are permitted as separators"),
                };

                let separator_token_kind = format_ident!("{name}", name = separator_scanner.name());
                let context = format_ident!("{context_name}");

                let parser = body.to_parser_code(context_name, is_trivia);

                quote! {
                    SeparatedHelper::run::<{ LexicalContext::#context as u8}, Self>(
                        input,
                        |input| #parser,
                        TokenKind::#separator_token_kind,
                        self,
                    )
                }
            }
            Self::TerminatedBy(body, terminator, _) => {
                let terminator_scanner = match terminator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => scanner,
                    _ => unreachable!("Only tokens are permitted as terminators"),
                };

                let terminator_token_kind =
                    format_ident!("{name}", name = terminator_scanner.name());

                let context = format_ident!("{context_name}");

                let delimiters = format_ident!(
                    "{context_name}_delimiters",
                    context_name = context_name.to_snake_case()
                );

                let parse_token = format_ident!(
                    "{context_name}_parse_token_with_trivia",
                    context_name = context_name.to_snake_case()
                );

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.applicable_version_quality_ranges().wrap_code(
                    quote! {
                        seq.elem(#parser
                            .recover_until_with_nested_delims(input,
                                |input| Lexer::next_token::<{ LexicalContext::#context as u8 }>(self, input),
                                |input| Lexer::leading_trivia(self, input),
                                TokenKind::#terminator_token_kind,
                                Self::#delimiters(),
                            )
                        )?;
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        #body_parser
                        seq.elem(self.#parse_token(input, TokenKind::#terminator_token_kind))?;
                        seq.finish()
                    })
                }
            }
        }
    }

    fn applicable_version_quality_ranges(&self) -> Vec<VersionQualityRange> {
        match self {
            ParserDefinitionNode::Versioned(_, version_quality_ranges, _) => {
                version_quality_ranges.clone()
            }

            ParserDefinitionNode::Optional(node, _)
            | ParserDefinitionNode::ZeroOrMore(node, _)
            | ParserDefinitionNode::OneOrMore(node, _) => node.applicable_version_quality_ranges(),

            _ => vec![],
        }
    }
}

pub trait VersionQualityRangeVecExtensions {
    fn wrap_code(&self, if_true: TokenStream, if_false: Option<TokenStream>) -> TokenStream;
    fn disambiguating_name_suffix(&self) -> String;
}

impl VersionQualityRangeVecExtensions for Vec<VersionQualityRange> {
    fn wrap_code(&self, if_true: TokenStream, if_false: Option<TokenStream>) -> TokenStream {
        if self.is_empty() {
            if_true
        } else {
            let flags = self.iter().map(|vqr| {
                let flag = format_ident!(
                    "version_is_at_least_{v}",
                    v = &vqr.from.0.to_string().replace('.', "_")
                );
                if vqr.quality.0 == VersionQuality::Introduced {
                    quote! { self.#flag }
                } else {
                    quote! { !self.#flag }
                }
            });
            let else_part = if_false.map(|if_false| quote! { else { #if_false } });
            quote! { if #(#flags)&&* { #if_true } #else_part }
        }
    }

    fn disambiguating_name_suffix(&self) -> String {
        let mut suffix = String::new();
        for vqr in self {
            suffix.push_str("_");
            suffix.push_str(&vqr.quality.0.to_string().to_lowercase());
            suffix.push_str("_from_");
            suffix.push_str(&vqr.from.0.to_string().replace('.', "_"));
        }
        suffix
    }
}
