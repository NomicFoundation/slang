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
            Some(quote! { ParserResult::no_match(vec![]) }),
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
                    ZeroOrMoreHelper::run(stream, |stream| #parser)
                }
            }

            Self::OneOrMore(node, _) => {
                let parser = node.to_parser_code(context_name, is_trivia);
                quote! {
                    OneOrMoreHelper::run(stream, |stream| #parser)
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
                            node.applicable_version_quality_ranges().wrap_code(
                                quote! {
                                    if helper.handle_next_result(#parser) {
                                        break;
                                    }
                                },
                                None,
                            )
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
            }

            Self::Choice(nodes, _) => {
                let parsers = nodes
                    .iter()
                    .map(|node| {
                        let parser = node.to_parser_code(context_name, is_trivia);
                        node.applicable_version_quality_ranges().wrap_code(
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

            Self::ScannerDefinition(scanner_definition, _) => {
                let kind = format_ident!("{name}", name = scanner_definition.name());
                if is_trivia {
                    let function_name =
                        format_ident!("{}_parse_token", context_name.to_snake_case());
                    quote! {
                        self.#function_name(stream, TokenKind::#kind)
                    }
                } else {
                    let function_name =
                        format_ident!("{}_parse_token_with_trivia", context_name.to_snake_case());
                    quote! {
                        self.#function_name(stream, TokenKind::#kind)
                    }
                }
            }

            Self::TriviaParserDefinition(trivia_parser_definition, _) => {
                let function_name = format_ident!(
                    "{snake_case}",
                    snake_case = trivia_parser_definition.name().to_snake_case()
                );
                quote! { self.#function_name(stream) }
            }

            Self::ParserDefinition(parser_definition, _) => {
                if is_trivia {
                    unreachable!(
                        "Trivia productions can only reference trivia or token productions"
                    )
                }
                let function_name = format_ident!(
                    "{snake_case}",
                    snake_case = parser_definition.name().to_snake_case()
                );
                quote! {
                    self.#function_name(stream)
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
                quote! { self.#function_name(stream) }
            }

            Self::DelimitedBy(open, body, close, loc) => {
                Self::Sequence(vec![*open.clone(), *body.clone(), *close.clone()], *loc)
                    .to_parser_code(context_name, is_trivia)
            }

            Self::SeparatedBy(body, separator, loc) => Self::Sequence(
                vec![
                    *body.clone(),
                    Self::ZeroOrMore(
                        Box::new(Self::Sequence(
                            vec![*separator.clone(), *body.clone()],
                            *loc,
                        )),
                        *loc,
                    ),
                ],
                *loc,
            )
            .to_parser_code(context_name, is_trivia),

            Self::TerminatedBy(body, terminator, loc) => {
                Self::Sequence(vec![*body.clone(), *terminator.clone()], *loc)
                    .to_parser_code(context_name, is_trivia)
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
                if vqr.quality.0 == VersionQuality::Enabled {
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
