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
    FieldName, IsLexicalContext, LexicalContext, LexicalContextType, RuleKind, TokenKind,
};
use crate::lexer::{KeywordScan, Lexer, ScannedToken};
#[cfg(feature = "slang_napi_interfaces")]
use crate::napi_interface::parse_output::ParseOutput as NAPIParseOutput;
use crate::parse_output::ParseOutput;
use crate::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, RecoverFromNoMatch, SeparatedHelper, SequenceHelper, ZeroOrMoreHelper,
};

#[derive(Debug)]
#[cfg_attr(feature = "slang_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_at_least_0_4_12: bool,
    pub(crate) version_is_at_least_0_4_14: bool,
    pub(crate) version_is_at_least_0_4_21: bool,
    pub(crate) version_is_at_least_0_4_22: bool,
    pub(crate) version_is_at_least_0_5_0: bool,
    pub(crate) version_is_at_least_0_5_3: bool,
    pub(crate) version_is_at_least_0_5_10: bool,
    pub(crate) version_is_at_least_0_5_12: bool,
    pub(crate) version_is_at_least_0_5_14: bool,
    pub(crate) version_is_at_least_0_6_0: bool,
    pub(crate) version_is_at_least_0_6_2: bool,
    pub(crate) version_is_at_least_0_6_5: bool,
    pub(crate) version_is_at_least_0_6_8: bool,
    pub(crate) version_is_at_least_0_6_11: bool,
    pub(crate) version_is_at_least_0_7_0: bool,
    pub(crate) version_is_at_least_0_7_1: bool,
    pub(crate) version_is_at_least_0_7_4: bool,
    pub(crate) version_is_at_least_0_8_0: bool,
    pub(crate) version_is_at_least_0_8_4: bool,
    pub(crate) version_is_at_least_0_8_7: bool,
    pub(crate) version_is_at_least_0_8_8: bool,
    pub(crate) version_is_at_least_0_8_13: bool,
    pub(crate) version_is_at_least_0_8_18: bool,
    pub(crate) version_is_at_least_0_8_19: bool,
    pub(crate) version_is_at_least_0_8_22: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported Solidity language version '{0}'.")]
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
    pub const SUPPORTED_VERSIONS: &[Version] = &[
        Version::new(0, 4, 11),
        Version::new(0, 4, 12),
        Version::new(0, 4, 13),
        Version::new(0, 4, 14),
        Version::new(0, 4, 15),
        Version::new(0, 4, 16),
        Version::new(0, 4, 17),
        Version::new(0, 4, 18),
        Version::new(0, 4, 19),
        Version::new(0, 4, 20),
        Version::new(0, 4, 21),
        Version::new(0, 4, 22),
        Version::new(0, 4, 23),
        Version::new(0, 4, 24),
        Version::new(0, 4, 25),
        Version::new(0, 4, 26),
        Version::new(0, 5, 0),
        Version::new(0, 5, 1),
        Version::new(0, 5, 2),
        Version::new(0, 5, 3),
        Version::new(0, 5, 4),
        Version::new(0, 5, 5),
        Version::new(0, 5, 6),
        Version::new(0, 5, 7),
        Version::new(0, 5, 8),
        Version::new(0, 5, 9),
        Version::new(0, 5, 10),
        Version::new(0, 5, 11),
        Version::new(0, 5, 12),
        Version::new(0, 5, 13),
        Version::new(0, 5, 14),
        Version::new(0, 5, 15),
        Version::new(0, 5, 16),
        Version::new(0, 5, 17),
        Version::new(0, 6, 0),
        Version::new(0, 6, 1),
        Version::new(0, 6, 2),
        Version::new(0, 6, 3),
        Version::new(0, 6, 4),
        Version::new(0, 6, 5),
        Version::new(0, 6, 6),
        Version::new(0, 6, 7),
        Version::new(0, 6, 8),
        Version::new(0, 6, 9),
        Version::new(0, 6, 10),
        Version::new(0, 6, 11),
        Version::new(0, 6, 12),
        Version::new(0, 7, 0),
        Version::new(0, 7, 1),
        Version::new(0, 7, 2),
        Version::new(0, 7, 3),
        Version::new(0, 7, 4),
        Version::new(0, 7, 5),
        Version::new(0, 7, 6),
        Version::new(0, 8, 0),
        Version::new(0, 8, 1),
        Version::new(0, 8, 2),
        Version::new(0, 8, 3),
        Version::new(0, 8, 4),
        Version::new(0, 8, 5),
        Version::new(0, 8, 6),
        Version::new(0, 8, 7),
        Version::new(0, 8, 8),
        Version::new(0, 8, 9),
        Version::new(0, 8, 10),
        Version::new(0, 8, 11),
        Version::new(0, 8, 12),
        Version::new(0, 8, 13),
        Version::new(0, 8, 14),
        Version::new(0, 8, 15),
        Version::new(0, 8, 16),
        Version::new(0, 8, 17),
        Version::new(0, 8, 18),
        Version::new(0, 8, 19),
        Version::new(0, 8, 20),
        Version::new(0, 8, 21),
        Version::new(0, 8, 22),
    ];

    pub fn new(version: Version) -> std::result::Result<Self, Error> {
        if Self::SUPPORTED_VERSIONS.binary_search(&version).is_ok() {
            Ok(Self {
                version_is_at_least_0_4_12: Version::new(0, 4, 12) <= version,
                version_is_at_least_0_4_14: Version::new(0, 4, 14) <= version,
                version_is_at_least_0_4_21: Version::new(0, 4, 21) <= version,
                version_is_at_least_0_4_22: Version::new(0, 4, 22) <= version,
                version_is_at_least_0_5_0: Version::new(0, 5, 0) <= version,
                version_is_at_least_0_5_3: Version::new(0, 5, 3) <= version,
                version_is_at_least_0_5_10: Version::new(0, 5, 10) <= version,
                version_is_at_least_0_5_12: Version::new(0, 5, 12) <= version,
                version_is_at_least_0_5_14: Version::new(0, 5, 14) <= version,
                version_is_at_least_0_6_0: Version::new(0, 6, 0) <= version,
                version_is_at_least_0_6_2: Version::new(0, 6, 2) <= version,
                version_is_at_least_0_6_5: Version::new(0, 6, 5) <= version,
                version_is_at_least_0_6_8: Version::new(0, 6, 8) <= version,
                version_is_at_least_0_6_11: Version::new(0, 6, 11) <= version,
                version_is_at_least_0_7_0: Version::new(0, 7, 0) <= version,
                version_is_at_least_0_7_1: Version::new(0, 7, 1) <= version,
                version_is_at_least_0_7_4: Version::new(0, 7, 4) <= version,
                version_is_at_least_0_8_0: Version::new(0, 8, 0) <= version,
                version_is_at_least_0_8_4: Version::new(0, 8, 4) <= version,
                version_is_at_least_0_8_7: Version::new(0, 8, 7) <= version,
                version_is_at_least_0_8_8: Version::new(0, 8, 8) <= version,
                version_is_at_least_0_8_13: Version::new(0, 8, 13) <= version,
                version_is_at_least_0_8_18: Version::new(0, 8, 18) <= version,
                version_is_at_least_0_8_19: Version::new(0, 8, 19) <= version,
                version_is_at_least_0_8_22: Version::new(0, 8, 22) <= version,
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version))
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn abi_coder_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::AbicoderKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::AbicoderKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Version,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ABICoderPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn additive_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::AdditiveExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn address_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::AddressKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AddressKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::PayableKeyword,
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::PayableKeyword,
                    ),
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::AddressType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn and_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::AndExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.positional_arguments_declaration(input);
            choice.consider(input, result)?;
            let result = self.named_arguments_declaration(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenBracket,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBracket,
                ),
            )?;
            seq.elem(
                self.array_values(input)
                    .with_name(FieldName::Items)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBracket,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseBracket,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseBracket,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ArrayExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_type_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.type_name(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::TypeName => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::ArrayTypeName => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_values(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.expression(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::ArrayValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_14 {
            OneOrMoreHelper::run(input, |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                )
                .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::AsciiStringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::AssemblyFlags)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.assembly_flags(input)
                    .with_name(FieldName::Flags)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::AssemblyFlagsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::AssemblyKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AssemblyKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Label,
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::AsciiStringLiteral,
                    ),
                ),
            )?;
            seq.elem_named(
                FieldName::Flags,
                OptionalHelper::transform(self.assembly_flags_declaration(input)),
            )?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::AssemblyStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assignment_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::AssignmentExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_and_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::BitwiseAndExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_or_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::BitwiseOrExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn bitwise_xor_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::BitwiseXorExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenBrace,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.statements(input))
                    .with_name(FieldName::Statements)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseBrace,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::Block)
    }

    #[allow(unused_assignments, unused_parens)]
    fn break_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::BreakKeyword,
                )
                .with_name(FieldName::BreakKeyword)
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::BreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::CatchKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CatchKeyword,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Error,
                    OptionalHelper::transform(self.catch_clause_error(input)),
                )?;
                seq.elem_named(FieldName::Body, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause_error(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::Name,
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    ),
                )?;
                seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClauseError)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clauses(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.catch_clause(input).with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClauses)
    }

    #[allow(unused_assignments, unused_parens)]
    fn comparison_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::ComparisonExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn conditional_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::ConditionalExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn constant_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(FieldName::TypeName, self.type_name(input))?;
                        seq.elem_named(
                            FieldName::ConstantKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::ConstantKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Name,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Equal,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Equal,
                            ),
                        )?;
                        seq.elem_named(FieldName::Value, self.expression(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstantDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::InternalKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PublicKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            OneOrMoreHelper::run(input, |input| {
                self.constructor_attribute(input).with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::ConstructorKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::ConstructorKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
                seq.elem_named(
                    FieldName::Attributes,
                    OptionalHelper::transform(self.constructor_attributes(input)),
                )?;
                seq.elem_named(FieldName::Body, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn continue_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ContinueKeyword,
                )
                .with_name(FieldName::ContinueKeyword)
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            if self.version_is_at_least_0_6_0 {
                seq.elem_named(
                    FieldName::AbstractKeyword,
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AbstractKeyword,
                        ),
                    ),
                )?;
            }
            seq.elem_named(
                FieldName::ContractKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ContractKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Inheritence,
                OptionalHelper::transform(self.inheritance_specifier(input)),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.contract_members(input))
                        .with_name(FieldName::Members)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ContractDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.using_directive(input);
            choice.consider(input, result)?;
            let result = self.function_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_4_22 {
                let result = self.constructor_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.receive_function_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.fallback_function_definition(input);
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_6_0 {
                let result = self.unnamed_function_definition(input);
                choice.consider(input, result)?;
            }
            let result = self.modifier_definition(input);
            choice.consider(input, result)?;
            let result = self.struct_definition(input);
            choice.consider(input, result)?;
            let result = self.enum_definition(input);
            choice.consider(input, result)?;
            let result = self.event_definition(input);
            choice.consider(input, result)?;
            let result = self.state_variable_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_4 {
                let result = self.error_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_8 {
                let result = self.user_defined_value_type_definition(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ContractMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::ContractMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Literal,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::DecimalLiteral,
                ),
            )?;
            seq.elem_named(
                FieldName::Unit,
                OptionalHelper::transform(self.number_unit(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::DecimalNumberExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn delete_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::DeleteKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::DeleteKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Expression, self.expression(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::DeleteStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn do_while_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::DoKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::DoKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Body, self.statement(input))?;
                    seq.elem_named(
                        FieldName::WhileKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::WhileKeyword,
                        ),
                    )?;
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem_named(
                            FieldName::OpenParen,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::OpenParen,
                            ),
                        )?;
                        seq.elem(
                            self.expression(input)
                                .with_name(FieldName::Condition)
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                input,
                                self,
                                TokenKind::CloseParen,
                                RecoverFromNoMatch::Yes,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::CloseParen,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::CloseParen,
                            ),
                        )?;
                        seq.finish()
                    }))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::DoWhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn elementary_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::BoolKeyword,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_8_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ByteKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::StringKeyword,
            );
            choice.consider(input, result)?;
            let result = self.address_type(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::BytesKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::IntKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::UintKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::FixedKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::UfixedKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ElementaryType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn else_branch(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ElseKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ElseKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ElseBranch)
    }

    #[allow(unused_assignments, unused_parens)]
    fn emit_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_21 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(
                            FieldName::EmitKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::EmitKeyword,
                            ),
                        )?;
                        seq.elem_named(FieldName::Event, self.identifier_path(input))?;
                        seq.elem_named(FieldName::Arguments, self.arguments_declaration(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::EmitStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::EnumKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::EnumKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.enum_members(input))
                        .with_name(FieldName::Members)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::EnumDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::EnumMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn equality_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::EqualityExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(
                            FieldName::ErrorKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::ErrorKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Name,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Members,
                            self.error_parameters_declaration(input),
                        )?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(FieldName::TypeName, self.type_name(input))?;
                seq.elem_named(
                    FieldName::Name,
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| self.error_parameter(input).with_name(FieldName::Item),
                TokenKind::Comma,
                FieldName::Separator,
            )
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.error_parameters(input))
                        .with_name(FieldName::Parameters)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseParen,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::EventKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::EventKeyword,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Name,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Parameters,
                        self.event_parameters_declaration(input),
                    )?;
                    seq.elem_named(
                        FieldName::AnonymousKeyword,
                        OptionalHelper::transform(
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::AnonymousKeyword,
                            ),
                        ),
                    )?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::EventDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::TypeName, self.type_name(input))?;
            seq.elem_named(
                FieldName::IndexedKeyword,
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::IndexedKeyword,
                    ),
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Identifier,
                    ),
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::EventParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.event_parameter(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::EventParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.event_parameters(input))
                    .with_name(FieldName::Parameters)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::EventParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_feature(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                input,
                TokenKind::Identifier,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                input,
                TokenKind::AsciiStringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ExperimentalFeature)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ExperimentalKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::ExperimentalKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Feature, self.experimental_feature(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ExperimentalPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn exponentiation_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::ExponentiationExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_left_assignment_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::AssignmentExpression,
                1u8,
                1u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Equal,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::BarEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PlusEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::CaretEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::SlashEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PercentEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AsteriskEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AmpersandEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThanLessThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_conditional_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::ConditionalExpression,
                3u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::QuestionMark,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::QuestionMark,
                        ),
                    )?;
                    seq.elem_named(FieldName::TrueExpression, self.expression(input))?;
                    seq.elem_named(
                        FieldName::Colon,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Colon,
                        ),
                    )?;
                    seq.elem_named(FieldName::FalseExpression, self.expression(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_left_or_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::OrExpression,
                5u8,
                5u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::BarBar,
                )
                .with_name(FieldName::Operator),
            )
        };
        let parse_left_and_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::AndExpression,
                7u8,
                7u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AmpersandAmpersand,
                )
                .with_name(FieldName::Operator),
            )
        };
        let parse_left_equality_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::EqualityExpression,
                9u8,
                9u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::EqualEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::BangEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_comparison_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::ComparisonExpression,
                11u8,
                11u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_bitwise_or_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BitwiseOrExpression,
                13u8,
                13u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(input, TokenKind::Bar)
                    .with_name(FieldName::Operator),
            )
        };
        let parse_left_bitwise_xor_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BitwiseXorExpression,
                15u8,
                15u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Caret,
                )
                .with_name(FieldName::Operator),
            )
        };
        let parse_left_bitwise_and_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BitwiseAndExpression,
                17u8,
                17u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Ampersand,
                )
                .with_name(FieldName::Operator),
            )
        };
        let parse_left_shift_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::ShiftExpression,
                19u8,
                19u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThanLessThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanGreaterThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_additive_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::AdditiveExpression,
                21u8,
                21u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Plus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Minus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_multiplicative_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::MultiplicativeExpression,
                23u8,
                23u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Asterisk,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Slash,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Percent,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_left_exponentiation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::ExponentiationExpression,
                25u8,
                25u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    if !self.version_is_at_least_0_6_0 {
                        let result = self
                            .parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::AsteriskAsterisk,
                            )
                            .with_name(FieldName::Operator);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_right_exponentiation_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::ExponentiationExpression,
                27u8 + 1,
                27u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    if self.version_is_at_least_0_6_0 {
                        let result = self
                            .parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::AsteriskAsterisk,
                            )
                            .with_name(FieldName::Operator);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_postfix_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::PostfixExpression,
                29u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PlusPlus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusMinus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_prefix_prefix_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_prefix_operator(
                RuleKind::PrefixExpression,
                31u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PlusPlus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusMinus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Tilde,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Bang,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Minus,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_5_0 {
                        let result = self
                            .parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Plus,
                            )
                            .with_name(FieldName::Operator);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_postfix_function_call_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::FunctionCallExpression,
                33u8,
                SequenceHelper::run(|mut seq| {
                    if self.version_is_at_least_0_6_2 {
                        seq.elem_named(
                            FieldName::Options,
                            OptionalHelper::transform(self.function_call_options(input)),
                        )?;
                    }
                    seq.elem_named(FieldName::Arguments, self.arguments_declaration(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_postfix_member_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::MemberAccessExpression,
                35u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::Period,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Period,
                        ),
                    )?;
                    seq.elem_named(FieldName::Member, self.member_access(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_postfix_index_access_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::IndexAccessExpression,
                37u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem_named(
                        FieldName::OpenBracket,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenBracket,
                        ),
                    )?;
                    seq.elem(
                        SequenceHelper::run(|mut seq| {
                            seq.elem_named(
                                FieldName::Start,
                                OptionalHelper::transform(self.expression(input)),
                            )?;
                            seq.elem_named(
                                FieldName::End,
                                OptionalHelper::transform(self.index_access_end(input)),
                            )?;
                            seq.finish()
                        })
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBracket,
                            RecoverFromNoMatch::Yes,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::CloseBracket,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::CloseBracket,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_prefix_prefix_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.new_expression(input);
                choice.consider(input, result)?;
                let result = self.tuple_expression(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_5_3 {
                    let result = self.type_expression(input);
                    choice.consider(input, result)?;
                }
                let result = self.array_expression(input);
                choice.consider(input, result)?;
                let result = self.hex_number_expression(input);
                choice.consider(input, result)?;
                let result = self.decimal_number_expression(input);
                choice.consider(input, result)?;
                let result = self.string_expression(input);
                choice.consider(input, result)?;
                let result = self.elementary_type(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_0 {
                    let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::PayableKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::TrueKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FalseKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_conditional_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_postfix_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_function_call_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_member_access_expression(input);
                choice.consider(input, result)?;
                let result = parse_postfix_index_access_expression(input);
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
                let result = parse_left_assignment_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_or_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_and_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_equality_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_comparison_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_or_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_xor_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_bitwise_and_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_shift_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_additive_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_multiplicative_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_exponentiation_expression(input);
                choice.consider(input, result)?;
                let result = parse_right_exponentiation_expression(input);
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
            RuleKind::Expression,
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.expression(input)
                    .with_name(FieldName::Expression)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ExpressionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PureKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ViewKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.fallback_function_attribute(input)
                    .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::FallbackKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::FallbackKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
                seq.elem_named(
                    FieldName::Attributes,
                    OptionalHelper::transform(self.fallback_function_attributes(input)),
                )?;
                seq.elem_named(
                    FieldName::Returns,
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_named(FieldName::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ForKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ForKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(
                            FieldName::Initialization,
                            self.for_statement_initialization(input),
                        )?;
                        seq.elem_named(FieldName::Condition, self.for_statement_condition(input))?;
                        seq.elem_named(
                            FieldName::Iterator,
                            OptionalHelper::transform(self.expression(input)),
                        )?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
                )?;
                seq.elem_named(
                    FieldName::CloseParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named(FieldName::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement_condition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ForStatementCondition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement_initialization(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            let result = self.variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.tuple_deconstruction_statement(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ForStatementInitialization)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.modifier_invocation(input);
            choice.consider(input, result)?;
            let result = self.override_specifier(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ConstantKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ExternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PayableKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PureKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ViewKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::FunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.function_attribute(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::FunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_body(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.block(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::Semicolon,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::FunctionBody)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::FunctionCallExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_options(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_2 {
            ChoiceHelper::run(input, |mut choice, input| {
                if self.version_is_at_least_0_6_2 && !self.version_is_at_least_0_8_0 {
                    let result = self.named_argument_groups(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_8_0 {
                    let result = self.named_argument_group(input);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::FunctionKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FunctionKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Name, self.function_name(input))?;
            seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
            seq.elem_named(
                FieldName::Attributes,
                OptionalHelper::transform(self.function_attributes(input)),
            )?;
            seq.elem_named(
                FieldName::Returns,
                OptionalHelper::transform(self.returns_declaration(input)),
            )?;
            seq.elem_named(FieldName::Body, self.function_body(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::FunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::Identifier,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::FallbackKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ReceiveKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::FunctionName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::FunctionKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FunctionKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
            seq.elem_named(
                FieldName::Attributes,
                OptionalHelper::transform(self.function_type_attributes(input)),
            )?;
            seq.elem_named(
                FieldName::Returns,
                OptionalHelper::transform(self.returns_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::FunctionType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ExternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PureKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ViewKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PayableKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::FunctionTypeAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.function_type_attribute(input)
                .with_name(FieldName::Item)
        })
        .with_kind(RuleKind::FunctionTypeAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Literal,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::HexLiteral,
                ),
            )?;
            if !self.version_is_at_least_0_5_0 {
                seq.elem_named(
                    FieldName::Unit,
                    OptionalHelper::transform(self.number_unit(input)),
                )?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::HexNumberExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_14 {
            OneOrMoreHelper::run(input, |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::HexStringLiteral,
                )
                .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::HexStringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_path(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Period,
            FieldName::Separator,
        )
        .with_kind(RuleKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::IfKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::IfKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    self.expression(input)
                        .with_name(FieldName::Condition)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
                )?;
                seq.elem_named(
                    FieldName::CloseParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named(FieldName::Body, self.statement(input))?;
            seq.elem_named(
                FieldName::ElseBranch,
                OptionalHelper::transform(self.else_branch(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::IfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_alias(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::AsKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Identifier,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportAlias)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.path_import(input);
            choice.consider(input, result)?;
            let result = self.named_import(input);
            choice.consider(input, result)?;
            let result = self.import_deconstruction(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ImportClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    self.import_deconstruction_symbols(input)
                        .with_name(FieldName::Symbols)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named(
                FieldName::FromKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FromKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Path,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction_symbol(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Alias,
                OptionalHelper::transform(self.import_alias(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportDeconstructionSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction_symbols(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.import_deconstruction_symbol(input)
                    .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::ImportDeconstructionSymbols)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::ImportKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ImportKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Clause, self.import_clause(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn index_access_end(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Colon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Colon,
                ),
            )?;
            seq.elem_named(
                FieldName::End,
                OptionalHelper::transform(self.expression(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::IndexAccessEnd)
    }

    #[allow(unused_assignments, unused_parens)]
    fn index_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::IndexAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::IsKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::IsKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Types, self.inheritance_types(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::TypeName, self.identifier_path(input))?;
            seq.elem_named(
                FieldName::Arguments,
                OptionalHelper::transform(self.arguments_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_types(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.inheritance_type(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::InheritanceTypes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::InterfaceKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::InterfaceKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Inheritence,
                OptionalHelper::transform(self.inheritance_specifier(input)),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.interface_members(input))
                        .with_name(FieldName::Members)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::InterfaceDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::InterfaceMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result =
                    self.parse_token::<LexicalContextType::Default>(input, TokenKind::Whitespace);
                choice.consider(input, result)?;
                let result =
                    self.parse_token::<LexicalContextType::Default>(input, TokenKind::EndOfLine);
                choice.consider(input, result)?;
                let result = self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::SingleLineComment,
                );
                choice.consider(input, result)?;
                let result = self
                    .parse_token::<LexicalContextType::Default>(input, TokenKind::MultiLineComment);
                choice.consider(input, result)?;
                let result = self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::SingleLineNatSpecComment,
                );
                choice.consider(input, result)?;
                let result = self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::MultiLineNatSpecComment,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::LeadingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::LibraryKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::LibraryKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.library_members(input))
                        .with_name(FieldName::Members)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::LibraryDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.contract_member(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::LibraryMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::KeyType, self.mapping_key_type(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_named(
                    FieldName::Name,
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    ),
                )?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::MappingKey)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.elementary_type(input);
            choice.consider(input, result)?;
            let result = self.identifier_path(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::MappingKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::MappingKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(FieldName::KeyType, self.mapping_key(input))?;
                        seq.elem_named(
                            FieldName::EqualGreaterThan,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::EqualGreaterThan,
                            ),
                        )?;
                        seq.elem_named(FieldName::ValueType, self.mapping_value(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
                )?;
                seq.elem_named(
                    FieldName::CloseParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::MappingType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::TypeName, self.type_name(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_named(
                    FieldName::Name,
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    ),
                )?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::MappingValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn member_access(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::Identifier,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::AddressKeyword,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::MemberAccess)
    }

    #[allow(unused_assignments, unused_parens)]
    fn member_access_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::MemberAccessExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.override_specifier(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::ModifierAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.modifier_attribute(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::ModifierAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ModifierKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ModifierKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Parameters,
                OptionalHelper::transform(self.parameters_declaration(input)),
            )?;
            seq.elem_named(
                FieldName::Attributes,
                OptionalHelper::transform(self.modifier_attributes(input)),
            )?;
            seq.elem_named(FieldName::Body, self.function_body(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::Name, self.identifier_path(input))?;
            seq.elem_named(
                FieldName::Arguments,
                OptionalHelper::transform(self.arguments_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ModifierInvocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn multiplicative_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::MultiplicativeExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Colon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Colon,
                ),
            )?;
            seq.elem_named(FieldName::Value, self.expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgument)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument_group(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenBrace,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.named_arguments(input))
                    .with_name(FieldName::Arguments)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseBrace,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgumentGroup)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument_groups(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_2 && !self.version_is_at_least_0_8_0 {
            OneOrMoreHelper::run(input, |input| {
                self.named_argument_group(input).with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::NamedArgumentGroups)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.named_argument(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::NamedArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.named_argument_group(input))
                    .with_name(FieldName::Arguments)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_import(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Asterisk,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Asterisk,
                ),
            )?;
            seq.elem_named(FieldName::Alias, self.import_alias(input))?;
            seq.elem_named(
                FieldName::FromKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FromKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Path,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn new_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::NewKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::NewKeyword,
                ),
            )?;
            seq.elem_named(FieldName::TypeName, self.type_name(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::NewExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn number_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::WeiKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_11 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::GweiKeyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_7_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::SzaboKeyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_7_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FinneyKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::EtherKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::SecondsKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::MinutesKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::HoursKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::DaysKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::WeeksKeyword,
            );
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::YearsKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::NumberUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn or_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::OrExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_paths(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.identifier_path(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::OverridePaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_paths_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.override_paths(input)
                    .with_name(FieldName::Paths)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::OverridePathsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::OverrideKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OverrideKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Overridden,
                OptionalHelper::transform(self.override_paths_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::TypeName, self.type_name(input))?;
            seq.elem_named(
                FieldName::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                FieldName::Name,
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Identifier,
                    ),
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::Parameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.parameter(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::Parameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.parameters(input))
                    .with_name(FieldName::Parameters)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn path_import(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Path,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                ),
            )?;
            seq.elem_named(
                FieldName::Alias,
                OptionalHelper::transform(self.import_alias(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.expression(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::PositionalArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.positional_arguments(input))
                    .with_name(FieldName::Arguments)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::PositionalArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn postfix_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::PostfixExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.abi_coder_pragma(input);
            choice.consider(input, result)?;
            let result = self.experimental_pragma(input);
            choice.consider(input, result)?;
            let result = self.version_pragma(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::Pragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::PragmaKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::PragmaKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Pragma, self.pragma(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Pragma>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::PragmaDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn prefix_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::PrefixExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::VirtualKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.receive_function_attribute(input)
                    .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::ReceiveKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::ReceiveKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
                seq.elem_named(
                    FieldName::Attributes,
                    OptionalHelper::transform(self.receive_function_attributes(input)),
                )?;
                seq.elem_named(FieldName::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn return_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::ReturnKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ReturnKeyword,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Expression,
                        OptionalHelper::transform(self.expression(input)),
                    )?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::ReturnStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn returns_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ReturnsKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ReturnsKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Variables, self.parameters_declaration(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn revert_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(
                            FieldName::RevertKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::RevertKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Error,
                            OptionalHelper::transform(self.identifier_path(input)),
                        )?;
                        seq.elem_named(FieldName::Arguments, self.arguments_declaration(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::RevertStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn shift_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::Expression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::ShiftExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.source_unit_members(input))
            .with_name(FieldName::Members)
            .with_kind(RuleKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.pragma_directive(input);
            choice.consider(input, result)?;
            let result = self.import_directive(input);
            choice.consider(input, result)?;
            let result = self.contract_definition(input);
            choice.consider(input, result)?;
            let result = self.interface_definition(input);
            choice.consider(input, result)?;
            let result = self.library_definition(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.struct_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.enum_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_1 {
                let result = self.function_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_4 {
                let result = self.constant_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_4 {
                let result = self.error_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_8 {
                let result = self.user_defined_value_type_definition(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_13 {
                let result = self.using_directive(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_22 {
                let result = self.event_definition(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::SourceUnitMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.source_unit_member(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::SourceUnitMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.override_specifier(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::ConstantKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::InternalKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PrivateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::PublicKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_5 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ImmutableKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::StateVariableAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.state_variable_attribute(input)
                .with_name(FieldName::Item)
        })
        .with_kind(RuleKind::StateVariableAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(FieldName::TypeName, self.type_name(input))?;
                    seq.elem_named(
                        FieldName::Attributes,
                        OptionalHelper::transform(self.state_variable_attributes(input)),
                    )?;
                    seq.elem_named(
                        FieldName::Name,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Value,
                        OptionalHelper::transform(self.state_variable_definition_value(input)),
                    )?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::StateVariableDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Equal,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Equal,
                ),
            )?;
            seq.elem_named(FieldName::Value, self.expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::StateVariableDefinitionValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.expression_statement(input);
            choice.consider(input, result)?;
            let result = self.variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.tuple_deconstruction_statement(input);
            choice.consider(input, result)?;
            let result = self.if_statement(input);
            choice.consider(input, result)?;
            let result = self.for_statement(input);
            choice.consider(input, result)?;
            let result = self.while_statement(input);
            choice.consider(input, result)?;
            let result = self.do_while_statement(input);
            choice.consider(input, result)?;
            let result = self.continue_statement(input);
            choice.consider(input, result)?;
            let result = self.break_statement(input);
            choice.consider(input, result)?;
            let result = self.delete_statement(input);
            choice.consider(input, result)?;
            let result = self.return_statement(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.throw_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_21 {
                let result = self.emit_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_6_0 {
                let result = self.try_statement(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_4 {
                let result = self.revert_statement(input);
                choice.consider(input, result)?;
            }
            let result = self.assembly_statement(input);
            choice.consider(input, result)?;
            let result = self.block(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_0 {
                let result = self.unchecked_block(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.statement(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::Statements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn storage_location(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::MemoryKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::StorageKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CallDataKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::StorageLocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if !self.version_is_at_least_0_5_14 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::HexStringLiteral,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_5_14 {
                let result = self.hex_string_literals(input);
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_5_14 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_5_14 {
                let result = self.ascii_string_literals(input);
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_7_0 {
                let result = self.unicode_string_literals(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::StringExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::StructKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::StructKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.struct_members(input))
                        .with_name(FieldName::Members)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::StructDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(FieldName::TypeName, self.type_name(input))?;
                    seq.elem_named(
                        FieldName::Name,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::StructMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.struct_member(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::StructMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn throw_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::ThrowKeyword,
                    )
                    .with_name(FieldName::ThrowKeyword)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ThrowStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.parse_token::<LexicalContextType::Default>(input, TokenKind::Whitespace),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::SingleLineComment,
                ),
            ))?;
            seq.elem(self.parse_token::<LexicalContextType::Default>(input, TokenKind::EndOfLine))?;
            seq.finish()
        })
        .with_kind(RuleKind::TrailingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn try_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::TryKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::TryKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Expression, self.expression(input))?;
                seq.elem_named(
                    FieldName::Returns,
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_named(FieldName::Body, self.block(input))?;
                seq.elem_named(FieldName::CatchClauses, self.catch_clauses(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::TryStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_element(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.tuple_member(input))
            .with_name(FieldName::Member)
            .with_kind(RuleKind::TupleDeconstructionElement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_elements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| {
                self.tuple_deconstruction_element(input)
                    .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::TupleDeconstructionElements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    if !self.version_is_at_least_0_5_0 {
                        seq.elem_named(
                            FieldName::VarKeyword,
                            OptionalHelper::transform(
                                self.parse_token_with_trivia::<LexicalContextType::Default>(
                                    input,
                                    TokenKind::VarKeyword,
                                ),
                            ),
                        )?;
                    }
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem_named(
                            FieldName::OpenParen,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::OpenParen,
                            ),
                        )?;
                        seq.elem(
                            self.tuple_deconstruction_elements(input)
                                .with_name(FieldName::Elements)
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TokenKind::CloseParen,
                                    RecoverFromNoMatch::Yes,
                                ),
                        )?;
                        seq.elem_named(
                            FieldName::CloseParen,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::CloseParen,
                            ),
                        )?;
                        seq.finish()
                    }))?;
                    seq.elem_named(
                        FieldName::Equal,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Equal,
                        ),
                    )?;
                    seq.elem_named(FieldName::Expression, self.expression(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.tuple_values(input)
                    .with_name(FieldName::Items)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.typed_tuple_member(input);
            choice.consider(input, result)?;
            let result = self.untyped_tuple_member(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.expression(input))
            .with_name(FieldName::Expression)
            .with_kind(RuleKind::TupleValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.tuple_value(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::TupleValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_3 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::TypeKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::TypeKeyword,
                    ),
                )?;
                seq.elem(SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem_named(
                        FieldName::OpenParen,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenParen,
                        ),
                    )?;
                    seq.elem(
                        self.type_name(input)
                            .with_name(FieldName::TypeName)
                            .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseParen,
                            RecoverFromNoMatch::Yes,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::CloseParen,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::CloseParen,
                        ),
                    )?;
                    seq.finish()
                }))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::TypeExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_name(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_postfix_array_type_name = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::ArrayTypeName,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem_named(
                        FieldName::OpenBracket,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenBracket,
                        ),
                    )?;
                    seq.elem(
                        OptionalHelper::transform(self.expression(input))
                            .with_name(FieldName::Index)
                            .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                input,
                                self,
                                TokenKind::CloseBracket,
                                RecoverFromNoMatch::Yes,
                            ),
                    )?;
                    seq.elem_named(
                        FieldName::CloseBracket,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::CloseBracket,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.function_type(input);
                choice.consider(input, result)?;
                let result = self.mapping_type(input);
                choice.consider(input, result)?;
                let result = self.elementary_type(input);
                choice.consider(input, result)?;
                let result = self.identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_array_type_name(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            RuleKind::TypeName,
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::TypeName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn typed_tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::TypeName, self.type_name(input))?;
            seq.elem_named(
                FieldName::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::TypedTupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unchecked_block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::UncheckedKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::UncheckedKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Block, self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UncheckedBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            OneOrMoreHelper::run(input, |input| {
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::UnicodeStringLiteral,
                )
                .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnicodeStringLiterals)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attribute(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ExternalKeyword,
                );
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::InternalKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PayableKeyword,
                );
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::PublicKeyword,
                    );
                    choice.consider(input, result)?;
                }
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::PureKeyword,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ViewKeyword,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.unnamed_function_attribute(input)
                    .with_name(FieldName::Item)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::FunctionKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::FunctionKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Parameters, self.parameters_declaration(input))?;
                seq.elem_named(
                    FieldName::Attributes,
                    OptionalHelper::transform(self.unnamed_function_attributes(input)),
                )?;
                seq.elem_named(FieldName::Body, self.function_body(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn untyped_tuple_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::StorageLocation,
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::UntypedTupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn user_defined_value_type_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_8 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named(
                            FieldName::TypeKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::TypeKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::Name,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named(
                            FieldName::IsKeyword,
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::IsKeyword,
                            ),
                        )?;
                        seq.elem_named(FieldName::ValueType, self.elementary_type(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Semicolon,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::Semicolon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UserDefinedValueTypeDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_alias(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_19 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::AsKeyword,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::AsKeyword,
                    ),
                )?;
                seq.elem_named(FieldName::Operator, self.using_operator(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UsingAlias)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_clause(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.identifier_path(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_13 {
                let result = self.using_deconstruction(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::UsingClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    self.using_deconstruction_symbols(input)
                        .with_name(FieldName::Symbols)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    FieldName::CloseBrace,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UsingDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction_symbol(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(FieldName::Name, self.identifier_path(input))?;
                if self.version_is_at_least_0_8_19 {
                    seq.elem_named(
                        FieldName::Alias,
                        OptionalHelper::transform(self.using_alias(input)),
                    )?;
                }
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UsingDeconstructionSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction_symbols(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SeparatedHelper::run::<_, LexicalContextType::Default>(
                input,
                self,
                |input| {
                    self.using_deconstruction_symbol(input)
                        .with_name(FieldName::Item)
                },
                TokenKind::Comma,
                FieldName::Separator,
            )
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UsingDeconstructionSymbols)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::UsingKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::UsingKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Clause, self.using_clause(input))?;
                    seq.elem_named(
                        FieldName::ForKeyword,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ForKeyword,
                        ),
                    )?;
                    seq.elem_named(FieldName::Target, self.using_target(input))?;
                    if self.version_is_at_least_0_8_13 {
                        seq.elem_named(
                            FieldName::GlobalKeyword,
                            OptionalHelper::transform(
                                self.parse_token_with_trivia::<LexicalContextType::Default>(
                                    input,
                                    TokenKind::GlobalKeyword,
                                ),
                            ),
                        )?;
                    }
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::UsingDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_operator(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_19 {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Ampersand,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Asterisk,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::BangEqual,
                );
                choice.consider(input, result)?;
                let result = self
                    .parse_token_with_trivia::<LexicalContextType::Default>(input, TokenKind::Bar);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Caret,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::EqualEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::GreaterThan,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::GreaterThanEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::LessThan,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::LessThanEqual,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Minus,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Percent,
                );
                choice.consider(input, result)?;
                let result = self
                    .parse_token_with_trivia::<LexicalContextType::Default>(input, TokenKind::Plus);
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Slash,
                );
                choice.consider(input, result)?;
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Tilde,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UsingOperator)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_target(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.type_name(input);
            choice.consider(input, result)?;
            let result = self
                .parse_token_with_trivia::<LexicalContextType::Default>(input, TokenKind::Asterisk);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::UsingTarget)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::VariableType,
                        self.variable_declaration_type(input),
                    )?;
                    seq.elem_named(
                        FieldName::StorageLocation,
                        OptionalHelper::transform(self.storage_location(input)),
                    )?;
                    seq.elem_named(
                        FieldName::Name,
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Value,
                        OptionalHelper::transform(self.variable_declaration_value(input)),
                    )?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.type_name(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::VarKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::VariableDeclarationType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::Equal,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Equal,
                ),
            )?;
            seq.elem_named(FieldName::Expression, self.expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::VariableDeclarationValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::SolidityKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::SolidityKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Expressions,
                self.version_pragma_expressions(input),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::VersionPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_left_version_pragma_or_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaOrExpression,
                1u8,
                1u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::BarBar,
                )
                .with_name(FieldName::Operator),
            )
        };
        let parse_left_version_pragma_range_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaRangeExpression,
                3u8,
                3u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(input, TokenKind::Minus)
                    .with_name(FieldName::Operator),
            )
        };
        let parse_prefix_version_pragma_prefix_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_prefix_operator(
                RuleKind::VersionPragmaPrefixExpression,
                5u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::Caret,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::Tilde,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::Equal,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::LessThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::GreaterThan,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::LessThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::GreaterThanEqual,
                        )
                        .with_name(FieldName::Operator);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_prefix_version_pragma_prefix_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.version_pragma_specifier(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        };
        let binary_operand_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, prefix_operator_parser))?;
                seq.elem(primary_expression_parser(input))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_left_version_pragma_or_expression(input);
                choice.consider(input, result)?;
                let result = parse_left_version_pragma_range_expression(input);
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
            RuleKind::VersionPragmaExpression,
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::VersionPragmaExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expressions(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.version_pragma_expression(input)
                .with_name(FieldName::Item)
        })
        .with_kind(RuleKind::VersionPragmaExpressions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_or_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.version_pragma_expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::VersionPragmaExpression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::VersionPragmaOrExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_prefix_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.version_pragma_expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::VersionPragmaExpression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::VersionPragmaPrefixExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_range_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.version_pragma_expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::VersionPragmaExpression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::VersionPragmaRangeExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_specifier(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Pragma>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::VersionPragmaValue,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Period,
            FieldName::Separator,
        )
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::WhileKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::WhileKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    FieldName::OpenParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    self.expression(input)
                        .with_name(FieldName::Condition)
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
                )?;
                seq.elem_named(
                    FieldName::CloseParen,
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named(FieldName::Body, self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_expression(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::YulArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(FieldName::Names, self.yul_identifier_paths(input))?;
            seq.elem_named(
                FieldName::ColonEqual,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::ColonEqual,
                ),
            )?;
            seq.elem_named(FieldName::Expression, self.yul_expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_block(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenBrace,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.yul_statements(input))
                    .with_name(FieldName::Statements)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseBrace,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::CloseBrace,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::YulBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_break_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.parse_token_with_trivia::<LexicalContextType::Yul>(input, TokenKind::YulBreakKeyword)
            .with_name(FieldName::BreakKeyword)
            .with_kind(RuleKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_built_in_function(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulAddKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulAddModKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulAddressKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulAndKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulBalanceKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulBlockHashKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulByteKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallCodeKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallDataCopyKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallDataLoadKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallDataSizeKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallerKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCallValueKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCoinBaseKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulCreateKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulDelegateCallKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulDivKeyword,
            );
            choice.consider(input, result)?;
            let result = self
                .parse_token_with_trivia::<LexicalContextType::Yul>(input, TokenKind::YulEqKeyword);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulExpKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulExtCodeCopyKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulExtCodeSizeKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulGasKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulGasLimitKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulGasPriceKeyword,
            );
            choice.consider(input, result)?;
            let result = self
                .parse_token_with_trivia::<LexicalContextType::Yul>(input, TokenKind::YulGtKeyword);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulInvalidKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulIsZeroKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLog0Keyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLog1Keyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLog2Keyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLog3Keyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLog4Keyword,
            );
            choice.consider(input, result)?;
            let result = self
                .parse_token_with_trivia::<LexicalContextType::Yul>(input, TokenKind::YulLtKeyword);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMLoadKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulModKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMSizeKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMStore8Keyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMStoreKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMulKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulMulModKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulNotKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulNumberKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulOriginKeyword,
            );
            choice.consider(input, result)?;
            let result = self
                .parse_token_with_trivia::<LexicalContextType::Yul>(input, TokenKind::YulOrKeyword);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulPopKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulReturnKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulRevertKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSDivKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSelfDestructKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSgtKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSignExtendKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSLoadKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSltKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSModKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSStoreKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulStopKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSubKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulTimestampKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulXorKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_4_12 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulKeccak256Keyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulSha3Keyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulSuicideKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_12 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulReturnDataCopyKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_12 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulReturnDataSizeKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_12 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulStaticCallKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_4_12 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulCreate2Keyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_5_0 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulExtCodeHashKeyword,
                );
                choice.consider(input, result)?;
            }
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSarKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulShlKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulShrKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulChainIdKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulSelfBalanceKeyword,
            );
            choice.consider(input, result)?;
            if self.version_is_at_least_0_8_7 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulBaseFeeKeyword,
                );
                choice.consider(input, result)?;
            }
            if !self.version_is_at_least_0_8_18 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulDifficultyKeyword,
                );
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_18 {
                let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulPrevRandaoKeyword,
                );
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::YulBuiltInFunction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.parse_token_with_trivia::<LexicalContextType::Yul>(
            input,
            TokenKind::YulContinueKeyword,
        )
        .with_name(FieldName::ContinueKeyword)
        .with_kind(RuleKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_default_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::DefaultKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulDefaultKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulDefaultCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let parse_postfix_yul_function_call_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::YulFunctionCallExpression,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem_named(
                        FieldName::OpenParen,
                        self.parse_token_with_trivia::<LexicalContextType::Yul>(
                            input,
                            TokenKind::OpenParen,
                        ),
                    )?;
                    seq.elem(
                        OptionalHelper::transform(self.yul_arguments(input))
                            .with_name(FieldName::Arguments)
                            .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                                input,
                                self,
                                TokenKind::CloseParen,
                                RecoverFromNoMatch::Yes,
                            ),
                    )?;
                    seq.elem_named(
                        FieldName::CloseParen,
                        self.parse_token_with_trivia::<LexicalContextType::Yul>(
                            input,
                            TokenKind::CloseParen,
                        ),
                    )?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.yul_literal(input);
                choice.consider(input, result)?;
                let result = self.yul_built_in_function(input);
                choice.consider(input, result)?;
                let result = self.yul_identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name(FieldName::Variant)
        };
        let postfix_operator_parser = |input: &mut ParserContext<'_>| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_postfix_yul_function_call_expression(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext<'_>| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, postfix_operator_parser))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            RuleKind::YulExpression,
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::YulExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_for_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ForKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulForKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Initialization, self.yul_block(input))?;
            seq.elem_named(FieldName::Condition, self.yul_expression(input))?;
            seq.elem_named(FieldName::Iterator, self.yul_block(input))?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_call_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        let result = self.yul_expression(input);
        let ParserResult::Match(r#match) = &result else {
            return result;
        };
        match &r#match.nodes[..] {
            [cst::NamedNode {
                name: _,
                node: cst::Node::Rule(node),
            }] if node.kind == RuleKind::YulExpression => match &node.children[..] {
                [inner @ cst::NamedNode {
                    name: _,
                    node: cst::Node::Rule(rule),
                }] if rule.kind == RuleKind::YulFunctionCallExpression => {
                    ParserResult::r#match(vec![inner.clone()], r#match.expected_tokens.clone())
                }
                _ => ParserResult::no_match(vec![]),
            },
            _ => ParserResult::no_match(vec![]),
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::FunctionKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulFunctionKeyword,
                ),
            )?;
            seq.elem_named(
                FieldName::Name,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIdentifier,
                ),
            )?;
            seq.elem_named(
                FieldName::Parameters,
                self.yul_parameters_declaration(input),
            )?;
            seq.elem_named(
                FieldName::Returns,
                OptionalHelper::transform(self.yul_returns_declaration(input)),
            )?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_path(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIdentifier,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Period,
            FieldName::Separator,
        )
        .with_kind(RuleKind::YulIdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_paths(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_identifier_path(input).with_name(FieldName::Item),
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::YulIdentifierPaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::IfKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIfKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Condition, self.yul_expression(input))?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_label(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    FieldName::Label,
                    self.parse_token_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TokenKind::YulIdentifier,
                    ),
                )?;
                seq.elem_named(
                    FieldName::Colon,
                    self.parse_token_with_trivia::<LexicalContextType::Yul>(
                        input,
                        TokenKind::Colon,
                    ),
                )?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::YulLabel)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLeaveKeyword,
            )
            .with_name(FieldName::LeaveKeyword)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::YulLeaveStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulTrueKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulFalseKeyword,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulDecimalLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulHexLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::HexStringLiteral,
            );
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::AsciiStringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::YulLiteral)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIdentifier,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::YulParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenParen,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.yul_parameters(input))
                    .with_name(FieldName::Parameters)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseParen,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::CloseParen,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::YulParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_return_variables(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| {
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIdentifier,
                )
                .with_name(FieldName::Item)
            },
            TokenKind::Comma,
            FieldName::Separator,
        )
        .with_kind(RuleKind::YulReturnVariables)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::MinusGreaterThan,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::MinusGreaterThan,
                ),
            )?;
            seq.elem_named(FieldName::Variables, self.yul_return_variables(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.yul_block(input);
            choice.consider(input, result)?;
            let result = self.yul_function_definition(input);
            choice.consider(input, result)?;
            let result = self.yul_variable_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_assignment_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_if_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_for_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_switch_statement(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.yul_leave_statement(input);
                choice.consider(input, result)?;
            }
            let result = self.yul_break_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_continue_statement(input);
            choice.consider(input, result)?;
            if !self.version_is_at_least_0_5_0 {
                let result = self.yul_label(input);
                choice.consider(input, result)?;
            }
            let result = self.yul_expression(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.yul_statement(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::YulStatements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.yul_default_case(input);
            choice.consider(input, result)?;
            let result = self.yul_value_case(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.yul_switch_case(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::YulSwitchCases)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::SwitchKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulSwitchKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Expression, self.yul_expression(input))?;
            seq.elem_named(FieldName::Cases, self.yul_switch_cases(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulSwitchStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_value_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::CaseKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulCaseKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Value, self.yul_literal(input))?;
            seq.elem_named(FieldName::Body, self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulValueCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::LetKeyword,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulLetKeyword,
                ),
            )?;
            seq.elem_named(FieldName::Names, self.yul_identifier_paths(input))?;
            seq.elem_named(
                FieldName::Value,
                OptionalHelper::transform(self.yul_variable_declaration_value(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::YulVariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_declaration_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                FieldName::ColonEqual,
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::ColonEqual,
                ),
            )?;
            seq.elem_named(FieldName::Expression, self.yul_expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulVariableDeclarationValue)
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
    fn ascii_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            self.single_quoted_ascii_string_literal(input),
            self.double_quoted_ascii_string_literal(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digits(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_one_or_more!(input, scan_char_range!(input, '0'..='9')),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_chars!(input, '_'),
                    scan_one_or_more!(input, scan_char_range!(input, '0'..='9'))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_exponent(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_choice!(input, scan_chars!(input, 'e'), scan_chars!(input, 'E')),
            scan_optional!(input, scan_chars!(input, '-')),
            self.decimal_digits(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_not_followed_by!(
                        input,
                        self.decimal_digits(input),
                        scan_chars!(input, '.')
                    ),
                    scan_optional!(input, self.decimal_exponent(input))
                ),
                self.identifier_start(input)
            ),
            if !self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        scan_not_followed_by!(
                            input,
                            scan_sequence!(self.decimal_digits(input), scan_chars!(input, '.')),
                            self.decimal_digits(input)
                        ),
                        scan_optional!(input, self.decimal_exponent(input))
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            },
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_chars!(input, '.'),
                    self.decimal_digits(input),
                    scan_optional!(input, self.decimal_exponent(input))
                ),
                self.identifier_start(input)
            ),
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    self.decimal_digits(input),
                    scan_chars!(input, '.'),
                    self.decimal_digits(input),
                    scan_optional!(input, self.decimal_exponent(input))
                ),
                self.identifier_start(input)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_ascii_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '"'),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    scan_char_range!(input, ' '..='!'),
                    scan_char_range!(input, '#'..='['),
                    scan_char_range!(input, ']'..='~')
                )
            ),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_hex_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '"'),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_unicode_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
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
        } else {
            false
        }
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
    fn hex_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            scan_not_followed_by!(
                input,
                scan_sequence!(
                    scan_chars!(input, '0', 'x'),
                    scan_one_or_more!(input, self.hex_character(input)),
                    scan_zero_or_more!(
                        input,
                        scan_sequence!(
                            scan_chars!(input, '_'),
                            scan_one_or_more!(input, self.hex_character(input))
                        )
                    )
                ),
                self.identifier_start(input)
            ),
            if !self.version_is_at_least_0_5_0 {
                scan_not_followed_by!(
                    input,
                    scan_sequence!(
                        scan_chars!(input, '0', 'X'),
                        scan_one_or_more!(input, self.hex_character(input)),
                        scan_zero_or_more!(
                            input,
                            scan_sequence!(
                                scan_chars!(input, '_'),
                                scan_one_or_more!(input, self.hex_character(input))
                            )
                        )
                    ),
                    self.identifier_start(input)
                )
            } else {
                false
            }
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_contents(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            self.hex_character(input),
            self.hex_character(input),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_optional!(input, scan_chars!(input, '_')),
                    self.hex_character(input),
                    self.hex_character(input)
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            self.single_quoted_hex_string_literal(input),
            self.double_quoted_hex_string_literal(input)
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
    fn multi_line_nat_spec_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '/', '*', '*'),
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
    fn single_line_nat_spec_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '/', '/', '/'),
            scan_zero_or_more!(input, scan_none_of!(input, '\r', '\n'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_ascii_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '\''),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    scan_char_range!(input, ' '..='&'),
                    scan_char_range!(input, '('..='['),
                    scan_char_range!(input, ']'..='~')
                )
            ),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_hex_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '\''),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_unicode_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_none_of!(input, '\'', '\\', '\r', '\n')
                    )
                ),
                scan_chars!(input, '\'')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn slash(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_chars!(input, '/'),
            scan_choice!(
                input,
                scan_chars!(input, '='),
                scan_chars!(input, '/'),
                scan_chars!(input, '*')
            )
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
    fn unicode_string_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_choice!(
            input,
            if self.version_is_at_least_0_7_0 {
                self.single_quoted_unicode_string_literal(input)
            } else {
                false
            },
            if self.version_is_at_least_0_7_0 {
                self.double_quoted_unicode_string_literal(input)
            } else {
                false
            }
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_value(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, 'x'),
                scan_chars!(input, 'X'),
                scan_chars!(input, '*'),
                scan_char_range!(input, '0'..='9')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, input: &mut ParserContext<'_>) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(input, scan_chars!(input, ' '), scan_chars!(input, '\t'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_decimal_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, '0'),
                scan_sequence!(
                    scan_char_range!(input, '1'..='9'),
                    scan_zero_or_more!(input, scan_char_range!(input, '0'..='9'))
                )
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_hex_literal(&self, input: &mut ParserContext<'_>) -> bool {
        scan_not_followed_by!(
            input,
            scan_sequence!(
                scan_chars!(input, '0', 'x'),
                scan_one_or_more!(input, self.hex_character(input))
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier(&self, input: &mut ParserContext<'_>) -> bool {
        self.raw_identifier(input)
    }

    // Keyword scanners
    #[inline]
    fn bytes_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if scan_sequence!(
                scan_chars!(input, 'b', 'y', 't', 'e', 's'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1')
                    )
                )
            ) {
                KeywordScan::Reserved(TokenKind::BytesKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn fixed_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if scan_chars!(input, 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TokenKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '6')
                )
            ) {
                KeywordScan::Reserved(TokenKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '4', '8', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '8'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '8', 'x', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                    scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                    scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                )
            ) {
                KeywordScan::Reserved(TokenKind::FixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                    scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                    scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TokenKind::FixedKeyword)
                } else {
                    KeywordScan::Present(TokenKind::FixedKeyword)
                }
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '5', '6'),
                    scan_chars!(input, '2', '4', '8'),
                    scan_chars!(input, '2', '4', '0'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '2', '3', '2'),
                    scan_chars!(input, '2', '2', '4'),
                    scan_chars!(input, '2', '1', '6'),
                    scan_chars!(input, '2', '0', '8'),
                    scan_chars!(input, '2', '0', '0'),
                    scan_chars!(input, '1', '9', '2'),
                    scan_chars!(input, '1', '8', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9'),
                    scan_chars!(input, '7', '9'),
                    scan_chars!(input, '7', '8'),
                    scan_chars!(input, '7', '7'),
                    scan_chars!(input, '7', '6'),
                    scan_chars!(input, '7', '5'),
                    scan_chars!(input, '7', '4'),
                    scan_chars!(input, '7', '3'),
                    scan_chars!(input, '7', '1'),
                    scan_chars!(input, '7', '0'),
                    scan_chars!(input, '7'),
                    scan_chars!(input, '6', '9'),
                    scan_chars!(input, '6', '8'),
                    scan_chars!(input, '6', '7'),
                    scan_chars!(input, '6', '6'),
                    scan_chars!(input, '6', '5'),
                    scan_chars!(input, '6', '3'),
                    scan_chars!(input, '6', '2'),
                    scan_chars!(input, '6', '1'),
                    scan_chars!(input, '6', '0'),
                    scan_chars!(input, '6'),
                    scan_chars!(input, '5', '9'),
                    scan_chars!(input, '5', '8'),
                    scan_chars!(input, '5', '7'),
                    scan_chars!(input, '5', '5'),
                    scan_chars!(input, '5', '4'),
                    scan_chars!(input, '5', '3'),
                    scan_chars!(input, '5', '2'),
                    scan_chars!(input, '5', '1'),
                    scan_chars!(input, '5', '0'),
                    scan_chars!(input, '5'),
                    scan_chars!(input, '4', '9'),
                    scan_chars!(input, '4', '7'),
                    scan_chars!(input, '4', '6'),
                    scan_chars!(input, '4', '5'),
                    scan_chars!(input, '4', '4'),
                    scan_chars!(input, '4', '3'),
                    scan_chars!(input, '4', '2'),
                    scan_chars!(input, '4', '1'),
                    scan_chars!(input, '4'),
                    scan_chars!(input, '3', '9'),
                    scan_chars!(input, '3', '8'),
                    scan_chars!(input, '3', '7'),
                    scan_chars!(input, '3', '6'),
                    scan_chars!(input, '3', '5'),
                    scan_chars!(input, '3', '4'),
                    scan_chars!(input, '3', '3'),
                    scan_chars!(input, '3', '1'),
                    scan_chars!(input, '3', '0'),
                    scan_chars!(input, '3'),
                    scan_chars!(input, '2', '9'),
                    scan_chars!(input, '2', '8'),
                    scan_chars!(input, '2', '7'),
                    scan_chars!(input, '2', '6'),
                    scan_chars!(input, '2', '5'),
                    scan_chars!(input, '2', '3'),
                    scan_chars!(input, '2', '2'),
                    scan_chars!(input, '2', '1'),
                    scan_chars!(input, '2', '0'),
                    scan_chars!(input, '2'),
                    scan_chars!(input, '1', '9'),
                    scan_chars!(input, '1', '8'),
                    scan_chars!(input, '1', '7'),
                    scan_chars!(input, '1', '5'),
                    scan_chars!(input, '1', '4'),
                    scan_chars!(input, '1', '3'),
                    scan_chars!(input, '1', '2'),
                    scan_chars!(input, '1', '1'),
                    scan_chars!(input, '1', '0'),
                    scan_chars!(input, '1'),
                    scan_chars!(input, '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TokenKind::FixedKeyword)
                } else {
                    KeywordScan::Present(TokenKind::FixedKeyword)
                }
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn int_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if scan_sequence!(
                scan_chars!(input, 'i', 'n', 't'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    )
                )
            ) {
                KeywordScan::Reserved(TokenKind::IntKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn ufixed_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TokenKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '1', '6')
                )
            ) {
                KeywordScan::Reserved(TokenKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '4', '8', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '8'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '8', 'x', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                    scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                    scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                    scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                    scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                    scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                )
            ) {
                KeywordScan::Reserved(TokenKind::UfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                    scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                    scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                    scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                    scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                    scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                    scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                    scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                    scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                    scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                    scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                    scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                    scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                    scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                    scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                    scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                    scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                    scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                    scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                    scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                    scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                    scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                    scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                    scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                    scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                    scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                    scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                    scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                    scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TokenKind::UfixedKeyword)
                } else {
                    KeywordScan::Present(TokenKind::UfixedKeyword)
                }
            } else {
                KeywordScan::Absent
            },
            if scan_sequence!(
                scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9', '6'),
                    scan_chars!(input, '8', '8'),
                    scan_chars!(input, '8', '0'),
                    scan_chars!(input, '8'),
                    scan_chars!(input, '7', '2'),
                    scan_chars!(input, '6', '4'),
                    scan_chars!(input, '5', '6'),
                    scan_chars!(input, '4', '8'),
                    scan_chars!(input, '4', '0'),
                    scan_chars!(input, '3', '2'),
                    scan_chars!(input, '2', '5', '6'),
                    scan_chars!(input, '2', '4', '8'),
                    scan_chars!(input, '2', '4', '0'),
                    scan_chars!(input, '2', '4'),
                    scan_chars!(input, '2', '3', '2'),
                    scan_chars!(input, '2', '2', '4'),
                    scan_chars!(input, '2', '1', '6'),
                    scan_chars!(input, '2', '0', '8'),
                    scan_chars!(input, '2', '0', '0'),
                    scan_chars!(input, '1', '9', '2'),
                    scan_chars!(input, '1', '8', '4'),
                    scan_chars!(input, '1', '7', '6'),
                    scan_chars!(input, '1', '6', '8'),
                    scan_chars!(input, '1', '6', '0'),
                    scan_chars!(input, '1', '6'),
                    scan_chars!(input, '1', '5', '2'),
                    scan_chars!(input, '1', '4', '4'),
                    scan_chars!(input, '1', '3', '6'),
                    scan_chars!(input, '1', '2', '8'),
                    scan_chars!(input, '1', '2', '0'),
                    scan_chars!(input, '1', '1', '2'),
                    scan_chars!(input, '1', '0', '4')
                ),
                scan_chars!(input, 'x'),
                scan_choice!(
                    input,
                    scan_chars!(input, '9'),
                    scan_chars!(input, '7', '9'),
                    scan_chars!(input, '7', '8'),
                    scan_chars!(input, '7', '7'),
                    scan_chars!(input, '7', '6'),
                    scan_chars!(input, '7', '5'),
                    scan_chars!(input, '7', '4'),
                    scan_chars!(input, '7', '3'),
                    scan_chars!(input, '7', '1'),
                    scan_chars!(input, '7', '0'),
                    scan_chars!(input, '7'),
                    scan_chars!(input, '6', '9'),
                    scan_chars!(input, '6', '8'),
                    scan_chars!(input, '6', '7'),
                    scan_chars!(input, '6', '6'),
                    scan_chars!(input, '6', '5'),
                    scan_chars!(input, '6', '3'),
                    scan_chars!(input, '6', '2'),
                    scan_chars!(input, '6', '1'),
                    scan_chars!(input, '6', '0'),
                    scan_chars!(input, '6'),
                    scan_chars!(input, '5', '9'),
                    scan_chars!(input, '5', '8'),
                    scan_chars!(input, '5', '7'),
                    scan_chars!(input, '5', '5'),
                    scan_chars!(input, '5', '4'),
                    scan_chars!(input, '5', '3'),
                    scan_chars!(input, '5', '2'),
                    scan_chars!(input, '5', '1'),
                    scan_chars!(input, '5', '0'),
                    scan_chars!(input, '5'),
                    scan_chars!(input, '4', '9'),
                    scan_chars!(input, '4', '7'),
                    scan_chars!(input, '4', '6'),
                    scan_chars!(input, '4', '5'),
                    scan_chars!(input, '4', '4'),
                    scan_chars!(input, '4', '3'),
                    scan_chars!(input, '4', '2'),
                    scan_chars!(input, '4', '1'),
                    scan_chars!(input, '4'),
                    scan_chars!(input, '3', '9'),
                    scan_chars!(input, '3', '8'),
                    scan_chars!(input, '3', '7'),
                    scan_chars!(input, '3', '6'),
                    scan_chars!(input, '3', '5'),
                    scan_chars!(input, '3', '4'),
                    scan_chars!(input, '3', '3'),
                    scan_chars!(input, '3', '1'),
                    scan_chars!(input, '3', '0'),
                    scan_chars!(input, '3'),
                    scan_chars!(input, '2', '9'),
                    scan_chars!(input, '2', '8'),
                    scan_chars!(input, '2', '7'),
                    scan_chars!(input, '2', '6'),
                    scan_chars!(input, '2', '5'),
                    scan_chars!(input, '2', '3'),
                    scan_chars!(input, '2', '2'),
                    scan_chars!(input, '2', '1'),
                    scan_chars!(input, '2', '0'),
                    scan_chars!(input, '2'),
                    scan_chars!(input, '1', '9'),
                    scan_chars!(input, '1', '8'),
                    scan_chars!(input, '1', '7'),
                    scan_chars!(input, '1', '5'),
                    scan_chars!(input, '1', '4'),
                    scan_chars!(input, '1', '3'),
                    scan_chars!(input, '1', '2'),
                    scan_chars!(input, '1', '1'),
                    scan_chars!(input, '1', '0'),
                    scan_chars!(input, '1'),
                    scan_chars!(input, '0')
                )
            ) {
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved(TokenKind::UfixedKeyword)
                } else {
                    KeywordScan::Present(TokenKind::UfixedKeyword)
                }
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn uint_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if scan_sequence!(
                scan_chars!(input, 'u', 'i', 'n', 't'),
                scan_optional!(
                    input,
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    )
                )
            ) {
                KeywordScan::Reserved(TokenKind::UintKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_bytes_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'b', 'y', 't', 'e', 's'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7'),
                            scan_chars!(input, '6'),
                            scan_chars!(input, '5'),
                            scan_chars!(input, '4'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '3', '1'),
                            scan_chars!(input, '3', '0'),
                            scan_chars!(input, '3'),
                            scan_chars!(input, '2', '9'),
                            scan_chars!(input, '2', '8'),
                            scan_chars!(input, '2', '7'),
                            scan_chars!(input, '2', '6'),
                            scan_chars!(input, '2', '5'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3'),
                            scan_chars!(input, '2', '2'),
                            scan_chars!(input, '2', '1'),
                            scan_chars!(input, '2', '0'),
                            scan_chars!(input, '2'),
                            scan_chars!(input, '1', '9'),
                            scan_chars!(input, '1', '8'),
                            scan_chars!(input, '1', '7'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5'),
                            scan_chars!(input, '1', '4'),
                            scan_chars!(input, '1', '3'),
                            scan_chars!(input, '1', '2'),
                            scan_chars!(input, '1', '1'),
                            scan_chars!(input, '1', '0'),
                            scan_chars!(input, '1')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulBytesKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_fixed_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if !self.version_is_at_least_0_7_1 && scan_chars!(input, 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TokenKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '4', '8', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '8'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '8', 'x', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                        scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                        scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                        scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                        scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '7', '9'),
                        scan_chars!(input, '7', '8'),
                        scan_chars!(input, '7', '7'),
                        scan_chars!(input, '7', '6'),
                        scan_chars!(input, '7', '5'),
                        scan_chars!(input, '7', '4'),
                        scan_chars!(input, '7', '3'),
                        scan_chars!(input, '7', '1'),
                        scan_chars!(input, '7', '0'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6', '9'),
                        scan_chars!(input, '6', '8'),
                        scan_chars!(input, '6', '7'),
                        scan_chars!(input, '6', '6'),
                        scan_chars!(input, '6', '5'),
                        scan_chars!(input, '6', '3'),
                        scan_chars!(input, '6', '2'),
                        scan_chars!(input, '6', '1'),
                        scan_chars!(input, '6', '0'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5', '9'),
                        scan_chars!(input, '5', '8'),
                        scan_chars!(input, '5', '7'),
                        scan_chars!(input, '5', '5'),
                        scan_chars!(input, '5', '4'),
                        scan_chars!(input, '5', '3'),
                        scan_chars!(input, '5', '2'),
                        scan_chars!(input, '5', '1'),
                        scan_chars!(input, '5', '0'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4', '9'),
                        scan_chars!(input, '4', '7'),
                        scan_chars!(input, '4', '6'),
                        scan_chars!(input, '4', '5'),
                        scan_chars!(input, '4', '4'),
                        scan_chars!(input, '4', '3'),
                        scan_chars!(input, '4', '2'),
                        scan_chars!(input, '4', '1'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '9'),
                        scan_chars!(input, '3', '8'),
                        scan_chars!(input, '3', '7'),
                        scan_chars!(input, '3', '6'),
                        scan_chars!(input, '3', '5'),
                        scan_chars!(input, '3', '4'),
                        scan_chars!(input, '3', '3'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1'),
                        scan_chars!(input, '0')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulFixedKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_int_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'i', 'n', 't'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9', '6'),
                            scan_chars!(input, '8', '8'),
                            scan_chars!(input, '8', '0'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7', '2'),
                            scan_chars!(input, '6', '4'),
                            scan_chars!(input, '5', '6'),
                            scan_chars!(input, '4', '8'),
                            scan_chars!(input, '4', '0'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '2', '5', '6'),
                            scan_chars!(input, '2', '4', '8'),
                            scan_chars!(input, '2', '4', '0'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3', '2'),
                            scan_chars!(input, '2', '2', '4'),
                            scan_chars!(input, '2', '1', '6'),
                            scan_chars!(input, '2', '0', '8'),
                            scan_chars!(input, '2', '0', '0'),
                            scan_chars!(input, '1', '9', '2'),
                            scan_chars!(input, '1', '8', '4'),
                            scan_chars!(input, '1', '7', '6'),
                            scan_chars!(input, '1', '6', '8'),
                            scan_chars!(input, '1', '6', '0'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5', '2'),
                            scan_chars!(input, '1', '4', '4'),
                            scan_chars!(input, '1', '3', '6'),
                            scan_chars!(input, '1', '2', '8'),
                            scan_chars!(input, '1', '2', '0'),
                            scan_chars!(input, '1', '1', '2'),
                            scan_chars!(input, '1', '0', '4')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulIntKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_ufixed_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if !self.version_is_at_least_0_7_1 && scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd') {
                KeywordScan::Reserved(TokenKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '4', '8', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '1', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '1', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '2', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '1', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '8'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '8', 'x', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '0', '0', 'x', '1', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '5', '6'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '8'),
                        scan_chars!(input, '1', '9', '2', 'x', '4', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '3', '2'),
                        scan_chars!(input, '1', '9', '2', 'x', '2', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '1', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '6', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '5', '6'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '8'),
                        scan_chars!(input, '1', '8', '4', 'x', '4', '0'),
                        scan_chars!(input, '1', '8', '4', 'x', '3', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '2', '4'),
                        scan_chars!(input, '1', '8', '4', 'x', '1', '6')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '2', '5', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '5', '6', 'x', '4', '0'),
                        scan_chars!(input, '2', '5', '6', 'x', '3', '2'),
                        scan_chars!(input, '2', '5', '6', 'x', '2', '4'),
                        scan_chars!(input, '2', '5', '6', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '8', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '8', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '8', 'x', '2', '4'),
                        scan_chars!(input, '2', '4', '8', 'x', '1', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '6', '4'),
                        scan_chars!(input, '2', '4', '0', 'x', '5', '6'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '8'),
                        scan_chars!(input, '2', '4', '0', 'x', '4', '0'),
                        scan_chars!(input, '2', '4', '0', 'x', '3', '2'),
                        scan_chars!(input, '2', '4', '0', 'x', '2', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '8', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '7', '2'),
                        scan_chars!(input, '2', '3', '2', 'x', '6', '4'),
                        scan_chars!(input, '2', '3', '2', 'x', '5', '6'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '8'),
                        scan_chars!(input, '2', '3', '2', 'x', '4', '0'),
                        scan_chars!(input, '2', '3', '2', 'x', '3', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '8', '0'),
                        scan_chars!(input, '2', '2', '4', 'x', '7', '2'),
                        scan_chars!(input, '2', '2', '4', 'x', '6', '4'),
                        scan_chars!(input, '2', '2', '4', 'x', '5', '6'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '8'),
                        scan_chars!(input, '2', '2', '4', 'x', '4', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '8', '0'),
                        scan_chars!(input, '2', '1', '6', 'x', '7', '2'),
                        scan_chars!(input, '2', '1', '6', 'x', '6', '4'),
                        scan_chars!(input, '2', '1', '6', 'x', '5', '6'),
                        scan_chars!(input, '2', '1', '6', 'x', '4', '8'),
                        scan_chars!(input, '2', '0', '8', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '8', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '8', 'x', '6', '4'),
                        scan_chars!(input, '2', '0', '8', 'x', '5', '6'),
                        scan_chars!(input, '2', '0', '0', 'x', '8', '0'),
                        scan_chars!(input, '2', '0', '0', 'x', '7', '2'),
                        scan_chars!(input, '2', '0', '0', 'x', '6', '4'),
                        scan_chars!(input, '1', '9', '2', 'x', '8', '0'),
                        scan_chars!(input, '1', '9', '2', 'x', '7', '2'),
                        scan_chars!(input, '1', '8', '4', 'x', '8', '0')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            },
            if self.version_is_at_least_0_4_14
                && !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9', '6'),
                        scan_chars!(input, '8', '8'),
                        scan_chars!(input, '8', '0'),
                        scan_chars!(input, '8'),
                        scan_chars!(input, '7', '2'),
                        scan_chars!(input, '6', '4'),
                        scan_chars!(input, '5', '6'),
                        scan_chars!(input, '4', '8'),
                        scan_chars!(input, '4', '0'),
                        scan_chars!(input, '3', '2'),
                        scan_chars!(input, '2', '5', '6'),
                        scan_chars!(input, '2', '4', '8'),
                        scan_chars!(input, '2', '4', '0'),
                        scan_chars!(input, '2', '4'),
                        scan_chars!(input, '2', '3', '2'),
                        scan_chars!(input, '2', '2', '4'),
                        scan_chars!(input, '2', '1', '6'),
                        scan_chars!(input, '2', '0', '8'),
                        scan_chars!(input, '2', '0', '0'),
                        scan_chars!(input, '1', '9', '2'),
                        scan_chars!(input, '1', '8', '4'),
                        scan_chars!(input, '1', '7', '6'),
                        scan_chars!(input, '1', '6', '8'),
                        scan_chars!(input, '1', '6', '0'),
                        scan_chars!(input, '1', '6'),
                        scan_chars!(input, '1', '5', '2'),
                        scan_chars!(input, '1', '4', '4'),
                        scan_chars!(input, '1', '3', '6'),
                        scan_chars!(input, '1', '2', '8'),
                        scan_chars!(input, '1', '2', '0'),
                        scan_chars!(input, '1', '1', '2'),
                        scan_chars!(input, '1', '0', '4')
                    ),
                    scan_chars!(input, 'x'),
                    scan_choice!(
                        input,
                        scan_chars!(input, '9'),
                        scan_chars!(input, '7', '9'),
                        scan_chars!(input, '7', '8'),
                        scan_chars!(input, '7', '7'),
                        scan_chars!(input, '7', '6'),
                        scan_chars!(input, '7', '5'),
                        scan_chars!(input, '7', '4'),
                        scan_chars!(input, '7', '3'),
                        scan_chars!(input, '7', '1'),
                        scan_chars!(input, '7', '0'),
                        scan_chars!(input, '7'),
                        scan_chars!(input, '6', '9'),
                        scan_chars!(input, '6', '8'),
                        scan_chars!(input, '6', '7'),
                        scan_chars!(input, '6', '6'),
                        scan_chars!(input, '6', '5'),
                        scan_chars!(input, '6', '3'),
                        scan_chars!(input, '6', '2'),
                        scan_chars!(input, '6', '1'),
                        scan_chars!(input, '6', '0'),
                        scan_chars!(input, '6'),
                        scan_chars!(input, '5', '9'),
                        scan_chars!(input, '5', '8'),
                        scan_chars!(input, '5', '7'),
                        scan_chars!(input, '5', '5'),
                        scan_chars!(input, '5', '4'),
                        scan_chars!(input, '5', '3'),
                        scan_chars!(input, '5', '2'),
                        scan_chars!(input, '5', '1'),
                        scan_chars!(input, '5', '0'),
                        scan_chars!(input, '5'),
                        scan_chars!(input, '4', '9'),
                        scan_chars!(input, '4', '7'),
                        scan_chars!(input, '4', '6'),
                        scan_chars!(input, '4', '5'),
                        scan_chars!(input, '4', '4'),
                        scan_chars!(input, '4', '3'),
                        scan_chars!(input, '4', '2'),
                        scan_chars!(input, '4', '1'),
                        scan_chars!(input, '4'),
                        scan_chars!(input, '3', '9'),
                        scan_chars!(input, '3', '8'),
                        scan_chars!(input, '3', '7'),
                        scan_chars!(input, '3', '6'),
                        scan_chars!(input, '3', '5'),
                        scan_chars!(input, '3', '4'),
                        scan_chars!(input, '3', '3'),
                        scan_chars!(input, '3', '1'),
                        scan_chars!(input, '3', '0'),
                        scan_chars!(input, '3'),
                        scan_chars!(input, '2', '9'),
                        scan_chars!(input, '2', '8'),
                        scan_chars!(input, '2', '7'),
                        scan_chars!(input, '2', '6'),
                        scan_chars!(input, '2', '5'),
                        scan_chars!(input, '2', '3'),
                        scan_chars!(input, '2', '2'),
                        scan_chars!(input, '2', '1'),
                        scan_chars!(input, '2', '0'),
                        scan_chars!(input, '2'),
                        scan_chars!(input, '1', '9'),
                        scan_chars!(input, '1', '8'),
                        scan_chars!(input, '1', '7'),
                        scan_chars!(input, '1', '5'),
                        scan_chars!(input, '1', '4'),
                        scan_chars!(input, '1', '3'),
                        scan_chars!(input, '1', '2'),
                        scan_chars!(input, '1', '1'),
                        scan_chars!(input, '1', '0'),
                        scan_chars!(input, '1'),
                        scan_chars!(input, '0')
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulUfixedKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    #[inline]
    fn yul_uint_keyword(&self, input: &mut ParserContext<'_>, ident: &str) -> KeywordScan {
        scan_keyword_choice!(
            input,
            ident,
            if !self.version_is_at_least_0_7_1
                && scan_sequence!(
                    scan_chars!(input, 'u', 'i', 'n', 't'),
                    scan_optional!(
                        input,
                        scan_choice!(
                            input,
                            scan_chars!(input, '9', '6'),
                            scan_chars!(input, '8', '8'),
                            scan_chars!(input, '8', '0'),
                            scan_chars!(input, '8'),
                            scan_chars!(input, '7', '2'),
                            scan_chars!(input, '6', '4'),
                            scan_chars!(input, '5', '6'),
                            scan_chars!(input, '4', '8'),
                            scan_chars!(input, '4', '0'),
                            scan_chars!(input, '3', '2'),
                            scan_chars!(input, '2', '5', '6'),
                            scan_chars!(input, '2', '4', '8'),
                            scan_chars!(input, '2', '4', '0'),
                            scan_chars!(input, '2', '4'),
                            scan_chars!(input, '2', '3', '2'),
                            scan_chars!(input, '2', '2', '4'),
                            scan_chars!(input, '2', '1', '6'),
                            scan_chars!(input, '2', '0', '8'),
                            scan_chars!(input, '2', '0', '0'),
                            scan_chars!(input, '1', '9', '2'),
                            scan_chars!(input, '1', '8', '4'),
                            scan_chars!(input, '1', '7', '6'),
                            scan_chars!(input, '1', '6', '8'),
                            scan_chars!(input, '1', '6', '0'),
                            scan_chars!(input, '1', '6'),
                            scan_chars!(input, '1', '5', '2'),
                            scan_chars!(input, '1', '4', '4'),
                            scan_chars!(input, '1', '3', '6'),
                            scan_chars!(input, '1', '2', '8'),
                            scan_chars!(input, '1', '2', '0'),
                            scan_chars!(input, '1', '1', '2'),
                            scan_chars!(input, '1', '0', '4')
                        )
                    )
                )
            {
                KeywordScan::Reserved(TokenKind::YulUintKeyword)
            } else {
                KeywordScan::Absent
            }
        )
    }

    pub fn parse(&self, kind: RuleKind, input: &str) -> ParseOutput {
        match kind {
            RuleKind::ABICoderPragma => Self::abi_coder_pragma.parse(self, input, true),
            RuleKind::AdditiveExpression => Self::additive_expression.parse(self, input, true),
            RuleKind::AddressType => Self::address_type.parse(self, input, true),
            RuleKind::AndExpression => Self::and_expression.parse(self, input, true),
            RuleKind::ArgumentsDeclaration => Self::arguments_declaration.parse(self, input, true),
            RuleKind::ArrayExpression => Self::array_expression.parse(self, input, true),
            RuleKind::ArrayTypeName => Self::array_type_name.parse(self, input, true),
            RuleKind::ArrayValues => Self::array_values.parse(self, input, true),
            RuleKind::AsciiStringLiterals => Self::ascii_string_literals.parse(self, input, true),
            RuleKind::AssemblyFlags => Self::assembly_flags.parse(self, input, true),
            RuleKind::AssemblyFlagsDeclaration => {
                Self::assembly_flags_declaration.parse(self, input, true)
            }
            RuleKind::AssemblyStatement => Self::assembly_statement.parse(self, input, true),
            RuleKind::AssignmentExpression => Self::assignment_expression.parse(self, input, true),
            RuleKind::BitwiseAndExpression => Self::bitwise_and_expression.parse(self, input, true),
            RuleKind::BitwiseOrExpression => Self::bitwise_or_expression.parse(self, input, true),
            RuleKind::BitwiseXorExpression => Self::bitwise_xor_expression.parse(self, input, true),
            RuleKind::Block => Self::block.parse(self, input, true),
            RuleKind::BreakStatement => Self::break_statement.parse(self, input, true),
            RuleKind::CatchClause => Self::catch_clause.parse(self, input, true),
            RuleKind::CatchClauseError => Self::catch_clause_error.parse(self, input, true),
            RuleKind::CatchClauses => Self::catch_clauses.parse(self, input, true),
            RuleKind::ComparisonExpression => Self::comparison_expression.parse(self, input, true),
            RuleKind::ConditionalExpression => {
                Self::conditional_expression.parse(self, input, true)
            }
            RuleKind::ConstantDefinition => Self::constant_definition.parse(self, input, true),
            RuleKind::ConstructorAttribute => Self::constructor_attribute.parse(self, input, true),
            RuleKind::ConstructorAttributes => {
                Self::constructor_attributes.parse(self, input, true)
            }
            RuleKind::ConstructorDefinition => {
                Self::constructor_definition.parse(self, input, true)
            }
            RuleKind::ContinueStatement => Self::continue_statement.parse(self, input, true),
            RuleKind::ContractDefinition => Self::contract_definition.parse(self, input, true),
            RuleKind::ContractMember => Self::contract_member.parse(self, input, true),
            RuleKind::ContractMembers => Self::contract_members.parse(self, input, true),
            RuleKind::DecimalNumberExpression => {
                Self::decimal_number_expression.parse(self, input, true)
            }
            RuleKind::DeleteStatement => Self::delete_statement.parse(self, input, true),
            RuleKind::DoWhileStatement => Self::do_while_statement.parse(self, input, true),
            RuleKind::ElementaryType => Self::elementary_type.parse(self, input, true),
            RuleKind::ElseBranch => Self::else_branch.parse(self, input, true),
            RuleKind::EmitStatement => Self::emit_statement.parse(self, input, true),
            RuleKind::EnumDefinition => Self::enum_definition.parse(self, input, true),
            RuleKind::EnumMembers => Self::enum_members.parse(self, input, true),
            RuleKind::EqualityExpression => Self::equality_expression.parse(self, input, true),
            RuleKind::ErrorDefinition => Self::error_definition.parse(self, input, true),
            RuleKind::ErrorParameter => Self::error_parameter.parse(self, input, true),
            RuleKind::ErrorParameters => Self::error_parameters.parse(self, input, true),
            RuleKind::ErrorParametersDeclaration => {
                Self::error_parameters_declaration.parse(self, input, true)
            }
            RuleKind::EventDefinition => Self::event_definition.parse(self, input, true),
            RuleKind::EventParameter => Self::event_parameter.parse(self, input, true),
            RuleKind::EventParameters => Self::event_parameters.parse(self, input, true),
            RuleKind::EventParametersDeclaration => {
                Self::event_parameters_declaration.parse(self, input, true)
            }
            RuleKind::ExperimentalFeature => Self::experimental_feature.parse(self, input, true),
            RuleKind::ExperimentalPragma => Self::experimental_pragma.parse(self, input, true),
            RuleKind::ExponentiationExpression => {
                Self::exponentiation_expression.parse(self, input, true)
            }
            RuleKind::Expression => Self::expression.parse(self, input, true),
            RuleKind::ExpressionStatement => Self::expression_statement.parse(self, input, true),
            RuleKind::FallbackFunctionAttribute => {
                Self::fallback_function_attribute.parse(self, input, true)
            }
            RuleKind::FallbackFunctionAttributes => {
                Self::fallback_function_attributes.parse(self, input, true)
            }
            RuleKind::FallbackFunctionDefinition => {
                Self::fallback_function_definition.parse(self, input, true)
            }
            RuleKind::ForStatement => Self::for_statement.parse(self, input, true),
            RuleKind::ForStatementCondition => {
                Self::for_statement_condition.parse(self, input, true)
            }
            RuleKind::ForStatementInitialization => {
                Self::for_statement_initialization.parse(self, input, true)
            }
            RuleKind::FunctionAttribute => Self::function_attribute.parse(self, input, true),
            RuleKind::FunctionAttributes => Self::function_attributes.parse(self, input, true),
            RuleKind::FunctionBody => Self::function_body.parse(self, input, true),
            RuleKind::FunctionCallExpression => {
                Self::function_call_expression.parse(self, input, true)
            }
            RuleKind::FunctionCallOptions => Self::function_call_options.parse(self, input, true),
            RuleKind::FunctionDefinition => Self::function_definition.parse(self, input, true),
            RuleKind::FunctionName => Self::function_name.parse(self, input, true),
            RuleKind::FunctionType => Self::function_type.parse(self, input, true),
            RuleKind::FunctionTypeAttribute => {
                Self::function_type_attribute.parse(self, input, true)
            }
            RuleKind::FunctionTypeAttributes => {
                Self::function_type_attributes.parse(self, input, true)
            }
            RuleKind::HexNumberExpression => Self::hex_number_expression.parse(self, input, true),
            RuleKind::HexStringLiterals => Self::hex_string_literals.parse(self, input, true),
            RuleKind::IdentifierPath => Self::identifier_path.parse(self, input, true),
            RuleKind::IfStatement => Self::if_statement.parse(self, input, true),
            RuleKind::ImportAlias => Self::import_alias.parse(self, input, true),
            RuleKind::ImportClause => Self::import_clause.parse(self, input, true),
            RuleKind::ImportDeconstruction => Self::import_deconstruction.parse(self, input, true),
            RuleKind::ImportDeconstructionSymbol => {
                Self::import_deconstruction_symbol.parse(self, input, true)
            }
            RuleKind::ImportDeconstructionSymbols => {
                Self::import_deconstruction_symbols.parse(self, input, true)
            }
            RuleKind::ImportDirective => Self::import_directive.parse(self, input, true),
            RuleKind::IndexAccessEnd => Self::index_access_end.parse(self, input, true),
            RuleKind::IndexAccessExpression => {
                Self::index_access_expression.parse(self, input, true)
            }
            RuleKind::InheritanceSpecifier => Self::inheritance_specifier.parse(self, input, true),
            RuleKind::InheritanceType => Self::inheritance_type.parse(self, input, true),
            RuleKind::InheritanceTypes => Self::inheritance_types.parse(self, input, true),
            RuleKind::InterfaceDefinition => Self::interface_definition.parse(self, input, true),
            RuleKind::InterfaceMembers => Self::interface_members.parse(self, input, true),
            RuleKind::LeadingTrivia => Self::leading_trivia.parse(self, input, false),
            RuleKind::LibraryDefinition => Self::library_definition.parse(self, input, true),
            RuleKind::LibraryMembers => Self::library_members.parse(self, input, true),
            RuleKind::MappingKey => Self::mapping_key.parse(self, input, true),
            RuleKind::MappingKeyType => Self::mapping_key_type.parse(self, input, true),
            RuleKind::MappingType => Self::mapping_type.parse(self, input, true),
            RuleKind::MappingValue => Self::mapping_value.parse(self, input, true),
            RuleKind::MemberAccess => Self::member_access.parse(self, input, true),
            RuleKind::MemberAccessExpression => {
                Self::member_access_expression.parse(self, input, true)
            }
            RuleKind::ModifierAttribute => Self::modifier_attribute.parse(self, input, true),
            RuleKind::ModifierAttributes => Self::modifier_attributes.parse(self, input, true),
            RuleKind::ModifierDefinition => Self::modifier_definition.parse(self, input, true),
            RuleKind::ModifierInvocation => Self::modifier_invocation.parse(self, input, true),
            RuleKind::MultiplicativeExpression => {
                Self::multiplicative_expression.parse(self, input, true)
            }
            RuleKind::NamedArgument => Self::named_argument.parse(self, input, true),
            RuleKind::NamedArgumentGroup => Self::named_argument_group.parse(self, input, true),
            RuleKind::NamedArgumentGroups => Self::named_argument_groups.parse(self, input, true),
            RuleKind::NamedArguments => Self::named_arguments.parse(self, input, true),
            RuleKind::NamedArgumentsDeclaration => {
                Self::named_arguments_declaration.parse(self, input, true)
            }
            RuleKind::NamedImport => Self::named_import.parse(self, input, true),
            RuleKind::NewExpression => Self::new_expression.parse(self, input, true),
            RuleKind::NumberUnit => Self::number_unit.parse(self, input, true),
            RuleKind::OrExpression => Self::or_expression.parse(self, input, true),
            RuleKind::OverridePaths => Self::override_paths.parse(self, input, true),
            RuleKind::OverridePathsDeclaration => {
                Self::override_paths_declaration.parse(self, input, true)
            }
            RuleKind::OverrideSpecifier => Self::override_specifier.parse(self, input, true),
            RuleKind::Parameter => Self::parameter.parse(self, input, true),
            RuleKind::Parameters => Self::parameters.parse(self, input, true),
            RuleKind::ParametersDeclaration => {
                Self::parameters_declaration.parse(self, input, true)
            }
            RuleKind::PathImport => Self::path_import.parse(self, input, true),
            RuleKind::PositionalArguments => Self::positional_arguments.parse(self, input, true),
            RuleKind::PositionalArgumentsDeclaration => {
                Self::positional_arguments_declaration.parse(self, input, true)
            }
            RuleKind::PostfixExpression => Self::postfix_expression.parse(self, input, true),
            RuleKind::Pragma => Self::pragma.parse(self, input, true),
            RuleKind::PragmaDirective => Self::pragma_directive.parse(self, input, true),
            RuleKind::PrefixExpression => Self::prefix_expression.parse(self, input, true),
            RuleKind::ReceiveFunctionAttribute => {
                Self::receive_function_attribute.parse(self, input, true)
            }
            RuleKind::ReceiveFunctionAttributes => {
                Self::receive_function_attributes.parse(self, input, true)
            }
            RuleKind::ReceiveFunctionDefinition => {
                Self::receive_function_definition.parse(self, input, true)
            }
            RuleKind::ReturnStatement => Self::return_statement.parse(self, input, true),
            RuleKind::ReturnsDeclaration => Self::returns_declaration.parse(self, input, true),
            RuleKind::RevertStatement => Self::revert_statement.parse(self, input, true),
            RuleKind::ShiftExpression => Self::shift_expression.parse(self, input, true),
            RuleKind::SourceUnit => Self::source_unit.parse(self, input, true),
            RuleKind::SourceUnitMember => Self::source_unit_member.parse(self, input, true),
            RuleKind::SourceUnitMembers => Self::source_unit_members.parse(self, input, true),
            RuleKind::StateVariableAttribute => {
                Self::state_variable_attribute.parse(self, input, true)
            }
            RuleKind::StateVariableAttributes => {
                Self::state_variable_attributes.parse(self, input, true)
            }
            RuleKind::StateVariableDefinition => {
                Self::state_variable_definition.parse(self, input, true)
            }
            RuleKind::StateVariableDefinitionValue => {
                Self::state_variable_definition_value.parse(self, input, true)
            }
            RuleKind::Statement => Self::statement.parse(self, input, true),
            RuleKind::Statements => Self::statements.parse(self, input, true),
            RuleKind::StorageLocation => Self::storage_location.parse(self, input, true),
            RuleKind::StringExpression => Self::string_expression.parse(self, input, true),
            RuleKind::StructDefinition => Self::struct_definition.parse(self, input, true),
            RuleKind::StructMember => Self::struct_member.parse(self, input, true),
            RuleKind::StructMembers => Self::struct_members.parse(self, input, true),
            RuleKind::ThrowStatement => Self::throw_statement.parse(self, input, true),
            RuleKind::TrailingTrivia => Self::trailing_trivia.parse(self, input, false),
            RuleKind::TryStatement => Self::try_statement.parse(self, input, true),
            RuleKind::TupleDeconstructionElement => {
                Self::tuple_deconstruction_element.parse(self, input, true)
            }
            RuleKind::TupleDeconstructionElements => {
                Self::tuple_deconstruction_elements.parse(self, input, true)
            }
            RuleKind::TupleDeconstructionStatement => {
                Self::tuple_deconstruction_statement.parse(self, input, true)
            }
            RuleKind::TupleExpression => Self::tuple_expression.parse(self, input, true),
            RuleKind::TupleMember => Self::tuple_member.parse(self, input, true),
            RuleKind::TupleValue => Self::tuple_value.parse(self, input, true),
            RuleKind::TupleValues => Self::tuple_values.parse(self, input, true),
            RuleKind::TypeExpression => Self::type_expression.parse(self, input, true),
            RuleKind::TypeName => Self::type_name.parse(self, input, true),
            RuleKind::TypedTupleMember => Self::typed_tuple_member.parse(self, input, true),
            RuleKind::UncheckedBlock => Self::unchecked_block.parse(self, input, true),
            RuleKind::UnicodeStringLiterals => {
                Self::unicode_string_literals.parse(self, input, true)
            }
            RuleKind::UnnamedFunctionAttribute => {
                Self::unnamed_function_attribute.parse(self, input, true)
            }
            RuleKind::UnnamedFunctionAttributes => {
                Self::unnamed_function_attributes.parse(self, input, true)
            }
            RuleKind::UnnamedFunctionDefinition => {
                Self::unnamed_function_definition.parse(self, input, true)
            }
            RuleKind::UntypedTupleMember => Self::untyped_tuple_member.parse(self, input, true),
            RuleKind::UserDefinedValueTypeDefinition => {
                Self::user_defined_value_type_definition.parse(self, input, true)
            }
            RuleKind::UsingAlias => Self::using_alias.parse(self, input, true),
            RuleKind::UsingClause => Self::using_clause.parse(self, input, true),
            RuleKind::UsingDeconstruction => Self::using_deconstruction.parse(self, input, true),
            RuleKind::UsingDeconstructionSymbol => {
                Self::using_deconstruction_symbol.parse(self, input, true)
            }
            RuleKind::UsingDeconstructionSymbols => {
                Self::using_deconstruction_symbols.parse(self, input, true)
            }
            RuleKind::UsingDirective => Self::using_directive.parse(self, input, true),
            RuleKind::UsingOperator => Self::using_operator.parse(self, input, true),
            RuleKind::UsingTarget => Self::using_target.parse(self, input, true),
            RuleKind::VariableDeclarationStatement => {
                Self::variable_declaration_statement.parse(self, input, true)
            }
            RuleKind::VariableDeclarationType => {
                Self::variable_declaration_type.parse(self, input, true)
            }
            RuleKind::VariableDeclarationValue => {
                Self::variable_declaration_value.parse(self, input, true)
            }
            RuleKind::VersionPragma => Self::version_pragma.parse(self, input, true),
            RuleKind::VersionPragmaExpression => {
                Self::version_pragma_expression.parse(self, input, true)
            }
            RuleKind::VersionPragmaExpressions => {
                Self::version_pragma_expressions.parse(self, input, true)
            }
            RuleKind::VersionPragmaOrExpression => {
                Self::version_pragma_or_expression.parse(self, input, true)
            }
            RuleKind::VersionPragmaPrefixExpression => {
                Self::version_pragma_prefix_expression.parse(self, input, true)
            }
            RuleKind::VersionPragmaRangeExpression => {
                Self::version_pragma_range_expression.parse(self, input, true)
            }
            RuleKind::VersionPragmaSpecifier => {
                Self::version_pragma_specifier.parse(self, input, true)
            }
            RuleKind::WhileStatement => Self::while_statement.parse(self, input, true),
            RuleKind::YulArguments => Self::yul_arguments.parse(self, input, true),
            RuleKind::YulAssignmentStatement => {
                Self::yul_assignment_statement.parse(self, input, true)
            }
            RuleKind::YulBlock => Self::yul_block.parse(self, input, true),
            RuleKind::YulBreakStatement => Self::yul_break_statement.parse(self, input, true),
            RuleKind::YulBuiltInFunction => Self::yul_built_in_function.parse(self, input, true),
            RuleKind::YulContinueStatement => Self::yul_continue_statement.parse(self, input, true),
            RuleKind::YulDefaultCase => Self::yul_default_case.parse(self, input, true),
            RuleKind::YulExpression => Self::yul_expression.parse(self, input, true),
            RuleKind::YulForStatement => Self::yul_for_statement.parse(self, input, true),
            RuleKind::YulFunctionCallExpression => {
                Self::yul_function_call_expression.parse(self, input, true)
            }
            RuleKind::YulFunctionDefinition => {
                Self::yul_function_definition.parse(self, input, true)
            }
            RuleKind::YulIdentifierPath => Self::yul_identifier_path.parse(self, input, true),
            RuleKind::YulIdentifierPaths => Self::yul_identifier_paths.parse(self, input, true),
            RuleKind::YulIfStatement => Self::yul_if_statement.parse(self, input, true),
            RuleKind::YulLabel => Self::yul_label.parse(self, input, true),
            RuleKind::YulLeaveStatement => Self::yul_leave_statement.parse(self, input, true),
            RuleKind::YulLiteral => Self::yul_literal.parse(self, input, true),
            RuleKind::YulParameters => Self::yul_parameters.parse(self, input, true),
            RuleKind::YulParametersDeclaration => {
                Self::yul_parameters_declaration.parse(self, input, true)
            }
            RuleKind::YulReturnVariables => Self::yul_return_variables.parse(self, input, true),
            RuleKind::YulReturnsDeclaration => {
                Self::yul_returns_declaration.parse(self, input, true)
            }
            RuleKind::YulStatement => Self::yul_statement.parse(self, input, true),
            RuleKind::YulStatements => Self::yul_statements.parse(self, input, true),
            RuleKind::YulSwitchCase => Self::yul_switch_case.parse(self, input, true),
            RuleKind::YulSwitchCases => Self::yul_switch_cases.parse(self, input, true),
            RuleKind::YulSwitchStatement => Self::yul_switch_statement.parse(self, input, true),
            RuleKind::YulValueCase => Self::yul_value_case.parse(self, input, true),
            RuleKind::YulVariableDeclarationStatement => {
                Self::yul_variable_declaration_statement.parse(self, input, true)
            }
            RuleKind::YulVariableDeclarationValue => {
                Self::yul_variable_declaration_value.parse(self, input, true)
            }
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

    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TokenKind, TokenKind)] {
        match LexCtx::value() {
            LexicalContext::Default => &[
                (TokenKind::OpenBrace, TokenKind::CloseBrace),
                (TokenKind::OpenBracket, TokenKind::CloseBracket),
                (TokenKind::OpenParen, TokenKind::CloseParen),
            ],
            LexicalContext::Pragma => &[],
            LexicalContext::Yul => &[
                (TokenKind::OpenBrace, TokenKind::CloseBrace),
                (TokenKind::OpenParen, TokenKind::CloseParen),
            ],
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

                        longest_token = Some(TokenKind::$kind);
                    }
                    input.set_position(save);
                )*
            };
        }

        match LexCtx::value() {
            LexicalContext::Default => {
                if let Some(kind) = match input.next() {
                    Some('!') => match input.next() {
                        Some('=') => Some(TokenKind::BangEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Bang)
                        }
                        None => Some(TokenKind::Bang),
                    },
                    Some('%') => match input.next() {
                        Some('=') => Some(TokenKind::PercentEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Percent)
                        }
                        None => Some(TokenKind::Percent),
                    },
                    Some('&') => match input.next() {
                        Some('&') => Some(TokenKind::AmpersandAmpersand),
                        Some('=') => Some(TokenKind::AmpersandEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Ampersand)
                        }
                        None => Some(TokenKind::Ampersand),
                    },
                    Some('(') => Some(TokenKind::OpenParen),
                    Some(')') => Some(TokenKind::CloseParen),
                    Some('*') => match input.next() {
                        Some('*') => Some(TokenKind::AsteriskAsterisk),
                        Some('=') => Some(TokenKind::AsteriskEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Asterisk)
                        }
                        None => Some(TokenKind::Asterisk),
                    },
                    Some('+') => match input.next() {
                        Some('+') => Some(TokenKind::PlusPlus),
                        Some('=') => Some(TokenKind::PlusEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Plus)
                        }
                        None => Some(TokenKind::Plus),
                    },
                    Some(',') => Some(TokenKind::Comma),
                    Some('-') => match input.next() {
                        Some('-') => Some(TokenKind::MinusMinus),
                        Some('=') => Some(TokenKind::MinusEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Minus)
                        }
                        None => Some(TokenKind::Minus),
                    },
                    Some('.') => Some(TokenKind::Period),
                    Some('/') => {
                        if scan_chars!(input, '=') {
                            Some(TokenKind::SlashEqual)
                        } else {
                            None
                        }
                    }
                    Some(':') => Some(TokenKind::Colon),
                    Some(';') => Some(TokenKind::Semicolon),
                    Some('<') => match input.next() {
                        Some('<') => match input.next() {
                            Some('=') => Some(TokenKind::LessThanLessThanEqual),
                            Some(_) => {
                                input.undo();
                                Some(TokenKind::LessThanLessThan)
                            }
                            None => Some(TokenKind::LessThanLessThan),
                        },
                        Some('=') => Some(TokenKind::LessThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::LessThan)
                        }
                        None => Some(TokenKind::LessThan),
                    },
                    Some('=') => match input.next() {
                        Some('=') => Some(TokenKind::EqualEqual),
                        Some('>') => Some(TokenKind::EqualGreaterThan),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Equal)
                        }
                        None => Some(TokenKind::Equal),
                    },
                    Some('>') => match input.next() {
                        Some('=') => Some(TokenKind::GreaterThanEqual),
                        Some('>') => match input.next() {
                            Some('=') => Some(TokenKind::GreaterThanGreaterThanEqual),
                            Some('>') => match input.next() {
                                Some('=') => {
                                    Some(TokenKind::GreaterThanGreaterThanGreaterThanEqual)
                                }
                                Some(_) => {
                                    input.undo();
                                    Some(TokenKind::GreaterThanGreaterThanGreaterThan)
                                }
                                None => Some(TokenKind::GreaterThanGreaterThanGreaterThan),
                            },
                            Some(_) => {
                                input.undo();
                                Some(TokenKind::GreaterThanGreaterThan)
                            }
                            None => Some(TokenKind::GreaterThanGreaterThan),
                        },
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::GreaterThan)
                        }
                        None => Some(TokenKind::GreaterThan),
                    },
                    Some('?') => Some(TokenKind::QuestionMark),
                    Some('[') => Some(TokenKind::OpenBracket),
                    Some(']') => Some(TokenKind::CloseBracket),
                    Some('^') => match input.next() {
                        Some('=') => Some(TokenKind::CaretEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Caret)
                        }
                        None => Some(TokenKind::Caret),
                    },
                    Some('{') => Some(TokenKind::OpenBrace),
                    Some('|') => match input.next() {
                        Some('=') => Some(TokenKind::BarEqual),
                        Some('|') => Some(TokenKind::BarBar),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Bar)
                        }
                        None => Some(TokenKind::Bar),
                    },
                    Some('}') => Some(TokenKind::CloseBrace),
                    Some('~') => Some(TokenKind::Tilde),
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
                    { AsciiStringLiteral = ascii_string_literal }
                    { DecimalLiteral = decimal_literal }
                    { EndOfLine = end_of_line }
                    { HexLiteral = hex_literal }
                    { HexStringLiteral = hex_string_literal }
                    { MultiLineComment = multi_line_comment }
                    { MultiLineNatSpecComment = multi_line_nat_spec_comment }
                    { SingleLineComment = single_line_comment }
                    { SingleLineNatSpecComment = single_line_nat_spec_comment }
                    { Slash = slash }
                    { UnicodeStringLiteral = unicode_string_literal }
                    { Whitespace = whitespace }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_token.filter(|tok| [TokenKind::Identifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('a') => match input.next() {
                            Some('b') => {
                                if scan_chars!(input, 's', 't', 'r', 'a', 'c', 't') {
                                    KeywordScan::Reserved(TokenKind::AbstractKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('d') => {
                                if scan_chars!(input, 'd', 'r', 'e', 's', 's') {
                                    KeywordScan::Reserved(TokenKind::AddressKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    KeywordScan::Reserved(TokenKind::AfterKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::AliasKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                                    KeywordScan::Reserved(TokenKind::AnonymousKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::ApplyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        KeywordScan::Reserved(TokenKind::AssemblyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Reserved(TokenKind::AsKeyword)
                                }
                                None => KeywordScan::Reserved(TokenKind::AsKeyword),
                            },
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::AutoKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('b') => match input.next() {
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    KeywordScan::Reserved(TokenKind::BoolKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    KeywordScan::Reserved(TokenKind::BreakKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('y') => {
                                if scan_chars!(input, 't', 'e') {
                                    KeywordScan::Reserved(TokenKind::ByteKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('c') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l', 'd', 'a', 't', 'a') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::CallDataKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TokenKind::CaseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        KeywordScan::Reserved(TokenKind::CatchKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => match input.next() {
                                Some('n') => match input.next() {
                                    Some('s') => {
                                        if scan_chars!(input, 't') {
                                            match input.next() {
                                                Some('a') => {
                                                    if scan_chars!(input, 'n', 't') {
                                                        KeywordScan::Reserved(
                                                            TokenKind::ConstantKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some('r') => {
                                                    if scan_chars!(input, 'u', 'c', 't', 'o', 'r') {
                                                        if self.version_is_at_least_0_5_0 {
                                                            KeywordScan::Reserved(
                                                                TokenKind::ConstructorKeyword,
                                                            )
                                                        } else if self.version_is_at_least_0_4_22 {
                                                            KeywordScan::Present(
                                                                TokenKind::ConstructorKeyword,
                                                            )
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Absent
                                                }
                                                None => KeywordScan::Absent,
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => match input.next() {
                                        Some('i') => {
                                            if scan_chars!(input, 'n', 'u', 'e') {
                                                KeywordScan::Reserved(TokenKind::ContinueKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(input, 'a', 'c', 't') {
                                                KeywordScan::Reserved(TokenKind::ContractKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    },
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('p') => {
                                    if scan_chars!(input, 'y', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::CopyOfKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    KeywordScan::Reserved(TokenKind::DaysKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            KeywordScan::Reserved(TokenKind::DefaultKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(TokenKind::DefineKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e', 't', 'e') {
                                        KeywordScan::Reserved(TokenKind::DeleteKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => KeywordScan::Reserved(TokenKind::DoKeyword),
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('e') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 's', 'e') {
                                    KeywordScan::Reserved(TokenKind::ElseKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('m') => {
                                if scan_chars!(input, 'i', 't') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::EmitKeyword)
                                    } else if self.version_is_at_least_0_4_21 {
                                        KeywordScan::Present(TokenKind::EmitKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'u', 'm') {
                                    KeywordScan::Reserved(TokenKind::EnumKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'r', 'o', 'r') {
                                    if self.version_is_at_least_0_8_4 {
                                        KeywordScan::Present(TokenKind::ErrorKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => {
                                if scan_chars!(input, 'h', 'e', 'r') {
                                    KeywordScan::Reserved(TokenKind::EtherKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'n', 't') {
                                    KeywordScan::Reserved(TokenKind::EventKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('x') => {
                                if scan_chars!(input, 't', 'e', 'r', 'n', 'a', 'l') {
                                    KeywordScan::Reserved(TokenKind::ExternalKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::FallbackKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Present(TokenKind::FallbackKeyword)
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                KeywordScan::Reserved(TokenKind::FalseKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                KeywordScan::Reserved(TokenKind::FinalKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    KeywordScan::Reserved(TokenKind::FinneyKeyword)
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TokenKind::ForKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'o', 'm') {
                                    if true {
                                        KeywordScan::Present(TokenKind::FromKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    KeywordScan::Reserved(TokenKind::FunctionKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('g') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 'o', 'b', 'a', 'l') {
                                    if self.version_is_at_least_0_8_13 {
                                        KeywordScan::Present(TokenKind::GlobalKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'e', 'i') {
                                    if self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TokenKind::GweiKeyword)
                                    } else if self.version_is_at_least_0_6_11 {
                                        KeywordScan::Present(TokenKind::GweiKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    KeywordScan::Reserved(TokenKind::HexKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    KeywordScan::Reserved(TokenKind::HoursKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('i') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TokenKind::IfKeyword),
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::ImmutableKeyword)
                                        } else if self.version_is_at_least_0_6_5 {
                                            KeywordScan::Present(TokenKind::ImmutableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(TokenKind::ImplementsKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            KeywordScan::Reserved(TokenKind::ImportKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        KeywordScan::Reserved(TokenKind::IndexedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        KeywordScan::Reserved(TokenKind::InlineKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    KeywordScan::Reserved(
                                                        TokenKind::InterfaceKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    KeywordScan::Reserved(
                                                        TokenKind::InternalKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Reserved(TokenKind::InKeyword)
                                }
                                None => KeywordScan::Reserved(TokenKind::InKeyword),
                            },
                            Some('s') => KeywordScan::Reserved(TokenKind::IsKeyword),
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('l') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 't') {
                                    KeywordScan::Reserved(TokenKind::LetKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    KeywordScan::Reserved(TokenKind::LibraryKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::MacroKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        KeywordScan::Reserved(TokenKind::MappingKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        KeywordScan::Reserved(TokenKind::MatchKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    KeywordScan::Reserved(TokenKind::MemoryKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    KeywordScan::Reserved(TokenKind::MinutesKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r') {
                                    KeywordScan::Reserved(TokenKind::ModifierKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'a', 'b', 'l', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::MutableKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    KeywordScan::Reserved(TokenKind::NewKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'l', 'l') {
                                    KeywordScan::Reserved(TokenKind::NullKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('o') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TokenKind::OfKeyword),
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::OverrideKeyword)
                                    } else {
                                        KeywordScan::Present(TokenKind::OverrideKeyword)
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::PartialKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        KeywordScan::Reserved(TokenKind::PayableKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        KeywordScan::Reserved(TokenKind::PragmaKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        KeywordScan::Reserved(TokenKind::PrivateKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::PromiseKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        KeywordScan::Reserved(TokenKind::PublicKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TokenKind::PureKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0 {
                                                KeywordScan::Reserved(TokenKind::ReceiveKeyword)
                                            } else {
                                                KeywordScan::Present(TokenKind::ReceiveKeyword)
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                KeywordScan::Reserved(TokenKind::ReferenceKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            KeywordScan::Reserved(TokenKind::RelocatableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n') {
                                            match input.next() {
                                                Some('s') => {
                                                    KeywordScan::Reserved(TokenKind::ReturnsKeyword)
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Reserved(TokenKind::ReturnKeyword)
                                                }
                                                None => {
                                                    KeywordScan::Reserved(TokenKind::ReturnKeyword)
                                                }
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('v') => {
                                        if scan_chars!(input, 'e', 'r', 't') {
                                            if self.version_is_at_least_0_8_4 {
                                                KeywordScan::Present(TokenKind::RevertKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::SealedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        KeywordScan::Reserved(TokenKind::SecondsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'z', 'e', 'o', 'f') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::SizeOfKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        KeywordScan::Reserved(TokenKind::StaticKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'r', 'a', 'g', 'e') {
                                        KeywordScan::Reserved(TokenKind::StorageKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            KeywordScan::Reserved(TokenKind::StringKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            KeywordScan::Reserved(TokenKind::StructKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => {
                                if scan_chars!(input, 'p', 'p', 'o', 'r', 't', 's') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::SupportsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    KeywordScan::Reserved(TokenKind::SwitchKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TokenKind::SzaboKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('t') => match input.next() {
                            Some('h') => {
                                if scan_chars!(input, 'r', 'o', 'w') {
                                    KeywordScan::Reserved(TokenKind::ThrowKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => match input.next() {
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TokenKind::TrueKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => KeywordScan::Reserved(TokenKind::TryKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0 {
                                                    KeywordScan::Reserved(TokenKind::TypeDefKeyword)
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                KeywordScan::Reserved(TokenKind::TypeOfKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TokenKind::TypeKeyword)
                                        }
                                        None => KeywordScan::Reserved(TokenKind::TypeKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0 {
                                        KeywordScan::Reserved(TokenKind::UncheckedKeyword)
                                    } else if self.version_is_at_least_0_8_0 {
                                        KeywordScan::Present(TokenKind::UncheckedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    KeywordScan::Reserved(TokenKind::UsingKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('v') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TokenKind::VarKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        KeywordScan::Reserved(TokenKind::ViewKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0 {
                                            KeywordScan::Reserved(TokenKind::VirtualKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        KeywordScan::Reserved(TokenKind::WeeksKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => KeywordScan::Reserved(TokenKind::WeiKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    KeywordScan::Reserved(TokenKind::WhileKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                KeywordScan::Reserved(TokenKind::YearsKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    // Perf: only scan for a compound keyword if we didn't already find one
                    let mut kw_scan = kw_scan;
                    if kw_scan == KeywordScan::Absent {
                        input.set_position(save);

                        // TODO: Don't allocate a string here
                        let ident_value = input.content(save.utf8..furthest_position.utf8);

                        for keyword_compound_scanner in [
                            Self::bytes_keyword,
                            Self::fixed_keyword,
                            Self::int_keyword,
                            Self::ufixed_keyword,
                            Self::uint_keyword,
                        ] {
                            match keyword_compound_scanner(self, input, &ident_value) {
                                _ if input.position() < furthest_position => { /* Strict prefix */ }
                                KeywordScan::Absent => {}
                                value => kw_scan = value,
                            }
                            input.set_position(save);
                        }
                    }

                    input.set_position(furthest_position);
                    return Some(ScannedToken::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            LexicalContext::Pragma => {
                if let Some(kind) = match input.next() {
                    Some('-') => Some(TokenKind::Minus),
                    Some('.') => Some(TokenKind::Period),
                    Some(';') => Some(TokenKind::Semicolon),
                    Some('<') => match input.next() {
                        Some('=') => Some(TokenKind::LessThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::LessThan)
                        }
                        None => Some(TokenKind::LessThan),
                    },
                    Some('=') => Some(TokenKind::Equal),
                    Some('>') => match input.next() {
                        Some('=') => Some(TokenKind::GreaterThanEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::GreaterThan)
                        }
                        None => Some(TokenKind::GreaterThan),
                    },
                    Some('^') => Some(TokenKind::Caret),
                    Some('|') => {
                        if scan_chars!(input, '|') {
                            Some(TokenKind::BarBar)
                        } else {
                            None
                        }
                    }
                    Some('~') => Some(TokenKind::Tilde),
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
                    { AsciiStringLiteral = ascii_string_literal }
                    { VersionPragmaValue = version_pragma_value }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_token.filter(|tok| [TokenKind::Identifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('a') => {
                            if scan_chars!(input, 'b', 'i', 'c', 'o', 'd', 'e', 'r') {
                                KeywordScan::Reserved(TokenKind::AbicoderKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('e') => {
                            if scan_chars!(
                                input, 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'
                            ) {
                                KeywordScan::Reserved(TokenKind::ExperimentalKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('p') => {
                            if scan_chars!(input, 'r', 'a', 'g', 'm', 'a') {
                                KeywordScan::Reserved(TokenKind::PragmaKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => {
                            if scan_chars!(input, 'o', 'l', 'i', 'd', 'i', 't', 'y') {
                                KeywordScan::Reserved(TokenKind::SolidityKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
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
            LexicalContext::Yul => {
                if let Some(kind) = match input.next() {
                    Some('(') => Some(TokenKind::OpenParen),
                    Some(')') => Some(TokenKind::CloseParen),
                    Some(',') => Some(TokenKind::Comma),
                    Some('-') => {
                        if scan_chars!(input, '>') {
                            Some(TokenKind::MinusGreaterThan)
                        } else {
                            None
                        }
                    }
                    Some('.') => Some(TokenKind::Period),
                    Some(':') => match input.next() {
                        Some('=') => Some(TokenKind::ColonEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Colon)
                        }
                        None => Some(TokenKind::Colon),
                    },
                    Some('{') => Some(TokenKind::OpenBrace),
                    Some('}') => Some(TokenKind::CloseBrace),
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
                    { AsciiStringLiteral = ascii_string_literal }
                    { HexStringLiteral = hex_string_literal }
                    { YulDecimalLiteral = yul_decimal_literal }
                    { YulHexLiteral = yul_hex_literal }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { YulIdentifier = yul_identifier }
                }

                // We have an identifier; we need to check if it's a keyword
                if let Some(identifier) =
                    longest_token.filter(|tok| [TokenKind::YulIdentifier].contains(tok))
                {
                    let kw_scan = match input.next() {
                        Some('a') => match input.next() {
                            Some('b') => {
                                if scan_chars!(input, 's', 't', 'r', 'a', 'c', 't') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulAbstractKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('d') => {
                                if scan_chars!(input, 'd') {
                                    match input.next() {
                                        Some('m') => {
                                            if scan_chars!(input, 'o', 'd') {
                                                KeywordScan::Reserved(TokenKind::YulAddModKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(input, 'e', 's', 's') {
                                                KeywordScan::Reserved(TokenKind::YulAddressKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TokenKind::YulAddKeyword)
                                        }
                                        None => KeywordScan::Reserved(TokenKind::YulAddKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulAfterKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulAliasKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('n') => match input.next() {
                                Some('d') => KeywordScan::Reserved(TokenKind::YulAndKeyword),
                                Some('o') => {
                                    if scan_chars!(input, 'n', 'y', 'm', 'o', 'u', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulAnonymousKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulApplyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulAssemblyKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulAsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulAsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                            },
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulAutoKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('b') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'a', 'n', 'c', 'e') {
                                        KeywordScan::Reserved(TokenKind::YulBalanceKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'f', 'e', 'e') {
                                        if self.version_is_at_least_0_8_7 {
                                            KeywordScan::Reserved(TokenKind::YulBaseFeeKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('l') => {
                                if scan_chars!(input, 'o', 'c', 'k', 'h', 'a', 's', 'h') {
                                    KeywordScan::Reserved(TokenKind::YulBlockHashKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    if !self.version_is_at_least_0_5_10 {
                                        KeywordScan::Reserved(TokenKind::YulBoolKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    KeywordScan::Reserved(TokenKind::YulBreakKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('y') => {
                                if scan_chars!(input, 't', 'e') {
                                    KeywordScan::Reserved(TokenKind::YulByteKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('c') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l') {
                                        match input.next() {
                                            Some('c') => {
                                                if scan_chars!(input, 'o', 'd', 'e') {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulCallCodeKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('d') => {
                                                if scan_chars!(input, 'a', 't', 'a') {
                                                    match input.next() {
                                                        Some('c') => {
                                                            if scan_chars!(input, 'o', 'p', 'y') {
                                                                KeywordScan :: Reserved (TokenKind :: YulCallDataCopyKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some('l') => {
                                                            if scan_chars!(input, 'o', 'a', 'd') {
                                                                KeywordScan :: Reserved (TokenKind :: YulCallDataLoadKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some('s') => {
                                                            if scan_chars!(input, 'i', 'z', 'e') {
                                                                KeywordScan :: Reserved (TokenKind :: YulCallDataSizeKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some(_) => {
                                                            input.undo();
                                                            if self.version_is_at_least_0_5_0
                                                                && !self.version_is_at_least_0_7_1
                                                            {
                                                                KeywordScan::Reserved(
                                                                    TokenKind::YulCallDataKeyword,
                                                                )
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        None => {
                                                            if self.version_is_at_least_0_5_0
                                                                && !self.version_is_at_least_0_7_1
                                                            {
                                                                KeywordScan::Reserved(
                                                                    TokenKind::YulCallDataKeyword,
                                                                )
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('e') => {
                                                if scan_chars!(input, 'r') {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulCallerKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('v') => {
                                                if scan_chars!(input, 'a', 'l', 'u', 'e') {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulCallValueKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Reserved(TokenKind::YulCallKeyword)
                                            }
                                            None => {
                                                KeywordScan::Reserved(TokenKind::YulCallKeyword)
                                            }
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TokenKind::YulCaseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulCatchKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'a', 'i', 'n', 'i', 'd') {
                                    if self.version_is_at_least_0_5_12 {
                                        KeywordScan::Reserved(TokenKind::YulChainIdKeyword)
                                    } else {
                                        KeywordScan::Present(TokenKind::YulChainIdKeyword)
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 'n', 'b', 'a', 's', 'e') {
                                        KeywordScan::Reserved(TokenKind::YulCoinBaseKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('n') => match input.next() {
                                    Some('s') => {
                                        if scan_chars!(input, 't') {
                                            match input.next() {
                                                Some('a') => {
                                                    if scan_chars!(input, 'n', 't') {
                                                        if !self.version_is_at_least_0_7_1 {
                                                            KeywordScan::Reserved(
                                                                TokenKind::YulConstantKeyword,
                                                            )
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some('r') => {
                                                    if scan_chars!(input, 'u', 'c', 't', 'o', 'r') {
                                                        if self.version_is_at_least_0_5_0
                                                            && !self.version_is_at_least_0_7_1
                                                        {
                                                            KeywordScan::Reserved(
                                                                TokenKind::YulConstructorKeyword,
                                                            )
                                                        } else {
                                                            KeywordScan::Absent
                                                        }
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Absent
                                                }
                                                None => KeywordScan::Absent,
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => match input.next() {
                                        Some('i') => {
                                            if scan_chars!(input, 'n', 'u', 'e') {
                                                KeywordScan::Reserved(TokenKind::YulContinueKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(input, 'a', 'c', 't') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulContractKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    },
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('p') => {
                                    if scan_chars!(input, 'y', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulCopyOfKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 't', 'e') {
                                    match input.next() {
                                        Some('2') => {
                                            if self.version_is_at_least_0_4_12 {
                                                KeywordScan::Reserved(TokenKind::YulCreate2Keyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TokenKind::YulCreateKeyword)
                                        }
                                        None => KeywordScan::Reserved(TokenKind::YulCreateKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulDaysKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            KeywordScan::Reserved(TokenKind::YulDefaultKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(TokenKind::YulDefineKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e') {
                                        match input.next() {
                                            Some('g') => {
                                                if scan_chars!(
                                                    input, 'a', 't', 'e', 'c', 'a', 'l', 'l'
                                                ) {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulDelegateCallKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('t') => {
                                                if scan_chars!(input, 'e') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulDeleteKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => match input.next() {
                                Some('f') => {
                                    if scan_chars!(input, 'f', 'i', 'c', 'u', 'l', 't', 'y') {
                                        KeywordScan::Reserved(TokenKind::YulDifficultyKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('v') => KeywordScan::Reserved(TokenKind::YulDivKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TokenKind::YulDoKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('e') => {
                            match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 's', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulElseKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('m') => {
                                    if scan_chars!(input, 'i', 't') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulEmitKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('n') => {
                                    if scan_chars!(input, 'u', 'm') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulEnumKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('q') => KeywordScan::Reserved(TokenKind::YulEqKeyword),
                                Some('t') => {
                                    if scan_chars!(input, 'h', 'e', 'r') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulEtherKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('v') => {
                                    if scan_chars!(input, 'e', 'n', 't') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulEventKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('x') => match input.next() {
                                    Some('p') => KeywordScan::Reserved(TokenKind::YulExpKeyword),
                                    Some('t') => {
                                        match input.next() {
                                            Some('c') => {
                                                if scan_chars!(input, 'o', 'd', 'e') {
                                                    match input.next() {
                                                        Some('c') => {
                                                            if scan_chars!(input, 'o', 'p', 'y') {
                                                                KeywordScan :: Reserved (TokenKind :: YulExtCodeCopyKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some('h') => {
                                                            if scan_chars!(input, 'a', 's', 'h') {
                                                                if self.version_is_at_least_0_5_0 {
                                                                    KeywordScan :: Reserved (TokenKind :: YulExtCodeHashKeyword)
                                                                } else {
                                                                    KeywordScan::Absent
                                                                }
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some('s') => {
                                                            if scan_chars!(input, 'i', 'z', 'e') {
                                                                KeywordScan :: Reserved (TokenKind :: YulExtCodeSizeKeyword)
                                                            } else {
                                                                KeywordScan::Absent
                                                            }
                                                        }
                                                        Some(_) => {
                                                            input.undo();
                                                            KeywordScan::Absent
                                                        }
                                                        None => KeywordScan::Absent,
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('e') => {
                                                if scan_chars!(input, 'r', 'n', 'a', 'l') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulExternalKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            }
                        }
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulFallbackKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                KeywordScan::Reserved(TokenKind::YulFalseKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulFinalKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulFinneyKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    KeywordScan::Reserved(TokenKind::YulForKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    KeywordScan::Reserved(TokenKind::YulFunctionKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('g') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 's') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'i', 'm', 'i', 't') {
                                                KeywordScan::Reserved(TokenKind::YulGasLimitKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('p') => {
                                            if scan_chars!(input, 'r', 'i', 'c', 'e') {
                                                KeywordScan::Reserved(TokenKind::YulGasPriceKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TokenKind::YulGasKeyword)
                                        }
                                        None => KeywordScan::Reserved(TokenKind::YulGasKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => KeywordScan::Reserved(TokenKind::YulGtKeyword),
                            Some('w') => {
                                if scan_chars!(input, 'e', 'i') {
                                    if self.version_is_at_least_0_7_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulGweiKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    KeywordScan::Reserved(TokenKind::YulHexKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulHoursKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('i') => match input.next() {
                            Some('f') => KeywordScan::Reserved(TokenKind::YulIfKeyword),
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulImmutableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TokenKind::YulImplementsKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulImportKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulIndexedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulInlineKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulInterfaceKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulInternalKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('v') => {
                                    if scan_chars!(input, 'a', 'l', 'i', 'd') {
                                        KeywordScan::Reserved(TokenKind::YulInvalidKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_6_8 {
                                        KeywordScan::Reserved(TokenKind::YulInKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_6_8 {
                                        KeywordScan::Reserved(TokenKind::YulInKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                            },
                            Some('s') => match input.next() {
                                Some('z') => {
                                    if scan_chars!(input, 'e', 'r', 'o') {
                                        KeywordScan::Reserved(TokenKind::YulIsZeroKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulIsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulIsKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('k') => {
                            if scan_chars!(input, 'e', 'c', 'c', 'a', 'k', '2', '5', '6') {
                                if self.version_is_at_least_0_4_12 {
                                    KeywordScan::Reserved(TokenKind::YulKeccak256Keyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('l') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'v', 'e') {
                                        if self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulLeaveKeyword)
                                        } else if self.version_is_at_least_0_6_0 {
                                            KeywordScan::Present(TokenKind::YulLeaveKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => KeywordScan::Reserved(TokenKind::YulLetKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulLibraryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'g') {
                                    match input.next() {
                                        Some('0') => {
                                            KeywordScan::Reserved(TokenKind::YulLog0Keyword)
                                        }
                                        Some('1') => {
                                            KeywordScan::Reserved(TokenKind::YulLog1Keyword)
                                        }
                                        Some('2') => {
                                            KeywordScan::Reserved(TokenKind::YulLog2Keyword)
                                        }
                                        Some('3') => {
                                            KeywordScan::Reserved(TokenKind::YulLog3Keyword)
                                        }
                                        Some('4') => {
                                            KeywordScan::Reserved(TokenKind::YulLog4Keyword)
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Absent
                                        }
                                        None => KeywordScan::Absent,
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => KeywordScan::Reserved(TokenKind::YulLtKeyword),
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulMacroKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulMappingKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulMatchKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulMemoryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulMinutesKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'o', 'a', 'd') {
                                    KeywordScan::Reserved(TokenKind::YulMLoadKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd') {
                                    match input.next() {
                                        Some('i') => {
                                            if scan_chars!(input, 'f', 'i', 'e', 'r') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulModifierKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            KeywordScan::Reserved(TokenKind::YulModKeyword)
                                        }
                                        None => KeywordScan::Reserved(TokenKind::YulModKeyword),
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 'z', 'e') {
                                        KeywordScan::Reserved(TokenKind::YulMSizeKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'o', 'r', 'e') {
                                        match input.next() {
                                            Some('8') => {
                                                KeywordScan::Reserved(TokenKind::YulMStore8Keyword)
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Reserved(TokenKind::YulMStoreKeyword)
                                            }
                                            None => {
                                                KeywordScan::Reserved(TokenKind::YulMStoreKeyword)
                                            }
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('l') => match input.next() {
                                    Some('m') => {
                                        if scan_chars!(input, 'o', 'd') {
                                            KeywordScan::Reserved(TokenKind::YulMulModKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Reserved(TokenKind::YulMulKeyword)
                                    }
                                    None => KeywordScan::Reserved(TokenKind::YulMulKeyword),
                                },
                                Some('t') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulMutableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulNewKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 't') {
                                    KeywordScan::Reserved(TokenKind::YulNotKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('u') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulNullKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('m') => {
                                    if scan_chars!(input, 'b', 'e', 'r') {
                                        KeywordScan::Reserved(TokenKind::YulNumberKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('o') => match input.next() {
                            Some('f') => {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TokenKind::YulOfKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => match input.next() {
                                Some('i') => {
                                    if scan_chars!(input, 'g', 'i', 'n') {
                                        KeywordScan::Reserved(TokenKind::YulOriginKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Reserved(TokenKind::YulOrKeyword)
                                }
                                None => KeywordScan::Reserved(TokenKind::YulOrKeyword),
                            },
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulOverrideKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulPartialKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulPayableKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('o') => {
                                if scan_chars!(input, 'p') {
                                    KeywordScan::Reserved(TokenKind::YulPopKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulPragmaKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('e') => {
                                    if scan_chars!(input, 'v', 'r', 'a', 'n', 'd', 'a', 'o') {
                                        if self.version_is_at_least_0_8_18 {
                                            KeywordScan::Reserved(TokenKind::YulPrevRandaoKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulPrivateKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulPromiseKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulPublicKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulPureKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(TokenKind::YulReceiveKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                KeywordScan::Reserved(
                                                    TokenKind::YulReferenceKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(
                                                    TokenKind::YulRelocatableKeyword,
                                                )
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n') {
                                            match input.next() {
                                                Some('d') => {
                                                    if scan_chars!(input, 'a', 't', 'a') {
                                                        match input.next() {
                                                            Some('c') => {
                                                                if scan_chars!(input, 'o', 'p', 'y')
                                                                {
                                                                    if self
                                                                        .version_is_at_least_0_4_12
                                                                    {
                                                                        KeywordScan :: Reserved (TokenKind :: YulReturnDataCopyKeyword)
                                                                    } else {
                                                                        KeywordScan::Absent
                                                                    }
                                                                } else {
                                                                    KeywordScan::Absent
                                                                }
                                                            }
                                                            Some('s') => {
                                                                if scan_chars!(input, 'i', 'z', 'e')
                                                                {
                                                                    if self
                                                                        .version_is_at_least_0_4_12
                                                                    {
                                                                        KeywordScan :: Reserved (TokenKind :: YulReturnDataSizeKeyword)
                                                                    } else {
                                                                        KeywordScan::Absent
                                                                    }
                                                                } else {
                                                                    KeywordScan::Absent
                                                                }
                                                            }
                                                            Some(_) => {
                                                                input.undo();
                                                                KeywordScan::Absent
                                                            }
                                                            None => KeywordScan::Absent,
                                                        }
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some('s') => {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulReturnsKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulReturnKeyword,
                                                    )
                                                }
                                                None => KeywordScan::Reserved(
                                                    TokenKind::YulReturnKeyword,
                                                ),
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('v') => {
                                        if scan_chars!(input, 'e', 'r', 't') {
                                            KeywordScan::Reserved(TokenKind::YulRevertKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('s') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    if self.version_is_at_least_0_4_21 {
                                        KeywordScan::Reserved(TokenKind::YulSarKeyword)
                                    } else {
                                        KeywordScan::Present(TokenKind::YulSarKeyword)
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('d') => {
                                if scan_chars!(input, 'i', 'v') {
                                    KeywordScan::Reserved(TokenKind::YulSDivKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulSealedKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulSecondsKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'f') {
                                        match input.next() {
                                            Some('b') => {
                                                if scan_chars!(input, 'a', 'l', 'a', 'n', 'c', 'e')
                                                {
                                                    if self.version_is_at_least_0_5_12 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulSelfBalanceKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Present(
                                                            TokenKind::YulSelfBalanceKeyword,
                                                        )
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some('d') => {
                                                if scan_chars!(
                                                    input, 'e', 's', 't', 'r', 'u', 'c', 't'
                                                ) {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulSelfDestructKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                KeywordScan::Absent
                                            }
                                            None => KeywordScan::Absent,
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('g') => {
                                if scan_chars!(input, 't') {
                                    KeywordScan::Reserved(TokenKind::YulSgtKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('h') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, '3') {
                                        if !self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::YulSha3Keyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('l') => {
                                    if self.version_is_at_least_0_4_21 {
                                        KeywordScan::Reserved(TokenKind::YulShlKeyword)
                                    } else {
                                        KeywordScan::Present(TokenKind::YulShlKeyword)
                                    }
                                }
                                Some('r') => {
                                    if self.version_is_at_least_0_4_21 {
                                        KeywordScan::Reserved(TokenKind::YulShrKeyword)
                                    } else {
                                        KeywordScan::Present(TokenKind::YulShrKeyword)
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('i') => match input.next() {
                                Some('g') => {
                                    if scan_chars!(input, 'n', 'e', 'x', 't', 'e', 'n', 'd') {
                                        KeywordScan::Reserved(TokenKind::YulSignExtendKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('z') => {
                                    if scan_chars!(input, 'e', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulSizeOfKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('l') => match input.next() {
                                Some('o') => {
                                    if scan_chars!(input, 'a', 'd') {
                                        KeywordScan::Reserved(TokenKind::YulSLoadKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('t') => KeywordScan::Reserved(TokenKind::YulSltKeyword),
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('m') => {
                                if scan_chars!(input, 'o', 'd') {
                                    KeywordScan::Reserved(TokenKind::YulSModKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 't', 'o', 'r', 'e') {
                                    KeywordScan::Reserved(TokenKind::YulSStoreKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        match input.next() {
                                            Some('c') => {
                                                if scan_chars!(input, 'a', 'l', 'l') {
                                                    if self.version_is_at_least_0_4_12 {
                                                        KeywordScan::Reserved(
                                                            TokenKind::YulStaticCallKeyword,
                                                        )
                                                    } else {
                                                        KeywordScan::Absent
                                                    }
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulStaticKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                            None => {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulStaticKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            }
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('o') => match input.next() {
                                    Some('p') => KeywordScan::Reserved(TokenKind::YulStopKeyword),
                                    Some('r') => {
                                        if scan_chars!(input, 'a', 'g', 'e') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulStorageKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulStringKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulStructKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        KeywordScan::Absent
                                    }
                                    None => KeywordScan::Absent,
                                },
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('u') => match input.next() {
                                Some('b') => KeywordScan::Reserved(TokenKind::YulSubKeyword),
                                Some('i') => {
                                    if scan_chars!(input, 'c', 'i', 'd', 'e') {
                                        if !self.version_is_at_least_0_5_0 {
                                            KeywordScan::Reserved(TokenKind::YulSuicideKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'o', 'r', 't', 's') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulSupportsKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    KeywordScan::Reserved(TokenKind::YulSwitchKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        KeywordScan::Reserved(TokenKind::YulSzaboKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('t') => match input.next() {
                            Some('h') => {
                                if scan_chars!(input, 'r', 'o', 'w') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulThrowKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'm', 'e', 's', 't', 'a', 'm', 'p') {
                                    KeywordScan::Reserved(TokenKind::YulTimestampKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('r') => match input.next() {
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        KeywordScan::Reserved(TokenKind::YulTrueKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('y') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulTryKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulTypeDefKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    KeywordScan::Reserved(
                                                        TokenKind::YulTypeOfKeyword,
                                                    )
                                                } else {
                                                    KeywordScan::Absent
                                                }
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulTypeKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                        None => {
                                            if !self.version_is_at_least_0_7_1 {
                                                KeywordScan::Reserved(TokenKind::YulTypeKeyword)
                                            } else {
                                                KeywordScan::Absent
                                            }
                                        }
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        KeywordScan::Reserved(TokenKind::YulUncheckedKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulUsingKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('v') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    if !self.version_is_at_least_0_6_5 {
                                        KeywordScan::Reserved(TokenKind::YulVarKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulViewKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            KeywordScan::Reserved(TokenKind::YulVirtualKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            KeywordScan::Reserved(TokenKind::YulWeeksKeyword)
                                        } else {
                                            KeywordScan::Absent
                                        }
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some('i') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulWeiKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    KeywordScan::Absent
                                }
                                None => KeywordScan::Absent,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    if !self.version_is_at_least_0_7_1 {
                                        KeywordScan::Reserved(TokenKind::YulWhileKeyword)
                                    } else {
                                        KeywordScan::Absent
                                    }
                                } else {
                                    KeywordScan::Absent
                                }
                            }
                            Some(_) => {
                                input.undo();
                                KeywordScan::Absent
                            }
                            None => KeywordScan::Absent,
                        },
                        Some('x') => {
                            if scan_chars!(input, 'o', 'r') {
                                KeywordScan::Reserved(TokenKind::YulXorKeyword)
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                if !self.version_is_at_least_0_7_1 {
                                    KeywordScan::Reserved(TokenKind::YulYearsKeyword)
                                } else {
                                    KeywordScan::Absent
                                }
                            } else {
                                KeywordScan::Absent
                            }
                        }
                        Some(_) => {
                            input.undo();
                            KeywordScan::Absent
                        }
                        None => KeywordScan::Absent,
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        _ if input.position() < furthest_position => KeywordScan::Absent,
                        value => value,
                    };

                    // Perf: only scan for a compound keyword if we didn't already find one
                    let mut kw_scan = kw_scan;
                    if kw_scan == KeywordScan::Absent {
                        input.set_position(save);

                        // TODO: Don't allocate a string here
                        let ident_value = input.content(save.utf8..furthest_position.utf8);

                        for keyword_compound_scanner in [
                            Self::yul_bytes_keyword,
                            Self::yul_fixed_keyword,
                            Self::yul_int_keyword,
                            Self::yul_ufixed_keyword,
                            Self::yul_uint_keyword,
                        ] {
                            match keyword_compound_scanner(self, input, &ident_value) {
                                _ if input.position() < furthest_position => { /* Strict prefix */ }
                                KeywordScan::Absent => {}
                                value => kw_scan = value,
                            }
                            input.set_position(save);
                        }
                    }

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
                Some(ScannedToken::Single(TokenKind::SKIPPED))
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
        #[napi(ts_arg_type = "kinds.RuleKind")] kind: RuleKind,
        input: String,
    ) -> NAPIParseOutput {
        self.parse(kind, input.as_str()).into()
    }
}
