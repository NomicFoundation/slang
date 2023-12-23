use codegen_grammar::{
    Named, ParserDefinitionNode, ParserDefinitionRef, TriviaParserDefinitionRef, VersionQuality,
    VersionQualityRange,
};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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
    #[allow(clippy::too_many_lines)] // giant switch over parser definition node types
    fn to_parser_code(&self, context_name: &'static str, is_trivia: bool) -> TokenStream {
        let context = format_ident!("{context_name}");
        let lex_ctx = quote! { LexicalContextType::#context };

        match self {
            Self::Versioned(body, _) => body.to_parser_code(context_name, is_trivia),

            Self::Optional(node) => {
                let parser = node.to_parser_code(context_name, is_trivia);
                quote! {
                    OptionalHelper::transform(#parser)
                }
            }

            Self::ZeroOrMore(Named { name, node }) => {
                let parser = node.to_parser_code(context_name, is_trivia);

                let parser = if name.is_empty() {
                    parser
                } else {
                    quote! {
                        #parser.with_name(#name)
                    }
                };

                quote! {
                    ZeroOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::OneOrMore(Named { name, node }) => {
                let parser = node.to_parser_code(context_name, is_trivia);

                let parser = if name.is_empty() {
                    parser
                } else {
                    quote! {
                        #parser.with_name(#name)
                    }
                };

                quote! {
                    OneOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::Sequence(nodes) => match &nodes[..] {
                [Named { name, node }] => {
                    let parser = node.to_parser_code(context_name, is_trivia);

                    if name.is_empty() {
                        parser
                    } else {
                        quote! {
                            #parser.with_name(#name)
                        }
                    }
                }
                nodes => make_sequence_versioned(nodes.iter().map(|Named { name, node }| {
                    (
                        node.to_parser_code(context_name, is_trivia),
                        name.clone(),
                        node.applicable_version_quality_ranges(),
                    )
                })),
            },

            Self::Choice(Named { name, node: nodes }) => {
                let parser = make_choice_versioned(nodes.iter().map(|node| {
                    (
                        node.to_parser_code(context_name, is_trivia),
                        node.applicable_version_quality_ranges(),
                    )
                }));

                if name.is_empty() {
                    parser
                } else {
                    quote! {
                        #parser.with_name(#name)
                    }
                }
            }

            Self::ScannerDefinition(scanner_definition) => {
                let kind = format_ident!("{name}", name = scanner_definition.name());

                let parse_token = if is_trivia {
                    format_ident!("parse_token")
                } else {
                    format_ident!("parse_token_with_trivia")
                };

                quote! {
                    self.#parse_token::<#lex_ctx>(input, TokenKind::#kind)
                }
            }

            Self::TriviaParserDefinition(trivia_parser_definition) => {
                let function_name =
                    format_ident!("{}", trivia_parser_definition.name().to_snake_case());

                quote! { self.#function_name(input) }
            }

            Self::ParserDefinition(parser_definition) => {
                assert!(
                    !is_trivia,
                    "Trivia productions can only reference trivia or token productions"
                );

                if parser_definition.is_inline() {
                    parser_definition.to_parser_code()
                } else {
                    let function_name =
                        format_ident!("{}", parser_definition.name().to_snake_case());

                    quote! {
                        self.#function_name(input)
                    }
                }
            }

            Self::PrecedenceParserDefinition(precedence_parser_definition) => {
                assert!(
                    !is_trivia,
                    "Trivia productions can only reference trivia or token productions"
                );

                let function_name =
                    format_ident!("{}", precedence_parser_definition.name().to_snake_case());

                quote! { self.#function_name(input) }
            }

            Self::DelimitedBy(open, body, close) => {
                let open_field_name = &open.name;
                let close_field_name = &close.name;
                let [open_delim, close_delim] = match (open.as_ref(), close.as_ref()) {
                    (
                        ParserDefinitionNode::ScannerDefinition(open, ..),
                        ParserDefinitionNode::ScannerDefinition(close, ..),
                    ) => [open, close].map(|scanner| format_ident!("{}", scanner.name())),
                    _ => unreachable!("Only tokens are permitted as delimiters"),
                };

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.applicable_version_quality_ranges().wrap_code(
                    quote! {
                        seq.elem(#parser
                            .recover_until_with_nested_delims::<_, #lex_ctx>(input,
                                self,
                                TokenKind::#close_delim,
                                RecoverFromNoMatch::Yes,
                            )
                        )?;
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::#close_delim);
                        let input = delim_guard.ctx();

                        seq.elem_named(#open_field_name, self.parse_token_with_trivia::<#lex_ctx>(input, TokenKind::#open_delim))?;
                        #body_parser
                        seq.elem_named(#close_field_name, self.parse_token_with_trivia::<#lex_ctx>(input, TokenKind::#close_delim))?;
                        seq.finish()
                    })
                }
            }

            Self::SeparatedBy(body, separator) => {
                let separator_field_name = &separator.name;
                let separator = match separator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => {
                        format_ident!("{name}", name = scanner.name())
                    }
                    _ => unreachable!("Only tokens are permitted as separators"),
                };

                let body_field_name = &body.name;
                let parser = body.to_parser_code(context_name, is_trivia);

                quote! {
                    SeparatedHelper::run::<_, #lex_ctx>(
                        input,
                        self,
                        |input| #parser.with_name(#body_field_name),
                        TokenKind::#separator,
                        #separator_field_name,
                    )
                }
            }
            Self::TerminatedBy(body, terminator) => {
                let terminator_field_name = &terminator.name;

                let terminator = match terminator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => {
                        format_ident!("{name}", name = scanner.name())
                    }
                    _ => unreachable!("Only tokens are permitted as terminators"),
                };

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.applicable_version_quality_ranges().wrap_code(
                    quote! {
                        seq.elem(
                            #parser
                            .recover_until_with_nested_delims::<_, #lex_ctx>(input,
                                self,
                                TokenKind::#terminator,
                                RecoverFromNoMatch::No,
                            )
                        )?;
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        #body_parser
                        seq.elem_named(
                            #terminator_field_name,
                            self.parse_token_with_trivia::<#lex_ctx>(input, TokenKind::#terminator)
                        )?;
                        seq.finish()
                    })
                }
            }
        }
    }

    fn applicable_version_quality_ranges(&self) -> Vec<VersionQualityRange> {
        match self {
            ParserDefinitionNode::Versioned(_, version_quality_ranges) => {
                version_quality_ranges.clone()
            }

            ParserDefinitionNode::Optional(node)
            | ParserDefinitionNode::ZeroOrMore(Named { node, .. })
            | ParserDefinitionNode::OneOrMore(Named { node, .. }) => {
                node.applicable_version_quality_ranges()
            }

            _ => vec![],
        }
    }
}

pub trait VersionQualityRangeVecExtensions {
    fn wrap_code(&self, if_true: TokenStream, if_false: Option<TokenStream>) -> TokenStream;
    // Quotes a boolean expression that is satisfied for the given version quality ranges
    fn as_bool_expr(&self) -> TokenStream;
}

impl VersionQualityRangeVecExtensions for Vec<VersionQualityRange> {
    fn as_bool_expr(&self) -> TokenStream {
        if self.is_empty() {
            quote!(true)
        } else {
            let flags = self.iter().map(|vqr| {
                let flag = format_ident!(
                    "version_is_at_least_{v}",
                    v = &vqr.from.to_string().replace('.', "_")
                );
                if vqr.quality == VersionQuality::Introduced {
                    quote! { self.#flag }
                } else {
                    quote! { !self.#flag }
                }
            });
            quote! { #(#flags)&&* }
        }
    }

    fn wrap_code(&self, if_true: TokenStream, if_false: Option<TokenStream>) -> TokenStream {
        if self.is_empty() {
            if_true
        } else {
            let condition = self.as_bool_expr();

            let else_part = if_false.map(|if_false| quote! { else { #if_false } });
            quote! { if #condition { #if_true } #else_part }
        }
    }
}

pub fn make_sequence(parsers: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    make_sequence_versioned(
        parsers
            .into_iter()
            .map(|parser| (parser, String::new(), vec![])),
    )
}

pub fn make_sequence_versioned(
    parsers: impl IntoIterator<Item = (TokenStream, String, Vec<VersionQualityRange>)>,
) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|(parser, name, versions)| {
            let code = if name.is_empty() {
                quote! { seq.elem(#parser)?; }
            } else {
                quote! { seq.elem_named(#name, #parser)?; }
            };

            versions.wrap_code(code, None)
        })
        .collect::<Vec<_>>();
    quote! {
        SequenceHelper::run(|mut seq| {
            #(#parsers)*
            seq.finish()
        })
    }
}

pub fn make_choice(parsers: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    make_choice_versioned(parsers.into_iter().map(|parser| (parser, vec![])))
}

fn make_choice_versioned(
    parsers: impl IntoIterator<Item = (TokenStream, Vec<VersionQualityRange>)>,
) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|(parser, versions)| {
            versions.wrap_code(
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
