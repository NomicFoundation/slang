// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use {napi::bindgen_prelude::*, napi_derive::napi};

use semver::Version;

use super::{
    kinds::{ProductionKind, RuleKind, TokenKind},
    lexer::Lexer,
    parse_output::ParseOutput,
    support::*,
};

pub use super::kinds::LexicalContext;

#[cfg(feature = "slang_napi_interfaces")]
use super::napi::napi_parse_output::ParseOutput as NAPIParseOutput;

#[derive(Debug)]
#[cfg_attr(feature = "slang_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_at_least_0_4_21: bool,
    pub(crate) version_is_at_least_0_4_22: bool,
    pub(crate) version_is_at_least_0_5_0: bool,
    pub(crate) version_is_at_least_0_5_3: bool,
    pub(crate) version_is_at_least_0_6_0: bool,
    pub(crate) version_is_at_least_0_6_2: bool,
    pub(crate) version_is_at_least_0_6_5: bool,
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
    ];

    pub fn new(version: Version) -> std::result::Result<Self, Error> {
        if Self::SUPPORTED_VERSIONS.contains(&version) {
            Ok(Self {
                version_is_at_least_0_4_21: Version::new(0, 4, 21) <= version,
                version_is_at_least_0_4_22: Version::new(0, 4, 22) <= version,
                version_is_at_least_0_5_0: Version::new(0, 5, 0) <= version,
                version_is_at_least_0_5_3: Version::new(0, 5, 3) <= version,
                version_is_at_least_0_6_0: Version::new(0, 6, 0) <= version,
                version_is_at_least_0_6_2: Version::new(0, 6, 2) <= version,
                version_is_at_least_0_6_5: Version::new(0, 6, 5) <= version,
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
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version))
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    #[allow(dead_code)]
    fn default_parse_token_with_trivia(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::Default as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    fn default_parse_token(&self, input: &mut ParserContext, kind: TokenKind) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::Default as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    const fn default_delimiters() -> &'static [(TokenKind, TokenKind)] {
        &[
            (TokenKind::OpenBrace, TokenKind::CloseBrace),
            (TokenKind::OpenBracket, TokenKind::CloseBracket),
            (TokenKind::OpenParen, TokenKind::CloseParen),
        ]
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token_with_trivia(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::VersionPragma as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::VersionPragma as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    const fn version_pragma_delimiters() -> &'static [(TokenKind, TokenKind)] {
        &[]
    }

    #[allow(dead_code)]
    fn yul_block_parse_token_with_trivia(
        &self,
        input: &mut ParserContext,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::YulBlock as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    fn yul_block_parse_token(&self, input: &mut ParserContext, kind: TokenKind) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::YulBlock as u8 }>(self, input, kind)
    }

    #[allow(dead_code)]
    const fn yul_block_delimiters() -> &'static [(TokenKind, TokenKind)] {
        &[
            (TokenKind::OpenBrace, TokenKind::CloseBrace),
            (TokenKind::OpenParen, TokenKind::CloseParen),
        ]
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn abi_coder_pragma(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ABICoderKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.finish()
        })
        .with_kind(RuleKind::ABICoderPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn address_type(&self, input: &mut ParserContext) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AddressKeyword))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword),
                ))?;
                seq.finish()
            });
            choice.consider(input, result)?;
            let result = self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_kind(RuleKind::AddressType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn arguments_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
            seq.elem(
                OptionalHelper::transform(ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.positional_arguments_list(input);
                    choice.consider(input, result)?;
                    let result = self.named_arguments_declaration(input);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }))
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::CloseParen,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
            seq.finish()
        })
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBracket))?;
            seq.elem(
                self.array_values_list(input)
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseBracket,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBracket))?;
            seq.finish()
        })
        .with_kind(RuleKind::ArrayExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_values_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.expression(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.expression(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ArrayValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literals_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral)
        })
        .with_kind(RuleKind::AsciiStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral),
                    )?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::AssemblyFlagsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AssemblyKeyword))?;
            seq.elem(OptionalHelper::transform(
                self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral),
            ))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(
                    self.assembly_flags_list(input)
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseParen,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            })))?;
            seq.elem(self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::AssemblyStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn block(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
            seq.elem(
                OptionalHelper::transform(self.statements_list(input))
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseBrace,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
            seq.finish()
        })
        .with_kind(RuleKind::Block)
    }

    #[allow(unused_assignments, unused_parens)]
    fn break_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.default_parse_token_with_trivia(input, TokenKind::BreakKeyword)
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::BreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CatchKeyword))?;
                seq.elem(OptionalHelper::transform(self.catch_clause_error(input)))?;
                seq.elem(self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause_error(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                ))?;
                seq.elem(self.parameters_declaration(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClauseError)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clauses_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| self.catch_clause(input))
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClausesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constant_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_7_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.type_name(input))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::ConstantKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                        )?;
                        seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Equal))?;
                        seq.elem(self.expression(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstantDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            OneOrMoreHelper::run(input, |input| {
                if self.version_is_at_least_0_4_22 {
                    ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.modifier_invocation(input);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::InternalKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PublicKeyword);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    })
                } else {
                    ParserResult::disabled()
                }
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(input, TokenKind::ConstructorKeyword),
                )?;
                seq.elem(self.parameters_declaration(input))?;
                seq.elem(OptionalHelper::transform(
                    self.constructor_attributes_list(input),
                ))?;
                seq.elem(self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn continue_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.default_parse_token_with_trivia(input, TokenKind::ContinueKeyword)
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::ContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            if self.version_is_at_least_0_6_0 {
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::AbstractKeyword),
                ))?;
            }
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ContractKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(OptionalHelper::transform(self.inheritance_specifier(input)))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    OptionalHelper::transform(self.contract_members_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ContractDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.using_directive(input);
                choice.consider(input, result)?;
                let result = self.function_definition(input);
                choice.consider(input, result)?;
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
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.fallback_function_definition(input);
                        choice.consider(input, result)?;
                        let result = self.receive_function_definition(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    });
                    choice.consider(input, result)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(input);
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
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::ContractMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    self.deconstruction_import_symbols_list(input)
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FromKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral))?;
            seq.finish()
        })
        .with_kind(RuleKind::DeconstructionImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbol(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                seq.finish()
            })))?;
            seq.finish()
        })
        .with_kind(RuleKind::DeconstructionImportSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbols_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.deconstruction_import_symbol(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.deconstruction_import_symbol(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::DeconstructionImportSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn delete_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::DeleteKeyword),
                    )?;
                    seq.elem(self.expression(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::DeleteStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn do_while_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::DoKeyword))?;
                    seq.elem(self.statement(input))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::WhileKeyword))?;
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::OpenParen),
                        )?;
                        seq.elem(self.expression(input).recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseParen,
                            Self::default_delimiters(),
                        ))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    }))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::DoWhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn emit_statement(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_4_21 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::EmitKeyword),
                        )?;
                        seq.elem(self.identifier_path(input))?;
                        seq.elem(self.arguments_declaration(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::EmitStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_file_trivia(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.default_parse_token(input, TokenKind::Whitespace);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::EndOfLine);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::MultilineComment);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::SingleLineComment);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::EnumKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    OptionalHelper::transform(self.identifiers_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::EnumDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::ErrorKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                        )?;
                        seq.elem(SequenceHelper::run(|mut seq| {
                            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                            let input = delim_guard.ctx();
                            seq.elem(
                                self.default_parse_token_with_trivia(input, TokenKind::OpenParen),
                            )?;
                            seq.elem(
                                OptionalHelper::transform(self.error_parameters_list(input))
                                    .recover_until_with_nested_delims(
                                        input,
                                        |input| {
                                            Lexer::next_token::<{ LexicalContext::Default as u8 }>(
                                                self, input,
                                            )
                                        },
                                        |input| Lexer::leading_trivia(self, input),
                                        TokenKind::CloseParen,
                                        Self::default_delimiters(),
                                    ),
                            )?;
                            seq.elem(
                                self.default_parse_token_with_trivia(input, TokenKind::CloseParen),
                            )?;
                            seq.finish()
                        }))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameter(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.type_name(input))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                ))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.error_parameter(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                        seq.elem(self.error_parameter(input))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::EventKeyword))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::OpenParen),
                        )?;
                        seq.elem(
                            OptionalHelper::transform(self.event_parameters_list(input))
                                .recover_until_with_nested_delims(
                                    input,
                                    |input| {
                                        Lexer::next_token::<{ LexicalContext::Default as u8 }>(
                                            self, input,
                                        )
                                    },
                                    |input| Lexer::leading_trivia(self, input),
                                    TokenKind::CloseParen,
                                    Self::default_delimiters(),
                                ),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    }))?;
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(input, TokenKind::AnonymousKeyword),
                    ))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::EventDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameter(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.type_name(input))?;
            seq.elem(OptionalHelper::transform(
                self.default_parse_token_with_trivia(input, TokenKind::IndexedKeyword),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.default_parse_token_with_trivia(input, TokenKind::Identifier),
            ))?;
            seq.finish()
        })
        .with_kind(RuleKind::EventParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.event_parameter(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.event_parameter(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::EventParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ExperimentalKeyword))?;
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::Identifier);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ExperimentalPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, input: &mut ParserContext) -> ParserResult {
        let parse_assignment_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                1u8,
                1u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Equal);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::BarEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::PlusEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::MinusEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::CaretEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::SlashEqual);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::PercentEqual);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::AsteriskEqual);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::AmpersandEqual);
                    choice.consider(input, result)?;
                    let result = self
                        .default_parse_token_with_trivia(input, TokenKind::LessThanLessThanEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(
                        input,
                        TokenKind::GreaterThanGreaterThanEqual,
                    );
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(
                        input,
                        TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                    );
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_conditional_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::ConditionalExpression,
                3u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::QuestionMark))?;
                    seq.elem(self.expression(input))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Colon))?;
                    seq.elem(self.expression(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_or_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                5u8,
                5u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::BarBar),
            )
        };
        let parse_and_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                7u8,
                7u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::AmpersandAmpersand),
            )
        };
        let parse_equality_comparison_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                9u8,
                9u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::EqualEqual);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::BangEqual);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_order_comparison_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                11u8,
                11u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::LessThan);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::GreaterThan);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::LessThanEqual);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::GreaterThanEqual);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_bitwise_or_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                13u8,
                13u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::Bar),
            )
        };
        let parse_bitwise_x_or_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                15u8,
                15u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::Caret),
            )
        };
        let parse_bitwise_and_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                17u8,
                17u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::Ampersand),
            )
        };
        let parse_shift_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                19u8,
                19u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::LessThanLessThan);
                    choice.consider(input, result)?;
                    let result = self
                        .default_parse_token_with_trivia(input, TokenKind::GreaterThanGreaterThan);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(
                        input,
                        TokenKind::GreaterThanGreaterThanGreaterThan,
                    );
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_add_sub_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                21u8,
                21u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Plus);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Minus);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_mul_div_mod_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                23u8,
                23u8 + 1,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Asterisk);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Slash);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Percent);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_exponentiation_operator_removed_from_0_6_0 = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                25u8,
                25u8 + 1,
                self.default_parse_token_with_trivia(input, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_exponentiation_operator_introduced_from_0_6_0 = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                27u8 + 1,
                27u8,
                self.default_parse_token_with_trivia(input, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_unary_postfix_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::UnaryPostfixExpression,
                29u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::PlusPlus);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::MinusMinus);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let parse_unary_prefix_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_prefix_operator(
                RuleKind::UnaryPrefixExpression,
                31u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::PlusPlus);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::MinusMinus);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Tilde);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Bang);
                    choice.consider(input, result)?;
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Minus);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_5_0 {
                        let result = self.default_parse_token_with_trivia(input, TokenKind::Plus);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                }),
            )
        };
        let parse_function_call_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::FunctionCallExpression,
                33u8,
                SequenceHelper::run(|mut seq| {
                    if self.version_is_at_least_0_6_2 {
                        seq.elem(OptionalHelper::transform(self.function_call_options(input)))?;
                    }
                    seq.elem(self.arguments_declaration(input))?;
                    seq.finish()
                }),
            )
        };
        let parse_member_access_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::MemberAccessExpression,
                35u8,
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Period))?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::Identifier);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::AddressKeyword);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.finish()
                }),
            )
        };
        let parse_index_access_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::IndexAccessExpression,
                37u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBracket))?;
                    seq.elem(
                        SequenceHelper::run(|mut seq| {
                            seq.elem(OptionalHelper::transform(self.expression(input)))?;
                            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                                seq.elem(
                                    self.default_parse_token_with_trivia(input, TokenKind::Colon),
                                )?;
                                seq.elem(OptionalHelper::transform(self.expression(input)))?;
                                seq.finish()
                            })))?;
                            seq.finish()
                        })
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBracket,
                            Self::default_delimiters(),
                        ),
                    )?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBracket))?;
                    seq.finish()
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_unary_prefix_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.new_expression(input);
                choice.consider(input, result)?;
                let result = self.tuple_expression(input);
                choice.consider(input, result)?;
                let result = self.array_expression(input);
                choice.consider(input, result)?;
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::TrueKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::FalseKeyword);
                    choice.consider(input, result)?;
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = self.numeric_expression(input);
                choice.consider(input, result)?;
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.hex_string_literals_list(input);
                    choice.consider(input, result)?;
                    let result = self.ascii_string_literals_list(input);
                    choice.consider(input, result)?;
                    if self.version_is_at_least_0_7_0 {
                        let result = self.unicode_string_literals_list(input);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::BoolKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::StringKeyword);
                    choice.consider(input, result)?;
                    let result = self.address_type(input);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::FixedBytesType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedFixedType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedFixedType);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_8_0 {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ByteKeyword);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::Identifier);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_5_3 {
                    let result = self.type_expression(input);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
        };
        let postfix_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_conditional_operator(input);
                choice.consider(input, result)?;
                let result = parse_unary_postfix_operator(input);
                choice.consider(input, result)?;
                let result = parse_function_call_operator(input);
                choice.consider(input, result)?;
                let result = parse_member_access_operator(input);
                choice.consider(input, result)?;
                let result = parse_index_access_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let binary_operand_parser = |input: &mut ParserContext| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    prefix_operator_parser(input)
                }))?;
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    postfix_operator_parser(input)
                }))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_assignment_operator(input);
                choice.consider(input, result)?;
                let result = parse_or_operator(input);
                choice.consider(input, result)?;
                let result = parse_and_operator(input);
                choice.consider(input, result)?;
                let result = parse_equality_comparison_operator(input);
                choice.consider(input, result)?;
                let result = parse_order_comparison_operator(input);
                choice.consider(input, result)?;
                let result = parse_bitwise_or_operator(input);
                choice.consider(input, result)?;
                let result = parse_bitwise_x_or_operator(input);
                choice.consider(input, result)?;
                let result = parse_bitwise_and_operator(input);
                choice.consider(input, result)?;
                let result = parse_shift_operator(input);
                choice.consider(input, result)?;
                let result = parse_add_sub_operator(input);
                choice.consider(input, result)?;
                let result = parse_mul_div_mod_operator(input);
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_removed_from_0_6_0(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_introduced_from_0_6_0(input);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext| {
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
            Some(RuleKind::Expression),
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.expression(input).recover_until_with_nested_delims(
                input,
                |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                |input| Lexer::leading_trivia(self, input),
                TokenKind::Semicolon,
                Self::default_delimiters(),
            ))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::ExpressionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                if self.version_is_at_least_0_6_0 {
                    ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.modifier_invocation(input);
                        choice.consider(input, result)?;
                        let result = self.override_specifier(input);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ExternalKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PureKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ViewKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::VirtualKeyword);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    })
                } else {
                    ParserResult::disabled()
                }
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FallbackKeyword))?;
                seq.elem(self.parameters_declaration(input))?;
                seq.elem(OptionalHelper::transform(
                    self.fallback_function_attributes_list(input),
                ))?;
                seq.elem(OptionalHelper::transform(self.returns_declaration(input)))?;
                seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                    choice.consider(input, result)?;
                    let result = self.block(input);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ForKeyword))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                            let result = ChoiceHelper::run(input, |mut choice, input| {
                                let result = self.expression_statement(input);
                                choice.consider(input, result)?;
                                let result = self.variable_declaration_statement(input);
                                choice.consider(input, result)?;
                                let result = self.tuple_deconstruction_statement(input);
                                choice.consider(input, result)?;
                                choice.finish(input)
                            });
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                            choice.consider(input, result)?;
                            choice.finish(input)
                        }))?;
                        seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                            let result = self.expression_statement(input);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                            choice.consider(input, result)?;
                            choice.finish(input)
                        }))?;
                        seq.elem(OptionalHelper::transform(self.expression(input)))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            }))?;
            seq.elem(self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.modifier_invocation(input);
                choice.consider(input, result)?;
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::ExternalKeyword);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::InternalKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PrivateKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PublicKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PureKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::ViewKeyword);
                choice.consider(input, result)?;
                if !self.version_is_at_least_0_5_0 {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::ConstantKeyword);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::VirtualKeyword);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::FunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_options(&self, input: &mut ParserContext) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            if self.version_is_at_least_0_6_2 && !self.version_is_at_least_0_8_0 {
                let result =
                    OneOrMoreHelper::run(input, |input| self.named_arguments_declaration(input));
                choice.consider(input, result)?;
            }
            if self.version_is_at_least_0_8_0 {
                let result = self.named_arguments_declaration(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FunctionKeyword))?;
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result = self.default_parse_token_with_trivia(input, TokenKind::Identifier);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::FallbackKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::ReceiveKeyword);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.elem(self.parameters_declaration(input))?;
            seq.elem(OptionalHelper::transform(
                self.function_attributes_list(input),
            ))?;
            seq.elem(OptionalHelper::transform(self.returns_declaration(input)))?;
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result = self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                choice.consider(input, result)?;
                let result = self.block(input);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::FunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FunctionKeyword))?;
            seq.elem(self.parameters_declaration(input))?;
            seq.elem(OptionalHelper::transform(
                self.function_type_attributes_list(input),
            ))?;
            seq.elem(OptionalHelper::transform(self.returns_declaration(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::FunctionType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::InternalKeyword);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::ExternalKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PrivateKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PublicKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PureKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::ViewKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::FunctionTypeAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.default_parse_token_with_trivia(input, TokenKind::HexStringLiteral)
        })
        .with_kind(RuleKind::HexStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_path(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Period))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_paths_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.identifier_path(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.identifier_path(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::IdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifiers_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::IdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::IfKeyword))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(self.expression(input).recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::CloseParen,
                    Self::default_delimiters(),
                ))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            }))?;
            seq.elem(self.statement(input))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ElseKeyword))?;
                seq.elem(self.statement(input))?;
                seq.finish()
            })))?;
            seq.finish()
        })
        .with_kind(RuleKind::IfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::ImportKeyword),
                    )?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.path_import(input);
                        choice.consider(input, result)?;
                        let result = self.named_import(input);
                        choice.consider(input, result)?;
                        let result = self.deconstruction_import(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::ImportDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_specifier(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::IsKeyword))?;
            seq.elem(self.inheritance_types_list(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.identifier_path(input))?;
            seq.elem(OptionalHelper::transform(self.arguments_declaration(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_types_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.inheritance_type(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.inheritance_type(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::InheritanceTypesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::InterfaceKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(OptionalHelper::transform(self.inheritance_specifier(input)))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    OptionalHelper::transform(self.interface_members_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::InterfaceDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_members_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.using_directive(input);
                choice.consider(input, result)?;
                let result = self.function_definition(input);
                choice.consider(input, result)?;
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
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.fallback_function_definition(input);
                        choice.consider(input, result)?;
                        let result = self.receive_function_definition(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    });
                    choice.consider(input, result)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(input);
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
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::InterfaceMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.default_parse_token(input, TokenKind::Whitespace);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::EndOfLine);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::MultilineComment);
                choice.consider(input, result)?;
                let result = self.default_parse_token(input, TokenKind::SingleLineComment);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::LeadingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::LibraryKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    OptionalHelper::transform(self.library_members_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::LibraryDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_members_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.using_directive(input);
                choice.consider(input, result)?;
                let result = self.function_definition(input);
                choice.consider(input, result)?;
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
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.fallback_function_definition(input);
                        choice.consider(input, result)?;
                        let result = self.receive_function_definition(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    });
                    choice.consider(input, result)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(input);
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
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::LibraryMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key_type(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::BoolKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::StringKeyword);
                    choice.consider(input, result)?;
                    let result = self.address_type(input);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::FixedBytesType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedFixedType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedFixedType);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_8_0 {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ByteKeyword);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = self.identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                ))?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::MappingKeyword))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.mapping_key_type(input))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(
                                input,
                                TokenKind::EqualGreaterThan,
                            ),
                        )?;
                        seq.elem(self.mapping_value_type(input))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::MappingType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_value_type(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.type_name(input))?;
            if self.version_is_at_least_0_8_18 {
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                ))?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::MappingValueType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::VirtualKeyword);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::ModifierAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ModifierKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(OptionalHelper::transform(
                self.parameters_declaration(input),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.modifier_attributes_list(input),
            ))?;
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result = self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                choice.consider(input, result)?;
                let result = self.block(input);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.identifier_path(input))?;
            seq.elem(OptionalHelper::transform(self.arguments_declaration(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::ModifierInvocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Colon))?;
            seq.elem(self.expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgument)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
            seq.elem(
                OptionalHelper::transform(self.named_arguments_list(input))
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseBrace,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.named_argument(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.named_argument(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_import(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Asterisk))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FromKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral))?;
            seq.finish()
        })
        .with_kind(RuleKind::NamedImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn new_expression(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::NewKeyword))?;
            seq.elem(self.type_name(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::NewExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn numeric_expression(&self, input: &mut ParserContext) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::HexLiteral))?;
                if !self.version_is_at_least_0_5_0 {
                    seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                        input,
                        |mut choice, input| {
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::DaysKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::EtherKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::HoursKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::MinutesKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::SecondsKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::WeeksKeyword);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::WeiKeyword);
                            choice.consider(input, result)?;
                            if !self.version_is_at_least_0_5_0 {
                                let result = self.default_parse_token_with_trivia(
                                    input,
                                    TokenKind::YearsKeyword,
                                );
                                choice.consider(input, result)?;
                            }
                            if self.version_is_at_least_0_6_11 {
                                let result = self
                                    .default_parse_token_with_trivia(input, TokenKind::GweiKeyword);
                                choice.consider(input, result)?;
                            }
                            if !self.version_is_at_least_0_7_0 {
                                let result = ChoiceHelper::run(input, |mut choice, input| {
                                    let result = self.default_parse_token_with_trivia(
                                        input,
                                        TokenKind::FinneyKeyword,
                                    );
                                    choice.consider(input, result)?;
                                    let result = self.default_parse_token_with_trivia(
                                        input,
                                        TokenKind::SzaboKeyword,
                                    );
                                    choice.consider(input, result)?;
                                    choice.finish(input)
                                });
                                choice.consider(input, result)?;
                            }
                            choice.finish(input)
                        },
                    )))?;
                }
                seq.finish()
            });
            choice.consider(input, result)?;
            let result = SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::DecimalLiteral))?;
                seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                    input,
                    |mut choice, input| {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::DaysKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::EtherKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::HoursKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::MinutesKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::SecondsKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::WeeksKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::WeiKeyword);
                        choice.consider(input, result)?;
                        if !self.version_is_at_least_0_5_0 {
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::YearsKeyword);
                            choice.consider(input, result)?;
                        }
                        if self.version_is_at_least_0_6_11 {
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::GweiKeyword);
                            choice.consider(input, result)?;
                        }
                        if !self.version_is_at_least_0_7_0 {
                            let result = ChoiceHelper::run(input, |mut choice, input| {
                                let result = self.default_parse_token_with_trivia(
                                    input,
                                    TokenKind::FinneyKeyword,
                                );
                                choice.consider(input, result)?;
                                let result = self.default_parse_token_with_trivia(
                                    input,
                                    TokenKind::SzaboKeyword,
                                );
                                choice.consider(input, result)?;
                                choice.finish(input)
                            });
                            choice.consider(input, result)?;
                        }
                        choice.finish(input)
                    },
                )))?;
                seq.finish()
            });
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_kind(RuleKind::NumericExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_specifier(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OverrideKeyword))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(
                    OptionalHelper::transform(self.identifier_paths_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseParen,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            })))?;
            seq.finish()
        })
        .with_kind(RuleKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.type_name(input))?;
            seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                input,
                |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::MemoryKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::StorageKeyword);
                    choice.consider(input, result)?;
                    if self.version_is_at_least_0_5_0 {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::CalldataKeyword);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                },
            )))?;
            seq.elem(OptionalHelper::transform(
                self.default_parse_token_with_trivia(input, TokenKind::Identifier),
            ))?;
            seq.finish()
        })
        .with_kind(RuleKind::Parameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
            seq.elem(
                OptionalHelper::transform(self.parameters_list(input))
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
            seq.finish()
        })
        .with_kind(RuleKind::ParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.parameter(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.parameter(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::ParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn path_import(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                seq.finish()
            })))?;
            seq.finish()
        })
        .with_kind(RuleKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.expression(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.expression(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::PositionalArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::PragmaKeyword),
                    )?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.abi_coder_pragma(input);
                        choice.consider(input, result)?;
                        let result = self.experimental_pragma(input);
                        choice.consider(input, result)?;
                        let result = self.version_pragma(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::PragmaDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                if self.version_is_at_least_0_6_0 {
                    ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.modifier_invocation(input);
                        choice.consider(input, result)?;
                        let result = self.override_specifier(input);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ExternalKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::VirtualKeyword);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    })
                } else {
                    ParserResult::disabled()
                }
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ReceiveKeyword))?;
                seq.elem(self.parameters_declaration(input))?;
                seq.elem(OptionalHelper::transform(
                    self.receive_function_attributes_list(input),
                ))?;
                seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                    choice.consider(input, result)?;
                    let result = self.block(input);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn return_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::ReturnKeyword),
                    )?;
                    seq.elem(OptionalHelper::transform(self.expression(input)))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::ReturnStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn returns_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ReturnsKeyword))?;
            seq.elem(self.parameters_declaration(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::ReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn revert_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(input, TokenKind::RevertKeyword),
                    )?;
                    seq.elem(OptionalHelper::transform(self.identifier_path(input)))?;
                    seq.elem(self.arguments_declaration(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::RevertStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.source_unit_members_list(input),
            ))?;
            seq.elem(OptionalHelper::transform(self.end_of_file_trivia(input)))?;
            seq.finish()
        })
        .with_kind(RuleKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
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
                    let result = ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.struct_definition(input);
                        choice.consider(input, result)?;
                        let result = self.enum_definition(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    });
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
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::SourceUnitMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.override_specifier(input);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::ConstantKeyword);
                choice.consider(input, result)?;
                let result =
                    self.default_parse_token_with_trivia(input, TokenKind::InternalKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PrivateKeyword);
                choice.consider(input, result)?;
                let result = self.default_parse_token_with_trivia(input, TokenKind::PublicKeyword);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_6_5 {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::ImmutableKeyword);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::StateVariableAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.type_name(input))?;
                    seq.elem(OptionalHelper::transform(
                        self.state_variable_attributes_list(input),
                    ))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                    seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Equal))?;
                        seq.elem(self.expression(input))?;
                        seq.finish()
                    })))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::StateVariableDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statement(&self, input: &mut ParserContext) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = ChoiceHelper::run(input, |mut choice, input| {
                let result = self.expression_statement(input);
                choice.consider(input, result)?;
                let result = self.variable_declaration_statement(input);
                choice.consider(input, result)?;
                let result = self.tuple_deconstruction_statement(input);
                choice.consider(input, result)?;
                choice.finish(input)
            });
            choice.consider(input, result)?;
            let result = ChoiceHelper::run(input, |mut choice, input| {
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
                let result = self.revert_statement(input);
                choice.consider(input, result)?;
                if self.version_is_at_least_0_4_21 {
                    let result = self.emit_statement(input);
                    choice.consider(input, result)?;
                }
                if !self.version_is_at_least_0_5_0 {
                    let result = self.throw_statement(input);
                    choice.consider(input, result)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = self.try_statement(input);
                    choice.consider(input, result)?;
                }
                choice.finish(input)
            });
            choice.consider(input, result)?;
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
        .with_kind(RuleKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.statement(input))
            .with_kind(RuleKind::StatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::StructKeyword))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
                seq.elem(
                    OptionalHelper::transform(self.struct_members_list(input))
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::CloseBrace,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
                seq.finish()
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::StructDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_member(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.type_name(input))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::StructMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_members_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.struct_member(input))
            .with_kind(RuleKind::StructMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn throw_statement(&self, input: &mut ParserContext) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(input, TokenKind::ThrowKeyword)
                        .recover_until_with_nested_delims(
                            input,
                            |input| {
                                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input)
                            },
                            |input| Lexer::leading_trivia(self, input),
                            TokenKind::Semicolon,
                            Self::default_delimiters(),
                        ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ThrowStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(
                self.default_parse_token(input, TokenKind::Whitespace),
            ))?;
            seq.elem(OptionalHelper::transform(
                self.default_parse_token(input, TokenKind::SingleLineComment),
            ))?;
            seq.elem(self.default_parse_token(input, TokenKind::EndOfLine))?;
            seq.finish()
        })
        .with_kind(RuleKind::TrailingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn try_statement(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::TryKeyword))?;
                seq.elem(self.expression(input))?;
                seq.elem(OptionalHelper::transform(self.returns_declaration(input)))?;
                seq.elem(self.block(input))?;
                seq.elem(self.catch_clauses_list(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::TryStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(SequenceHelper::run(|mut seq| {
                        let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                        let input = delim_guard.ctx();
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::OpenParen),
                        )?;
                        seq.elem(
                            OptionalHelper::transform(self.tuple_members_list(input))
                                .recover_until_with_nested_delims(
                                    input,
                                    |input| {
                                        Lexer::next_token::<{ LexicalContext::Default as u8 }>(
                                            self, input,
                                        )
                                    },
                                    |input| Lexer::leading_trivia(self, input),
                                    TokenKind::CloseParen,
                                    Self::default_delimiters(),
                                ),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    }))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Equal))?;
                    seq.elem(self.expression(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_expression(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
            seq.elem(
                self.tuple_values_list(input)
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_member(&self, input: &mut ParserContext) -> ParserResult {
        OptionalHelper::transform(ChoiceHelper::run(input, |mut choice, input| {
            let result = SequenceHelper::run(|mut seq| {
                seq.elem(self.type_name(input))?;
                seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                    input,
                    |mut choice, input| {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::MemoryKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::StorageKeyword);
                        choice.consider(input, result)?;
                        if self.version_is_at_least_0_5_0 {
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::CalldataKeyword);
                            choice.consider(input, result)?;
                        }
                        choice.finish(input)
                    },
                )))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                seq.finish()
            });
            choice.consider(input, result)?;
            let result = SequenceHelper::run(|mut seq| {
                seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                    input,
                    |mut choice, input| {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::MemoryKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::StorageKeyword);
                        choice.consider(input, result)?;
                        if self.version_is_at_least_0_5_0 {
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::CalldataKeyword);
                            choice.consider(input, result)?;
                        }
                        choice.finish(input)
                    },
                )))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
                seq.finish()
            });
            choice.consider(input, result)?;
            choice.finish(input)
        }))
        .with_kind(RuleKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_members_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.tuple_member(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.tuple_member(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(OptionalHelper::transform(self.expression(input)))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(OptionalHelper::transform(self.expression(input)))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::TupleValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_5_3 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::TypeKeyword))?;
                seq.elem(SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                    seq.elem(self.type_name(input).recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::default_delimiters(),
                    ))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
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
    fn type_name(&self, input: &mut ParserContext) -> ParserResult {
        let parse_array_type_name_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::ArrayTypeName,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
                    let input = delim_guard.ctx();
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBracket))?;
                    seq.elem(
                        OptionalHelper::transform(self.expression(input))
                            .recover_until_with_nested_delims(
                                input,
                                |input| {
                                    Lexer::next_token::<{ LexicalContext::Default as u8 }>(
                                        self, input,
                                    )
                                },
                                |input| Lexer::leading_trivia(self, input),
                                TokenKind::CloseBracket,
                                Self::default_delimiters(),
                            ),
                    )?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBracket))?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = self.function_type(input);
                choice.consider(input, result)?;
                let result = self.mapping_type(input);
                choice.consider(input, result)?;
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::BoolKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::StringKeyword);
                    choice.consider(input, result)?;
                    let result = self.address_type(input);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::FixedBytesType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedIntegerType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::SignedFixedType);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::UnsignedFixedType);
                    choice.consider(input, result)?;
                    if !self.version_is_at_least_0_8_0 {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ByteKeyword);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = self.identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let postfix_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_array_type_name_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    postfix_operator_parser(input)
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::TypeName),
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::TypeName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unchecked_block(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_8_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::UncheckedKeyword))?;
                seq.elem(self.block(input))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UncheckedBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literals_list(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            OneOrMoreHelper::run(input, |input| {
                self.default_parse_token_with_trivia(input, TokenKind::UnicodeStringLiteral)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnicodeStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes_list(&self, input: &mut ParserContext) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(input, |input| {
                if !self.version_is_at_least_0_6_0 {
                    ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.modifier_invocation(input);
                        choice.consider(input, result)?;
                        let result = self.override_specifier(input);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ExternalKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PayableKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::PureKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::ViewKeyword);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    })
                } else {
                    ParserResult::disabled()
                }
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_definition(&self, input: &mut ParserContext) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::FunctionKeyword))?;
                seq.elem(self.parameters_declaration(input))?;
                seq.elem(OptionalHelper::transform(
                    self.unnamed_function_attributes_list(input),
                ))?;
                seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::Semicolon);
                    choice.consider(input, result)?;
                    let result = self.block(input);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn user_defined_value_type_definition(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_8_8 {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::TypeKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::Identifier),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(input, TokenKind::IsKeyword),
                        )?;
                        seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::BoolKeyword);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::StringKeyword);
                            choice.consider(input, result)?;
                            let result = self.address_type(input);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::FixedBytesType);
                            choice.consider(input, result)?;
                            let result = self.default_parse_token_with_trivia(
                                input,
                                TokenKind::SignedIntegerType,
                            );
                            choice.consider(input, result)?;
                            let result = self.default_parse_token_with_trivia(
                                input,
                                TokenKind::UnsignedIntegerType,
                            );
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::SignedFixedType);
                            choice.consider(input, result)?;
                            let result = self.default_parse_token_with_trivia(
                                input,
                                TokenKind::UnsignedFixedType,
                            );
                            choice.consider(input, result)?;
                            if !self.version_is_at_least_0_8_0 {
                                let result = self
                                    .default_parse_token_with_trivia(input, TokenKind::ByteKeyword);
                                choice.consider(input, result)?;
                            }
                            choice.finish(input)
                        }))?;
                        seq.finish()
                    })
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::Semicolon,
                        Self::default_delimiters(),
                    ),
                )?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
                seq.finish()
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UserDefinedValueTypeDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::UsingKeyword))?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result = self.using_directive_path(input);
                        choice.consider(input, result)?;
                        let result = self.using_directive_deconstruction(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::ForKeyword))?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::Asterisk);
                        choice.consider(input, result)?;
                        let result = self.type_name(input);
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(input, TokenKind::GlobalKeyword),
                    ))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::UsingDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_deconstruction(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
            seq.elem(
                self.using_directive_symbols_list(input)
                    .recover_until_with_nested_delims(
                        input,
                        |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseBrace,
                        Self::default_delimiters(),
                    ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
            seq.finish()
        })
        .with_kind(RuleKind::UsingDirectiveDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_path(&self, input: &mut ParserContext) -> ParserResult {
        self.identifier_path(input)
            .with_kind(RuleKind::UsingDirectivePath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbol(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.identifier_path(input))?;
            if self.version_is_at_least_0_8_19 {
                seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::AsKeyword))?;
                    seq.elem(if self.version_is_at_least_0_8_19 {
                        ChoiceHelper::run(input, |mut choice, input| {
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Ampersand);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Asterisk);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::BangEqual);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Bar);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Caret);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::EqualEqual);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::GreaterThan);
                            choice.consider(input, result)?;
                            let result = self.default_parse_token_with_trivia(
                                input,
                                TokenKind::GreaterThanEqual,
                            );
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::LessThan);
                            choice.consider(input, result)?;
                            let result = self
                                .default_parse_token_with_trivia(input, TokenKind::LessThanEqual);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Minus);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Percent);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Plus);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Slash);
                            choice.consider(input, result)?;
                            let result =
                                self.default_parse_token_with_trivia(input, TokenKind::Tilde);
                            choice.consider(input, result)?;
                            choice.finish(input)
                        })
                    } else {
                        ParserResult::disabled()
                    })?;
                    seq.finish()
                })))?;
            }
            seq.finish()
        })
        .with_kind(RuleKind::UsingDirectiveSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbols_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.using_directive_symbol(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.using_directive_symbol(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::UsingDirectiveSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                if !self.version_is_at_least_0_5_0 {
                    let result = self.default_parse_token_with_trivia(input, TokenKind::VarKeyword);
                    choice.consider(input, result)?;
                }
                let result = self.type_name(input);
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.elem(OptionalHelper::transform(ChoiceHelper::run(
                input,
                |mut choice, input| {
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::MemoryKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.default_parse_token_with_trivia(input, TokenKind::StorageKeyword);
                    choice.consider(input, result)?;
                    if self.version_is_at_least_0_5_0 {
                        let result =
                            self.default_parse_token_with_trivia(input, TokenKind::CalldataKeyword);
                        choice.consider(input, result)?;
                    }
                    choice.finish(input)
                },
            )))?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Identifier))?;
            seq.finish()
        })
        .with_kind(RuleKind::VariableDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.variable_declaration(input))?;
                    seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Equal))?;
                        seq.elem(self.expression(input))?;
                        seq.finish()
                    })))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::Semicolon,
                    Self::default_delimiters(),
                ),
            )?;
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::Semicolon))?;
            seq.finish()
        })
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.version_pragma_parse_token_with_trivia(input, TokenKind::SolidityKeyword),
            )?;
            seq.elem(self.version_pragma_expressions_list(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::VersionPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expression(&self, input: &mut ParserContext) -> ParserResult {
        let parse_version_pragma_or_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaBinaryExpression,
                1u8,
                1u8 + 1,
                self.version_pragma_parse_token_with_trivia(input, TokenKind::BarBar),
            )
        };
        let parse_version_pragma_range_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaBinaryExpression,
                3u8,
                3u8 + 1,
                self.version_pragma_parse_token_with_trivia(input, TokenKind::Minus),
            )
        };
        let parse_version_pragma_unary_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_prefix_operator(
                RuleKind::VersionPragmaUnaryExpression,
                5u8,
                ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::Caret);
                    choice.consider(input, result)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::Tilde);
                    choice.consider(input, result)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::Equal);
                    choice.consider(input, result)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::LessThan);
                    choice.consider(input, result)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::GreaterThan);
                    choice.consider(input, result)?;
                    let result = self
                        .version_pragma_parse_token_with_trivia(input, TokenKind::LessThanEqual);
                    choice.consider(input, result)?;
                    let result = self
                        .version_pragma_parse_token_with_trivia(input, TokenKind::GreaterThanEqual);
                    choice.consider(input, result)?;
                    choice.finish(input)
                }),
            )
        };
        let prefix_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_version_pragma_unary_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let primary_expression_parser =
            |input: &mut ParserContext| self.version_pragma_specifier(input);
        let binary_operand_parser = |input: &mut ParserContext| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    prefix_operator_parser(input)
                }))?;
                seq.elem(primary_expression_parser(input))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_version_pragma_or_operator(input);
                choice.consider(input, result)?;
                let result = parse_version_pragma_range_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext| {
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
            Some(RuleKind::VersionPragmaExpression),
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::VersionPragmaExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expressions_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.version_pragma_expression(input))
            .with_kind(RuleKind::VersionPragmaExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_specifier(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                self.version_pragma_parse_token_with_trivia(input, TokenKind::VersionPragmaValue),
            )?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.version_pragma_parse_token_with_trivia(input, TokenKind::Period),
                    )?;
                    seq.elem(self.version_pragma_parse_token_with_trivia(
                        input,
                        TokenKind::VersionPragmaValue,
                    ))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.default_parse_token_with_trivia(input, TokenKind::WhileKeyword))?;
            seq.elem(SequenceHelper::run(|mut seq| {
                let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                let input = delim_guard.ctx();
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                seq.elem(self.expression(input).recover_until_with_nested_delims(
                    input,
                    |input| Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, input),
                    |input| Lexer::leading_trivia(self, input),
                    TokenKind::CloseParen,
                    Self::default_delimiters(),
                ))?;
                seq.elem(self.default_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                seq.finish()
            }))?;
            seq.elem(self.statement(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_identifier_paths_list(input))?;
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::ColonEqual))?;
            seq.elem(self.yul_expression(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_block(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBrace);
            let input = delim_guard.ctx();
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::OpenBrace))?;
            seq.elem(
                OptionalHelper::transform(self.yul_statements_list(input))
                    .recover_until_with_nested_delims(
                        input,
                        |input| {
                            Lexer::next_token::<{ LexicalContext::YulBlock as u8 }>(self, input)
                        },
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseBrace,
                        Self::yul_block_delimiters(),
                    ),
            )?;
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::CloseBrace))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_break_statement(&self, input: &mut ParserContext) -> ParserResult {
        self.yul_block_parse_token_with_trivia(input, TokenKind::BreakKeyword)
            .with_kind(RuleKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, input: &mut ParserContext) -> ParserResult {
        self.yul_block_parse_token_with_trivia(input, TokenKind::ContinueKeyword)
            .with_kind(RuleKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_declaration_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::LetKeyword))?;
            seq.elem(self.yul_identifier_paths_list(input))?;
            seq.elem(OptionalHelper::transform(SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::ColonEqual))?;
                seq.elem(self.yul_expression(input))?;
                seq.finish()
            })))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expression(&self, input: &mut ParserContext) -> ParserResult {
        let parse_yul_function_call_operator = |input: &mut ParserContext| {
            PrecedenceHelper::to_postfix_operator(
                RuleKind::YulFunctionCallExpression,
                1u8,
                SequenceHelper::run(|mut seq| {
                    let mut delim_guard = input.open_delim(TokenKind::CloseParen);
                    let input = delim_guard.ctx();
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::OpenParen))?;
                    seq.elem(
                        OptionalHelper::transform(self.yul_expressions_list(input))
                            .recover_until_with_nested_delims(
                                input,
                                |input| {
                                    Lexer::next_token::<{ LexicalContext::YulBlock as u8 }>(
                                        self, input,
                                    )
                                },
                                |input| Lexer::leading_trivia(self, input),
                                TokenKind::CloseParen,
                                Self::yul_block_delimiters(),
                            ),
                    )?;
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::CloseParen))?;
                    seq.finish()
                }),
            )
        };
        let primary_expression_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = ChoiceHelper::run(input, |mut choice, input| {
                    let result =
                        self.yul_block_parse_token_with_trivia(input, TokenKind::TrueKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.yul_block_parse_token_with_trivia(input, TokenKind::FalseKeyword);
                    choice.consider(input, result)?;
                    let result =
                        self.yul_block_parse_token_with_trivia(input, TokenKind::YulHexLiteral);
                    choice.consider(input, result)?;
                    let result =
                        self.yul_block_parse_token_with_trivia(input, TokenKind::YulDecimalLiteral);
                    choice.consider(input, result)?;
                    let result =
                        self.yul_block_parse_token_with_trivia(input, TokenKind::HexStringLiteral);
                    choice.consider(input, result)?;
                    let result = self
                        .yul_block_parse_token_with_trivia(input, TokenKind::AsciiStringLiteral);
                    choice.consider(input, result)?;
                    choice.finish(input)
                });
                choice.consider(input, result)?;
                let result = self.yul_identifier_path(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let postfix_operator_parser = |input: &mut ParserContext| {
            ChoiceHelper::run(input, |mut choice, input| {
                let result = parse_yul_function_call_operator(input);
                choice.consider(input, result)?;
                choice.finish(input)
            })
        };
        let linear_expression_parser = |input: &mut ParserContext| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(input))?;
                seq.elem(ZeroOrMoreHelper::run(input, |input| {
                    postfix_operator_parser(input)
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::YulExpression),
            linear_expression_parser(input),
        )
        .with_kind(RuleKind::YulExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expressions_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_expression(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.yul_expression(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_for_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::ForKeyword))?;
            seq.elem(self.yul_block(input))?;
            seq.elem(self.yul_expression(input))?;
            seq.elem(self.yul_block(input))?;
            seq.elem(self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_definition(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::FunctionKeyword))?;
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::YulIdentifier))?;
            seq.elem(self.yul_parameters_declaration(input))?;
            seq.elem(OptionalHelper::transform(
                self.yul_returns_declaration(input),
            ))?;
            seq.elem(self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_path(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::YulIdentifier))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::Period))?;
                    seq.elem(
                        self.yul_block_parse_token_with_trivia(input, TokenKind::YulIdentifier),
                    )?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_paths_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_identifier_path(input))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(self.yul_identifier_path(input))?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifiers_list(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::YulIdentifier))?;
            seq.elem(ZeroOrMoreHelper::run(input, |input| {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::Comma))?;
                    seq.elem(
                        self.yul_block_parse_token_with_trivia(input, TokenKind::YulIdentifier),
                    )?;
                    seq.finish()
                })
            }))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::IfKeyword))?;
            seq.elem(self.yul_expression(input))?;
            seq.elem(self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, input: &mut ParserContext) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.yul_block_parse_token_with_trivia(input, TokenKind::LeaveKeyword)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::YulLeaveStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseParen);
            let input = delim_guard.ctx();
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::OpenParen))?;
            seq.elem(
                OptionalHelper::transform(self.yul_identifiers_list(input))
                    .recover_until_with_nested_delims(
                        input,
                        |input| {
                            Lexer::next_token::<{ LexicalContext::YulBlock as u8 }>(self, input)
                        },
                        |input| Lexer::leading_trivia(self, input),
                        TokenKind::CloseParen,
                        Self::yul_block_delimiters(),
                    ),
            )?;
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::CloseParen))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::MinusGreaterThan))?;
            seq.elem(self.yul_identifiers_list(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statement(&self, input: &mut ParserContext) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.yul_block(input);
            choice.consider(input, result)?;
            let result = self.yul_function_definition(input);
            choice.consider(input, result)?;
            let result = self.yul_declaration_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_assignment_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_if_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_for_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_switch_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_break_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_continue_statement(input);
            choice.consider(input, result)?;
            let result = self.yul_expression(input);
            choice.consider(input, result)?;
            if self.version_is_at_least_0_6_0 {
                let result = self.yul_leave_statement(input);
                choice.consider(input, result)?;
            }
            choice.finish(input)
        })
        .with_kind(RuleKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.yul_statement(input))
            .with_kind(RuleKind::YulStatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_case(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                let result =
                    self.yul_block_parse_token_with_trivia(input, TokenKind::DefaultKeyword);
                choice.consider(input, result)?;
                let result = SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.yul_block_parse_token_with_trivia(input, TokenKind::CaseKeyword),
                    )?;
                    seq.elem(ChoiceHelper::run(input, |mut choice, input| {
                        let result =
                            self.yul_block_parse_token_with_trivia(input, TokenKind::TrueKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.yul_block_parse_token_with_trivia(input, TokenKind::FalseKeyword);
                        choice.consider(input, result)?;
                        let result =
                            self.yul_block_parse_token_with_trivia(input, TokenKind::YulHexLiteral);
                        choice.consider(input, result)?;
                        let result = self
                            .yul_block_parse_token_with_trivia(input, TokenKind::YulDecimalLiteral);
                        choice.consider(input, result)?;
                        let result = self
                            .yul_block_parse_token_with_trivia(input, TokenKind::HexStringLiteral);
                        choice.consider(input, result)?;
                        let result = self.yul_block_parse_token_with_trivia(
                            input,
                            TokenKind::AsciiStringLiteral,
                        );
                        choice.consider(input, result)?;
                        choice.finish(input)
                    }))?;
                    seq.finish()
                });
                choice.consider(input, result)?;
                choice.finish(input)
            }))?;
            seq.elem(self.yul_block(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases_list(&self, input: &mut ParserContext) -> ParserResult {
        OneOrMoreHelper::run(input, |input| self.yul_switch_case(input))
            .with_kind(RuleKind::YulSwitchCasesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, input: &mut ParserContext) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(self.yul_block_parse_token_with_trivia(input, TokenKind::SwitchKeyword))?;
            seq.elem(self.yul_expression(input))?;
            seq.elem(self.yul_switch_cases_list(input))?;
            seq.finish()
        })
        .with_kind(RuleKind::YulSwitchStatement)
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_double_quote_or_backslash(&self, input: &mut ParserContext) -> bool {
        scan_choice!(
            input,
            scan_char_range!(input, ' ', '!'),
            scan_char_range!(input, '#', '['),
            scan_char_range!(input, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_single_quote_or_backslash(&self, input: &mut ParserContext) -> bool {
        scan_choice!(
            input,
            scan_char_range!(input, ' ', '&'),
            scan_char_range!(input, '(', '['),
            scan_char_range!(input, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, input: &mut ParserContext) -> bool {
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
    fn ascii_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_not_followed_by!(
            input,
            scan_choice!(
                input,
                self.single_quoted_ascii_string_literal(input),
                self.double_quoted_ascii_string_literal(input)
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digit(&self, input: &mut ParserContext) -> bool {
        scan_char_range!(input, '0', '9')
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digits(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_one_or_more!(input, self.decimal_digit(input)),
            scan_zero_or_more!(
                input,
                scan_sequence!(
                    scan_chars!(input, '_'),
                    scan_one_or_more!(input, self.decimal_digit(input))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_exponent(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_choice!(input, scan_chars!(input, 'e'), scan_chars!(input, 'E')),
            scan_optional!(input, scan_chars!(input, '-')),
            self.decimal_digits(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_literal(&self, input: &mut ParserContext) -> bool {
        scan_not_followed_by!(
            input,
            scan_sequence!(
                scan_choice!(
                    input,
                    if !self.version_is_at_least_0_5_0 {
                        scan_sequence!(
                            self.decimal_digits(input),
                            scan_optional!(
                                input,
                                scan_sequence!(
                                    scan_chars!(input, '.'),
                                    scan_optional!(input, self.decimal_digits(input))
                                )
                            )
                        )
                    } else {
                        false
                    },
                    if self.version_is_at_least_0_5_0 {
                        scan_sequence!(
                            self.decimal_digits(input),
                            scan_optional!(
                                input,
                                scan_sequence!(scan_chars!(input, '.'), self.decimal_digits(input))
                            )
                        )
                    } else {
                        false
                    },
                    scan_sequence!(scan_chars!(input, '.'), self.decimal_digits(input))
                ),
                scan_optional!(input, self.decimal_exponent(input))
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_ascii_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, '"'),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    self.ascii_character_without_double_quote_or_backslash(input)
                )
            ),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_hex_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '"'),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_unicode_string_literal(&self, input: &mut ParserContext) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_none_of!(input, '\n', '\r', '"', '\\')
                    )
                ),
                scan_chars!(input, '"')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_optional!(input, scan_chars!(input, '\r')),
            scan_chars!(input, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, input: &mut ParserContext) -> bool {
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
    fn fixed_bytes_type(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'b', 'y', 't', 'e', 's'),
            self.fixed_bytes_type_size(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_bytes_type_size(&self, input: &mut ParserContext) -> bool {
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
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_type_size(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_one_or_more!(input, scan_char_range!(input, '0', '9')),
            scan_chars!(input, 'x'),
            scan_one_or_more!(input, scan_char_range!(input, '0', '9'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'x'),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, input: &mut ParserContext) -> bool {
        scan_choice!(
            input,
            self.decimal_digit(input),
            scan_char_range!(input, 'A', 'F'),
            scan_char_range!(input, 'a', 'f')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_literal(&self, input: &mut ParserContext) -> bool {
        scan_not_followed_by!(
            input,
            scan_sequence!(
                scan_choice!(
                    input,
                    scan_chars!(input, '0', 'x'),
                    if !self.version_is_at_least_0_5_0 {
                        scan_chars!(input, '0', 'X')
                    } else {
                        false
                    }
                ),
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
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_contents(&self, input: &mut ParserContext) -> bool {
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
    fn hex_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_not_followed_by!(
            input,
            scan_choice!(
                input,
                self.single_quoted_hex_string_literal(input),
                self.double_quoted_hex_string_literal(input)
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, input: &mut ParserContext) -> bool {
        self.raw_identifier(input)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, input: &mut ParserContext) -> bool {
        scan_choice!(
            input,
            self.identifier_start(input),
            scan_char_range!(input, '0', '9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, input: &mut ParserContext) -> bool {
        scan_choice!(
            input,
            scan_chars!(input, '_'),
            scan_chars!(input, '$'),
            scan_char_range!(input, 'A', 'Z'),
            scan_char_range!(input, 'a', 'z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn integer_type_size(&self, input: &mut ParserContext) -> bool {
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
    }

    #[allow(unused_assignments, unused_parens)]
    fn multiline_comment(&self, input: &mut ParserContext) -> bool {
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
    fn raw_identifier(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            self.identifier_start(input),
            scan_zero_or_more!(input, self.identifier_part(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_fixed_type(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(input, self.fixed_type_size(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_integer_type(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'i', 'n', 't'),
            scan_optional!(input, self.integer_type_size(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, '/', '/'),
            scan_zero_or_more!(input, scan_none_of!(input, '\n', '\r'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_ascii_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, '\''),
            scan_zero_or_more!(
                input,
                scan_choice!(
                    input,
                    self.escape_sequence(input),
                    self.ascii_character_without_single_quote_or_backslash(input)
                )
            ),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_hex_string_literal(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'h', 'e', 'x', '\''),
            scan_optional!(input, self.hex_string_contents(input)),
            scan_chars!(input, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_unicode_string_literal(&self, input: &mut ParserContext) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(input, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    input,
                    scan_choice!(
                        input,
                        self.escape_sequence(input),
                        scan_none_of!(input, '\n', '\r', '\'', '\\')
                    )
                ),
                scan_chars!(input, '\'')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u'),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input),
            self.hex_character(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literal(&self, input: &mut ParserContext) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_not_followed_by!(
                input,
                scan_choice!(
                    input,
                    self.single_quoted_unicode_string_literal(input),
                    self.double_quoted_unicode_string_literal(input)
                ),
                self.identifier_start(input)
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_fixed_type(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u', 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(input, self.fixed_type_size(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_integer_type(&self, input: &mut ParserContext) -> bool {
        scan_sequence!(
            scan_chars!(input, 'u', 'i', 'n', 't'),
            scan_optional!(input, self.integer_type_size(input))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_value(&self, input: &mut ParserContext) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, 'x'),
                scan_chars!(input, 'X'),
                scan_chars!(input, '*'),
                scan_char_range!(input, '0', '9')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, input: &mut ParserContext) -> bool {
        scan_one_or_more!(
            input,
            scan_choice!(input, scan_chars!(input, ' '), scan_chars!(input, '\t'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_decimal_literal(&self, input: &mut ParserContext) -> bool {
        scan_not_followed_by!(
            input,
            scan_choice!(
                input,
                scan_chars!(input, '0'),
                scan_sequence!(
                    scan_char_range!(input, '1', '9'),
                    scan_zero_or_more!(input, self.decimal_digit(input))
                )
            ),
            self.identifier_start(input)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_hex_literal(&self, input: &mut ParserContext) -> bool {
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
    fn yul_identifier(&self, input: &mut ParserContext) -> bool {
        self.raw_identifier(input)
    }

    pub fn scan(&self, lexical_context: LexicalContext, input: &str) -> Option<TokenKind> {
        let mut input = ParserContext::new(input);
        match lexical_context {
            LexicalContext::Default => {
                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, &mut input)
            }
            LexicalContext::VersionPragma => {
                Lexer::next_token::<{ LexicalContext::VersionPragma as u8 }>(self, &mut input)
            }
            LexicalContext::YulBlock => {
                Lexer::next_token::<{ LexicalContext::YulBlock as u8 }>(self, &mut input)
            }
        }
    }

    pub fn parse(&self, production_kind: ProductionKind, input: &str) -> ParseOutput {
        match production_kind {
            ProductionKind::ABICoderPragma => Self::abi_coder_pragma.parse(self, input),
            ProductionKind::AddressType => Self::address_type.parse(self, input),
            ProductionKind::ArgumentsDeclaration => Self::arguments_declaration.parse(self, input),
            ProductionKind::ArrayExpression => Self::array_expression.parse(self, input),
            ProductionKind::ArrayValuesList => Self::array_values_list.parse(self, input),
            ProductionKind::AsciiStringLiteralsList => {
                Self::ascii_string_literals_list.parse(self, input)
            }
            ProductionKind::AssemblyFlagsList => Self::assembly_flags_list.parse(self, input),
            ProductionKind::AssemblyStatement => Self::assembly_statement.parse(self, input),
            ProductionKind::Block => Self::block.parse(self, input),
            ProductionKind::BreakStatement => Self::break_statement.parse(self, input),
            ProductionKind::CatchClause => Self::catch_clause.parse(self, input),
            ProductionKind::CatchClauseError => Self::catch_clause_error.parse(self, input),
            ProductionKind::CatchClausesList => Self::catch_clauses_list.parse(self, input),
            ProductionKind::ConstantDefinition => Self::constant_definition.parse(self, input),
            ProductionKind::ConstructorAttributesList => {
                Self::constructor_attributes_list.parse(self, input)
            }
            ProductionKind::ConstructorDefinition => {
                Self::constructor_definition.parse(self, input)
            }
            ProductionKind::ContinueStatement => Self::continue_statement.parse(self, input),
            ProductionKind::ContractDefinition => Self::contract_definition.parse(self, input),
            ProductionKind::ContractMembersList => Self::contract_members_list.parse(self, input),
            ProductionKind::DeconstructionImport => Self::deconstruction_import.parse(self, input),
            ProductionKind::DeconstructionImportSymbol => {
                Self::deconstruction_import_symbol.parse(self, input)
            }
            ProductionKind::DeconstructionImportSymbolsList => {
                Self::deconstruction_import_symbols_list.parse(self, input)
            }
            ProductionKind::DeleteStatement => Self::delete_statement.parse(self, input),
            ProductionKind::DoWhileStatement => Self::do_while_statement.parse(self, input),
            ProductionKind::EmitStatement => Self::emit_statement.parse(self, input),
            ProductionKind::EndOfFileTrivia => Self::end_of_file_trivia.parse(self, input),
            ProductionKind::EnumDefinition => Self::enum_definition.parse(self, input),
            ProductionKind::ErrorDefinition => Self::error_definition.parse(self, input),
            ProductionKind::ErrorParameter => Self::error_parameter.parse(self, input),
            ProductionKind::ErrorParametersList => Self::error_parameters_list.parse(self, input),
            ProductionKind::EventDefinition => Self::event_definition.parse(self, input),
            ProductionKind::EventParameter => Self::event_parameter.parse(self, input),
            ProductionKind::EventParametersList => Self::event_parameters_list.parse(self, input),
            ProductionKind::ExperimentalPragma => Self::experimental_pragma.parse(self, input),
            ProductionKind::Expression => Self::expression.parse(self, input),
            ProductionKind::ExpressionStatement => Self::expression_statement.parse(self, input),
            ProductionKind::FallbackFunctionAttributesList => {
                Self::fallback_function_attributes_list.parse(self, input)
            }
            ProductionKind::FallbackFunctionDefinition => {
                Self::fallback_function_definition.parse(self, input)
            }
            ProductionKind::ForStatement => Self::for_statement.parse(self, input),
            ProductionKind::FunctionAttributesList => {
                Self::function_attributes_list.parse(self, input)
            }
            ProductionKind::FunctionCallOptions => Self::function_call_options.parse(self, input),
            ProductionKind::FunctionDefinition => Self::function_definition.parse(self, input),
            ProductionKind::FunctionType => Self::function_type.parse(self, input),
            ProductionKind::FunctionTypeAttributesList => {
                Self::function_type_attributes_list.parse(self, input)
            }
            ProductionKind::HexStringLiteralsList => {
                Self::hex_string_literals_list.parse(self, input)
            }
            ProductionKind::IdentifierPath => Self::identifier_path.parse(self, input),
            ProductionKind::IdentifierPathsList => Self::identifier_paths_list.parse(self, input),
            ProductionKind::IdentifiersList => Self::identifiers_list.parse(self, input),
            ProductionKind::IfStatement => Self::if_statement.parse(self, input),
            ProductionKind::ImportDirective => Self::import_directive.parse(self, input),
            ProductionKind::InheritanceSpecifier => Self::inheritance_specifier.parse(self, input),
            ProductionKind::InheritanceType => Self::inheritance_type.parse(self, input),
            ProductionKind::InheritanceTypesList => Self::inheritance_types_list.parse(self, input),
            ProductionKind::InterfaceDefinition => Self::interface_definition.parse(self, input),
            ProductionKind::InterfaceMembersList => Self::interface_members_list.parse(self, input),
            ProductionKind::LeadingTrivia => Self::leading_trivia.parse(self, input),
            ProductionKind::LibraryDefinition => Self::library_definition.parse(self, input),
            ProductionKind::LibraryMembersList => Self::library_members_list.parse(self, input),
            ProductionKind::MappingKeyType => Self::mapping_key_type.parse(self, input),
            ProductionKind::MappingType => Self::mapping_type.parse(self, input),
            ProductionKind::MappingValueType => Self::mapping_value_type.parse(self, input),
            ProductionKind::ModifierAttributesList => {
                Self::modifier_attributes_list.parse(self, input)
            }
            ProductionKind::ModifierDefinition => Self::modifier_definition.parse(self, input),
            ProductionKind::ModifierInvocation => Self::modifier_invocation.parse(self, input),
            ProductionKind::NamedArgument => Self::named_argument.parse(self, input),
            ProductionKind::NamedArgumentsDeclaration => {
                Self::named_arguments_declaration.parse(self, input)
            }
            ProductionKind::NamedArgumentsList => Self::named_arguments_list.parse(self, input),
            ProductionKind::NamedImport => Self::named_import.parse(self, input),
            ProductionKind::NewExpression => Self::new_expression.parse(self, input),
            ProductionKind::NumericExpression => Self::numeric_expression.parse(self, input),
            ProductionKind::OverrideSpecifier => Self::override_specifier.parse(self, input),
            ProductionKind::Parameter => Self::parameter.parse(self, input),
            ProductionKind::ParametersDeclaration => {
                Self::parameters_declaration.parse(self, input)
            }
            ProductionKind::ParametersList => Self::parameters_list.parse(self, input),
            ProductionKind::PathImport => Self::path_import.parse(self, input),
            ProductionKind::PositionalArgumentsList => {
                Self::positional_arguments_list.parse(self, input)
            }
            ProductionKind::PragmaDirective => Self::pragma_directive.parse(self, input),
            ProductionKind::ReceiveFunctionAttributesList => {
                Self::receive_function_attributes_list.parse(self, input)
            }
            ProductionKind::ReceiveFunctionDefinition => {
                Self::receive_function_definition.parse(self, input)
            }
            ProductionKind::ReturnStatement => Self::return_statement.parse(self, input),
            ProductionKind::ReturnsDeclaration => Self::returns_declaration.parse(self, input),
            ProductionKind::RevertStatement => Self::revert_statement.parse(self, input),
            ProductionKind::SourceUnit => Self::source_unit.parse(self, input),
            ProductionKind::SourceUnitMembersList => {
                Self::source_unit_members_list.parse(self, input)
            }
            ProductionKind::StateVariableAttributesList => {
                Self::state_variable_attributes_list.parse(self, input)
            }
            ProductionKind::StateVariableDefinition => {
                Self::state_variable_definition.parse(self, input)
            }
            ProductionKind::Statement => Self::statement.parse(self, input),
            ProductionKind::StatementsList => Self::statements_list.parse(self, input),
            ProductionKind::StructDefinition => Self::struct_definition.parse(self, input),
            ProductionKind::StructMember => Self::struct_member.parse(self, input),
            ProductionKind::StructMembersList => Self::struct_members_list.parse(self, input),
            ProductionKind::ThrowStatement => Self::throw_statement.parse(self, input),
            ProductionKind::TrailingTrivia => Self::trailing_trivia.parse(self, input),
            ProductionKind::TryStatement => Self::try_statement.parse(self, input),
            ProductionKind::TupleDeconstructionStatement => {
                Self::tuple_deconstruction_statement.parse(self, input)
            }
            ProductionKind::TupleExpression => Self::tuple_expression.parse(self, input),
            ProductionKind::TupleMember => Self::tuple_member.parse(self, input),
            ProductionKind::TupleMembersList => Self::tuple_members_list.parse(self, input),
            ProductionKind::TupleValuesList => Self::tuple_values_list.parse(self, input),
            ProductionKind::TypeExpression => Self::type_expression.parse(self, input),
            ProductionKind::TypeName => Self::type_name.parse(self, input),
            ProductionKind::UncheckedBlock => Self::unchecked_block.parse(self, input),
            ProductionKind::UnicodeStringLiteralsList => {
                Self::unicode_string_literals_list.parse(self, input)
            }
            ProductionKind::UnnamedFunctionAttributesList => {
                Self::unnamed_function_attributes_list.parse(self, input)
            }
            ProductionKind::UnnamedFunctionDefinition => {
                Self::unnamed_function_definition.parse(self, input)
            }
            ProductionKind::UserDefinedValueTypeDefinition => {
                Self::user_defined_value_type_definition.parse(self, input)
            }
            ProductionKind::UsingDirective => Self::using_directive.parse(self, input),
            ProductionKind::UsingDirectiveDeconstruction => {
                Self::using_directive_deconstruction.parse(self, input)
            }
            ProductionKind::UsingDirectivePath => Self::using_directive_path.parse(self, input),
            ProductionKind::UsingDirectiveSymbol => Self::using_directive_symbol.parse(self, input),
            ProductionKind::UsingDirectiveSymbolsList => {
                Self::using_directive_symbols_list.parse(self, input)
            }
            ProductionKind::VariableDeclaration => Self::variable_declaration.parse(self, input),
            ProductionKind::VariableDeclarationStatement => {
                Self::variable_declaration_statement.parse(self, input)
            }
            ProductionKind::VersionPragma => Self::version_pragma.parse(self, input),
            ProductionKind::VersionPragmaExpression => {
                Self::version_pragma_expression.parse(self, input)
            }
            ProductionKind::VersionPragmaExpressionsList => {
                Self::version_pragma_expressions_list.parse(self, input)
            }
            ProductionKind::VersionPragmaSpecifier => {
                Self::version_pragma_specifier.parse(self, input)
            }
            ProductionKind::WhileStatement => Self::while_statement.parse(self, input),
            ProductionKind::YulAssignmentStatement => {
                Self::yul_assignment_statement.parse(self, input)
            }
            ProductionKind::YulBlock => Self::yul_block.parse(self, input),
            ProductionKind::YulBreakStatement => Self::yul_break_statement.parse(self, input),
            ProductionKind::YulContinueStatement => Self::yul_continue_statement.parse(self, input),
            ProductionKind::YulDeclarationStatement => {
                Self::yul_declaration_statement.parse(self, input)
            }
            ProductionKind::YulExpression => Self::yul_expression.parse(self, input),
            ProductionKind::YulExpressionsList => Self::yul_expressions_list.parse(self, input),
            ProductionKind::YulForStatement => Self::yul_for_statement.parse(self, input),
            ProductionKind::YulFunctionDefinition => {
                Self::yul_function_definition.parse(self, input)
            }
            ProductionKind::YulIdentifierPath => Self::yul_identifier_path.parse(self, input),
            ProductionKind::YulIdentifierPathsList => {
                Self::yul_identifier_paths_list.parse(self, input)
            }
            ProductionKind::YulIdentifiersList => Self::yul_identifiers_list.parse(self, input),
            ProductionKind::YulIfStatement => Self::yul_if_statement.parse(self, input),
            ProductionKind::YulLeaveStatement => Self::yul_leave_statement.parse(self, input),
            ProductionKind::YulParametersDeclaration => {
                Self::yul_parameters_declaration.parse(self, input)
            }
            ProductionKind::YulReturnsDeclaration => {
                Self::yul_returns_declaration.parse(self, input)
            }
            ProductionKind::YulStatement => Self::yul_statement.parse(self, input),
            ProductionKind::YulStatementsList => Self::yul_statements_list.parse(self, input),
            ProductionKind::YulSwitchCase => Self::yul_switch_case.parse(self, input),
            ProductionKind::YulSwitchCasesList => Self::yul_switch_cases_list.parse(self, input),
            ProductionKind::YulSwitchStatement => Self::yul_switch_statement.parse(self, input),
        }
    }
}

impl Lexer for Language {
    fn leading_trivia(&self, input: &mut ParserContext) -> ParserResult {
        Language::leading_trivia(self, input)
    }

    fn trailing_trivia(&self, input: &mut ParserContext) -> ParserResult {
        Language::trailing_trivia(self, input)
    }

    fn next_token<const LEX_CTX: u8>(&self, input: &mut ParserContext) -> Option<TokenKind> {
        let save = input.position();
        let mut furthest_position = input.position();
        let mut longest_token = None;

        match LexicalContext::from_repr(LEX_CTX).unwrap() {
            LexicalContext::Default => {
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

                if let Some(kind) = match input.next() {
                    Some('a') => match input.next() {
                        Some('b') => match input.next() {
                            Some('i') => scan_chars!(input, 'c', 'o', 'd', 'e', 'r')
                                .then_some(TokenKind::ABICoderKeyword),
                            Some('s') => scan_chars!(input, 't', 'r', 'a', 'c', 't')
                                .then_some(TokenKind::AbstractKeyword),
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('d') => scan_chars!(input, 'd', 'r', 'e', 's', 's')
                            .then_some(TokenKind::AddressKeyword),
                        Some('f') => {
                            scan_chars!(input, 't', 'e', 'r').then_some(TokenKind::AfterKeyword)
                        }
                        Some('l') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(input, 'i', 'a', 's').then_some(TokenKind::AliasKeyword)
                            } else {
                                None
                            }
                        }
                        Some('n') => scan_chars!(input, 'o', 'n', 'y', 'm', 'o', 'u', 's')
                            .then_some(TokenKind::AnonymousKeyword),
                        Some('p') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(input, 'p', 'l', 'y').then_some(TokenKind::ApplyKeyword)
                            } else {
                                None
                            }
                        }
                        Some('s') => match input.next() {
                            Some('s') => scan_chars!(input, 'e', 'm', 'b', 'l', 'y')
                                .then_some(TokenKind::AssemblyKeyword),
                            Some(_) => {
                                input.undo();
                                Some(TokenKind::AsKeyword)
                            }
                            None => Some(TokenKind::AsKeyword),
                        },
                        Some('u') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(input, 't', 'o').then_some(TokenKind::AutoKeyword)
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
                        Some('o') => scan_chars!(input, 'o', 'l').then_some(TokenKind::BoolKeyword),
                        Some('r') => {
                            scan_chars!(input, 'e', 'a', 'k').then_some(TokenKind::BreakKeyword)
                        }
                        Some('y') => scan_chars!(input, 't', 'e').then_some(TokenKind::ByteKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('c') => match input.next() {
                        Some('a') => match input.next() {
                            Some('l') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'l', 'd', 'a', 't', 'a')
                                        .then_some(TokenKind::CalldataKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('s') => scan_chars!(input, 'e').then_some(TokenKind::CaseKeyword),
                            Some('t') => {
                                scan_chars!(input, 'c', 'h').then_some(TokenKind::CatchKeyword)
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
                                            Some('a') => scan_chars!(input, 'n', 't')
                                                .then_some(TokenKind::ConstantKeyword),
                                            Some('r') => {
                                                if self.version_is_at_least_0_4_22 {
                                                    scan_chars!(input, 'u', 'c', 't', 'o', 'r')
                                                        .then_some(TokenKind::ConstructorKeyword)
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
                                    Some('i') => scan_chars!(input, 'n', 'u', 'e')
                                        .then_some(TokenKind::ContinueKeyword),
                                    Some('r') => scan_chars!(input, 'a', 'c', 't')
                                        .then_some(TokenKind::ContractKeyword),
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
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'y', 'o', 'f')
                                        .then_some(TokenKind::CopyofKeyword)
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
                        Some('a') => scan_chars!(input, 'y', 's').then_some(TokenKind::DaysKeyword),
                        Some('e') => match input.next() {
                            Some('f') => match input.next() {
                                Some('a') => scan_chars!(input, 'u', 'l', 't')
                                    .then_some(TokenKind::DefaultKeyword),
                                Some('i') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(input, 'n', 'e')
                                            .then_some(TokenKind::DefineKeyword)
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
                            Some('l') => scan_chars!(input, 'e', 't', 'e')
                                .then_some(TokenKind::DeleteKeyword),
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('o') => Some(TokenKind::DoKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('e') => match input.next() {
                        Some('l') => scan_chars!(input, 's', 'e').then_some(TokenKind::ElseKeyword),
                        Some('m') => {
                            if self.version_is_at_least_0_4_21 {
                                scan_chars!(input, 'i', 't').then_some(TokenKind::EmitKeyword)
                            } else {
                                None
                            }
                        }
                        Some('n') => scan_chars!(input, 'u', 'm').then_some(TokenKind::EnumKeyword),
                        Some('r') => {
                            if self.version_is_at_least_0_8_4 {
                                scan_chars!(input, 'r', 'o', 'r').then_some(TokenKind::ErrorKeyword)
                            } else {
                                None
                            }
                        }
                        Some('t') => {
                            scan_chars!(input, 'h', 'e', 'r').then_some(TokenKind::EtherKeyword)
                        }
                        Some('v') => {
                            scan_chars!(input, 'e', 'n', 't').then_some(TokenKind::EventKeyword)
                        }
                        Some('x') => match input.next() {
                            Some('p') => {
                                scan_chars!(input, 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l')
                                    .then_some(TokenKind::ExperimentalKeyword)
                            }
                            Some('t') => scan_chars!(input, 'e', 'r', 'n', 'a', 'l')
                                .then_some(TokenKind::ExternalKeyword),
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
                    Some('f') => match input.next() {
                        Some('a') => {
                            if scan_chars!(input, 'l') {
                                match input.next() {
                                    Some('l') => scan_chars!(input, 'b', 'a', 'c', 'k')
                                        .then_some(TokenKind::FallbackKeyword),
                                    Some('s') => {
                                        scan_chars!(input, 'e').then_some(TokenKind::FalseKeyword)
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
                                        scan_chars!(input, 'l').then_some(TokenKind::FinalKeyword)
                                    }
                                    Some('n') => scan_chars!(input, 'e', 'y')
                                        .then_some(TokenKind::FinneyKeyword),
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
                        Some('o') => scan_chars!(input, 'r').then_some(TokenKind::ForKeyword),
                        Some('r') => scan_chars!(input, 'o', 'm').then_some(TokenKind::FromKeyword),
                        Some('u') => scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n')
                            .then_some(TokenKind::FunctionKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('g') => match input.next() {
                        Some('l') => scan_chars!(input, 'o', 'b', 'a', 'l')
                            .then_some(TokenKind::GlobalKeyword),
                        Some('w') => {
                            if self.version_is_at_least_0_6_11 {
                                scan_chars!(input, 'e', 'i').then_some(TokenKind::GweiKeyword)
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
                        Some('e') => scan_chars!(input, 'x').then_some(TokenKind::HexKeyword),
                        Some('o') => {
                            scan_chars!(input, 'u', 'r', 's').then_some(TokenKind::HoursKeyword)
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('i') => match input.next() {
                        Some('f') => Some(TokenKind::IfKeyword),
                        Some('m') => match input.next() {
                            Some('m') => {
                                if self.version_is_at_least_0_6_5 {
                                    scan_chars!(input, 'u', 't', 'a', 'b', 'l', 'e')
                                        .then_some(TokenKind::ImmutableKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('p') => match input.next() {
                                Some('l') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(input, 'e', 'm', 'e', 'n', 't', 's')
                                            .then_some(TokenKind::ImplementsKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => {
                                    scan_chars!(input, 'r', 't').then_some(TokenKind::ImportKeyword)
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
                            Some('d') => scan_chars!(input, 'e', 'x', 'e', 'd')
                                .then_some(TokenKind::IndexedKeyword),
                            Some('l') => scan_chars!(input, 'i', 'n', 'e')
                                .then_some(TokenKind::InlineKeyword),
                            Some('t') => {
                                if scan_chars!(input, 'e', 'r') {
                                    match input.next() {
                                        Some('f') => scan_chars!(input, 'a', 'c', 'e')
                                            .then_some(TokenKind::InterfaceKeyword),
                                        Some('n') => scan_chars!(input, 'a', 'l')
                                            .then_some(TokenKind::InternalKeyword),
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
                                Some(TokenKind::InKeyword)
                            }
                            None => Some(TokenKind::InKeyword),
                        },
                        Some('s') => Some(TokenKind::IsKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('l') => match input.next() {
                        Some('e') => match input.next() {
                            Some('a') => {
                                if self.version_is_at_least_0_6_0 {
                                    scan_chars!(input, 'v', 'e').then_some(TokenKind::LeaveKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('t') => Some(TokenKind::LetKeyword),
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('i') => scan_chars!(input, 'b', 'r', 'a', 'r', 'y')
                            .then_some(TokenKind::LibraryKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('m') => match input.next() {
                        Some('a') => match input.next() {
                            Some('c') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'r', 'o').then_some(TokenKind::MacroKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('p') => scan_chars!(input, 'p', 'i', 'n', 'g')
                                .then_some(TokenKind::MappingKeyword),
                            Some('t') => {
                                scan_chars!(input, 'c', 'h').then_some(TokenKind::MatchKeyword)
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('e') => scan_chars!(input, 'm', 'o', 'r', 'y')
                            .then_some(TokenKind::MemoryKeyword),
                        Some('i') => scan_chars!(input, 'n', 'u', 't', 'e', 's')
                            .then_some(TokenKind::MinutesKeyword),
                        Some('o') => scan_chars!(input, 'd', 'i', 'f', 'i', 'e', 'r')
                            .then_some(TokenKind::ModifierKeyword),
                        Some('u') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(input, 't', 'a', 'b', 'l', 'e')
                                    .then_some(TokenKind::MutableKeyword)
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
                        Some('e') => scan_chars!(input, 'w').then_some(TokenKind::NewKeyword),
                        Some('u') => scan_chars!(input, 'l', 'l').then_some(TokenKind::NullKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('o') => match input.next() {
                        Some('f') => Some(TokenKind::OfKeyword),
                        Some('v') => scan_chars!(input, 'e', 'r', 'r', 'i', 'd', 'e')
                            .then_some(TokenKind::OverrideKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('p') => match input.next() {
                        Some('a') => match input.next() {
                            Some('r') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 't', 'i', 'a', 'l')
                                        .then_some(TokenKind::PartialKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('y') => scan_chars!(input, 'a', 'b', 'l', 'e')
                                .then_some(TokenKind::PayableKeyword),
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('r') => match input.next() {
                            Some('a') => scan_chars!(input, 'g', 'm', 'a')
                                .then_some(TokenKind::PragmaKeyword),
                            Some('i') => scan_chars!(input, 'v', 'a', 't', 'e')
                                .then_some(TokenKind::PrivateKeyword),
                            Some('o') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'm', 'i', 's', 'e')
                                        .then_some(TokenKind::PromiseKeyword)
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
                            Some('b') => scan_chars!(input, 'l', 'i', 'c')
                                .then_some(TokenKind::PublicKeyword),
                            Some('r') => scan_chars!(input, 'e').then_some(TokenKind::PureKeyword),
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
                                Some('c') => scan_chars!(input, 'e', 'i', 'v', 'e')
                                    .then_some(TokenKind::ReceiveKeyword),
                                Some('f') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(input, 'e', 'r', 'e', 'n', 'c', 'e')
                                            .then_some(TokenKind::ReferenceKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('l') => {
                                    scan_chars!(input, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e')
                                        .then_some(TokenKind::RelocatableKeyword)
                                }
                                Some('t') => {
                                    if scan_chars!(input, 'u', 'r', 'n') {
                                        match input.next() {
                                            Some('s') => Some(TokenKind::ReturnsKeyword),
                                            Some(_) => {
                                                input.undo();
                                                Some(TokenKind::ReturnKeyword)
                                            }
                                            None => Some(TokenKind::ReturnKeyword),
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('v') => scan_chars!(input, 'e', 'r', 't')
                                    .then_some(TokenKind::RevertKeyword),
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
                    Some('s') => {
                        match input.next() {
                            Some('e') => match input.next() {
                                Some('a') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(input, 'l', 'e', 'd')
                                            .then_some(TokenKind::SealedKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('c') => scan_chars!(input, 'o', 'n', 'd', 's')
                                    .then_some(TokenKind::SecondsKeyword),
                                Some(_) => {
                                    input.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('i') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'z', 'e', 'o', 'f')
                                        .then_some(TokenKind::SizeofKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('o') => scan_chars!(input, 'l', 'i', 'd', 'i', 't', 'y')
                                .then_some(TokenKind::SolidityKeyword),
                            Some('t') => match input.next() {
                                Some('a') => scan_chars!(input, 't', 'i', 'c')
                                    .then_some(TokenKind::StaticKeyword),
                                Some('o') => scan_chars!(input, 'r', 'a', 'g', 'e')
                                    .then_some(TokenKind::StorageKeyword),
                                Some('r') => match input.next() {
                                    Some('i') => scan_chars!(input, 'n', 'g')
                                        .then_some(TokenKind::StringKeyword),
                                    Some('u') => scan_chars!(input, 'c', 't')
                                        .then_some(TokenKind::StructKeyword),
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
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(input, 'p', 'p', 'o', 'r', 't', 's')
                                        .then_some(TokenKind::SupportsKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('w') => scan_chars!(input, 'i', 't', 'c', 'h')
                                .then_some(TokenKind::SwitchKeyword),
                            Some('z') => {
                                scan_chars!(input, 'a', 'b', 'o').then_some(TokenKind::SzaboKeyword)
                            }
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        }
                    }
                    Some('t') => match input.next() {
                        Some('h') => {
                            scan_chars!(input, 'r', 'o', 'w').then_some(TokenKind::ThrowKeyword)
                        }
                        Some('r') => match input.next() {
                            Some('u') => scan_chars!(input, 'e').then_some(TokenKind::TrueKeyword),
                            Some('y') => {
                                if self.version_is_at_least_0_6_0 {
                                    Some(TokenKind::TryKeyword)
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
                                        if self.version_is_at_least_0_5_0 {
                                            scan_chars!(input, 'e', 'f')
                                                .then_some(TokenKind::TypedefKeyword)
                                        } else {
                                            None
                                        }
                                    }
                                    Some('o') => {
                                        scan_chars!(input, 'f').then_some(TokenKind::TypeofKeyword)
                                    }
                                    Some(_) => {
                                        input.undo();
                                        Some(TokenKind::TypeKeyword)
                                    }
                                    None => Some(TokenKind::TypeKeyword),
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
                            if self.version_is_at_least_0_8_0 {
                                scan_chars!(input, 'c', 'h', 'e', 'c', 'k', 'e', 'd')
                                    .then_some(TokenKind::UncheckedKeyword)
                            } else {
                                None
                            }
                        }
                        Some('s') => {
                            scan_chars!(input, 'i', 'n', 'g').then_some(TokenKind::UsingKeyword)
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('v') => match input.next() {
                        Some('a') => scan_chars!(input, 'r').then_some(TokenKind::VarKeyword),
                        Some('i') => match input.next() {
                            Some('e') => scan_chars!(input, 'w').then_some(TokenKind::ViewKeyword),
                            Some('r') => {
                                if self.version_is_at_least_0_6_0 {
                                    scan_chars!(input, 't', 'u', 'a', 'l')
                                        .then_some(TokenKind::VirtualKeyword)
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
                                scan_chars!(input, 'k', 's').then_some(TokenKind::WeeksKeyword)
                            }
                            Some('i') => Some(TokenKind::WeiKeyword),
                            Some(_) => {
                                input.undo();
                                None
                            }
                            None => None,
                        },
                        Some('h') => {
                            scan_chars!(input, 'i', 'l', 'e').then_some(TokenKind::WhileKeyword)
                        }
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('y') => {
                        scan_chars!(input, 'e', 'a', 'r', 's').then_some(TokenKind::YearsKeyword)
                    }
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(input) {
                        furthest_position = input.position();
                        longest_token = Some(kind);
                    }
                }
                input.set_position(save);

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
                    longest_token = Some(kind);
                }
                input.set_position(save);

                longest_match! {
                        { AsciiStringLiteral = ascii_string_literal }
                        { DecimalLiteral = decimal_literal }
                        { EndOfLine = end_of_line }
                        { FixedBytesType = fixed_bytes_type }
                        { HexLiteral = hex_literal }
                        { HexStringLiteral = hex_string_literal }
                        { MultilineComment = multiline_comment }
                        { SignedFixedType = signed_fixed_type }
                        { SignedIntegerType = signed_integer_type }
                        { SingleLineComment = single_line_comment }
                        { UnicodeStringLiteral = unicode_string_literal }
                        { UnsignedFixedType = unsigned_fixed_type }
                        { UnsignedIntegerType = unsigned_integer_type }
                        { Whitespace = whitespace }
                        { Identifier = identifier }
                }
            }
            LexicalContext::VersionPragma => {
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

                if let Some(kind) = scan_chars!(input, 's', 'o', 'l', 'i', 'd', 'i', 't', 'y')
                    .then_some(TokenKind::SolidityKeyword)
                {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(input) {
                        furthest_position = input.position();
                        longest_token = Some(kind);
                    }
                }
                input.set_position(save);

                if let Some(kind) = match input.next() {
                    Some('-') => Some(TokenKind::Minus),
                    Some('.') => Some(TokenKind::Period),
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
                    Some('|') => scan_chars!(input, '|').then_some(TokenKind::BarBar),
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
                        { VersionPragmaValue = version_pragma_value }
                }
            }
            LexicalContext::YulBlock => {
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

                if let Some(kind) = match input.next() {
                    Some('b') => {
                        scan_chars!(input, 'r', 'e', 'a', 'k').then_some(TokenKind::BreakKeyword)
                    }
                    Some('c') => match input.next() {
                        Some('a') => scan_chars!(input, 's', 'e').then_some(TokenKind::CaseKeyword),
                        Some('o') => scan_chars!(input, 'n', 't', 'i', 'n', 'u', 'e')
                            .then_some(TokenKind::ContinueKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('d') => scan_chars!(input, 'e', 'f', 'a', 'u', 'l', 't')
                        .then_some(TokenKind::DefaultKeyword),
                    Some('f') => match input.next() {
                        Some('a') => {
                            scan_chars!(input, 'l', 's', 'e').then_some(TokenKind::FalseKeyword)
                        }
                        Some('o') => scan_chars!(input, 'r').then_some(TokenKind::ForKeyword),
                        Some('u') => scan_chars!(input, 'n', 'c', 't', 'i', 'o', 'n')
                            .then_some(TokenKind::FunctionKeyword),
                        Some(_) => {
                            input.undo();
                            None
                        }
                        None => None,
                    },
                    Some('h') => scan_chars!(input, 'e', 'x').then_some(TokenKind::HexKeyword),
                    Some('i') => scan_chars!(input, 'f').then_some(TokenKind::IfKeyword),
                    Some('l') => {
                        if scan_chars!(input, 'e') {
                            match input.next() {
                                Some('a') => {
                                    if self.version_is_at_least_0_6_0 {
                                        scan_chars!(input, 'v', 'e')
                                            .then_some(TokenKind::LeaveKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => Some(TokenKind::LetKeyword),
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
                    Some('s') => scan_chars!(input, 'w', 'i', 't', 'c', 'h')
                        .then_some(TokenKind::SwitchKeyword),
                    Some('t') => {
                        scan_chars!(input, 'r', 'u', 'e').then_some(TokenKind::TrueKeyword)
                    }
                    Some(_) => {
                        input.undo();
                        None
                    }
                    None => None,
                } {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(input) {
                        furthest_position = input.position();
                        longest_token = Some(kind);
                    }
                }
                input.set_position(save);

                if let Some(kind) = match input.next() {
                    Some('(') => Some(TokenKind::OpenParen),
                    Some(')') => Some(TokenKind::CloseParen),
                    Some(',') => Some(TokenKind::Comma),
                    Some('-') => scan_chars!(input, '>').then_some(TokenKind::MinusGreaterThan),
                    Some('.') => Some(TokenKind::Period),
                    Some(':') => scan_chars!(input, '=').then_some(TokenKind::ColonEqual),
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
                        { YulIdentifier = yul_identifier }
                }
            }
        }

        match longest_token {
            Some(..) => {
                input.set_position(furthest_position);
                longest_token
            }
            // Skip a character if possible and if we didn't recognize a token
            None if input.peek().is_some() => {
                let _ = input.next();
                Some(TokenKind::SKIPPED)
            }
            // EOF
            None => None,
        }
    }
}

#[cfg(feature = "slang_napi_interfaces")]
#[napi(namespace = "language")]
impl Language {
    #[napi(constructor)]
    pub fn new_napi(version: String) -> std::result::Result<Self, napi::Error> {
        let version =
            Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
        Self::new(version).map_err(|e| e.into())
    }

    #[napi(getter, js_name = "version")]
    pub fn version_napi(&self) -> String {
        self.version.to_string()
    }

    #[napi(js_name = "supportedVersions")]
    pub fn supported_versions_napi() -> Vec<String> {
        return Self::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect();
    }

    #[napi(js_name = "scan")]
    pub fn scan_napi(&self, lexical_context: LexicalContext, input: String) -> Option<TokenKind> {
        self.scan(lexical_context, input.as_str())
    }

    #[napi(js_name = "parse", ts_return_type = "ParseOutput")]
    pub fn parse_napi(&self, production_kind: ProductionKind, input: String) -> NAPIParseOutput {
        self.parse(production_kind, input.as_str()).into()
    }
}
