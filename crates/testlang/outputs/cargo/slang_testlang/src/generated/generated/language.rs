// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated; we can't reasonably satisfy some of these lints.
#![allow(
    clippy::if_not_else,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::struct_excessive_bools,
    clippy::similar_names,
    unused_imports
)]

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;
use semver::Version;

use crate::cst;
use crate::kinds::{
    EdgeLabel, IsLexicalContext, LexicalContext, LexicalContextType, NonTerminalKind, TerminalKind,
};
use crate::lexer::{KeywordScan, Lexer, ScannedToken};
#[cfg(feature = "slang_napi_interfaces")]
use crate::napi_interface::parse_output::ParseOutput as NAPIParseOutput;
use crate::parse_output::ParseOutput;
use crate::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, SeparatedHelper, SequenceHelper, TokenAcceptanceThreshold, ZeroOrMoreHelper,
};

#[derive(Debug)]
#[cfg_attr(feature = "slang_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version_is_at_least_1_0_0: bool,
    pub(crate) version: Version,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),

    #[cfg(feature = "slang_napi_interfaces")]
    #[error("Invalid semantic version '{0}'.")]
    InvalidSemanticVersion(String),
}

#[cfg(feature = "slang_napi_interfaces")]
impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}

impl Language {
    pub const SUPPORTED_VERSIONS: &'static [Version] = &[
        Version::new(1, 0, 0),
        Version::new(1, 0, 1),
        Version::new(1, 1, 0),
        Version::new(1, 1, 1),
    ];

    pub fn new(version: Version) -> std::result::Result<Self, Error> {
        if Self::SUPPORTED_VERSIONS.binary_search(&version).is_ok() {
            Ok(Self {
                version_is_at_least_1_0_0: Version::new(1, 0, 0) <= version,
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version))
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    } /********************************************
       *         Parser Functions
       ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn addition_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::NonTerminal(node),
                ..
            }] if node.kind == NonTerminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::NonTerminal(node),
                    ..
                }] if node.kind == NonTerminalKind::AdditionExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_left_addition_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                NonTerminalKind::AdditionExpression,
                1u8,
                1u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Plus,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_prefix_negation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_prefix_operator(
                NonTerminalKind::NegationExpression,
                3u8,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Bang,
                )
                .with_label(EdgeLabel::Operator),
            )
        };
        let parse_postfix_member_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                NonTerminalKind::MemberAccessExpression,
                5u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::Period,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Period,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Member,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TerminalKind::Identifier,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_prefix_negation_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::StringLiteral,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TerminalKind::Identifier,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_label(EdgeLabel::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_member_access_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let binary_operand_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, prefix_operator_parser))?;
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_left_addition_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(binary_operand_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(binary_operator_parser(input))?;
                        seq.elem(binary_operand_parser(input))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            NonTerminalKind::Expression,
            linear_expression_parser(input),
        )
        .with_kind(NonTerminalKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TerminalKind::StringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonTerminalKind::Literal)
    }

    #[allow(unused_assignments, unused_parens)]
    fn member_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::NonTerminal(node),
                ..
            }] if node.kind == NonTerminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::NonTerminal(node),
                    ..
                }] if node.kind == NonTerminalKind::MemberAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn negation_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::Edge {
                node: cst::Node::NonTerminal(node),
                ..
            }] if node.kind == NonTerminalKind::Expression => match &node.children[..] {
                [inner @ cst::Edge {
                    node: cst::Node::NonTerminal(node),
                    ..
                }] if node.kind == NonTerminalKind::NegationExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn separated_identifiers(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_1_0_0 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| {
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TerminalKind::Identifier,
                    )
                    .with_label(EdgeLabel::Item)
                },
                TerminalKind::Period,
                EdgeLabel::Separator,
            )
        } else {
            ParserResult::disabled()
        }
        .with_kind(NonTerminalKind::SeparatedIdentifiers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.source_unit_members(input)
            .with_label(EdgeLabel::Members)
            .with_kind(NonTerminalKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree(input);
            choice.consider(input, result)?;
            let result = self.expression(input);
            choice.consider(input, result)?;
            let result = self.separated_identifiers(input);
            choice.consider(input, result)?;
            let result = self.literal(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonTerminalKind::SourceUnitMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.source_unit_member(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonTerminalKind::SourceUnitMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_labeled(
                        EdgeLabel::Keyword,
                        self.parse_token_with_trivia::<LexicalContextType::Tree>(
                            input,
                            TerminalKind::TreeKeyword,
                        ),
                    )?;
                    seq.elem_labeled(
                        EdgeLabel::Name,
                        OptionalHelper::transform(
                            self.parse_token_with_trivia::<LexicalContextType::Tree>(
                                input,
                                TerminalKind::Identifier,
                            ),
                        ),
                    )?;
                    seq.elem_labeled(EdgeLabel::Node, self.tree_node(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                    input,
                    self,
                    TerminalKind::Semicolon,
                    TokenAcceptanceThreshold(1u8),
                ),
            )?;
            seq.elem_labeled(
                EdgeLabel::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonTerminalKind::Tree)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TerminalKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_labeled(
                EdgeLabel::OpenBracket,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::OpenBracket,
                ),
            )?;
            seq.elem(
                self.tree_node_children(input)
                    .with_label(EdgeLabel::Members)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                        input,
                        self,
                        TerminalKind::CloseBracket,
                        TokenAcceptanceThreshold(0u8),
                    ),
            )?;
            seq.elem_labeled(
                EdgeLabel::CloseBracket,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TerminalKind::CloseBracket,
                ),
            )?;
            seq.finish()
        })
        .with_kind(NonTerminalKind::TreeNode)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_child(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree_node(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Tree>(
                input,
                TerminalKind::DelimitedIdentifier,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_label(EdgeLabel::Variant)
        .with_kind(NonTerminalKind::TreeNodeChild)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_children(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.tree_node_child(input).with_label(EdgeLabel::Item)
        })
        .with_kind(NonTerminalKind::TreeNodeChildren)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self
                    .parse_token::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_token::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_token::<LexicalContextType::Default>(
                        input,
                        TerminalKind::SingleLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                let result = self
                    .parse_token::<LexicalContextType::Default>(
                        input,
                        TerminalKind::MultiLineComment,
                    )
                    .with_label(EdgeLabel::LeadingTrivia);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.parse_token::<LexicalContextType::Default>(input, TerminalKind::Whitespace)
                    .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.parse_token::<LexicalContextType::Default>(
                    input,
                    TerminalKind::SingleLineComment,
                )
                .with_label(EdgeLabel::TrailingTrivia),
            ))?;
            seq.elem(
                self.parse_token::<LexicalContextType::Default>(input, TerminalKind::EndOfLine)
                    .with_label(EdgeLabel::TrailingTrivia),
            )?;
            seq.finish()
        })
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, 't'),
            scan_chars!(input, 'r'),
            scan_chars!(input, 'n'),
            scan_chars!(input, '\\'),
            scan_chars!(input, '\''),
            scan_chars!(input, '"'),
            scan_chars!(input, '\r'),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.delimited_identifier_start(input),
            scan_zero_or_more!(input, self.delimited_identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier_part(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_char_range!(input, 'a'..='z'),
            scan_char_range!(input, '0'..='9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn delimited_identifier_start(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(input, scan_char_range!(input, 'A'..='Z'))
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_optional!(input, scan_chars!(input, '\r')),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '\\'),
            scan_choice!(
                input,
                self.ascii_escape(input),
                self.hex_byte_escape(input),
                self.unicode_escape(input)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'x'),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_char_range!(input, '0'..='9'),
            scan_char_range!(input, 'a'..='f'),
            scan_char_range!(input, 'A'..='F')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, input: &mut ParserContext<'_>) -> bool {
        self.raw_identifier(input)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            self.identifier_start(input),
            scan_char_range!(input, '0'..='9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_chars!(input, '$'),
            scan_char_range!(input, 'a'..='z'),
            scan_char_range!(input, 'A'..='Z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multi_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(input, scan_chars!(input, '/', '*'), scan_chars!(input, '*')),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    scan_none_of!(input, '*'),
                    scan_not_followed_by!(input, scan_chars!(input, '*'), scan_chars!(input, '/'))
                )
            ),
            scan_chars!(input, '*', '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn raw_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.identifier_start(input),
            scan_zero_or_more!(input, self.identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_not_followed_by!(input, scan_chars!(input, '/', '/'), scan_chars!(input, '/')),
            scan_zero_or_more!(input, scan_none_of!(input, '\r', '\n'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '"'),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    scan_none_of!(input, '"', '\\', '\r', '\n')
                )
            ),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u'),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(input, scan_chars!(input, ' '), scan_chars!(input, '\t'))
        )
    }

    pub fn parse(&self, kind: NonTerminalKind, input: &str) -> ParseOutput {
        match kind {
            NonTerminalKind::AdditionExpression => Self::addition_expression.parse(self, input),
            NonTerminalKind::Expression => Self::expression.parse(self, input),
            NonTerminalKind::Literal => Self::literal.parse(self, input),
            NonTerminalKind::MemberAccessExpression => {
                Self::member_access_expression.parse(self, input)
            }
            NonTerminalKind::NegationExpression => Self::negation_expression.parse(self, input),
            NonTerminalKind::SeparatedIdentifiers => Self::separated_identifiers.parse(self, input),
            NonTerminalKind::SourceUnit => Self::source_unit.parse(self, input),
            NonTerminalKind::SourceUnitMember => Self::source_unit_member.parse(self, input),
            NonTerminalKind::SourceUnitMembers => Self::source_unit_members.parse(self, input),
            NonTerminalKind::Tree => Self::tree.parse(self, input),
            NonTerminalKind::TreeNode => Self::tree_node.parse(self, input),
            NonTerminalKind::TreeNodeChild => Self::tree_node_child.parse(self, input),
            NonTerminalKind::TreeNodeChildren => Self::tree_node_children.parse(self, input),
        }
    }
}

impl Lexer for Language {
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        Language::leading_trivia(self, input)
    }

    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        Language::trailing_trivia(self, input)
    }

    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TerminalKind, TerminalKind)] {
        match LexCtx::value() {
            LexicalContext::Default => &[],
            LexicalContext::Tree => &[(TerminalKind::OpenBracket, TerminalKind::CloseBracket)],
        }
    }

    fn next_token<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedToken> {
        let save = input.position();
        let mut furthest_position = input.position();
        let mut longest_token = None;

        macro_rules! longest_match {
                ($( { $kind:ident = $function:ident } )*) => {
                    $(
                        if self.$function(input) && input.position() > furthest_position {
                            furthest_position = input.position();

                            longest_token = Some(TerminalKind::$kind);
                        }
                        input.set_position(save);
                    )*
                };
            }

        match LexCtx::value() {
            LexicalContext::Default => {
                if let Some(kind) = match input.next() {
                    Some('!') => Some(TerminalKind::Bang),
                    Some('+') => Some(TerminalKind::Plus),
                    Some('.') => Some(TerminalKind::Period),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_token = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { EndOfLine = end_of_line }
                    { Identifier = identifier }
                    { MultiLineComment = multi_line_comment }
                    { SingleLineComment = single_line_comment }
                    { StringLiteral = string_literal }
                    { Whitespace = whitespace }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {}

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) = longest_token.filter(|tok| [].contains(tok)) {
                    let kw_scan = KeywordScan::Absent;
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedToken::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            LexicalContext::Tree => {
                if let Some(kind) = match input.next() {
                    Some(';') => Some(TerminalKind::Semicolon),
                    Some('[') => Some(TerminalKind::OpenBracket),
                    Some(']') => Some(TerminalKind::CloseBracket),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_token = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                    { DelimitedIdentifier = delimited_identifier }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_token.filter(|tok| [TerminalKind::Identifier].contains(tok))
                {
                    let kw_scan = if scan_chars!(input, 't', 'r', 'e', 'e') {
                        KeywordScan::Reserved(TerminalKind::TreeKeyword)
                    } else {
                        KeywordScan::Absent
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedToken::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
        }

        match longest_token {
            Some(token) => {
                input.set_position(furthest_position);
                Some(ScannedToken::Single(token))
            }
            // Skip a character if possible and if we didn't recognize a token
            None if input.peek().is_some() => {
                let _ = input.next();
                Some(ScannedToken::Single(TerminalKind::SKIPPED))
            }
            None => None,
        }
    }
}

#[cfg(feature = "slang_napi_interfaces")]
// NAPI-exposed functions have to accept owned values.
#[allow(clippy::needless_pass_by_value)]
#[napi(namespace = "language")]
impl Language {
    #[napi(constructor, catch_unwind)]
    pub fn new_napi(version: String) -> std::result::Result<Self, napi::Error> {
        let version =
            Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
        Self::new(version).map_err(|e| e.into())
    }

    #[napi(getter, js_name = "version", catch_unwind)]
    pub fn version_napi(&self) -> String {
        self.version.to_string()
    }

    #[napi(js_name = "supportedVersions", catch_unwind)]
    pub fn supported_versions_napi() -> Vec<String> {
        return Self::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect();
    }

    #[napi(
        js_name = "parse",
        ts_return_type = "parse_output.ParseOutput",
        catch_unwind
    )]
    pub fn parse_napi(
        &self,
        #[napi(ts_arg_type = "kinds.NonTerminalKind")] kind: NonTerminalKind,
        input: String,
    ) -> NAPIParseOutput {
        self.parse(kind, input.as_str()).into()
    }
}
