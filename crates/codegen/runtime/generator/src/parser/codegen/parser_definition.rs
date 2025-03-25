use codegen_language_definition::model::{Identifier, VersionSpecifier};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::versioned::{Versioned as _, VersionedQuote as _};
use crate::parser::grammar::{
    Labeled, ParserDefinitionNode, ParserDefinitionRef, TriviaParserDefinitionRef,
};

pub trait ParserDefinitionCodegen {
    fn to_parser_code(&self) -> TokenStream;
}

impl ParserDefinitionCodegen for ParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        let code = self.node().version_specifier().to_conditional_code(
            self.node().to_parser_code(self.context(), false),
            Some(quote! { ParserResult::no_match(vec![]) }),
        );

        let nonterminal_kind = format_ident!("{}", self.name());
        quote! { #code.with_kind(NonterminalKind::#nonterminal_kind) }
    }
}

impl ParserDefinitionCodegen for TriviaParserDefinitionRef {
    fn to_parser_code(&self) -> TokenStream {
        self.node().to_parser_code(self.context(), true)
    }
}

pub(super) trait ParserDefinitionNodeCodegen {
    fn to_parser_code(&self, context_name: &Identifier, is_trivia: bool) -> TokenStream;
}

impl ParserDefinitionNodeCodegen for ParserDefinitionNode {
    #[allow(clippy::too_many_lines)] // giant switch over parser definition node types
    fn to_parser_code(&self, context_name: &Identifier, is_trivia: bool) -> TokenStream {
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

            Self::ZeroOrMore(Labeled { label, value }) => {
                let parser = value.to_parser_code(context_name, is_trivia);

                let parser = if label.is_empty() {
                    parser
                } else {
                    let name = format_ident!("{}", label.to_pascal_case());

                    quote! {
                        #parser.with_label(EdgeLabel::#name)
                    }
                };

                quote! {
                    ZeroOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::OneOrMore(Labeled { label, value }) => {
                let parser = value.to_parser_code(context_name, is_trivia);

                let parser = if label.is_empty() {
                    parser
                } else {
                    let name = format_ident!("{}", label.to_pascal_case());

                    quote! {
                        #parser.with_label(EdgeLabel::#name)
                    }
                };

                quote! {
                    OneOrMoreHelper::run(input, |input| #parser)
                }
            }

            Self::Sequence(nodes) => match &nodes[..] {
                [Labeled { label, value }] => {
                    let parser = value.to_parser_code(context_name, is_trivia);

                    if label.is_empty() {
                        parser
                    } else {
                        let name = format_ident!("{}", label.to_pascal_case());

                        quote! {
                            #parser.with_label(EdgeLabel::#name)
                        }
                    }
                }
                nodes => make_sequence_versioned(nodes.iter().map(|Labeled { label, value }| {
                    (
                        value.to_parser_code(context_name, is_trivia),
                        label.clone(),
                        value.version_specifier(),
                    )
                })),
            },

            Self::Choice(Labeled { label, value }) => {
                let parser = make_choice_versioned(value.iter().map(|node| {
                    (
                        node.to_parser_code(context_name, is_trivia),
                        node.version_specifier(),
                    )
                }));

                if label.is_empty() {
                    parser
                } else {
                    let name = format_ident!("{}", label.to_pascal_case());

                    quote! {
                        #parser.with_label(EdgeLabel::#name)
                    }
                }
            }

            Self::ScannerDefinition(scanner_definition) => {
                let kind = format_ident!("{name}", name = scanner_definition.name());

                let parse_terminal = if is_trivia {
                    format_ident!("parse_terminal")
                } else {
                    format_ident!("parse_terminal_with_trivia")
                };

                quote! {
                    self.#parse_terminal::<#lex_ctx>(input, TerminalKind::#kind)
                }
            }

            // Keyword scanner uses the promotion inside the parse_terminal
            Self::KeywordScannerDefinition(scanner_definition) => {
                let kind = format_ident!("{name}", name = scanner_definition.name);

                let parse_terminal = if is_trivia {
                    format_ident!("parse_terminal")
                } else {
                    format_ident!("parse_terminal_with_trivia")
                };

                quote! {
                    self.#parse_terminal::<#lex_ctx>(input, TerminalKind::#kind)
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

            Self::DelimitedBy(open, body, close, threshold) => {
                let open_label = format_ident!("{}", open.label.to_pascal_case());
                let close_label = format_ident!("{}", close.label.to_pascal_case());
                let [open_delim, close_delim] = match (open.as_ref(), close.as_ref()) {
                    (
                        ParserDefinitionNode::ScannerDefinition(open, ..),
                        ParserDefinitionNode::ScannerDefinition(close, ..),
                    ) => [open, close].map(|scanner| format_ident!("{}", scanner.name())),
                    _ => unreachable!("Only tokens are permitted as delimiters"),
                };
                let threshold = threshold.0;

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.version_specifier().to_conditional_code(
                    quote! {
                        match #parser {
                            // Make sure it matches the required threshold before attempting recovery:
                            result if result.matches_at_least_n_terminals(#threshold) => {
                                seq.elem(result.recover_until_with_nested_delims::<_, #lex_ctx>(
                                    input,
                                    self,
                                    TerminalKind::#close_delim,
                                ))?;
                            },
                            // Otherwise, reject the entire sequence, including the open delimiter:
                            _ => {
                                return std::ops::ControlFlow::Break(ParserResult::no_match(vec![]));
                            },
                        }
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TerminalKind::#close_delim);
                        let input = delim_guard.ctx();

                        seq.elem_labeled(
                            EdgeLabel::#open_label,
                            self.parse_terminal_with_trivia::<#lex_ctx>(input, TerminalKind::#open_delim)
                        )?;
                        #body_parser
                        seq.elem_labeled(
                            EdgeLabel::#close_label,
                            self.parse_terminal_with_trivia::<#lex_ctx>(input, TerminalKind::#close_delim)
                        )?;
                        seq.finish()
                    })
                }
            }

            Self::SeparatedBy(body, separator) => {
                let separator_label = format_ident!("{}", separator.label.to_pascal_case());
                let separator = match separator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => {
                        format_ident!("{name}", name = scanner.name())
                    }
                    _ => unreachable!("Only tokens are permitted as separators"),
                };

                let body_label = format_ident!("{}", body.label.to_pascal_case());
                let parser = body.to_parser_code(context_name, is_trivia);

                quote! {
                    SeparatedHelper::run::<_, #lex_ctx>(
                        input,
                        self,
                        |input| #parser.with_label(EdgeLabel::#body_label),
                        TerminalKind::#separator,
                        EdgeLabel::#separator_label,
                    )
                }
            }
            Self::TerminatedBy(body, terminator) => {
                let terminator_label = format_ident!("{}", terminator.label.to_pascal_case());

                let terminator = match terminator.as_ref() {
                    ParserDefinitionNode::ScannerDefinition(scanner, ..) => {
                        format_ident!("{name}", name = scanner.name())
                    }
                    _ => unreachable!("Only tokens are permitted as terminators"),
                };

                let parser = body.to_parser_code(context_name, is_trivia);
                let body_parser = body.version_specifier().to_conditional_code(
                    quote! {
                        match #parser {
                            // Require at least one terminal before attempting recovery:
                            result if result.matches_at_least_n_terminals(1u8) => {
                                seq.elem(result.recover_until_with_nested_delims::<_, #lex_ctx>(
                                    input,
                                    self,
                                    TerminalKind::#terminator,
                                ))?;
                            },
                            // Otherwise, append the result as-is:
                            result => {
                                seq.elem(result)?;
                            },
                        }
                    },
                    None,
                );

                quote! {
                    SequenceHelper::run(|mut seq| {
                        #body_parser
                        seq.elem_labeled(
                            EdgeLabel::#terminator_label,
                            self.parse_terminal_with_trivia::<#lex_ctx>(input, TerminalKind::#terminator)
                        )?;
                        seq.finish()
                    })
                }
            }
        }
    }
}

pub fn make_sequence(parsers: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    make_sequence_versioned(
        parsers
            .into_iter()
            .map(|parser| (parser, String::new(), None)),
    )
}

pub fn make_sequence_versioned<'a>(
    parsers: impl IntoIterator<Item = (TokenStream, String, Option<&'a VersionSpecifier>)>,
) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|(parser, name, versions)| {
            let code = if name.is_empty() {
                quote! { seq.elem(#parser)?; }
            } else {
                let label = format_ident!("{}", name.to_pascal_case());

                quote! { seq.elem_labeled(EdgeLabel::#label, #parser)?; }
            };

            versions.to_conditional_code(code, None)
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
    make_choice_versioned(parsers.into_iter().map(|parser| (parser, None)))
}

fn make_choice_versioned<'a>(
    parsers: impl IntoIterator<Item = (TokenStream, Option<&'a VersionSpecifier>)>,
) -> TokenStream {
    let parsers = parsers
        .into_iter()
        .map(|(parser, versions)| {
            versions.to_conditional_code(
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
