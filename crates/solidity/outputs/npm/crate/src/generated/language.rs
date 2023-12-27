// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated; we can't reasonably satisfy some of these lints.
#![allow(
    clippy::if_not_else,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::struct_excessive_bools,
    clippy::similar_names
)]

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;
use semver::Version;

use crate::cst;
pub use crate::kinds::LexicalContext;
use crate::kinds::{IsLexicalContext, LexicalContextType, RuleKind, TokenKind};
use crate::lexer::{KeywordScan, Lexer};
#[cfg(feature = "slang_napi_interfaces")]
use crate::napi::napi_parse_output::ParseOutput as NAPIParseOutput;
use crate::parse_output::ParseOutput;
use crate::support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, RecoverFromNoMatch, SeparatedHelper, SequenceHelper, ZeroOrMoreHelper,
};

#[derive(Debug)]
#[cfg_attr(feature = "slang_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_at_least_0_0_0: bool,
    pub(crate) version_is_at_least_0_4_14: bool,
    pub(crate) version_is_at_least_0_4_21: bool,
    pub(crate) version_is_at_least_0_4_22: bool,
    pub(crate) version_is_at_least_0_5_0: bool,
    pub(crate) version_is_at_least_0_5_3: bool,
    pub(crate) version_is_at_least_0_5_10: bool,
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
                version_is_at_least_0_0_0: Version::new(0, 0, 0) <= version,
                version_is_at_least_0_4_14: Version::new(0, 4, 14) <= version,
                version_is_at_least_0_4_21: Version::new(0, 4, 21) <= version,
                version_is_at_least_0_4_22: Version::new(0, 4, 22) <= version,
                version_is_at_least_0_5_0: Version::new(0, 5, 0) <= version,
                version_is_at_least_0_5_3: Version::new(0, 5, 3) <= version,
                version_is_at_least_0_5_10: Version::new(0, 5, 10) <= version,
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
                "abicoder_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::AbicoderKeyword,
                ),
            )?;
            seq.elem_named(
                "version",
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
                "address_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AddressKeyword,
                ),
            )?;
            seq.elem_named(
                "payable_keyword",
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
        .with_name("variant")
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_bracket",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBracket,
                ),
            )?;
            seq.elem(
                self.array_values(input)
                    .with_name("items")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBracket,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_bracket",
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
            |input| self.expression(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::ArrayValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::AsciiStringLiteral,
            )
            .with_name("item")
        })
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
                .with_name("item")
            },
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::AssemblyFlags)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.assembly_flags(input)
                    .with_name("flags")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
                "assembly_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AssemblyKeyword,
                ),
            )?;
            seq.elem_named(
                "label",
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::AsciiStringLiteral,
                    ),
                ),
            )?;
            seq.elem_named(
                "flags",
                OptionalHelper::transform(self.assembly_flags_declaration(input)),
            )?;
            seq.elem_named("body", self.yul_block(input))?;
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
                "open_brace",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.statements(input))
                    .with_name("statements")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_brace",
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
                .with_name("break_keyword")
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                "semicolon",
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
                    "catch_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CatchKeyword,
                    ),
                )?;
                seq.elem_named(
                    "error",
                    OptionalHelper::transform(self.catch_clause_error(input)),
                )?;
                seq.elem_named("body", self.block(input))?;
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
                    "name",
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    ),
                )?;
                seq.elem_named("parameters", self.parameters_declaration(input))?;
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
            OneOrMoreHelper::run(input, |input| self.catch_clause(input).with_name("item"))
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
                        seq.elem_named("type_name", self.type_name(input))?;
                        seq.elem_named(
                            "constant_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::ConstantKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            "name",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named(
                            "equal",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Equal,
                            ),
                        )?;
                        seq.elem_named("value", self.expression(input))?;
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
                    "semicolon",
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
            .with_name("variant")
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            OneOrMoreHelper::run(input, |input| {
                self.constructor_attribute(input).with_name("item")
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
                    "constructor_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::ConstructorKeyword,
                    ),
                )?;
                seq.elem_named("parameters", self.parameters_declaration(input))?;
                seq.elem_named(
                    "attributes",
                    OptionalHelper::transform(self.constructor_attributes(input)),
                )?;
                seq.elem_named("body", self.block(input))?;
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
                .with_name("continue_keyword")
                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                "semicolon",
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
                    "abstract_keyword",
                    OptionalHelper::transform(
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AbstractKeyword,
                        ),
                    ),
                )?;
            }
            seq.elem_named(
                "contract_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ContractKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                "inheritence",
                OptionalHelper::transform(self.inheritance_specifier(input)),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.contract_members(input))
                        .with_name("members")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
        .with_name("variant")
        .with_kind(RuleKind::ContractMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.contract_member(input).with_name("item"))
            .with_kind(RuleKind::ContractMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "literal",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::DecimalLiteral,
                ),
            )?;
            seq.elem_named("unit", OptionalHelper::transform(self.number_unit(input)))?;
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
                        "delete_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::DeleteKeyword,
                        ),
                    )?;
                    seq.elem_named("expression", self.expression(input))?;
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
                "semicolon",
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
                        "do_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::DoKeyword,
                        ),
                    )?;
                    seq.elem_named("body", self.statement(input))?;
                    seq.elem_named(
                        "while_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::WhileKeyword,
                        ),
                    )?;
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem_named(
                            "open_paren",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::OpenParen,
                            ),
                        )?;
                        seq.elem(
                            self.expression(input)
                                .with_name("condition")
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                input,
                                self,
                                TokenKind::CloseParen,
                                RecoverFromNoMatch::Yes,
                            ),
                        )?;
                        seq.elem_named(
                            "close_paren",
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
                "semicolon",
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
                TokenKind::PayableKeyword,
            );
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
        .with_name("variant")
        .with_kind(RuleKind::ElementaryType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn else_branch(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "else_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ElseKeyword,
                ),
            )?;
            seq.elem_named("body", self.statement(input))?;
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
                            "emit_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::EmitKeyword,
                            ),
                        )?;
                        seq.elem_named("event", self.identifier_path(input))?;
                        seq.elem_named("arguments", self.arguments_declaration(input))?;
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
                    "semicolon",
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
    fn end_of_file_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result =
                    self.parse_token::<LexicalContextType::Default>(input, TokenKind::Whitespace);
                choice.consider(input, result)?;
                let result =
                    self.parse_token::<LexicalContextType::Default>(input, TokenKind::EndOfLine);
                choice.consider(input, result)?;
                let result = self
                    .parse_token::<LexicalContextType::Default>(input, TokenKind::MultilineComment);
                choice.consider(input, result)?;
                let result = self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::SingleLineComment,
                );
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "enum_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::EnumKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.enum_members(input))
                        .with_name("members")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
                .with_name("item")
            },
            TokenKind::Comma,
            "separator",
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
                            "error_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::ErrorKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            "name",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named("members", self.error_parameters_declaration(input))?;
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
                    "semicolon",
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
                seq.elem_named("type_name", self.type_name(input))?;
                seq.elem_named(
                    "name",
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
                |input| self.error_parameter(input).with_name("item"),
                TokenKind::Comma,
                "separator",
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
                    "open_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.error_parameters(input))
                        .with_name("parameters")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseParen,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_paren",
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
                        "event_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::EventKeyword,
                        ),
                    )?;
                    seq.elem_named(
                        "name",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named("parameters", self.event_parameters_declaration(input))?;
                    seq.elem_named(
                        "anonymous_keyword",
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
                "semicolon",
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
            seq.elem_named("type_name", self.type_name(input))?;
            seq.elem_named(
                "indexed_keyword",
                OptionalHelper::transform(
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::IndexedKeyword,
                    ),
                ),
            )?;
            seq.elem_named(
                "name",
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
            |input| self.event_parameter(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::EventParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.event_parameters(input))
                    .with_name("parameters")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
        .with_name("variant")
        .with_kind(RuleKind::ExperimentalFeature)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "experimental_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::ExperimentalKeyword,
                ),
            )?;
            seq.elem_named("feature", self.experimental_feature(input))?;
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::BarEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PlusEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::CaretEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::SlashEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::PercentEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AsteriskEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::AmpersandEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThanLessThanEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                        )
                        .with_name("operator");
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
                        "question_mark",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::QuestionMark,
                        ),
                    )?;
                    seq.elem_named("true_expression", self.expression(input))?;
                    seq.elem_named(
                        "colon",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Colon,
                        ),
                    )?;
                    seq.elem_named("false_expression", self.expression(input))?;
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
                .with_name("operator"),
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
                .with_name("operator"),
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::BangEqual,
                        )
                        .with_name("operator");
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThan,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::LessThanEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanEqual,
                        )
                        .with_name("operator");
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
                    .with_name("operator"),
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
                .with_name("operator"),
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
                .with_name("operator"),
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThan,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::GreaterThanGreaterThanGreaterThan,
                        )
                        .with_name("operator");
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Minus,
                        )
                        .with_name("operator");
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Slash,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Percent,
                        )
                        .with_name("operator");
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
                            .with_name("operator");
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
                            .with_name("operator");
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusMinus,
                        )
                        .with_name("operator");
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::MinusMinus,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Tilde,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Bang,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Minus,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_5_0 {
                        let result = self
                            .parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Plus,
                            )
                            .with_name("operator");
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
                            "options",
                            OptionalHelper::transform(self.function_call_options(input)),
                        )?;
                    }
                    seq.elem_named("arguments", self.arguments_declaration(input))?;
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
                        "period",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Period,
                        ),
                    )?;
                    seq.elem_named("member", self.member_access(input))?;
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
                        "open_bracket",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenBracket,
                        ),
                    )?;
                    seq.elem(
                        SequenceHelper::run(|mut seq| {
                            seq.elem_named(
                                "start",
                                OptionalHelper::transform(self.expression(input)),
                            )?;
                            seq.elem_named(
                                "end",
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
                        "close_bracket",
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
            .with_name("variant")
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
                    .with_name("expression")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
            )?;
            seq.elem_named(
                "semicolon",
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
            .with_name("variant")
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.fallback_function_attribute(input).with_name("item")
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
                    "fallback_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::FallbackKeyword,
                    ),
                )?;
                seq.elem_named("parameters", self.parameters_declaration(input))?;
                seq.elem_named(
                    "attributes",
                    OptionalHelper::transform(self.fallback_function_attributes(input)),
                )?;
                seq.elem_named(
                    "returns",
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_named("body", self.function_body(input))?;
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
                "for_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ForKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named("initialization", self.for_statement_initialization(input))?;
                        seq.elem_named("condition", self.for_statement_condition(input))?;
                        seq.elem_named(
                            "iterator",
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
                    "close_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named("body", self.statement(input))?;
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
        .with_name("variant")
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
        .with_name("variant")
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
        .with_name("variant")
        .with_kind(RuleKind::FunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.function_attribute(input).with_name("item")
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
        .with_name("variant")
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
            .with_name("variant")
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "function_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FunctionKeyword,
                ),
            )?;
            seq.elem_named("name", self.function_name(input))?;
            seq.elem_named("parameters", self.parameters_declaration(input))?;
            seq.elem_named(
                "attributes",
                OptionalHelper::transform(self.function_attributes(input)),
            )?;
            seq.elem_named(
                "returns",
                OptionalHelper::transform(self.returns_declaration(input)),
            )?;
            seq.elem_named("body", self.function_body(input))?;
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
        .with_name("variant")
        .with_kind(RuleKind::FunctionName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "function_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FunctionKeyword,
                ),
            )?;
            seq.elem_named("parameters", self.parameters_declaration(input))?;
            seq.elem_named(
                "attributes",
                OptionalHelper::transform(self.function_type_attributes(input)),
            )?;
            seq.elem_named(
                "returns",
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
        .with_name("variant")
        .with_kind(RuleKind::FunctionTypeAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.function_type_attribute(input).with_name("item")
        })
        .with_kind(RuleKind::FunctionTypeAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_number_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "literal",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::HexLiteral,
                ),
            )?;
            if !self.version_is_at_least_0_5_0 {
                seq.elem_named("unit", OptionalHelper::transform(self.number_unit(input)))?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::HexNumberExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::HexStringLiteral,
            )
            .with_name("item")
        })
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
                .with_name("item")
            },
            TokenKind::Period,
            "separator",
        )
        .with_kind(RuleKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "if_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::IfKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    self.expression(input)
                        .with_name("condition")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseParen,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named("body", self.statement(input))?;
            seq.elem_named(
                "else_branch",
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
                "as_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsKeyword,
                ),
            )?;
            seq.elem_named(
                "identifier",
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
        .with_name("variant")
        .with_kind(RuleKind::ImportClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    self.import_deconstruction_symbols(input)
                        .with_name("symbols")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseBrace,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named(
                "from_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FromKeyword,
                ),
            )?;
            seq.elem_named(
                "path",
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
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named("alias", OptionalHelper::transform(self.import_alias(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportDeconstructionSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_deconstruction_symbols(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.import_deconstruction_symbol(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::ImportDeconstructionSymbols)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        "import_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ImportKeyword,
                        ),
                    )?;
                    seq.elem_named("clause", self.import_clause(input))?;
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
                "semicolon",
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
                "colon",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Colon,
                ),
            )?;
            seq.elem_named("end", OptionalHelper::transform(self.expression(input)))?;
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
                "is_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::IsKeyword,
                ),
            )?;
            seq.elem_named("types", self.inheritance_types(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named("type_name", self.identifier_path(input))?;
            seq.elem_named(
                "arguments",
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
            |input| self.inheritance_type(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::InheritanceTypes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "interface_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::InterfaceKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                "inheritence",
                OptionalHelper::transform(self.inheritance_specifier(input)),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.interface_members(input))
                        .with_name("members")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
        OneOrMoreHelper::run(input, |input| self.contract_member(input).with_name("item"))
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
                let result = self
                    .parse_token::<LexicalContextType::Default>(input, TokenKind::MultilineComment);
                choice.consider(input, result)?;
                let result = self.parse_token::<LexicalContextType::Default>(
                    input,
                    TokenKind::SingleLineComment,
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
                "library_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::LibraryKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.library_members(input))
                        .with_name("members")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
        OneOrMoreHelper::run(input, |input| self.contract_member(input).with_name("item"))
            .with_kind(RuleKind::LibraryMembers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named("key_type", self.mapping_key_type(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_named(
                    "name",
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
        .with_name("variant")
        .with_kind(RuleKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "mapping_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::MappingKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem_named("key_type", self.mapping_key(input))?;
                        seq.elem_named(
                            "equal_greater_than",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::EqualGreaterThan,
                            ),
                        )?;
                        seq.elem_named("value_type", self.mapping_value(input))?;
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
                    "close_paren",
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
            seq.elem_named("type_name", self.type_name(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem_named(
                    "name",
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
        .with_name("variant")
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
        .with_name("variant")
        .with_kind(RuleKind::ModifierAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.modifier_attribute(input).with_name("item")
        })
        .with_kind(RuleKind::ModifierAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "modifier_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ModifierKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                "parameters",
                OptionalHelper::transform(self.parameters_declaration(input)),
            )?;
            seq.elem_named(
                "attributes",
                OptionalHelper::transform(self.modifier_attributes(input)),
            )?;
            seq.elem_named("body", self.function_body(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named("name", self.identifier_path(input))?;
            seq.elem_named(
                "arguments",
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
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem_named(
                "colon",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Colon,
                ),
            )?;
            seq.elem_named("value", self.expression(input))?;
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
                "open_brace",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.named_arguments(input))
                    .with_name("arguments")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_brace",
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
                self.named_argument_group(input).with_name("item")
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
            |input| self.named_argument(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::NamedArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.named_argument_group(input))
                    .with_name("arguments")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
                "asterisk",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Asterisk,
                ),
            )?;
            seq.elem_named("alias", self.import_alias(input))?;
            seq.elem_named(
                "from_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::FromKeyword,
                ),
            )?;
            seq.elem_named(
                "path",
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
                "new_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::NewKeyword,
                ),
            )?;
            seq.elem_named("type_name", self.type_name(input))?;
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
        .with_name("variant")
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
            |input| self.identifier_path(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::OverridePaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_paths_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.override_paths(input)
                    .with_name("paths")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
                "override_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OverrideKeyword,
                ),
            )?;
            seq.elem_named(
                "overridden",
                OptionalHelper::transform(self.override_paths_declaration(input)),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named("type_name", self.type_name(input))?;
            seq.elem_named(
                "storage_location",
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                "name",
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
            |input| self.parameter(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::Parameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.parameters(input))
                    .with_name("parameters")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
                "path",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::AsciiStringLiteral,
                ),
            )?;
            seq.elem_named("alias", OptionalHelper::transform(self.import_alias(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.expression(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::PositionalArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.positional_arguments(input))
                    .with_name("arguments")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
        .with_name("variant")
        .with_kind(RuleKind::Pragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        "pragma_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::PragmaKeyword,
                        ),
                    )?;
                    seq.elem_named("pragma", self.pragma(input))?;
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
                "semicolon",
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
            .with_name("variant")
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.receive_function_attribute(input).with_name("item")
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
                    "receive_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::ReceiveKeyword,
                    ),
                )?;
                seq.elem_named("parameters", self.parameters_declaration(input))?;
                seq.elem_named(
                    "attributes",
                    OptionalHelper::transform(self.receive_function_attributes(input)),
                )?;
                seq.elem_named("body", self.function_body(input))?;
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
                        "return_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ReturnKeyword,
                        ),
                    )?;
                    seq.elem_named(
                        "expression",
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
                "semicolon",
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
                "returns_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::ReturnsKeyword,
                ),
            )?;
            seq.elem_named("variables", self.parameters_declaration(input))?;
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
                            "revert_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::RevertKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            "error",
                            OptionalHelper::transform(self.identifier_path(input)),
                        )?;
                        seq.elem_named("arguments", self.arguments_declaration(input))?;
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
                    "semicolon",
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
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "members",
                OptionalHelper::transform(self.source_unit_members(input)),
            )?;
            seq.elem_named(
                "eof_trivia",
                OptionalHelper::transform(self.end_of_file_trivia(input)),
            )?;
            seq.finish()
        })
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
        .with_name("variant")
        .with_kind(RuleKind::SourceUnitMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.source_unit_member(input).with_name("item")
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
        .with_name("variant")
        .with_kind(RuleKind::StateVariableAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.state_variable_attribute(input).with_name("item")
        })
        .with_kind(RuleKind::StateVariableAttributes)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named("type_name", self.type_name(input))?;
                    seq.elem_named(
                        "attributes",
                        OptionalHelper::transform(self.state_variable_attributes(input)),
                    )?;
                    seq.elem_named(
                        "name",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named(
                        "value",
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
                "semicolon",
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
                "equal",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Equal,
                ),
            )?;
            seq.elem_named("value", self.expression(input))?;
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
        .with_name("variant")
        .with_kind(RuleKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.statement(input).with_name("item"))
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
        .with_name("variant")
        .with_kind(RuleKind::StorageLocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn string_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.hex_string_literals(input);
            choice.consider(input, result)?;
            let result = self.ascii_string_literals(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_7_0 {
                let result = self.unicode_string_literals(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_name("variant")
        .with_kind(RuleKind::StringExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "struct_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::StructKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Identifier,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    OptionalHelper::transform(self.struct_members(input))
                        .with_name("members")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
                    seq.elem_named("type_name", self.type_name(input))?;
                    seq.elem_named(
                        "name",
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
                "semicolon",
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
        OneOrMoreHelper::run(input, |input| self.struct_member(input).with_name("item"))
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
                    .with_name("throw_keyword")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::Semicolon,
                        RecoverFromNoMatch::No,
                    ),
                )?;
                seq.elem_named(
                    "semicolon",
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
                    "try_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::TryKeyword,
                    ),
                )?;
                seq.elem_named("expression", self.expression(input))?;
                seq.elem_named(
                    "returns",
                    OptionalHelper::transform(self.returns_declaration(input)),
                )?;
                seq.elem_named("body", self.block(input))?;
                seq.elem_named("catch_clauses", self.catch_clauses(input))?;
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
            .with_name("member")
            .with_kind(RuleKind::TupleDeconstructionElement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_elements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.tuple_deconstruction_element(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::TupleDeconstructionElements)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem_named(
                            "open_paren",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::OpenParen,
                            ),
                        )?;
                        seq.elem(
                            self.tuple_deconstruction_elements(input)
                                .with_name("elements")
                                .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                    input,
                                    self,
                                    TokenKind::CloseParen,
                                    RecoverFromNoMatch::Yes,
                                ),
                        )?;
                        seq.elem_named(
                            "close_paren",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::CloseParen,
                            ),
                        )?;
                        seq.finish()
                    }))?;
                    seq.elem_named(
                        "equal",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Equal,
                        ),
                    )?;
                    seq.elem_named("expression", self.expression(input))?;
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
                "semicolon",
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
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                self.tuple_values(input)
                    .with_name("items")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
        .with_name("variant")
        .with_kind(RuleKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OptionalHelper::transform(self.expression(input))
            .with_name("expression")
            .with_kind(RuleKind::TupleValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Default>(
            input,
            self,
            |input| self.tuple_value(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::TupleValues)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_5_3 {
            SequenceHelper::run(|mut seq| {
                seq.elem_named(
                    "type_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::TypeKeyword,
                    ),
                )?;
                seq.elem(SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem_named(
                        "open_paren",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenParen,
                        ),
                    )?;
                    seq.elem(
                        self.type_name(input)
                            .with_name("type_name")
                            .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                input,
                                self,
                                TokenKind::CloseParen,
                                RecoverFromNoMatch::Yes,
                            ),
                    )?;
                    seq.elem_named(
                        "close_paren",
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
                        "open_bracket",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::OpenBracket,
                        ),
                    )?;
                    seq.elem(
                        OptionalHelper::transform(self.expression(input))
                            .with_name("index")
                            .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                                input,
                                self,
                                TokenKind::CloseBracket,
                                RecoverFromNoMatch::Yes,
                            ),
                    )?;
                    seq.elem_named(
                        "close_bracket",
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
            .with_name("variant")
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
            seq.elem_named("type_name", self.type_name(input))?;
            seq.elem_named(
                "storage_location",
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                "name",
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
                    "unchecked_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::UncheckedKeyword,
                    ),
                )?;
                seq.elem_named("block", self.block(input))?;
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
                .with_name("item")
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
                choice.finish(input)
            })
            .with_name("variant")
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionAttribute)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                self.unnamed_function_attribute(input).with_name("item")
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
                    "function_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::FunctionKeyword,
                    ),
                )?;
                seq.elem_named("parameters", self.parameters_declaration(input))?;
                seq.elem_named(
                    "attributes",
                    OptionalHelper::transform(self.unnamed_function_attributes(input)),
                )?;
                seq.elem_named("body", self.function_body(input))?;
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
                "storage_location",
                OptionalHelper::transform(self.storage_location(input)),
            )?;
            seq.elem_named(
                "name",
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
                            "type_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::TypeKeyword,
                            ),
                        )?;
                        seq.elem_named(
                            "name",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::Identifier,
                            ),
                        )?;
                        seq.elem_named(
                            "is_keyword",
                            self.parse_token_with_trivia::<LexicalContextType::Default>(
                                input,
                                TokenKind::IsKeyword,
                            ),
                        )?;
                        seq.elem_named("value_type", self.elementary_type(input))?;
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
                    "semicolon",
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
                    "as_keyword",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::AsKeyword,
                    ),
                )?;
                seq.elem_named("operator", self.using_operator(input))?;
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
        .with_name("variant")
        .with_kind(RuleKind::UsingClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_deconstruction(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_8_13 {
            SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_brace",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenBrace,
                    ),
                )?;
                seq.elem(
                    self.using_deconstruction_symbols(input)
                        .with_name("symbols")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseBrace,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_brace",
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
                seq.elem_named("name", self.identifier_path(input))?;
                if self.version_is_at_least_0_8_19 {
                    seq.elem_named("alias", OptionalHelper::transform(self.using_alias(input)))?;
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
                |input| self.using_deconstruction_symbol(input).with_name("item"),
                TokenKind::Comma,
                "separator",
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
                        "using_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::UsingKeyword,
                        ),
                    )?;
                    seq.elem_named("clause", self.using_clause(input))?;
                    seq.elem_named(
                        "for_keyword",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::ForKeyword,
                        ),
                    )?;
                    seq.elem_named("target", self.using_target(input))?;
                    if self.version_is_at_least_0_8_13 {
                        seq.elem_named(
                            "global_keyword",
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
                "semicolon",
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
            .with_name("variant")
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
        .with_name("variant")
        .with_kind(RuleKind::UsingTarget)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named("variable_type", self.variable_declaration_type(input))?;
                    seq.elem_named(
                        "storage_location",
                        OptionalHelper::transform(self.storage_location(input)),
                    )?;
                    seq.elem_named(
                        "name",
                        self.parse_token_with_trivia::<LexicalContextType::Default>(
                            input,
                            TokenKind::Identifier,
                        ),
                    )?;
                    seq.elem_named(
                        "value",
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
                "semicolon",
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
        .with_name("variant")
        .with_kind(RuleKind::VariableDeclarationType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_value(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "equal",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::Equal,
                ),
            )?;
            seq.elem_named("expression", self.expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::VariableDeclarationValue)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "solidity_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(
                    input,
                    TokenKind::SolidityKeyword,
                ),
            )?;
            seq.elem_named("expressions", self.version_pragma_expressions(input))?;
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
                .with_name("operator"),
            )
        };
        let parse_left_version_pragma_range_expression = |input: &mut ParserContext<'_>| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaRangeExpression,
                3u8,
                3u8 + 1,
                self.parse_token_with_trivia::<LexicalContextType::Pragma>(input, TokenKind::Minus)
                    .with_name("operator"),
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
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::Tilde,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::Equal,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::LessThan,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::GreaterThan,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::LessThanEqual,
                        )
                        .with_name("operator");
                    choice.consider(input, result)?;
                    let result = self
                        .parse_token_with_trivia::<LexicalContextType::Pragma>(
                            input,
                            TokenKind::GreaterThanEqual,
                        )
                        .with_name("operator");
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
        let primary_expression_parser =
            |input: &mut ParserContext<'_>| self.version_pragma_specifier(input);
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
            self.version_pragma_expression(input).with_name("item")
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
                .with_name("item")
            },
            TokenKind::Period,
            "separator",
        )
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "while_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Default>(
                    input,
                    TokenKind::WhileKeyword,
                ),
            )?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem_named(
                    "open_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::OpenParen,
                    ),
                )?;
                seq.elem(
                    self.expression(input)
                        .with_name("condition")
                        .recover_until_with_nested_delims::<_, LexicalContextType::Default>(
                            input,
                            self,
                            TokenKind::CloseParen,
                            RecoverFromNoMatch::Yes,
                        ),
                )?;
                seq.elem_named(
                    "close_paren",
                    self.parse_token_with_trivia::<LexicalContextType::Default>(
                        input,
                        TokenKind::CloseParen,
                    ),
                )?;
                seq.finish()
            }))?;
            seq.elem_named("body", self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_arguments(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_expression(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::YulArguments)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named("names", self.yul_identifier_paths(input))?;
            seq.elem_named(
                "colon_equal",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::ColonEqual,
                ),
            )?;
            seq.elem_named("expression", self.yul_expression(input))?;
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
                "open_brace",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::OpenBrace,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.yul_statements(input))
                    .with_name("statements")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                        input,
                        self,
                        TokenKind::CloseBrace,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_brace",
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
            .with_name("break_keyword")
            .with_kind(RuleKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.parse_token_with_trivia::<LexicalContextType::Yul>(
            input,
            TokenKind::YulContinueKeyword,
        )
        .with_name("continue_keyword")
        .with_kind(RuleKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_default_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "default_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulDefaultKeyword,
                ),
            )?;
            seq.elem_named("body", self.yul_block(input))?;
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
                        "open_paren",
                        self.parse_token_with_trivia::<LexicalContextType::Yul>(
                            input,
                            TokenKind::OpenParen,
                        ),
                    )?;
                    seq.elem(
                        OptionalHelper::transform(self.yul_arguments(input))
                            .with_name("arguments")
                            .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                                input,
                                self,
                                TokenKind::CloseParen,
                                RecoverFromNoMatch::Yes,
                            ),
                    )?;
                    seq.elem_named(
                        "close_paren",
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
                let result = self.yul_identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
            .with_name("variant")
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
                "for_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulForKeyword,
                ),
            )?;
            seq.elem_named("initialization", self.yul_block(input))?;
            seq.elem_named("condition", self.yul_expression(input))?;
            seq.elem_named("iterator", self.yul_block(input))?;
            seq.elem_named("body", self.yul_block(input))?;
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
                "function_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulFunctionKeyword,
                ),
            )?;
            seq.elem_named(
                "name",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIdentifier,
                ),
            )?;
            seq.elem_named("parameters", self.yul_parameters_declaration(input))?;
            seq.elem_named(
                "returns",
                OptionalHelper::transform(self.yul_returns_declaration(input)),
            )?;
            seq.elem_named("body", self.yul_block(input))?;
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
                .with_name("item")
            },
            TokenKind::Period,
            "separator",
        )
        .with_kind(RuleKind::YulIdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_paths(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SeparatedHelper::run::<_, LexicalContextType::Yul>(
            input,
            self,
            |input| self.yul_identifier_path(input).with_name("item"),
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::YulIdentifierPaths)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "if_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulIfKeyword,
                ),
            )?;
            seq.elem_named("condition", self.yul_expression(input))?;
            seq.elem_named("body", self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.parse_token_with_trivia::<LexicalContextType::Yul>(
                input,
                TokenKind::YulLeaveKeyword,
            )
            .with_name("leave_keyword")
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
        .with_name("variant")
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
                .with_name("item")
            },
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::YulParameters)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem_named(
                "open_paren",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::OpenParen,
                ),
            )?;
            seq.elem(
                OptionalHelper::transform(self.yul_parameters(input))
                    .with_name("parameters")
                    .recover_until_with_nested_delims::<_, LexicalContextType::Yul>(
                        input,
                        self,
                        TokenKind::CloseParen,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                "close_paren",
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
                .with_name("item")
            },
            TokenKind::Comma,
            "separator",
        )
        .with_kind(RuleKind::YulReturnVariables)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "minus_greater_than",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::MinusGreaterThan,
                ),
            )?;
            seq.elem_named("variables", self.yul_return_variables(input))?;
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
            let result = self.yul_expression(input);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name("variant")
        .with_kind(RuleKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.yul_statement(input).with_name("item"))
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
        .with_name("variant")
        .with_kind(RuleKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.yul_switch_case(input).with_name("item"))
            .with_kind(RuleKind::YulSwitchCases)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "switch_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulSwitchKeyword,
                ),
            )?;
            seq.elem_named("expression", self.yul_expression(input))?;
            seq.elem_named("cases", self.yul_switch_cases(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulSwitchStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_value_case(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "case_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulCaseKeyword,
                ),
            )?;
            seq.elem_named("value", self.yul_literal(input))?;
            seq.elem_named("body", self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulValueCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_variable_declaration_statement(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem_named(
                "let_keyword",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::YulLetKeyword,
                ),
            )?;
            seq.elem_named("names", self.yul_identifier_paths(input))?;
            seq.elem_named(
                "value",
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
                "colon_equal",
                self.parse_token_with_trivia::<LexicalContextType::Yul>(
                    input,
                    TokenKind::ColonEqual,
                ),
            )?;
            seq.elem_named("expression", self.yul_expression(input))?;
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
    fn multiline_comment(&self, input: &mut ParserContext<'_>) -> bool {
        scan_sequence!(
            scan_chars!(input, '/'),
            scan_chars!(input, '*'),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    scan_none_of!(input, '*'),
                    scan_not_followed_by!(input, scan_chars!(input, '*'), scan_chars!(input, '/'))
                )
            ),
            scan_chars!(input, '*'),
            scan_chars!(input, '/')
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
            scan_chars!(input, '/', '/'),
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
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn bytes_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (true || true)
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
            if true {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn fixed_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        scan_keyword_choice!(
            input,
            if (true || true) && scan_chars!(input, 'f', 'i', 'x', 'e', 'd') {
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || true)
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
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || true)
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
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || self.version_is_at_least_0_4_14)
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
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || self.version_is_at_least_0_4_14)
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
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            }
        )
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn int_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (true || true)
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
            if true {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn ufixed_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        scan_keyword_choice!(
            input,
            if (true || true) && scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd') {
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || true)
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
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || true)
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
                if true {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || self.version_is_at_least_0_4_14)
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
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (true || self.version_is_at_least_0_4_14)
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
                if self.version_is_at_least_0_4_14 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            }
        )
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn uint_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (true || true)
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
            if true {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn yul_bytes_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (false || !self.version_is_at_least_0_7_1)
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
            if !self.version_is_at_least_0_7_1 {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn yul_fixed_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        scan_keyword_choice!(
            input,
            if (false || !self.version_is_at_least_0_7_1)
                && scan_chars!(input, 'f', 'i', 'x', 'e', 'd')
            {
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || !self.version_is_at_least_0_7_1)
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
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || !self.version_is_at_least_0_7_1)
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
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1)
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
                if self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1)
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
                if self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            }
        )
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn yul_int_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (false || !self.version_is_at_least_0_7_1)
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
            if !self.version_is_at_least_0_7_1 {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn yul_ufixed_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        scan_keyword_choice!(
            input,
            if (false || !self.version_is_at_least_0_7_1)
                && scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd')
            {
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || !self.version_is_at_least_0_7_1)
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
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || !self.version_is_at_least_0_7_1)
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
                if !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1)
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
                if self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            },
            if (false || self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1)
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
                if self.version_is_at_least_0_4_14 && !self.version_is_at_least_0_7_1 {
                    KeywordScan::Reserved
                } else {
                    KeywordScan::Present
                }
            } else {
                KeywordScan::Absent
            }
        )
    }
    #[allow(
        clippy::ifs_same_cond,
        clippy::eq_op,
        clippy::nonminimal_bool,
        clippy::overly_complex_bool_expr
    )]
    #[allow(clippy::wrong_self_convention)] // from_keyword refers to a "from" keyword
    #[inline]
    fn yul_uint_keyword(&self, input: &mut ParserContext<'_>) -> KeywordScan {
        if (false || !self.version_is_at_least_0_7_1)
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
            if !self.version_is_at_least_0_7_1 {
                KeywordScan::Reserved
            } else {
                KeywordScan::Present
            }
        } else {
            KeywordScan::Absent
        }
    }

    pub fn scan(&self, lexical_context: LexicalContext, input: &str) -> Vec<TokenKind> {
        let mut input = ParserContext::new(input);
        match lexical_context {
            LexicalContext::Default => {
                Lexer::next_token::<LexicalContextType::Default>(self, &mut input)
            }
            LexicalContext::Pragma => {
                Lexer::next_token::<LexicalContextType::Pragma>(self, &mut input)
            }
            LexicalContext::Yul => Lexer::next_token::<LexicalContextType::Yul>(self, &mut input),
        }
    }

    pub fn parse(&self, kind: RuleKind, input: &str) -> ParseOutput {
        match kind {
            RuleKind::ABICoderPragma => Self::abi_coder_pragma.parse(self, input),
            RuleKind::AdditiveExpression => Self::additive_expression.parse(self, input),
            RuleKind::AddressType => Self::address_type.parse(self, input),
            RuleKind::AndExpression => Self::and_expression.parse(self, input),
            RuleKind::ArgumentsDeclaration => Self::arguments_declaration.parse(self, input),
            RuleKind::ArrayExpression => Self::array_expression.parse(self, input),
            RuleKind::ArrayTypeName => Self::array_type_name.parse(self, input),
            RuleKind::ArrayValues => Self::array_values.parse(self, input),
            RuleKind::AsciiStringLiterals => Self::ascii_string_literals.parse(self, input),
            RuleKind::AssemblyFlags => Self::assembly_flags.parse(self, input),
            RuleKind::AssemblyFlagsDeclaration => {
                Self::assembly_flags_declaration.parse(self, input)
            }
            RuleKind::AssemblyStatement => Self::assembly_statement.parse(self, input),
            RuleKind::AssignmentExpression => Self::assignment_expression.parse(self, input),
            RuleKind::BitwiseAndExpression => Self::bitwise_and_expression.parse(self, input),
            RuleKind::BitwiseOrExpression => Self::bitwise_or_expression.parse(self, input),
            RuleKind::BitwiseXorExpression => Self::bitwise_xor_expression.parse(self, input),
            RuleKind::Block => Self::block.parse(self, input),
            RuleKind::BreakStatement => Self::break_statement.parse(self, input),
            RuleKind::CatchClause => Self::catch_clause.parse(self, input),
            RuleKind::CatchClauseError => Self::catch_clause_error.parse(self, input),
            RuleKind::CatchClauses => Self::catch_clauses.parse(self, input),
            RuleKind::ComparisonExpression => Self::comparison_expression.parse(self, input),
            RuleKind::ConditionalExpression => Self::conditional_expression.parse(self, input),
            RuleKind::ConstantDefinition => Self::constant_definition.parse(self, input),
            RuleKind::ConstructorAttribute => Self::constructor_attribute.parse(self, input),
            RuleKind::ConstructorAttributes => Self::constructor_attributes.parse(self, input),
            RuleKind::ConstructorDefinition => Self::constructor_definition.parse(self, input),
            RuleKind::ContinueStatement => Self::continue_statement.parse(self, input),
            RuleKind::ContractDefinition => Self::contract_definition.parse(self, input),
            RuleKind::ContractMember => Self::contract_member.parse(self, input),
            RuleKind::ContractMembers => Self::contract_members.parse(self, input),
            RuleKind::DecimalNumberExpression => Self::decimal_number_expression.parse(self, input),
            RuleKind::DeleteStatement => Self::delete_statement.parse(self, input),
            RuleKind::DoWhileStatement => Self::do_while_statement.parse(self, input),
            RuleKind::ElementaryType => Self::elementary_type.parse(self, input),
            RuleKind::ElseBranch => Self::else_branch.parse(self, input),
            RuleKind::EmitStatement => Self::emit_statement.parse(self, input),
            RuleKind::EndOfFileTrivia => Self::end_of_file_trivia.parse(self, input),
            RuleKind::EnumDefinition => Self::enum_definition.parse(self, input),
            RuleKind::EnumMembers => Self::enum_members.parse(self, input),
            RuleKind::EqualityExpression => Self::equality_expression.parse(self, input),
            RuleKind::ErrorDefinition => Self::error_definition.parse(self, input),
            RuleKind::ErrorParameter => Self::error_parameter.parse(self, input),
            RuleKind::ErrorParameters => Self::error_parameters.parse(self, input),
            RuleKind::ErrorParametersDeclaration => {
                Self::error_parameters_declaration.parse(self, input)
            }
            RuleKind::EventDefinition => Self::event_definition.parse(self, input),
            RuleKind::EventParameter => Self::event_parameter.parse(self, input),
            RuleKind::EventParameters => Self::event_parameters.parse(self, input),
            RuleKind::EventParametersDeclaration => {
                Self::event_parameters_declaration.parse(self, input)
            }
            RuleKind::ExperimentalFeature => Self::experimental_feature.parse(self, input),
            RuleKind::ExperimentalPragma => Self::experimental_pragma.parse(self, input),
            RuleKind::ExponentiationExpression => {
                Self::exponentiation_expression.parse(self, input)
            }
            RuleKind::Expression => Self::expression.parse(self, input),
            RuleKind::ExpressionStatement => Self::expression_statement.parse(self, input),
            RuleKind::FallbackFunctionAttribute => {
                Self::fallback_function_attribute.parse(self, input)
            }
            RuleKind::FallbackFunctionAttributes => {
                Self::fallback_function_attributes.parse(self, input)
            }
            RuleKind::FallbackFunctionDefinition => {
                Self::fallback_function_definition.parse(self, input)
            }
            RuleKind::ForStatement => Self::for_statement.parse(self, input),
            RuleKind::ForStatementCondition => Self::for_statement_condition.parse(self, input),
            RuleKind::ForStatementInitialization => {
                Self::for_statement_initialization.parse(self, input)
            }
            RuleKind::FunctionAttribute => Self::function_attribute.parse(self, input),
            RuleKind::FunctionAttributes => Self::function_attributes.parse(self, input),
            RuleKind::FunctionBody => Self::function_body.parse(self, input),
            RuleKind::FunctionCallExpression => Self::function_call_expression.parse(self, input),
            RuleKind::FunctionCallOptions => Self::function_call_options.parse(self, input),
            RuleKind::FunctionDefinition => Self::function_definition.parse(self, input),
            RuleKind::FunctionName => Self::function_name.parse(self, input),
            RuleKind::FunctionType => Self::function_type.parse(self, input),
            RuleKind::FunctionTypeAttribute => Self::function_type_attribute.parse(self, input),
            RuleKind::FunctionTypeAttributes => Self::function_type_attributes.parse(self, input),
            RuleKind::HexNumberExpression => Self::hex_number_expression.parse(self, input),
            RuleKind::HexStringLiterals => Self::hex_string_literals.parse(self, input),
            RuleKind::IdentifierPath => Self::identifier_path.parse(self, input),
            RuleKind::IfStatement => Self::if_statement.parse(self, input),
            RuleKind::ImportAlias => Self::import_alias.parse(self, input),
            RuleKind::ImportClause => Self::import_clause.parse(self, input),
            RuleKind::ImportDeconstruction => Self::import_deconstruction.parse(self, input),
            RuleKind::ImportDeconstructionSymbol => {
                Self::import_deconstruction_symbol.parse(self, input)
            }
            RuleKind::ImportDeconstructionSymbols => {
                Self::import_deconstruction_symbols.parse(self, input)
            }
            RuleKind::ImportDirective => Self::import_directive.parse(self, input),
            RuleKind::IndexAccessEnd => Self::index_access_end.parse(self, input),
            RuleKind::IndexAccessExpression => Self::index_access_expression.parse(self, input),
            RuleKind::InheritanceSpecifier => Self::inheritance_specifier.parse(self, input),
            RuleKind::InheritanceType => Self::inheritance_type.parse(self, input),
            RuleKind::InheritanceTypes => Self::inheritance_types.parse(self, input),
            RuleKind::InterfaceDefinition => Self::interface_definition.parse(self, input),
            RuleKind::InterfaceMembers => Self::interface_members.parse(self, input),
            RuleKind::LeadingTrivia => Self::leading_trivia.parse(self, input),
            RuleKind::LibraryDefinition => Self::library_definition.parse(self, input),
            RuleKind::LibraryMembers => Self::library_members.parse(self, input),
            RuleKind::MappingKey => Self::mapping_key.parse(self, input),
            RuleKind::MappingKeyType => Self::mapping_key_type.parse(self, input),
            RuleKind::MappingType => Self::mapping_type.parse(self, input),
            RuleKind::MappingValue => Self::mapping_value.parse(self, input),
            RuleKind::MemberAccess => Self::member_access.parse(self, input),
            RuleKind::MemberAccessExpression => Self::member_access_expression.parse(self, input),
            RuleKind::ModifierAttribute => Self::modifier_attribute.parse(self, input),
            RuleKind::ModifierAttributes => Self::modifier_attributes.parse(self, input),
            RuleKind::ModifierDefinition => Self::modifier_definition.parse(self, input),
            RuleKind::ModifierInvocation => Self::modifier_invocation.parse(self, input),
            RuleKind::MultiplicativeExpression => {
                Self::multiplicative_expression.parse(self, input)
            }
            RuleKind::NamedArgument => Self::named_argument.parse(self, input),
            RuleKind::NamedArgumentGroup => Self::named_argument_group.parse(self, input),
            RuleKind::NamedArgumentGroups => Self::named_argument_groups.parse(self, input),
            RuleKind::NamedArguments => Self::named_arguments.parse(self, input),
            RuleKind::NamedArgumentsDeclaration => {
                Self::named_arguments_declaration.parse(self, input)
            }
            RuleKind::NamedImport => Self::named_import.parse(self, input),
            RuleKind::NewExpression => Self::new_expression.parse(self, input),
            RuleKind::NumberUnit => Self::number_unit.parse(self, input),
            RuleKind::OrExpression => Self::or_expression.parse(self, input),
            RuleKind::OverridePaths => Self::override_paths.parse(self, input),
            RuleKind::OverridePathsDeclaration => {
                Self::override_paths_declaration.parse(self, input)
            }
            RuleKind::OverrideSpecifier => Self::override_specifier.parse(self, input),
            RuleKind::Parameter => Self::parameter.parse(self, input),
            RuleKind::Parameters => Self::parameters.parse(self, input),
            RuleKind::ParametersDeclaration => Self::parameters_declaration.parse(self, input),
            RuleKind::PathImport => Self::path_import.parse(self, input),
            RuleKind::PositionalArguments => Self::positional_arguments.parse(self, input),
            RuleKind::PositionalArgumentsDeclaration => {
                Self::positional_arguments_declaration.parse(self, input)
            }
            RuleKind::PostfixExpression => Self::postfix_expression.parse(self, input),
            RuleKind::Pragma => Self::pragma.parse(self, input),
            RuleKind::PragmaDirective => Self::pragma_directive.parse(self, input),
            RuleKind::PrefixExpression => Self::prefix_expression.parse(self, input),
            RuleKind::ReceiveFunctionAttribute => {
                Self::receive_function_attribute.parse(self, input)
            }
            RuleKind::ReceiveFunctionAttributes => {
                Self::receive_function_attributes.parse(self, input)
            }
            RuleKind::ReceiveFunctionDefinition => {
                Self::receive_function_definition.parse(self, input)
            }
            RuleKind::ReturnStatement => Self::return_statement.parse(self, input),
            RuleKind::ReturnsDeclaration => Self::returns_declaration.parse(self, input),
            RuleKind::RevertStatement => Self::revert_statement.parse(self, input),
            RuleKind::ShiftExpression => Self::shift_expression.parse(self, input),
            RuleKind::SourceUnit => Self::source_unit.parse(self, input),
            RuleKind::SourceUnitMember => Self::source_unit_member.parse(self, input),
            RuleKind::SourceUnitMembers => Self::source_unit_members.parse(self, input),
            RuleKind::StateVariableAttribute => Self::state_variable_attribute.parse(self, input),
            RuleKind::StateVariableAttributes => Self::state_variable_attributes.parse(self, input),
            RuleKind::StateVariableDefinition => Self::state_variable_definition.parse(self, input),
            RuleKind::StateVariableDefinitionValue => {
                Self::state_variable_definition_value.parse(self, input)
            }
            RuleKind::Statement => Self::statement.parse(self, input),
            RuleKind::Statements => Self::statements.parse(self, input),
            RuleKind::StorageLocation => Self::storage_location.parse(self, input),
            RuleKind::StringExpression => Self::string_expression.parse(self, input),
            RuleKind::StructDefinition => Self::struct_definition.parse(self, input),
            RuleKind::StructMember => Self::struct_member.parse(self, input),
            RuleKind::StructMembers => Self::struct_members.parse(self, input),
            RuleKind::ThrowStatement => Self::throw_statement.parse(self, input),
            RuleKind::TrailingTrivia => Self::trailing_trivia.parse(self, input),
            RuleKind::TryStatement => Self::try_statement.parse(self, input),
            RuleKind::TupleDeconstructionElement => {
                Self::tuple_deconstruction_element.parse(self, input)
            }
            RuleKind::TupleDeconstructionElements => {
                Self::tuple_deconstruction_elements.parse(self, input)
            }
            RuleKind::TupleDeconstructionStatement => {
                Self::tuple_deconstruction_statement.parse(self, input)
            }
            RuleKind::TupleExpression => Self::tuple_expression.parse(self, input),
            RuleKind::TupleMember => Self::tuple_member.parse(self, input),
            RuleKind::TupleValue => Self::tuple_value.parse(self, input),
            RuleKind::TupleValues => Self::tuple_values.parse(self, input),
            RuleKind::TypeExpression => Self::type_expression.parse(self, input),
            RuleKind::TypeName => Self::type_name.parse(self, input),
            RuleKind::TypedTupleMember => Self::typed_tuple_member.parse(self, input),
            RuleKind::UncheckedBlock => Self::unchecked_block.parse(self, input),
            RuleKind::UnicodeStringLiterals => Self::unicode_string_literals.parse(self, input),
            RuleKind::UnnamedFunctionAttribute => {
                Self::unnamed_function_attribute.parse(self, input)
            }
            RuleKind::UnnamedFunctionAttributes => {
                Self::unnamed_function_attributes.parse(self, input)
            }
            RuleKind::UnnamedFunctionDefinition => {
                Self::unnamed_function_definition.parse(self, input)
            }
            RuleKind::UntypedTupleMember => Self::untyped_tuple_member.parse(self, input),
            RuleKind::UserDefinedValueTypeDefinition => {
                Self::user_defined_value_type_definition.parse(self, input)
            }
            RuleKind::UsingAlias => Self::using_alias.parse(self, input),
            RuleKind::UsingClause => Self::using_clause.parse(self, input),
            RuleKind::UsingDeconstruction => Self::using_deconstruction.parse(self, input),
            RuleKind::UsingDeconstructionSymbol => {
                Self::using_deconstruction_symbol.parse(self, input)
            }
            RuleKind::UsingDeconstructionSymbols => {
                Self::using_deconstruction_symbols.parse(self, input)
            }
            RuleKind::UsingDirective => Self::using_directive.parse(self, input),
            RuleKind::UsingOperator => Self::using_operator.parse(self, input),
            RuleKind::UsingTarget => Self::using_target.parse(self, input),
            RuleKind::VariableDeclarationStatement => {
                Self::variable_declaration_statement.parse(self, input)
            }
            RuleKind::VariableDeclarationType => Self::variable_declaration_type.parse(self, input),
            RuleKind::VariableDeclarationValue => {
                Self::variable_declaration_value.parse(self, input)
            }
            RuleKind::VersionPragma => Self::version_pragma.parse(self, input),
            RuleKind::VersionPragmaExpression => Self::version_pragma_expression.parse(self, input),
            RuleKind::VersionPragmaExpressions => {
                Self::version_pragma_expressions.parse(self, input)
            }
            RuleKind::VersionPragmaOrExpression => {
                Self::version_pragma_or_expression.parse(self, input)
            }
            RuleKind::VersionPragmaPrefixExpression => {
                Self::version_pragma_prefix_expression.parse(self, input)
            }
            RuleKind::VersionPragmaRangeExpression => {
                Self::version_pragma_range_expression.parse(self, input)
            }
            RuleKind::VersionPragmaSpecifier => Self::version_pragma_specifier.parse(self, input),
            RuleKind::WhileStatement => Self::while_statement.parse(self, input),
            RuleKind::YulArguments => Self::yul_arguments.parse(self, input),
            RuleKind::YulAssignmentStatement => Self::yul_assignment_statement.parse(self, input),
            RuleKind::YulBlock => Self::yul_block.parse(self, input),
            RuleKind::YulBreakStatement => Self::yul_break_statement.parse(self, input),
            RuleKind::YulContinueStatement => Self::yul_continue_statement.parse(self, input),
            RuleKind::YulDefaultCase => Self::yul_default_case.parse(self, input),
            RuleKind::YulExpression => Self::yul_expression.parse(self, input),
            RuleKind::YulForStatement => Self::yul_for_statement.parse(self, input),
            RuleKind::YulFunctionCallExpression => {
                Self::yul_function_call_expression.parse(self, input)
            }
            RuleKind::YulFunctionDefinition => Self::yul_function_definition.parse(self, input),
            RuleKind::YulIdentifierPath => Self::yul_identifier_path.parse(self, input),
            RuleKind::YulIdentifierPaths => Self::yul_identifier_paths.parse(self, input),
            RuleKind::YulIfStatement => Self::yul_if_statement.parse(self, input),
            RuleKind::YulLeaveStatement => Self::yul_leave_statement.parse(self, input),
            RuleKind::YulLiteral => Self::yul_literal.parse(self, input),
            RuleKind::YulParameters => Self::yul_parameters.parse(self, input),
            RuleKind::YulParametersDeclaration => {
                Self::yul_parameters_declaration.parse(self, input)
            }
            RuleKind::YulReturnVariables => Self::yul_return_variables.parse(self, input),
            RuleKind::YulReturnsDeclaration => Self::yul_returns_declaration.parse(self, input),
            RuleKind::YulStatement => Self::yul_statement.parse(self, input),
            RuleKind::YulStatements => Self::yul_statements.parse(self, input),
            RuleKind::YulSwitchCase => Self::yul_switch_case.parse(self, input),
            RuleKind::YulSwitchCases => Self::yul_switch_cases.parse(self, input),
            RuleKind::YulSwitchStatement => Self::yul_switch_statement.parse(self, input),
            RuleKind::YulValueCase => Self::yul_value_case.parse(self, input),
            RuleKind::YulVariableDeclarationStatement => {
                Self::yul_variable_declaration_statement.parse(self, input)
            }
            RuleKind::YulVariableDeclarationValue => {
                Self::yul_variable_declaration_value.parse(self, input)
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
    ) -> Vec<TokenKind> {
        let save = input.position();
        let mut furthest_position = input.position();
        // TODO: Replace by a SmallVec<[_; 2]> or similar
        let mut longest_tokens = vec![];

        macro_rules! longest_match {
            ($( { $kind:ident = $function:ident } )*) => {
                $(
                    if self.$function(input) && input.position() > furthest_position {
                        furthest_position = input.position();

                        longest_tokens = vec![TokenKind::$kind];
                    }
                    input.set_position(save);
                )*
            };
        }
        macro_rules! longest_keyword_match {
            ($( { $kind:ident = $function:ident } )*) => {
                $(
                    match self.$function(input) {
                        KeywordScan::Absent => {},
                        KeywordScan::Present => {
                            if input.position() > furthest_position {
                                furthest_position = input.position();
                                longest_tokens = vec![TokenKind::$kind];
                            } else if input.position() == furthest_position {
                                longest_tokens.push(TokenKind::$kind);
                            }
                        }
                        KeywordScan::Reserved => {
                            // If we're reserved, we can't be ambiguous, so always overwrite, even if len is equal
                            if input.position() >= furthest_position {
                                furthest_position = input.position();
                                longest_tokens = vec![TokenKind::$kind];
                                // We're running after the identifier and we're checking if it's a reserved keyword
                                input.set_position(furthest_position);
                            }
                        }
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
                    Some('/') => match input.next() {
                        Some('=') => Some(TokenKind::SlashEqual),
                        Some(_) => {
                            input.undo();
                            Some(TokenKind::Slash)
                        }
                        None => Some(TokenKind::Slash),
                    },
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
                    longest_tokens = vec![kind];
                }
                input.set_position(save);

                longest_match! {
                    { AsciiStringLiteral = ascii_string_literal }
                    { DecimalLiteral = decimal_literal }
                    { EndOfLine = end_of_line }
                    { HexLiteral = hex_literal }
                    { HexStringLiteral = hex_string_literal }
                    { MultilineComment = multiline_comment }
                    { SingleLineComment = single_line_comment }
                    { UnicodeStringLiteral = unicode_string_literal }
                    { Whitespace = whitespace }
                }
                // Make sure promotable identifiers are last so they don't grab other things
                longest_match! {
                    { Identifier = identifier }
                }

                // Attempt keyword promotion if possible
                if longest_tokens
                    .iter()
                    .any(|tok| [TokenKind::Identifier].contains(tok))
                {
                    // Try fast path for atomic keywords
                    if let Some((scan, kind)) = match input.next() {
                        Some('a') => match input.next() {
                            Some('b') => {
                                if scan_chars!(input, 's', 't', 'r', 'a', 'c', 't') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AbstractKeyword))
                                    } else if self.version_is_at_least_0_6_0 {
                                        Some((KeywordScan::Present, TokenKind::AbstractKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('d') => {
                                if scan_chars!(input, 'd', 'r', 'e', 's', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AddressKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::AddressKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AfterKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::AfterKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::AliasKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::AliasKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AnonymousKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::AnonymousKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::ApplyKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::ApplyKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        if true {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::AssemblyKeyword,
                                            ))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::AssemblyKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AsKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::AsKeyword))
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::AsKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::AsKeyword))
                                    } else {
                                        None
                                    }
                                }
                            },
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::AutoKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::AutoKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('b') => match input.next() {
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::BoolKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::BoolKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::BreakKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::BreakKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('y') => {
                                if scan_chars!(input, 't', 'e') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ByteKeyword))
                                    } else if !self.version_is_at_least_0_8_0 {
                                        Some((KeywordScan::Present, TokenKind::ByteKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('c') => match input.next() {
                            Some('a') => match input.next() {
                                Some('l') => {
                                    if scan_chars!(input, 'l', 'd', 'a', 't', 'a') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::CallDataKeyword,
                                            ))
                                        } else if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Present, TokenKind::CallDataKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('s') => {
                                    if scan_chars!(input, 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::CaseKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::CaseKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::CatchKeyword))
                                        } else if self.version_is_at_least_0_6_0 {
                                            Some((KeywordScan::Present, TokenKind::CatchKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('o') => match input.next() {
                                Some('n') => match input.next() {
                                    Some('s') => {
                                        if scan_chars!(input, 't') {
                                            match input.next() {
                                                Some('a') => {
                                                    if scan_chars!(input, 'n', 't') {
                                                        if true {
                                                            Some((
                                                                KeywordScan::Reserved,
                                                                TokenKind::ConstantKeyword,
                                                            ))
                                                        } else if true {
                                                            Some((
                                                                KeywordScan::Present,
                                                                TokenKind::ConstantKeyword,
                                                            ))
                                                        } else {
                                                            None
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Some('r') => {
                                                    if scan_chars!(input, 'u', 'c', 't', 'o', 'r') {
                                                        if self.version_is_at_least_0_5_0 {
                                                            Some((
                                                                KeywordScan::Reserved,
                                                                TokenKind::ConstructorKeyword,
                                                            ))
                                                        } else if self.version_is_at_least_0_4_22 {
                                                            Some((
                                                                KeywordScan::Present,
                                                                TokenKind::ConstructorKeyword,
                                                            ))
                                                        } else {
                                                            None
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    None
                                                }
                                                None => None,
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('t') => match input.next() {
                                        Some('i') => {
                                            if scan_chars!(input, 'n', 'u', 'e') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::ContinueKeyword,
                                                    ))
                                                } else if true {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::ContinueKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(input, 'a', 'c', 't') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::ContractKeyword,
                                                    ))
                                                } else if true {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::ContractKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    },
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some('p') => {
                                    if scan_chars!(input, 'y', 'o', 'f') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Reserved, TokenKind::CopyOfKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::CopyOfKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::DaysKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::DaysKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::DefaultKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::DefaultKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::DefineKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::DefineKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e', 't', 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::DeleteKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::DeleteKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('o') => {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::DoKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::DoKeyword))
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('e') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 's', 'e') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ElseKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::ElseKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('m') => {
                                if scan_chars!(input, 'i', 't') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::EmitKeyword))
                                    } else if self.version_is_at_least_0_4_21 {
                                        Some((KeywordScan::Present, TokenKind::EmitKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'u', 'm') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::EnumKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::EnumKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'r', 'o', 'r') {
                                    if false {
                                        Some((KeywordScan::Reserved, TokenKind::ErrorKeyword))
                                    } else if self.version_is_at_least_0_8_4 {
                                        Some((KeywordScan::Present, TokenKind::ErrorKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('t') => {
                                if scan_chars!(input, 'h', 'e', 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::EtherKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::EtherKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'n', 't') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::EventKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::EventKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('x') => {
                                if scan_chars!(input, 't', 'e', 'r', 'n', 'a', 'l') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ExternalKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::ExternalKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::FallbackKeyword,
                                                    ))
                                                } else if true {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::FallbackKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::FalseKeyword,
                                                    ))
                                                } else if true {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::FalseKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::FinalKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::FinalKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::FinneyKeyword,
                                                    ))
                                                } else if !self.version_is_at_least_0_7_0 {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::FinneyKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ForKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::ForKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'o', 'm') {
                                    if false {
                                        Some((KeywordScan::Reserved, TokenKind::FromKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::FromKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::FunctionKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::FunctionKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('g') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 'o', 'b', 'a', 'l') {
                                    if false {
                                        Some((KeywordScan::Reserved, TokenKind::GlobalKeyword))
                                    } else if self.version_is_at_least_0_8_13 {
                                        Some((KeywordScan::Present, TokenKind::GlobalKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'e', 'i') {
                                    if self.version_is_at_least_0_7_0 {
                                        Some((KeywordScan::Reserved, TokenKind::GweiKeyword))
                                    } else if self.version_is_at_least_0_6_11 {
                                        Some((KeywordScan::Present, TokenKind::GweiKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::HexKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::HexKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::HoursKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::HoursKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('i') => match input.next() {
                            Some('f') => {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::IfKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::IfKeyword))
                                } else {
                                    None
                                }
                            }
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::ImmutableKeyword,
                                            ))
                                        } else if self.version_is_at_least_0_6_5 {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::ImmutableKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::ImplementsKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::ImplementsKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::ImportKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::ImportKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::IndexedKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::IndexedKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::InlineKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::InlineKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::InterfaceKeyword,
                                                        ))
                                                    } else if true {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::InterfaceKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::InternalKeyword,
                                                        ))
                                                    } else if true {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::InternalKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                None
                                            }
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::InKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::InKeyword))
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::InKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::InKeyword))
                                    } else {
                                        None
                                    }
                                }
                            },
                            Some('s') => {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::IsKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::IsKeyword))
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('l') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 't') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::LetKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::LetKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::LibraryKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::LibraryKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Reserved, TokenKind::MacroKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::MacroKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::MappingKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::MappingKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::MatchKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::MatchKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::MemoryKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::MemoryKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::MinutesKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::MinutesKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ModifierKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::ModifierKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'a', 'b', 'l', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::MutableKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::MutableKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::NewKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::NewKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'l', 'l') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::NullKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::NullKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('o') => match input.next() {
                            Some('f') => {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::OfKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::OfKeyword))
                                } else {
                                    None
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::OverrideKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::OverrideKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Reserved, TokenKind::PartialKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::PartialKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::PayableKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::PayableKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::PragmaKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::PragmaKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::PrivateKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::PrivateKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Reserved, TokenKind::PromiseKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::PromiseKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::PublicKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::PublicKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::PureKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::PureKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::ReceiveKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::ReceiveKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::ReferenceKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::ReferenceKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::RelocatableKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::RelocatableKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n') {
                                            match input.next() {
                                                Some('s') => {
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::ReturnsKeyword,
                                                        ))
                                                    } else if true {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::ReturnsKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::ReturnKeyword,
                                                        ))
                                                    } else if true {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::ReturnKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                                None => {
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::ReturnKeyword,
                                                        ))
                                                    } else if true {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::ReturnKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('v') => {
                                        if scan_chars!(input, 'e', 'r', 't') {
                                            if false {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::RevertKeyword,
                                                ))
                                            } else if self.version_is_at_least_0_8_4 {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::RevertKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                }
                            } else {
                                None
                            }
                        }
                        Some('s') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0 {
                                            Some((KeywordScan::Reserved, TokenKind::SealedKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::SealedKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::SecondsKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::SecondsKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'z', 'e', 'o', 'f') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::SizeOfKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::SizeOfKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::StaticKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::StaticKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'r', 'a', 'g', 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::StorageKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::StorageKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::StringKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::StringKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::StructKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::StructKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('u') => {
                                if scan_chars!(input, 'p', 'p', 'o', 'r', 't', 's') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::SupportsKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::SupportsKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::SwitchKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::SwitchKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        Some((KeywordScan::Reserved, TokenKind::SzaboKeyword))
                                    } else if !self.version_is_at_least_0_7_0 {
                                        Some((KeywordScan::Present, TokenKind::SzaboKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('t') => match input.next() {
                            Some('h') => {
                                if scan_chars!(input, 'r', 'o', 'w') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::ThrowKeyword))
                                    } else if !self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Present, TokenKind::ThrowKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => match input.next() {
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::TrueKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::TrueKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('y') => {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::TryKeyword))
                                    } else if self.version_is_at_least_0_6_0 {
                                        Some((KeywordScan::Present, TokenKind::TryKeyword))
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::TypeDefKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::TypeDefKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::TypeOfKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::TypeOfKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::TypeKeyword,
                                                ))
                                            } else if self.version_is_at_least_0_5_3 {
                                                Some((KeywordScan::Present, TokenKind::TypeKeyword))
                                            } else {
                                                None
                                            }
                                        }
                                        None => {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::TypeKeyword,
                                                ))
                                            } else if self.version_is_at_least_0_5_3 {
                                                Some((KeywordScan::Present, TokenKind::TypeKeyword))
                                            } else {
                                                None
                                            }
                                        }
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Reserved, TokenKind::UncheckedKeyword))
                                    } else if self.version_is_at_least_0_8_0 {
                                        Some((KeywordScan::Present, TokenKind::UncheckedKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::UsingKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::UsingKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('v') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::VarKeyword))
                                    } else if !self.version_is_at_least_0_5_0 {
                                        Some((KeywordScan::Present, TokenKind::VarKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::ViewKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::ViewKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0 {
                                            Some((KeywordScan::Reserved, TokenKind::VirtualKeyword))
                                        } else if self.version_is_at_least_0_6_0 {
                                            Some((KeywordScan::Present, TokenKind::VirtualKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::WeeksKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::WeeksKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('i') => {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::WeiKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::WeiKeyword))
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::WhileKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::WhileKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::YearsKeyword))
                                } else if !self.version_is_at_least_0_5_0 {
                                    Some((KeywordScan::Present, TokenKind::YearsKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    } {
                        match scan {
                            _ if input.position() < furthest_position => { /* Prefix, do nothing */
                            }
                            KeywordScan::Reserved => longest_tokens = vec![kind],
                            KeywordScan::Present => longest_tokens.push(kind),
                            KeywordScan::Absent => unreachable!(),
                        }
                    }
                    input.set_position(save);

                    longest_keyword_match! {
                        { BytesKeyword = bytes_keyword }
                        { FixedKeyword = fixed_keyword }
                        { IntKeyword = int_keyword }
                        { UfixedKeyword = ufixed_keyword }
                        { UintKeyword = uint_keyword }
                    }
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
                    longest_tokens = vec![kind];
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

                // Attempt keyword promotion if possible
                if longest_tokens
                    .iter()
                    .any(|tok| [TokenKind::Identifier].contains(tok))
                {
                    // Try fast path for atomic keywords
                    if let Some((scan, kind)) = match input.next() {
                        Some('a') => {
                            if scan_chars!(input, 'b', 'i', 'c', 'o', 'd', 'e', 'r') {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::AbicoderKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::AbicoderKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('e') => {
                            if scan_chars!(
                                input, 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'
                            ) {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::ExperimentalKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::ExperimentalKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('p') => {
                            if scan_chars!(input, 'r', 'a', 'g', 'm', 'a') {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::PragmaKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::PragmaKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('s') => {
                            if scan_chars!(input, 'o', 'l', 'i', 'd', 'i', 't', 'y') {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::SolidityKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::SolidityKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    } {
                        match scan {
                            _ if input.position() < furthest_position => { /* Prefix, do nothing */
                            }
                            KeywordScan::Reserved => longest_tokens = vec![kind],
                            KeywordScan::Present => longest_tokens.push(kind),
                            KeywordScan::Absent => unreachable!(),
                        }
                    }
                    input.set_position(save);

                    longest_keyword_match! {}
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
                    Some(':') => {
                        if scan_chars!(input, '=') {
                            Some(TokenKind::ColonEqual)
                        } else {
                            None
                        }
                    }
                    Some('{') => Some(TokenKind::OpenBrace),
                    Some('}') => Some(TokenKind::CloseBrace),
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = input.position();
                    longest_tokens = vec![kind];
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

                // Attempt keyword promotion if possible
                if longest_tokens
                    .iter()
                    .any(|tok| [TokenKind::YulIdentifier].contains(tok))
                {
                    // Try fast path for atomic keywords
                    if let Some((scan, kind)) = match input.next() {
                        Some('a') => match input.next() {
                            Some('b') => {
                                if scan_chars!(input, 's', 't', 'r', 'a', 'c', 't') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulAbstractKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAbstractKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('d') => {
                                if scan_chars!(input, 'd', 'r', 'e', 's', 's') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulAddressKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAddressKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('f') => {
                                if scan_chars!(input, 't', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulAfterKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAfterKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('l') => {
                                if scan_chars!(input, 'i', 'a', 's') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulAliasKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAliasKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((
                                            KeywordScan::Reserved,
                                            TokenKind::YulAnonymousKeyword,
                                        ))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAnonymousKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('p') => {
                                if scan_chars!(input, 'p', 'l', 'y') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulApplyKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulApplyKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('s') => match input.next() {
                                Some('s') => {
                                    if scan_chars!(input, 'e', 'm', 'b', 'l', 'y') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulAssemblyKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulAssemblyKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulAsKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAsKeyword))
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulAsKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAsKeyword))
                                    } else {
                                        None
                                    }
                                }
                            },
                            Some('u') => {
                                if scan_chars!(input, 't', 'o') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulAutoKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulAutoKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('b') => match input.next() {
                            Some('o') => {
                                if scan_chars!(input, 'o', 'l') {
                                    if !self.version_is_at_least_0_5_10 {
                                        Some((KeywordScan::Reserved, TokenKind::YulBoolKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulBoolKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => {
                                if scan_chars!(input, 'e', 'a', 'k') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulBreakKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::YulBreakKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('y') => {
                                if scan_chars!(input, 't', 'e') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulByteKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulByteKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('c') => {
                            match input.next() {
                                Some('a') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'l', 'd', 'a', 't', 'a') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulCallDataKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulCallDataKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('s') => {
                                        if scan_chars!(input, 'e') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulCaseKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulCaseKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'c', 'h') {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulCatchKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulCatchKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some('o') => {
                                    match input.next() {
                                        Some('n') => {
                                            match input.next() {
                                                Some('s') => {
                                                    if scan_chars!(input, 't') {
                                                        match input.next() {
                                                            Some('a') => {
                                                                if scan_chars!(input, 'n', 't') {
                                                                    if !self
                                                                        .version_is_at_least_0_7_1
                                                                    {
                                                                        Some ((KeywordScan :: Reserved , TokenKind :: YulConstantKeyword))
                                                                    } else if false {
                                                                        Some ((KeywordScan :: Present , TokenKind :: YulConstantKeyword))
                                                                    } else {
                                                                        None
                                                                    }
                                                                } else {
                                                                    None
                                                                }
                                                            }
                                                            Some('r') => {
                                                                if scan_chars!(
                                                                    input, 'u', 'c', 't', 'o', 'r'
                                                                ) {
                                                                    if self . version_is_at_least_0_5_0 && ! self . version_is_at_least_0_7_1 { Some ((KeywordScan :: Reserved , TokenKind :: YulConstructorKeyword)) } else if false { Some ((KeywordScan :: Present , TokenKind :: YulConstructorKeyword)) } else { None }
                                                                } else {
                                                                    None
                                                                }
                                                            }
                                                            Some(_) => {
                                                                input.undo();
                                                                None
                                                            }
                                                            None => None,
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Some('t') => {
                                                    match input.next() {
                                                        Some('i') => {
                                                            if scan_chars!(input, 'n', 'u', 'e') {
                                                                if true {
                                                                    Some ((KeywordScan :: Reserved , TokenKind :: YulContinueKeyword))
                                                                } else if true {
                                                                    Some ((KeywordScan :: Present , TokenKind :: YulContinueKeyword))
                                                                } else {
                                                                    None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        Some('r') => {
                                                            if scan_chars!(input, 'a', 'c', 't') {
                                                                if !self.version_is_at_least_0_7_1 {
                                                                    Some ((KeywordScan :: Reserved , TokenKind :: YulContractKeyword))
                                                                } else if false {
                                                                    Some ((KeywordScan :: Present , TokenKind :: YulContractKeyword))
                                                                } else {
                                                                    None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        Some(_) => {
                                                            input.undo();
                                                            None
                                                        }
                                                        None => None,
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    None
                                                }
                                                None => None,
                                            }
                                        }
                                        Some('p') => {
                                            if scan_chars!(input, 'y', 'o', 'f') {
                                                if self.version_is_at_least_0_5_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulCopyOfKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulCopyOfKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            }
                        }
                        Some('d') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'y', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulDaysKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulDaysKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('e') => match input.next() {
                                Some('f') => match input.next() {
                                    Some('a') => {
                                        if scan_chars!(input, 'u', 'l', 't') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulDefaultKeyword,
                                                ))
                                            } else if true {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulDefaultKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulDefineKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulDefineKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some('l') => {
                                    if scan_chars!(input, 'e', 't', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulDeleteKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulDeleteKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('o') => {
                                if !self.version_is_at_least_0_7_1 {
                                    Some((KeywordScan::Reserved, TokenKind::YulDoKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::YulDoKeyword))
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('e') => match input.next() {
                            Some('l') => {
                                if scan_chars!(input, 's', 'e') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulElseKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulElseKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('m') => {
                                if scan_chars!(input, 'i', 't') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulEmitKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulEmitKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                if scan_chars!(input, 'u', 'm') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulEnumKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulEnumKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('t') => {
                                if scan_chars!(input, 'h', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulEtherKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulEtherKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'n', 't') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulEventKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulEventKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('x') => {
                                if scan_chars!(input, 't', 'e', 'r', 'n', 'a', 'l') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulExternalKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulExternalKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('f') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'l') {
                                    match input.next() {
                                        Some('l') => {
                                            if scan_chars!(input, 'b', 'a', 'c', 'k') {
                                                if self.version_is_at_least_0_6_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulFallbackKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulFallbackKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('s') => {
                                            if scan_chars!(input, 'e') {
                                                if true {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulFalseKeyword,
                                                    ))
                                                } else if true {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulFalseKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n') {
                                    match input.next() {
                                        Some('a') => {
                                            if scan_chars!(input, 'l') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulFinalKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulFinalKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('n') => {
                                            if scan_chars!(input, 'e', 'y') {
                                                if !self.version_is_at_least_0_7_0 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulFinneyKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulFinneyKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'r') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulForKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::YulForKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulFunctionKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::YulFunctionKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('g') => {
                            if scan_chars!(input, 'w', 'e', 'i') {
                                if self.version_is_at_least_0_7_0 && !self.version_is_at_least_0_7_1
                                {
                                    Some((KeywordScan::Reserved, TokenKind::YulGweiKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::YulGweiKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('h') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'x') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulHexKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulHexKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'u', 'r', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulHoursKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulHoursKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('i') => match input.next() {
                            Some('f') => {
                                if true {
                                    Some((KeywordScan::Reserved, TokenKind::YulIfKeyword))
                                } else if true {
                                    Some((KeywordScan::Present, TokenKind::YulIfKeyword))
                                } else {
                                    None
                                }
                            }
                            Some('m') => match input.next() {
                                Some('m') => {
                                    if scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulImmutableKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulImmutableKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('p') => match input.next() {
                                    Some('l') => {
                                        if scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulImplementsKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulImplementsKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('o') => {
                                        if scan_chars!(input, 'r', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulImportKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulImportKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('n') => match input.next() {
                                Some('d') => {
                                    if scan_chars!(input, 'e', 'x', 'e', 'd') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulIndexedKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulIndexedKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('l') => {
                                    if scan_chars!(input, 'i', 'n', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulInlineKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulInlineKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'e', 'r') {
                                        match input.next() {
                                            Some('f') => {
                                                if scan_chars!(input, 'a', 'c', 'e') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::YulInterfaceKeyword,
                                                        ))
                                                    } else if false {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::YulInterfaceKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            Some('n') => {
                                                if scan_chars!(input, 'a', 'l') {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::YulInternalKeyword,
                                                        ))
                                                    } else if false {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::YulInternalKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            Some(_) => {
                                                input.undo();
                                                None
                                            }
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    if !self.version_is_at_least_0_6_8 {
                                        Some((KeywordScan::Reserved, TokenKind::YulInKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulInKeyword))
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    if !self.version_is_at_least_0_6_8 {
                                        Some((KeywordScan::Reserved, TokenKind::YulInKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulInKeyword))
                                    } else {
                                        None
                                    }
                                }
                            },
                            Some('s') => {
                                if !self.version_is_at_least_0_7_1 {
                                    Some((KeywordScan::Reserved, TokenKind::YulIsKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::YulIsKeyword))
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('l') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'v', 'e') {
                                        if self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulLeaveKeyword,
                                            ))
                                        } else if self.version_is_at_least_0_6_0 {
                                            Some((KeywordScan::Present, TokenKind::YulLeaveKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulLetKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::YulLetKeyword))
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'b', 'r', 'a', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulLibraryKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulLibraryKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('m') => match input.next() {
                            Some('a') => match input.next() {
                                Some('c') => {
                                    if scan_chars!(input, 'r', 'o') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulMacroKeyword,
                                            ))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::YulMacroKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('p') => {
                                    if scan_chars!(input, 'p', 'i', 'n', 'g') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulMappingKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulMappingKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'c', 'h') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulMatchKeyword,
                                            ))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::YulMatchKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('e') => {
                                if scan_chars!(input, 'm', 'o', 'r', 'y') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulMemoryKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulMemoryKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(input, 'n', 'u', 't', 'e', 's') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulMinutesKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulMinutesKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulModifierKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulModifierKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 't', 'a', 'b', 'l', 'e') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulMutableKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulMutableKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('n') => match input.next() {
                            Some('e') => {
                                if scan_chars!(input, 'w') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulNewKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulNewKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('u') => {
                                if scan_chars!(input, 'l', 'l') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulNullKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulNullKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('o') => match input.next() {
                            Some('f') => {
                                if !self.version_is_at_least_0_7_1 {
                                    Some((KeywordScan::Reserved, TokenKind::YulOfKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::YulOfKeyword))
                                } else {
                                    None
                                }
                            }
                            Some('v') => {
                                if scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulOverrideKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulOverrideKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('p') => match input.next() {
                            Some('a') => match input.next() {
                                Some('r') => {
                                    if scan_chars!(input, 't', 'i', 'a', 'l') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPartialKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPartialKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('y') => {
                                    if scan_chars!(input, 'a', 'b', 'l', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPayableKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPayableKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('r') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'g', 'm', 'a') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPragmaKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPragmaKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('i') => {
                                    if scan_chars!(input, 'v', 'a', 't', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPrivateKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPrivateKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'm', 'i', 's', 'e') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPromiseKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPromiseKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('u') => match input.next() {
                                Some('b') => {
                                    if scan_chars!(input, 'l', 'i', 'c') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulPublicKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulPublicKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((KeywordScan::Reserved, TokenKind::YulPureKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::YulPureKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('r') => {
                            if scan_chars!(input, 'e') {
                                match input.next() {
                                    Some('c') => {
                                        if scan_chars!(input, 'e', 'i', 'v', 'e') {
                                            if self.version_is_at_least_0_6_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulReceiveKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulReceiveKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('f') => {
                                        if scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e') {
                                            if self.version_is_at_least_0_5_0
                                                && !self.version_is_at_least_0_7_1
                                            {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulReferenceKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulReferenceKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('l') => {
                                        if scan_chars!(
                                            input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'
                                        ) {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulRelocatableKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulRelocatableKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('t') => {
                                        if scan_chars!(input, 'u', 'r', 'n') {
                                            match input.next() {
                                                Some('s') => {
                                                    if !self.version_is_at_least_0_7_1 {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::YulReturnsKeyword,
                                                        ))
                                                    } else if false {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::YulReturnsKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Some(_) => {
                                                    input.undo();
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::YulReturnKeyword,
                                                        ))
                                                    } else if false {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::YulReturnKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                                None => {
                                                    if true {
                                                        Some((
                                                            KeywordScan::Reserved,
                                                            TokenKind::YulReturnKeyword,
                                                        ))
                                                    } else if false {
                                                        Some((
                                                            KeywordScan::Present,
                                                            TokenKind::YulReturnKeyword,
                                                        ))
                                                    } else {
                                                        None
                                                    }
                                                }
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('v') => {
                                        if scan_chars!(input, 'e', 'r', 't') {
                                            if true {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulRevertKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulRevertKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                }
                            } else {
                                None
                            }
                        }
                        Some('s') => match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 'l', 'e', 'd') {
                                        if self.version_is_at_least_0_5_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulSealedKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulSealedKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('c') => {
                                    if scan_chars!(input, 'o', 'n', 'd', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulSecondsKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulSecondsKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('i') => {
                                if scan_chars!(input, 'z', 'e', 'o', 'f') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulSizeOfKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulSizeOfKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('t') => match input.next() {
                                Some('a') => {
                                    if scan_chars!(input, 't', 'i', 'c') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulStaticKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulStaticKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => {
                                    if scan_chars!(input, 'r', 'a', 'g', 'e') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulStorageKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulStorageKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => match input.next() {
                                    Some('i') => {
                                        if scan_chars!(input, 'n', 'g') {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulStringKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulStringKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some('u') => {
                                        if scan_chars!(input, 'c', 't') {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulStructKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulStructKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        input.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('u') => {
                                if scan_chars!(input, 'p', 'p', 'o', 'r', 't', 's') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((KeywordScan::Reserved, TokenKind::YulSupportsKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulSupportsKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('w') => {
                                if scan_chars!(input, 'i', 't', 'c', 'h') {
                                    if true {
                                        Some((KeywordScan::Reserved, TokenKind::YulSwitchKeyword))
                                    } else if true {
                                        Some((KeywordScan::Present, TokenKind::YulSwitchKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('z') => {
                                if scan_chars!(input, 'a', 'b', 'o') {
                                    if !self.version_is_at_least_0_7_0 {
                                        Some((KeywordScan::Reserved, TokenKind::YulSzaboKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulSzaboKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('t') => match input.next() {
                            Some('h') => {
                                if scan_chars!(input, 'r', 'o', 'w') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulThrowKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulThrowKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('r') => match input.next() {
                                Some('u') => {
                                    if scan_chars!(input, 'e') {
                                        if true {
                                            Some((KeywordScan::Reserved, TokenKind::YulTrueKeyword))
                                        } else if true {
                                            Some((KeywordScan::Present, TokenKind::YulTrueKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('y') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulTryKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulTryKeyword))
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('y') => {
                                if scan_chars!(input, 'p', 'e') {
                                    match input.next() {
                                        Some('d') => {
                                            if scan_chars!(input, 'e', 'f') {
                                                if self.version_is_at_least_0_5_0
                                                    && !self.version_is_at_least_0_7_1
                                                {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulTypeDefKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulTypeDefKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some('o') => {
                                            if scan_chars!(input, 'f') {
                                                if !self.version_is_at_least_0_7_1 {
                                                    Some((
                                                        KeywordScan::Reserved,
                                                        TokenKind::YulTypeOfKeyword,
                                                    ))
                                                } else if false {
                                                    Some((
                                                        KeywordScan::Present,
                                                        TokenKind::YulTypeOfKeyword,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        Some(_) => {
                                            input.undo();
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulTypeKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulTypeKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        }
                                        None => {
                                            if !self.version_is_at_least_0_7_1 {
                                                Some((
                                                    KeywordScan::Reserved,
                                                    TokenKind::YulTypeKeyword,
                                                ))
                                            } else if false {
                                                Some((
                                                    KeywordScan::Present,
                                                    TokenKind::YulTypeKeyword,
                                                ))
                                            } else {
                                                None
                                            }
                                        }
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('u') => match input.next() {
                            Some('n') => {
                                if scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                                    if self.version_is_at_least_0_5_0
                                        && !self.version_is_at_least_0_7_1
                                    {
                                        Some((
                                            KeywordScan::Reserved,
                                            TokenKind::YulUncheckedKeyword,
                                        ))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulUncheckedKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('s') => {
                                if scan_chars!(input, 'i', 'n', 'g') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulUsingKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulUsingKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('v') => match input.next() {
                            Some('a') => {
                                if scan_chars!(input, 'r') {
                                    if !self.version_is_at_least_0_6_5 {
                                        Some((KeywordScan::Reserved, TokenKind::YulVarKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulVarKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'w') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((KeywordScan::Reserved, TokenKind::YulViewKeyword))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::YulViewKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(input, 't', 'u', 'a', 'l') {
                                        if self.version_is_at_least_0_6_0
                                            && !self.version_is_at_least_0_7_1
                                        {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulVirtualKeyword,
                                            ))
                                        } else if false {
                                            Some((
                                                KeywordScan::Present,
                                                TokenKind::YulVirtualKeyword,
                                            ))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('w') => match input.next() {
                            Some('e') => match input.next() {
                                Some('e') => {
                                    if scan_chars!(input, 'k', 's') {
                                        if !self.version_is_at_least_0_7_1 {
                                            Some((
                                                KeywordScan::Reserved,
                                                TokenKind::YulWeeksKeyword,
                                            ))
                                        } else if false {
                                            Some((KeywordScan::Present, TokenKind::YulWeeksKeyword))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('i') => {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulWeiKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulWeiKeyword))
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('h') => {
                                if scan_chars!(input, 'i', 'l', 'e') {
                                    if !self.version_is_at_least_0_7_1 {
                                        Some((KeywordScan::Reserved, TokenKind::YulWhileKeyword))
                                    } else if false {
                                        Some((KeywordScan::Present, TokenKind::YulWhileKeyword))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('y') => {
                            if scan_chars!(input, 'e', 'a', 'r', 's') {
                                if !self.version_is_at_least_0_7_1 {
                                    Some((KeywordScan::Reserved, TokenKind::YulYearsKeyword))
                                } else if false {
                                    Some((KeywordScan::Present, TokenKind::YulYearsKeyword))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    } {
                        match scan {
                            _ if input.position() < furthest_position => { /* Prefix, do nothing */
                            }
                            KeywordScan::Reserved => longest_tokens = vec![kind],
                            KeywordScan::Present => longest_tokens.push(kind),
                            KeywordScan::Absent => unreachable!(),
                        }
                    }
                    input.set_position(save);

                    longest_keyword_match! {
                        { YulBytesKeyword = yul_bytes_keyword }
                        { YulFixedKeyword = yul_fixed_keyword }
                        { YulIntKeyword = yul_int_keyword }
                        { YulUfixedKeyword = yul_ufixed_keyword }
                        { YulUintKeyword = yul_uint_keyword }
                    }
                }
            }
        }

        match longest_tokens.as_slice() {
            [_, ..] => {
                input.set_position(furthest_position);
                longest_tokens
            }
            // Skip a character if possible and if we didn't recognize a token
            [] if input.peek().is_some() => {
                let _ = input.next();
                vec![TokenKind::SKIPPED]
            }
            [] => vec![],
        }
    }
}

#[cfg(feature = "slang_napi_interfaces")]
// NAPI-exposed functions have to accept owned values.
#[allow(clippy::needless_pass_by_value)]
#[napi(namespace = "language")]
impl Language {
    #[napi(constructor)]
    pub fn new_napi(version: String) -> std::result::Result<Self, napi::Error> {
        // IMPORTANT:
        // Make sure this does NOT panic.
        // '#[napi(catch_unwind)]' is not supported on constructors yet.
        // More Info: https://github.com/napi-rs/napi-rs/issues/1852

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
        js_name = "scan",
        ts_return_type = "kinds.TokenKind | null",
        catch_unwind
    )]
    pub fn scan_napi(&self, lexical_context: LexicalContext, input: String) -> Vec<TokenKind> {
        self.scan(lexical_context, input.as_str())
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
