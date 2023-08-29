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
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::Default as u8 }>(self, stream, kind)
    }

    #[allow(dead_code)]
    fn default_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::Default as u8 }>(self, stream, kind)
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token_with_trivia(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::VersionPragma as u8 }>(
            self, stream, kind,
        )
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::VersionPragma as u8 }>(self, stream, kind)
    }

    #[allow(dead_code)]
    fn yul_block_parse_token_with_trivia(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        Lexer::parse_token_with_trivia::<{ LexicalContext::YulBlock as u8 }>(self, stream, kind)
    }

    #[allow(dead_code)]
    fn yul_block_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        Lexer::parse_token::<{ LexicalContext::YulBlock as u8 }>(self, stream, kind)
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn abi_coder_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ABICoderKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ABICoderPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn address_type(&self, stream: &mut Stream) -> ParserResult {
        {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::AddressKeyword),
                        )?;
                        seq.elem(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword),
                        ))?;
                        seq.finish()
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        }
        .with_kind(RuleKind::AddressType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenParen))?;
                seq.elem(OptionalHelper::transform({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self.positional_arguments_list(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.named_arguments_declaration(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                }))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseParen))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket))?;
                seq.elem(self.array_values_list(stream))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ArrayExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.expression(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.expression(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ArrayValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral)
        })
        .with_kind(RuleKind::AsciiStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                )?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.default_parse_token_with_trivia(
                            stream,
                            TokenKind::AsciiStringLiteral,
                        ))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::AssemblyFlagsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::AssemblyKeyword))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                ))?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem(self.assembly_flags_list(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.elem(self.yul_block(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::AssemblyStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn block(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace))?;
                seq.elem(OptionalHelper::transform(self.statements_list(stream)))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::Block)
    }

    #[allow(unused_assignments, unused_parens)]
    fn break_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::BreakKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::BreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::CatchKeyword),
                    )?;
                    seq.elem(OptionalHelper::transform(self.catch_clause_error(stream)))?;
                    seq.elem(self.block(stream))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause_error(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    ))?;
                    seq.elem(self.parameters_declaration(stream))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClauseError)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clauses_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| self.catch_clause(stream))
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::CatchClausesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constant_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_7_4 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(self.type_name(stream))?;
                            seq.elem(self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ConstantKeyword,
                            ))?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            )?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                            )?;
                            seq.elem(self.expression(stream))?;
                            seq.finish()
                        })
                    })?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstantDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_4_22 {
                    {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.modifier_invocation(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::InternalKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    }
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
    fn constructor_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::ConstructorKeyword),
                    )?;
                    seq.elem(self.parameters_declaration(stream))?;
                    seq.elem(OptionalHelper::transform(
                        self.constructor_attributes_list(stream),
                    ))?;
                    seq.elem(self.block(stream))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ConstructorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn continue_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ContinueKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                if self.version_is_at_least_0_6_0 {
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::AbstractKeyword),
                    ))?;
                }
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ContractKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(OptionalHelper::transform(
                    self.inheritance_specifier(stream),
                ))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(OptionalHelper::transform(
                            self.contract_members_list(stream),
                        ))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ContractDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.using_directive(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.function_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.modifier_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.struct_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.enum_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.event_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.state_variable_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.fallback_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.receive_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    };
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::ContractMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(self.deconstruction_import_symbols_list(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::FromKeyword))?;
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                )?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::DeconstructionImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbol(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::DeconstructionImportSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.deconstruction_import_symbol(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.deconstruction_import_symbol(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::DeconstructionImportSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn delete_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::DeleteKeyword),
                        )?;
                        seq.elem(self.expression(stream))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::DeleteStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn do_while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::DoKeyword),
                        )?;
                        seq.elem(self.statement(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::WhileKeyword),
                        )?;
                        seq.elem({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::OpenParen,
                                ))?;
                                seq.elem(self.expression(stream))?;
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::CloseParen,
                                ))?;
                                seq.finish()
                            })
                        })?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::DoWhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn emit_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_21 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EmitKeyword,
                                ),
                            )?;
                            seq.elem(self.identifier_path(stream))?;
                            seq.elem(self.arguments_declaration(stream))?;
                            seq.finish()
                        })
                    })?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::EmitStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_file_trivia(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.default_parse_token(stream, TokenKind::Whitespace);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::EndOfLine);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::MultilineComment);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::SingleLineComment);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::EnumKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(OptionalHelper::transform(self.identifiers_list(stream)))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::EnumDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::ErrorKeyword,
                                ),
                            )?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            )?;
                            seq.elem({
                                SequenceHelper::run(|mut seq| {
                                    seq.elem(self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::OpenParen,
                                    ))?;
                                    seq.elem(OptionalHelper::transform(
                                        self.error_parameters_list(stream),
                                    ))?;
                                    seq.elem(self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CloseParen,
                                    ))?;
                                    seq.finish()
                                })
                            })?;
                            seq.finish()
                        })
                    })?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameter(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.type_name(stream))?;
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    ))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.error_parameter(stream))?;
                    seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                            )?;
                            seq.elem(self.error_parameter(stream))?;
                            seq.finish()
                        })
                    }))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ErrorParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::EventKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.elem({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::OpenParen,
                                ))?;
                                seq.elem(OptionalHelper::transform(
                                    self.event_parameters_list(stream),
                                ))?;
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::CloseParen,
                                ))?;
                                seq.finish()
                            })
                        })?;
                        seq.elem(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::AnonymousKeyword,
                            ),
                        ))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::EventDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.type_name(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::IndexedKeyword),
                ))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::EventParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.event_parameter(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.event_parameter(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::EventParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::ExperimentalKeyword),
                )?;
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ExperimentalPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_assignment_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 1u8, 1u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Equal);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::BarEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::CaretEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::SlashEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::PercentEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::AsteriskEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::AmpersandEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self
                        .default_parse_token_with_trivia(stream, TokenKind::LessThanLessThanEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanEqual,
                    );
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                    );
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_conditional_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::ConditionalExpression, 3u8, {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::QuestionMark),
                    )?;
                    seq.elem(self.expression(stream))?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Colon))?;
                    seq.elem(self.expression(stream))?;
                    seq.finish()
                })
            })
        };
        let parse_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                5u8,
                5u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::BarBar),
            )
        };
        let parse_and_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                7u8,
                7u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::AmpersandAmpersand),
            )
        };
        let parse_equality_comparison_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 9u8, 9u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::EqualEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::BangEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_order_comparison_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 11u8, 11u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::LessThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::GreaterThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::LessThanEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::GreaterThanEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_bitwise_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                13u8,
                13u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::Bar),
            )
        };
        let parse_bitwise_x_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                15u8,
                15u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::Caret),
            )
        };
        let parse_bitwise_and_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                17u8,
                17u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::Ampersand),
            )
        };
        let parse_shift_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 19u8, 19u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::LessThanLessThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self
                        .default_parse_token_with_trivia(stream, TokenKind::GreaterThanGreaterThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanGreaterThan,
                    );
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_add_sub_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 21u8, 21u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Plus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Minus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_mul_div_mod_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(RuleKind::BinaryExpression, 23u8, 23u8 + 1, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Asterisk);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Slash);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Percent);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_exponentiation_operator_removed_from_0_6_0 = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                25u8,
                25u8 + 1,
                self.default_parse_token_with_trivia(stream, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_exponentiation_operator_introduced_from_0_6_0 = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::BinaryExpression,
                27u8 + 1,
                27u8,
                self.default_parse_token_with_trivia(stream, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_unary_postfix_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::UnaryPostfixExpression, 29u8, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusPlus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusMinus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let parse_unary_prefix_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_prefix_operator(RuleKind::UnaryPrefixExpression, 31u8, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusPlus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusMinus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Tilde);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Bang);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Minus);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    if !self.version_is_at_least_0_5_0 {
                        let result = self.default_parse_token_with_trivia(stream, TokenKind::Plus);
                        choice.consider(result).pick_or_backtrack(stream)?;
                    }
                    choice.finish(stream)
                })
            })
        };
        let parse_function_call_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::FunctionCallExpression, 33u8, {
                SequenceHelper::run(|mut seq| {
                    if self.version_is_at_least_0_6_2 {
                        seq.elem(OptionalHelper::transform(
                            self.function_call_options(stream),
                        ))?;
                    }
                    seq.elem(self.arguments_declaration(stream))?;
                    seq.finish()
                })
            })
        };
        let parse_member_access_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::MemberAccessExpression, 35u8, {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Period))?;
                    seq.elem({
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::AddressKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    })?;
                    seq.finish()
                })
            })
        };
        let parse_index_access_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::IndexAccessExpression, 37u8, {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket))?;
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                            seq.elem(OptionalHelper::transform({
                                SequenceHelper::run(|mut seq| {
                                    seq.elem(self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::Colon,
                                    ))?;
                                    seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                                    seq.finish()
                                })
                            }))?;
                            seq.finish()
                        })
                    })?;
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket),
                    )?;
                    seq.finish()
                })
            })
        };
        let prefix_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_unary_prefix_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let primary_expression_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.new_expression(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.tuple_expression(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.array_expression(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::TrueKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FalseKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.numeric_expression(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self.hex_string_literals_list(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.ascii_string_literals_list(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if self.version_is_at_least_0_7_0 {
                            let result = self.unicode_string_literals_list(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::BoolKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StringKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.address_type(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FixedBytesType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedIntegerType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.default_parse_token_with_trivia(
                            stream,
                            TokenKind::UnsignedIntegerType,
                        );
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedFixedType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::UnsignedFixedType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if !self.version_is_at_least_0_8_0 {
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ByteKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_5_3 {
                    let result = self.type_expression(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_conditional_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_unary_postfix_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_function_call_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_member_access_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_index_access_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let binary_operand_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    prefix_operator_parser(stream)
                }))?;
                seq.elem(primary_expression_parser(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                }))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_assignment_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_or_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_and_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_equality_comparison_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_order_comparison_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_bitwise_or_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_bitwise_x_or_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_bitwise_and_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_shift_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_add_sub_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_mul_div_mod_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if !self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_removed_from_0_6_0(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_introduced_from_0_6_0(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        };
        let linear_expression_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(binary_operand_parser(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(binary_operator_parser(stream))?;
                        seq.elem(binary_operand_parser(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::Expression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.expression(stream))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ExpressionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_6_0 {
                    {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.modifier_invocation(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.override_specifier(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    }
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
    fn fallback_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::FallbackKeyword),
                    )?;
                    seq.elem(self.parameters_declaration(stream))?;
                    seq.elem(OptionalHelper::transform(
                        self.fallback_function_attributes_list(stream),
                    ))?;
                    seq.elem(OptionalHelper::transform(self.returns_declaration(stream)))?;
                    seq.elem({
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.block(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    })?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::FallbackFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ForKeyword))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem({
                            SequenceHelper::run(|mut seq| {
                                seq.elem({
                                    ChoiceHelper::run(stream, |mut choice, stream| {
                                        let result = {
                                            ChoiceHelper::run(stream, |mut choice, stream| {
                                                let result = self.expression_statement(stream);
                                                choice
                                                    .consider(result)
                                                    .pick_or_backtrack(stream)?;
                                                let result =
                                                    self.variable_declaration_statement(stream);
                                                choice
                                                    .consider(result)
                                                    .pick_or_backtrack(stream)?;
                                                let result =
                                                    self.tuple_deconstruction_statement(stream);
                                                choice
                                                    .consider(result)
                                                    .pick_or_backtrack(stream)?;
                                                choice.finish(stream)
                                            })
                                        };
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Semicolon,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        choice.finish(stream)
                                    })
                                })?;
                                seq.elem({
                                    ChoiceHelper::run(stream, |mut choice, stream| {
                                        let result = self.expression_statement(stream);
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Semicolon,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        choice.finish(stream)
                                    })
                                })?;
                                seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                                seq.finish()
                            })
                        })?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.statement(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.modifier_invocation(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.override_specifier(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ExternalKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                if !self.version_is_at_least_0_5_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::ConstantKeyword);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::FunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_options(&self, stream: &mut Stream) -> ParserResult {
        {
            ChoiceHelper::run(stream, |mut choice, stream| {
                if self.version_is_at_least_0_6_2 && !self.version_is_at_least_0_8_0 {
                    let result = OneOrMoreHelper::run(stream, |stream| {
                        self.named_arguments_declaration(stream)
                    });
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_0 {
                    let result = self.named_arguments_declaration(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        }
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword))?;
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::FallbackKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::ReceiveKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.elem(self.parameters_declaration(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.function_attributes_list(stream),
                ))?;
                seq.elem(OptionalHelper::transform(self.returns_declaration(stream)))?;
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.block(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::FunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword))?;
                seq.elem(self.parameters_declaration(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.function_type_attributes_list(stream),
                ))?;
                seq.elem(OptionalHelper::transform(self.returns_declaration(stream)))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::FunctionType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ExternalKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::FunctionTypeAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            self.default_parse_token_with_trivia(stream, TokenKind::HexStringLiteral)
        })
        .with_kind(RuleKind::HexStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Period))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.identifier_path(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.identifier_path(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::IdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::IdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::IfKeyword))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem(self.expression(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.statement(stream))?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::ElseKeyword),
                        )?;
                        seq.elem(self.statement(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::IfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::ImportKeyword),
                        )?;
                        seq.elem({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.path_import(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.named_import(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.deconstruction_import(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                choice.finish(stream)
                            })
                        })?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ImportDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::IsKeyword))?;
                seq.elem(self.inheritance_types_list(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.identifier_path(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.arguments_declaration(stream),
                ))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::InheritanceType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_types_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.inheritance_type(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.inheritance_type(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::InheritanceTypesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::InterfaceKeyword),
                )?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(OptionalHelper::transform(
                    self.inheritance_specifier(stream),
                ))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(OptionalHelper::transform(
                            self.interface_members_list(stream),
                        ))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::InterfaceDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.using_directive(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.function_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.modifier_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.struct_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.enum_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.event_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.state_variable_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.fallback_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.receive_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    };
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::InterfaceMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.default_parse_token(stream, TokenKind::Whitespace);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::EndOfLine);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::MultilineComment);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token(stream, TokenKind::SingleLineComment);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::LeadingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::LibraryKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(OptionalHelper::transform(self.library_members_list(stream)))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::LibraryDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.using_directive(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.function_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.modifier_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.struct_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.enum_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.event_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.state_variable_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.fallback_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.receive_function_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    };
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::LibraryMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key_type(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = {
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::BoolKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StringKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.address_type(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::FixedBytesType,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SignedIntegerType,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::UnsignedIntegerType,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SignedFixedType,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::UnsignedFixedType,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                if !self.version_is_at_least_0_8_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::ByteKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                choice.finish(stream)
                            })
                        };
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.identifier_path(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                if self.version_is_at_least_0_8_18 {
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    ))?;
                }
                seq.finish()
            })
        }
        .with_kind(RuleKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::MappingKeyword))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(self.mapping_key_type(stream))?;
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EqualGreaterThan,
                                ))?;
                                seq.elem(self.mapping_value_type(stream))?;
                                seq.finish()
                            })
                        })?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::MappingType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_value_type(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.type_name(stream))?;
                if self.version_is_at_least_0_8_18 {
                    seq.elem(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    ))?;
                }
                seq.finish()
            })
        }
        .with_kind(RuleKind::MappingValueType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.override_specifier(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::ModifierAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ModifierKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(OptionalHelper::transform(
                    self.parameters_declaration(stream),
                ))?;
                seq.elem(OptionalHelper::transform(
                    self.modifier_attributes_list(stream),
                ))?;
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.block(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.identifier_path(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.arguments_declaration(stream),
                ))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ModifierInvocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Colon))?;
                seq.elem(self.expression(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::NamedArgument)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace))?;
                seq.elem(OptionalHelper::transform(self.named_arguments_list(stream)))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::NamedArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.named_argument(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.named_argument(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::NamedArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_import(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Asterisk))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::FromKeyword))?;
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                )?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::NamedImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn new_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::NewKeyword))?;
                seq.elem(self.type_name(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::NewExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn numeric_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::HexLiteral),
                        )?;
                        if !self.version_is_at_least_0_5_0 {
                            seq.elem(OptionalHelper::transform({
                                ChoiceHelper::run(stream, |mut choice, stream| {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::DaysKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::EtherKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::HoursKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::MinutesKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SecondsKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::WeeksKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::WeiKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    if !self.version_is_at_least_0_5_0 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YearsKeyword,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                    }
                                    if self.version_is_at_least_0_6_11 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GweiKeyword,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                    }
                                    if !self.version_is_at_least_0_7_0 {
                                        let result = {
                                            ChoiceHelper::run(stream, |mut choice, stream| {
                                                let result = self.default_parse_token_with_trivia(
                                                    stream,
                                                    TokenKind::FinneyKeyword,
                                                );
                                                choice
                                                    .consider(result)
                                                    .pick_or_backtrack(stream)?;
                                                let result = self.default_parse_token_with_trivia(
                                                    stream,
                                                    TokenKind::SzaboKeyword,
                                                );
                                                choice
                                                    .consider(result)
                                                    .pick_or_backtrack(stream)?;
                                                choice.finish(stream)
                                            })
                                        };
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                    }
                                    choice.finish(stream)
                                })
                            }))?;
                        }
                        seq.finish()
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::DecimalLiteral),
                        )?;
                        seq.elem(OptionalHelper::transform({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::DaysKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EtherKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::HoursKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MinutesKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SecondsKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::WeeksKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self
                                    .default_parse_token_with_trivia(stream, TokenKind::WeiKeyword);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                if !self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::YearsKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                if self.version_is_at_least_0_6_11 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::GweiKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                if !self.version_is_at_least_0_7_0 {
                                    let result = {
                                        ChoiceHelper::run(stream, |mut choice, stream| {
                                            let result = self.default_parse_token_with_trivia(
                                                stream,
                                                TokenKind::FinneyKeyword,
                                            );
                                            choice.consider(result).pick_or_backtrack(stream)?;
                                            let result = self.default_parse_token_with_trivia(
                                                stream,
                                                TokenKind::SzaboKeyword,
                                            );
                                            choice.consider(result).pick_or_backtrack(stream)?;
                                            choice.finish(stream)
                                        })
                                    };
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                choice.finish(stream)
                            })
                        }))?;
                        seq.finish()
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        }
        .with_kind(RuleKind::NumericExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OverrideKeyword))?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem(OptionalHelper::transform(
                            self.identifier_paths_list(stream),
                        ))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.type_name(stream))?;
                seq.elem(OptionalHelper::transform({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::MemoryKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StorageKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if self.version_is_at_least_0_5_0 {
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::CalldataKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                }))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::Parameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenParen))?;
                seq.elem(OptionalHelper::transform(self.parameters_list(stream)))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseParen))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.parameter(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.parameter(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn path_import(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                )?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                        )?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.expression(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.expression(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::PositionalArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::PragmaKeyword),
                        )?;
                        seq.elem({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.abi_coder_pragma(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.experimental_pragma(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.version_pragma(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                choice.finish(stream)
                            })
                        })?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::PragmaDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_6_0 {
                    {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.modifier_invocation(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.override_specifier(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    }
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
    fn receive_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::ReceiveKeyword),
                    )?;
                    seq.elem(self.parameters_declaration(stream))?;
                    seq.elem(OptionalHelper::transform(
                        self.receive_function_attributes_list(stream),
                    ))?;
                    seq.elem({
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.block(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    })?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ReceiveFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn return_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::ReturnKeyword),
                        )?;
                        seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ReturnStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::ReturnsKeyword))?;
                seq.elem(self.parameters_declaration(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::ReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn revert_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::RevertKeyword),
                        )?;
                        seq.elem(OptionalHelper::transform(self.identifier_path(stream)))?;
                        seq.elem(self.arguments_declaration(stream))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::RevertStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(OptionalHelper::transform(
                    self.source_unit_members_list(stream),
                ))?;
                seq.elem(OptionalHelper::transform(self.end_of_file_trivia(stream)))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.pragma_directive(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.import_directive(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.contract_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.interface_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.library_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.struct_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.enum_definition(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    };
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_7_1 {
                    let result = self.function_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_7_4 {
                    let result = self.constant_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                if self.version_is_at_least_0_8_13 {
                    let result = self.using_directive(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::SourceUnitMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.override_specifier(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ConstantKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_6_5 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::ImmutableKeyword);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::StateVariableAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.type_name(stream))?;
                        seq.elem(OptionalHelper::transform(
                            self.state_variable_attributes_list(stream),
                        ))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.elem(OptionalHelper::transform({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(
                                    self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                                )?;
                                seq.elem(self.expression(stream))?;
                                seq.finish()
                            })
                        }))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::StateVariableDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statement(&self, stream: &mut Stream) -> ParserResult {
        {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self.expression_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.variable_declaration_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.tuple_deconstruction_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self.if_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.for_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.while_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.do_while_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.continue_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.break_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.delete_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.return_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.revert_statement(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if self.version_is_at_least_0_4_21 {
                            let result = self.emit_statement(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        if !self.version_is_at_least_0_5_0 {
                            let result = self.throw_statement(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        if self.version_is_at_least_0_6_0 {
                            let result = self.try_statement(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.assembly_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.block(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_8_0 {
                    let result = self.unchecked_block(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        }
        .with_kind(RuleKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.statement(stream))
            .with_kind(RuleKind::StatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::StructKeyword))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        )?;
                        seq.elem(OptionalHelper::transform(self.struct_members_list(stream)))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::StructDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_member(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.type_name(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::StructMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.struct_member(stream))
            .with_kind(RuleKind::StructMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn throw_statement(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::ThrowKeyword),
                    )?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::ThrowStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token(stream, TokenKind::Whitespace),
                ))?;
                seq.elem(OptionalHelper::transform(
                    self.default_parse_token(stream, TokenKind::SingleLineComment),
                ))?;
                seq.elem(self.default_parse_token(stream, TokenKind::EndOfLine))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::TrailingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn try_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::TryKeyword))?;
                    seq.elem(self.expression(stream))?;
                    seq.elem(OptionalHelper::transform(self.returns_declaration(stream)))?;
                    seq.elem(self.block(stream))?;
                    seq.elem(self.catch_clauses_list(stream))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::TryStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::OpenParen,
                                ))?;
                                seq.elem(OptionalHelper::transform(
                                    self.tuple_members_list(stream),
                                ))?;
                                seq.elem(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::CloseParen,
                                ))?;
                                seq.finish()
                            })
                        })?;
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Equal))?;
                        seq.elem(self.expression(stream))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenParen))?;
                seq.elem(self.tuple_values_list(stream))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseParen))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::TupleExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_member(&self, stream: &mut Stream) -> ParserResult {
        OptionalHelper::transform({
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.type_name(stream))?;
                        seq.elem(OptionalHelper::transform({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MemoryKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StorageKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                if self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CalldataKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                choice.finish(stream)
                            })
                        }))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(OptionalHelper::transform({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MemoryKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StorageKeyword,
                                );
                                choice.consider(result).pick_or_backtrack(stream)?;
                                if self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CalldataKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                }
                                choice.finish(stream)
                            })
                        }))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        )?;
                        seq.finish()
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        })
        .with_kind(RuleKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.tuple_member(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.tuple_member(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::TupleMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::TupleValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_5_3 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::TypeKeyword))?;
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                            )?;
                            seq.elem(self.type_name(stream))?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                            )?;
                            seq.finish()
                        })
                    })?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::TypeExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_name(&self, stream: &mut Stream) -> ParserResult {
        let parse_array_type_name_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::ArrayTypeName, 1u8, {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket))?;
                    seq.elem(OptionalHelper::transform(self.expression(stream)))?;
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket),
                    )?;
                    seq.finish()
                })
            })
        };
        let primary_expression_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.function_type(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.mapping_type(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::BoolKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StringKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.address_type(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FixedBytesType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedIntegerType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.default_parse_token_with_trivia(
                            stream,
                            TokenKind::UnsignedIntegerType,
                        );
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedFixedType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::UnsignedFixedType);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if !self.version_is_at_least_0_8_0 {
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ByteKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.identifier_path(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_array_type_name_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let linear_expression_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::TypeName),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::TypeName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unchecked_block(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::UncheckedKeyword),
                    )?;
                    seq.elem(self.block(stream))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UncheckedBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            OneOrMoreHelper::run(stream, |stream| {
                self.default_parse_token_with_trivia(stream, TokenKind::UnicodeStringLiteral)
            })
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnicodeStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if !self.version_is_at_least_0_6_0 {
                    {
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result = self.modifier_invocation(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.override_specifier(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    }
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
    fn unnamed_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem(
                        self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                    )?;
                    seq.elem(self.parameters_declaration(stream))?;
                    seq.elem(OptionalHelper::transform(
                        self.unnamed_function_attributes_list(stream),
                    ))?;
                    seq.elem({
                        ChoiceHelper::run(stream, |mut choice, stream| {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            let result = self.block(stream);
                            choice.consider(result).pick_or_backtrack(stream)?;
                            choice.finish(stream)
                        })
                    })?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UnnamedFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn user_defined_value_type_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_8 {
            {
                SequenceHelper::run(|mut seq| {
                    seq.elem({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::TypeKeyword,
                                ),
                            )?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            )?;
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::IsKeyword),
                            )?;
                            seq.elem({
                                ChoiceHelper::run(stream, |mut choice, stream| {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::BoolKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::StringKeyword,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.address_type(stream);
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::FixedBytesType,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SignedIntegerType,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::UnsignedIntegerType,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SignedFixedType,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::UnsignedFixedType,
                                    );
                                    choice.consider(result).pick_or_backtrack(stream)?;
                                    if !self.version_is_at_least_0_8_0 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::ByteKeyword,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                    }
                                    choice.finish(stream)
                                })
                            })?;
                            seq.finish()
                        })
                    })?;
                    seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                    seq.finish()
                })
            }
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::UserDefinedValueTypeDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::UsingKeyword),
                        )?;
                        seq.elem({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self.using_directive_path(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.using_directive_deconstruction(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                choice.finish(stream)
                            })
                        })?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::ForKeyword),
                        )?;
                        seq.elem({
                            ChoiceHelper::run(stream, |mut choice, stream| {
                                let result = self
                                    .default_parse_token_with_trivia(stream, TokenKind::Asterisk);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                let result = self.type_name(stream);
                                choice.consider(result).pick_or_backtrack(stream)?;
                                choice.finish(stream)
                            })
                        })?;
                        seq.elem(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(stream, TokenKind::GlobalKeyword),
                        ))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::UsingDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_deconstruction(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace))?;
                seq.elem(self.using_directive_symbols_list(stream))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::UsingDirectiveDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_path(&self, stream: &mut Stream) -> ParserResult {
        self.identifier_path(stream)
            .with_kind(RuleKind::UsingDirectivePath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbol(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.identifier_path(stream))?;
                if self.version_is_at_least_0_8_19 {
                    seq.elem(OptionalHelper::transform({
                        SequenceHelper::run(|mut seq| {
                            seq.elem(
                                self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                            )?;
                            seq.elem(if self.version_is_at_least_0_8_19 {
                                {
                                    ChoiceHelper::run(stream, |mut choice, stream| {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Ampersand,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Asterisk,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::BangEqual,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Bar,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Caret,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::EqualEqual,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GreaterThan,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GreaterThanEqual,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::LessThan,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::LessThanEqual,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Minus,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Percent,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Plus,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Slash,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Tilde,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        choice.finish(stream)
                                    })
                                }
                            } else {
                                ParserResult::disabled()
                            })?;
                            seq.finish()
                        })
                    }))?;
                }
                seq.finish()
            })
        }
        .with_kind(RuleKind::UsingDirectiveSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.using_directive_symbol(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.using_directive_symbol(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::UsingDirectiveSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        if !self.version_is_at_least_0_5_0 {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::VarKeyword);
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        let result = self.type_name(stream);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.elem(OptionalHelper::transform({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::MemoryKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StorageKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        if self.version_is_at_least_0_5_0 {
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::CalldataKeyword,
                            );
                            choice.consider(result).pick_or_backtrack(stream)?;
                        }
                        choice.finish(stream)
                    })
                }))?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Identifier))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::VariableDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.variable_declaration(stream))?;
                        seq.elem(OptionalHelper::transform({
                            SequenceHelper::run(|mut seq| {
                                seq.elem(
                                    self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                                )?;
                                seq.elem(self.expression(stream))?;
                                seq.finish()
                            })
                        }))?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::Semicolon))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.version_pragma_parse_token_with_trivia(stream, TokenKind::SolidityKeyword),
                )?;
                seq.elem(self.version_pragma_expressions_list(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::VersionPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_version_pragma_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaBinaryExpression,
                1u8,
                1u8 + 1,
                self.version_pragma_parse_token_with_trivia(stream, TokenKind::BarBar),
            )
        };
        let parse_version_pragma_range_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_binary_operator(
                RuleKind::VersionPragmaBinaryExpression,
                3u8,
                3u8 + 1,
                self.version_pragma_parse_token_with_trivia(stream, TokenKind::Minus),
            )
        };
        let parse_version_pragma_unary_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_prefix_operator(RuleKind::VersionPragmaUnaryExpression, 5u8, {
                ChoiceHelper::run(stream, |mut choice, stream| {
                    let result =
                        self.version_pragma_parse_token_with_trivia(stream, TokenKind::Caret);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(stream, TokenKind::Tilde);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(stream, TokenKind::Equal);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(stream, TokenKind::LessThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result =
                        self.version_pragma_parse_token_with_trivia(stream, TokenKind::GreaterThan);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self
                        .version_pragma_parse_token_with_trivia(stream, TokenKind::LessThanEqual);
                    choice.consider(result).pick_or_backtrack(stream)?;
                    let result = self.version_pragma_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanEqual,
                    );
                    choice.consider(result).pick_or_backtrack(stream)?;
                    choice.finish(stream)
                })
            })
        };
        let prefix_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_version_pragma_unary_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let primary_expression_parser = |stream: &mut Stream| self.version_pragma_specifier(stream);
        let binary_operand_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    prefix_operator_parser(stream)
                }))?;
                seq.elem(primary_expression_parser(stream))?;
                seq.finish()
            })
        };
        let binary_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_version_pragma_or_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = parse_version_pragma_range_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let linear_expression_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(binary_operand_parser(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(binary_operator_parser(stream))?;
                        seq.elem(binary_operand_parser(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::VersionPragmaExpression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::VersionPragmaExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.version_pragma_expression(stream))
            .with_kind(RuleKind::VersionPragmaExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.version_pragma_parse_token_with_trivia(
                    stream,
                    TokenKind::VersionPragmaValue,
                ))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.version_pragma_parse_token_with_trivia(stream, TokenKind::Period),
                        )?;
                        seq.elem(self.version_pragma_parse_token_with_trivia(
                            stream,
                            TokenKind::VersionPragmaValue,
                        ))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.default_parse_token_with_trivia(stream, TokenKind::WhileKeyword))?;
                seq.elem({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        )?;
                        seq.elem(self.expression(stream))?;
                        seq.elem(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        )?;
                        seq.finish()
                    })
                })?;
                seq.elem(self.statement(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_identifier_paths_list(stream))?;
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::ColonEqual))?;
                seq.elem(self.yul_expression(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_block(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenBrace))?;
                seq.elem(OptionalHelper::transform(self.yul_statements_list(stream)))?;
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseBrace))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_break_statement(&self, stream: &mut Stream) -> ParserResult {
        self.yul_block_parse_token_with_trivia(stream, TokenKind::BreakKeyword)
            .with_kind(RuleKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, stream: &mut Stream) -> ParserResult {
        self.yul_block_parse_token_with_trivia(stream, TokenKind::ContinueKeyword)
            .with_kind(RuleKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::LetKeyword))?;
                seq.elem(self.yul_identifier_paths_list(stream))?;
                seq.elem(OptionalHelper::transform({
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::ColonEqual),
                        )?;
                        seq.elem(self.yul_expression(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_yul_function_call_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_postfix_operator(RuleKind::YulFunctionCallExpression, 1u8, {
                SequenceHelper::run(|mut seq| {
                    seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenParen))?;
                    seq.elem(OptionalHelper::transform(self.yul_expressions_list(stream)))?;
                    seq.elem(
                        self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseParen),
                    )?;
                    seq.finish()
                })
            })
        };
        let primary_expression_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = {
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result =
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::TrueKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result =
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::FalseKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::YulHexLiteral);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.yul_block_parse_token_with_trivia(
                            stream,
                            TokenKind::YulDecimalLiteral,
                        );
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::HexStringLiteral);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = self.yul_block_parse_token_with_trivia(
                            stream,
                            TokenKind::AsciiStringLiteral,
                        );
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                };
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_identifier_path(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = parse_yul_function_call_operator(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                choice.finish(stream)
            })
        };
        let linear_expression_parser = |stream: &mut Stream| {
            SequenceHelper::run(|mut seq| {
                seq.elem(primary_expression_parser(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                }))?;
                seq.finish()
            })
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::YulExpression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::YulExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_expression(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.yul_expression(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::ForKeyword))?;
                seq.elem(self.yul_block(stream))?;
                seq.elem(self.yul_expression(stream))?;
                seq.elem(self.yul_block(stream))?;
                seq.elem(self.yul_block(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                )?;
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier))?;
                seq.elem(self.yul_parameters_declaration(stream))?;
                seq.elem(OptionalHelper::transform(
                    self.yul_returns_declaration(stream),
                ))?;
                seq.elem(self.yul_block(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::Period),
                        )?;
                        seq.elem(
                            self.yul_block_parse_token_with_trivia(
                                stream,
                                TokenKind::YulIdentifier,
                            ),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulIdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_identifier_path(stream))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(self.yul_identifier_path(stream))?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulIdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier))?;
                seq.elem(ZeroOrMoreHelper::run(stream, |stream| {
                    SequenceHelper::run(|mut seq| {
                        seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma))?;
                        seq.elem(
                            self.yul_block_parse_token_with_trivia(
                                stream,
                                TokenKind::YulIdentifier,
                            ),
                        )?;
                        seq.finish()
                    })
                }))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulIdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::IfKeyword))?;
                seq.elem(self.yul_expression(stream))?;
                seq.elem(self.yul_block(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.yul_block_parse_token_with_trivia(stream, TokenKind::LeaveKeyword)
        } else {
            ParserResult::disabled()
        }
        .with_kind(RuleKind::YulLeaveStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenParen))?;
                seq.elem(OptionalHelper::transform(self.yul_identifiers_list(stream)))?;
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseParen))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::MinusGreaterThan),
                )?;
                seq.elem(self.yul_identifiers_list(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            ChoiceHelper::run(stream, |mut choice, stream| {
                let result = self.yul_block(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_function_definition(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_declaration_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_assignment_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_if_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_for_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_switch_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_break_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_continue_statement(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                let result = self.yul_expression(stream);
                choice.consider(result).pick_or_backtrack(stream)?;
                if self.version_is_at_least_0_6_0 {
                    let result = self.yul_leave_statement(stream);
                    choice.consider(result).pick_or_backtrack(stream)?;
                }
                choice.finish(stream)
            })
        }
        .with_kind(RuleKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.yul_statement(stream))
            .with_kind(RuleKind::YulStatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_case(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem({
                    ChoiceHelper::run(stream, |mut choice, stream| {
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::DefaultKeyword);
                        choice.consider(result).pick_or_backtrack(stream)?;
                        let result = {
                            SequenceHelper::run(|mut seq| {
                                seq.elem(self.yul_block_parse_token_with_trivia(
                                    stream,
                                    TokenKind::CaseKeyword,
                                ))?;
                                seq.elem({
                                    ChoiceHelper::run(stream, |mut choice, stream| {
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::TrueKeyword,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::FalseKeyword,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YulHexLiteral,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YulDecimalLiteral,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::HexStringLiteral,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::AsciiStringLiteral,
                                        );
                                        choice.consider(result).pick_or_backtrack(stream)?;
                                        choice.finish(stream)
                                    })
                                })?;
                                seq.finish()
                            })
                        };
                        choice.consider(result).pick_or_backtrack(stream)?;
                        choice.finish(stream)
                    })
                })?;
                seq.elem(self.yul_block(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.yul_switch_case(stream))
            .with_kind(RuleKind::YulSwitchCasesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            SequenceHelper::run(|mut seq| {
                seq.elem(self.yul_block_parse_token_with_trivia(stream, TokenKind::SwitchKeyword))?;
                seq.elem(self.yul_expression(stream))?;
                seq.elem(self.yul_switch_cases_list(stream))?;
                seq.finish()
            })
        }
        .with_kind(RuleKind::YulSwitchStatement)
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_double_quote_or_backslash(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_char_range!(stream, ' ', '!'),
            scan_char_range!(stream, '#', '['),
            scan_char_range!(stream, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_single_quote_or_backslash(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_char_range!(stream, ' ', '&'),
            scan_char_range!(stream, '(', '['),
            scan_char_range!(stream, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, 't'),
            scan_chars!(stream, 'r'),
            scan_chars!(stream, 'n'),
            scan_chars!(stream, '\\'),
            scan_chars!(stream, '\''),
            scan_chars!(stream, '"'),
            scan_chars!(stream, '\r'),
            scan_chars!(stream, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_ascii_string_literal(stream),
            self.double_quoted_ascii_string_literal(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digit(&self, stream: &mut Stream) -> bool {
        scan_char_range!(stream, '0', '9')
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digits(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, self.decimal_digit(stream)),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '_'),
                    scan_one_or_more!(stream, self.decimal_digit(stream))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_exponent(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(stream, scan_chars!(stream, 'e'), scan_chars!(stream, 'E')),
            scan_optional!(stream, scan_chars!(stream, '-')),
            self.decimal_digits(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                if !self.version_is_at_least_0_5_0 {
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(
                                scan_chars!(stream, '.'),
                                scan_optional!(stream, self.decimal_digits(stream))
                            )
                        )
                    )
                } else {
                    false
                },
                if self.version_is_at_least_0_5_0 {
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
                        )
                    )
                } else {
                    false
                },
                scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
            ),
            scan_optional!(stream, self.decimal_exponent(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    self.ascii_character_without_double_quote_or_backslash(stream)
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x', '"'),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.escape_sequence(stream),
                        scan_none_of!(stream, '\n', '\r', '"', '\\')
                    )
                ),
                scan_chars!(stream, '"')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_optional!(stream, scan_chars!(stream, '\r')),
            scan_chars!(stream, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\\'),
            scan_choice!(
                stream,
                self.ascii_escape(stream),
                self.hex_byte_escape(stream),
                self.unicode_escape(stream)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'b', 'y', 't', 'e', 's'),
            self.fixed_bytes_type_size(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_bytes_type_size(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '9'),
            scan_chars!(stream, '8'),
            scan_chars!(stream, '7'),
            scan_chars!(stream, '6'),
            scan_chars!(stream, '5'),
            scan_chars!(stream, '4'),
            scan_chars!(stream, '3', '2'),
            scan_chars!(stream, '3', '1'),
            scan_chars!(stream, '3', '0'),
            scan_chars!(stream, '3'),
            scan_chars!(stream, '2', '9'),
            scan_chars!(stream, '2', '8'),
            scan_chars!(stream, '2', '7'),
            scan_chars!(stream, '2', '6'),
            scan_chars!(stream, '2', '5'),
            scan_chars!(stream, '2', '4'),
            scan_chars!(stream, '2', '3'),
            scan_chars!(stream, '2', '2'),
            scan_chars!(stream, '2', '1'),
            scan_chars!(stream, '2', '0'),
            scan_chars!(stream, '2'),
            scan_chars!(stream, '1', '9'),
            scan_chars!(stream, '1', '8'),
            scan_chars!(stream, '1', '7'),
            scan_chars!(stream, '1', '6'),
            scan_chars!(stream, '1', '5'),
            scan_chars!(stream, '1', '4'),
            scan_chars!(stream, '1', '3'),
            scan_chars!(stream, '1', '2'),
            scan_chars!(stream, '1', '1'),
            scan_chars!(stream, '1', '0'),
            scan_chars!(stream, '1')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_type_size(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, scan_char_range!(stream, '0', '9')),
            scan_chars!(stream, 'x'),
            scan_one_or_more!(stream, scan_char_range!(stream, '0', '9'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'x'),
            self.hex_character(stream),
            self.hex_character(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.decimal_digit(stream),
            scan_char_range!(stream, 'A', 'F'),
            scan_char_range!(stream, 'a', 'f')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_chars!(stream, '0', 'x'),
                if !self.version_is_at_least_0_5_0 {
                    scan_chars!(stream, '0', 'X')
                } else {
                    false
                }
            ),
            scan_one_or_more!(stream, self.hex_character(stream)),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '_'),
                    scan_one_or_more!(stream, self.hex_character(stream))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_contents(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            self.hex_character(stream),
            self.hex_character(stream),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_optional!(stream, scan_chars!(stream, '_')),
                    self.hex_character(stream),
                    self.hex_character(stream)
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_hex_string_literal(stream),
            self.double_quoted_hex_string_literal(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, stream: &mut Stream) -> bool {
        self.raw_identifier(stream)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.identifier_start(stream),
            scan_char_range!(stream, '0', '9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '_'),
            scan_chars!(stream, '$'),
            scan_char_range!(stream, 'A', 'Z'),
            scan_char_range!(stream, 'a', 'z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn integer_type_size(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '9', '6'),
            scan_chars!(stream, '8', '8'),
            scan_chars!(stream, '8', '0'),
            scan_chars!(stream, '8'),
            scan_chars!(stream, '7', '2'),
            scan_chars!(stream, '6', '4'),
            scan_chars!(stream, '5', '6'),
            scan_chars!(stream, '4', '8'),
            scan_chars!(stream, '4', '0'),
            scan_chars!(stream, '3', '2'),
            scan_chars!(stream, '2', '5', '6'),
            scan_chars!(stream, '2', '4', '8'),
            scan_chars!(stream, '2', '4', '0'),
            scan_chars!(stream, '2', '4'),
            scan_chars!(stream, '2', '3', '2'),
            scan_chars!(stream, '2', '2', '4'),
            scan_chars!(stream, '2', '1', '6'),
            scan_chars!(stream, '2', '0', '8'),
            scan_chars!(stream, '2', '0', '0'),
            scan_chars!(stream, '1', '9', '2'),
            scan_chars!(stream, '1', '8', '4'),
            scan_chars!(stream, '1', '7', '6'),
            scan_chars!(stream, '1', '6', '8'),
            scan_chars!(stream, '1', '6', '0'),
            scan_chars!(stream, '1', '6'),
            scan_chars!(stream, '1', '5', '2'),
            scan_chars!(stream, '1', '4', '4'),
            scan_chars!(stream, '1', '3', '6'),
            scan_chars!(stream, '1', '2', '8'),
            scan_chars!(stream, '1', '2', '0'),
            scan_chars!(stream, '1', '1', '2'),
            scan_chars!(stream, '1', '0', '4')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multiline_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/'),
            scan_chars!(stream, '*'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    scan_none_of!(stream, '*'),
                    scan_sequence!(scan_chars!(stream, '*'), scan_none_of!(stream, '/'))
                )
            ),
            scan_chars!(stream, '*'),
            scan_chars!(stream, '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn raw_identifier(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            self.identifier_start(stream),
            scan_zero_or_more!(stream, self.identifier_part(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(stream, self.fixed_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_integer_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'i', 'n', 't'),
            scan_optional!(stream, self.integer_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '/'),
            scan_zero_or_more!(stream, scan_none_of!(stream, '\n', '\r'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    self.ascii_character_without_single_quote_or_backslash(stream)
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x', '\''),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.escape_sequence(stream),
                        scan_none_of!(stream, '\n', '\r', '\'', '\\')
                    )
                ),
                scan_chars!(stream, '\'')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            self.hex_character(stream),
            self.hex_character(stream),
            self.hex_character(stream),
            self.hex_character(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_choice!(
                stream,
                self.single_quoted_unicode_string_literal(stream),
                self.double_quoted_unicode_string_literal(stream)
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(stream, self.fixed_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'i', 'n', 't'),
            scan_optional!(stream, self.integer_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_value(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_choice!(
                stream,
                scan_chars!(stream, 'x'),
                scan_chars!(stream, 'X'),
                scan_chars!(stream, '*'),
                scan_char_range!(stream, '0', '9')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_choice!(stream, scan_chars!(stream, ' '), scan_chars!(stream, '\t'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '0'),
            scan_sequence!(
                scan_char_range!(stream, '1', '9'),
                scan_zero_or_more!(stream, self.decimal_digit(stream))
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_hex_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '0', 'x'),
            scan_one_or_more!(stream, self.hex_character(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier(&self, stream: &mut Stream) -> bool {
        self.raw_identifier(stream)
    }

    pub fn scan(&self, lexical_context: LexicalContext, input: &str) -> Option<TokenKind> {
        let mut stream = Stream::new(input);
        match lexical_context {
            LexicalContext::Default => {
                Lexer::next_token::<{ LexicalContext::Default as u8 }>(self, &mut stream)
            }
            LexicalContext::VersionPragma => {
                Lexer::next_token::<{ LexicalContext::VersionPragma as u8 }>(self, &mut stream)
            }
            LexicalContext::YulBlock => {
                Lexer::next_token::<{ LexicalContext::YulBlock as u8 }>(self, &mut stream)
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
    fn leading_trivia(&self, stream: &mut Stream) -> ParserResult {
        Language::leading_trivia(self, stream)
    }

    fn trailing_trivia(&self, stream: &mut Stream) -> ParserResult {
        Language::trailing_trivia(self, stream)
    }

    fn next_token<const LEX_CTX: u8>(&self, stream: &mut Stream) -> Option<TokenKind> {
        let save = stream.position();
        let mut furthest_position = stream.position();
        let mut longest_token = None;

        match LexicalContext::from_repr(LEX_CTX).unwrap() {
            LexicalContext::Default => {
                macro_rules! longest_match {
                        ($( { $kind:ident = $function:ident } )*) => {
                            $(
                                if self.$function(stream) && stream.position() > furthest_position {
                                    furthest_position = stream.position();
                                    longest_token = Some(TokenKind::$kind);
                                }
                                stream.set_position(save);
                            )*
                        };
                    }

                if let Some(kind) = match stream.next() {
                    Some('a') => match stream.next() {
                        Some('b') => match stream.next() {
                            Some('i') => scan_chars!(stream, 'c', 'o', 'd', 'e', 'r')
                                .then_some(TokenKind::ABICoderKeyword),
                            Some('s') => scan_chars!(stream, 't', 'r', 'a', 'c', 't')
                                .then_some(TokenKind::AbstractKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('d') => scan_chars!(stream, 'd', 'r', 'e', 's', 's')
                            .then_some(TokenKind::AddressKeyword),
                        Some('f') => {
                            scan_chars!(stream, 't', 'e', 'r').then_some(TokenKind::AfterKeyword)
                        }
                        Some('l') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 'i', 'a', 's')
                                    .then_some(TokenKind::AliasKeyword)
                            } else {
                                None
                            }
                        }
                        Some('n') => scan_chars!(stream, 'o', 'n', 'y', 'm', 'o', 'u', 's')
                            .then_some(TokenKind::AnonymousKeyword),
                        Some('p') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 'p', 'l', 'y')
                                    .then_some(TokenKind::ApplyKeyword)
                            } else {
                                None
                            }
                        }
                        Some('s') => match stream.next() {
                            Some('s') => scan_chars!(stream, 'e', 'm', 'b', 'l', 'y')
                                .then_some(TokenKind::AssemblyKeyword),
                            Some(_) => {
                                stream.undo();
                                Some(TokenKind::AsKeyword)
                            }
                            None => Some(TokenKind::AsKeyword),
                        },
                        Some('u') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 't', 'o').then_some(TokenKind::AutoKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('b') => match stream.next() {
                        Some('o') => {
                            scan_chars!(stream, 'o', 'l').then_some(TokenKind::BoolKeyword)
                        }
                        Some('r') => {
                            scan_chars!(stream, 'e', 'a', 'k').then_some(TokenKind::BreakKeyword)
                        }
                        Some('y') => {
                            scan_chars!(stream, 't', 'e').then_some(TokenKind::ByteKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('c') => match stream.next() {
                        Some('a') => match stream.next() {
                            Some('l') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 'l', 'd', 'a', 't', 'a')
                                        .then_some(TokenKind::CalldataKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('s') => scan_chars!(stream, 'e').then_some(TokenKind::CaseKeyword),
                            Some('t') => {
                                scan_chars!(stream, 'c', 'h').then_some(TokenKind::CatchKeyword)
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('o') => match stream.next() {
                            Some('n') => match stream.next() {
                                Some('s') => {
                                    if scan_chars!(stream, 't') {
                                        match stream.next() {
                                            Some('a') => scan_chars!(stream, 'n', 't')
                                                .then_some(TokenKind::ConstantKeyword),
                                            Some('r') => {
                                                if self.version_is_at_least_0_4_22 {
                                                    scan_chars!(stream, 'u', 'c', 't', 'o', 'r')
                                                        .then_some(TokenKind::ConstructorKeyword)
                                                } else {
                                                    None
                                                }
                                            }
                                            Some(_) => {
                                                stream.undo();
                                                None
                                            }
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('t') => match stream.next() {
                                    Some('i') => scan_chars!(stream, 'n', 'u', 'e')
                                        .then_some(TokenKind::ContinueKeyword),
                                    Some('r') => scan_chars!(stream, 'a', 'c', 't')
                                        .then_some(TokenKind::ContractKeyword),
                                    Some(_) => {
                                        stream.undo();
                                        None
                                    }
                                    None => None,
                                },
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('p') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 'y', 'o', 'f')
                                        .then_some(TokenKind::CopyofKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('d') => match stream.next() {
                        Some('a') => {
                            scan_chars!(stream, 'y', 's').then_some(TokenKind::DaysKeyword)
                        }
                        Some('e') => match stream.next() {
                            Some('f') => match stream.next() {
                                Some('a') => scan_chars!(stream, 'u', 'l', 't')
                                    .then_some(TokenKind::DefaultKeyword),
                                Some('i') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(stream, 'n', 'e')
                                            .then_some(TokenKind::DefineKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            },
                            Some('l') => scan_chars!(stream, 'e', 't', 'e')
                                .then_some(TokenKind::DeleteKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('o') => Some(TokenKind::DoKeyword),
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('e') => {
                        match stream.next() {
                            Some('l') => {
                                scan_chars!(stream, 's', 'e').then_some(TokenKind::ElseKeyword)
                            }
                            Some('m') => {
                                if self.version_is_at_least_0_4_21 {
                                    scan_chars!(stream, 'i', 't').then_some(TokenKind::EmitKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                scan_chars!(stream, 'u', 'm').then_some(TokenKind::EnumKeyword)
                            }
                            Some('r') => {
                                if self.version_is_at_least_0_8_4 {
                                    scan_chars!(stream, 'r', 'o', 'r')
                                        .then_some(TokenKind::ErrorKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('t') => scan_chars!(stream, 'h', 'e', 'r')
                                .then_some(TokenKind::EtherKeyword),
                            Some('v') => scan_chars!(stream, 'e', 'n', 't')
                                .then_some(TokenKind::EventKeyword),
                            Some('x') => match stream.next() {
                                Some('p') => {
                                    scan_chars!(stream, 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l')
                                        .then_some(TokenKind::ExperimentalKeyword)
                                }
                                Some('t') => scan_chars!(stream, 'e', 'r', 'n', 'a', 'l')
                                    .then_some(TokenKind::ExternalKeyword),
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        }
                    }
                    Some('f') => {
                        match stream.next() {
                            Some('a') => {
                                if scan_chars!(stream, 'l') {
                                    match stream.next() {
                                        Some('l') => scan_chars!(stream, 'b', 'a', 'c', 'k')
                                            .then_some(TokenKind::FallbackKeyword),
                                        Some('s') => scan_chars!(stream, 'e')
                                            .then_some(TokenKind::FalseKeyword),
                                        Some(_) => {
                                            stream.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('i') => {
                                if scan_chars!(stream, 'n') {
                                    match stream.next() {
                                        Some('a') => scan_chars!(stream, 'l')
                                            .then_some(TokenKind::FinalKeyword),
                                        Some('n') => scan_chars!(stream, 'e', 'y')
                                            .then_some(TokenKind::FinneyKeyword),
                                        Some(_) => {
                                            stream.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => scan_chars!(stream, 'r').then_some(TokenKind::ForKeyword),
                            Some('r') => {
                                scan_chars!(stream, 'o', 'm').then_some(TokenKind::FromKeyword)
                            }
                            Some('u') => scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                                .then_some(TokenKind::FunctionKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        }
                    }
                    Some('g') => match stream.next() {
                        Some('l') => scan_chars!(stream, 'o', 'b', 'a', 'l')
                            .then_some(TokenKind::GlobalKeyword),
                        Some('w') => {
                            if self.version_is_at_least_0_6_11 {
                                scan_chars!(stream, 'e', 'i').then_some(TokenKind::GweiKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('h') => match stream.next() {
                        Some('e') => scan_chars!(stream, 'x').then_some(TokenKind::HexKeyword),
                        Some('o') => {
                            scan_chars!(stream, 'u', 'r', 's').then_some(TokenKind::HoursKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('i') => match stream.next() {
                        Some('f') => Some(TokenKind::IfKeyword),
                        Some('m') => match stream.next() {
                            Some('m') => {
                                if self.version_is_at_least_0_6_5 {
                                    scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e')
                                        .then_some(TokenKind::ImmutableKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('p') => match stream.next() {
                                Some('l') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(stream, 'e', 'm', 'e', 'n', 't', 's')
                                            .then_some(TokenKind::ImplementsKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('o') => scan_chars!(stream, 'r', 't')
                                    .then_some(TokenKind::ImportKeyword),
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('n') => match stream.next() {
                            Some('d') => scan_chars!(stream, 'e', 'x', 'e', 'd')
                                .then_some(TokenKind::IndexedKeyword),
                            Some('l') => scan_chars!(stream, 'i', 'n', 'e')
                                .then_some(TokenKind::InlineKeyword),
                            Some('t') => {
                                if scan_chars!(stream, 'e', 'r') {
                                    match stream.next() {
                                        Some('f') => scan_chars!(stream, 'a', 'c', 'e')
                                            .then_some(TokenKind::InterfaceKeyword),
                                        Some('n') => scan_chars!(stream, 'a', 'l')
                                            .then_some(TokenKind::InternalKeyword),
                                        Some(_) => {
                                            stream.undo();
                                            None
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                Some(TokenKind::InKeyword)
                            }
                            None => Some(TokenKind::InKeyword),
                        },
                        Some('s') => Some(TokenKind::IsKeyword),
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('l') => match stream.next() {
                        Some('e') => match stream.next() {
                            Some('a') => {
                                if self.version_is_at_least_0_6_0 {
                                    scan_chars!(stream, 'v', 'e').then_some(TokenKind::LeaveKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('t') => Some(TokenKind::LetKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('i') => scan_chars!(stream, 'b', 'r', 'a', 'r', 'y')
                            .then_some(TokenKind::LibraryKeyword),
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('m') => match stream.next() {
                        Some('a') => match stream.next() {
                            Some('c') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 'r', 'o').then_some(TokenKind::MacroKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('p') => scan_chars!(stream, 'p', 'i', 'n', 'g')
                                .then_some(TokenKind::MappingKeyword),
                            Some('t') => {
                                scan_chars!(stream, 'c', 'h').then_some(TokenKind::MatchKeyword)
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('e') => scan_chars!(stream, 'm', 'o', 'r', 'y')
                            .then_some(TokenKind::MemoryKeyword),
                        Some('i') => scan_chars!(stream, 'n', 'u', 't', 'e', 's')
                            .then_some(TokenKind::MinutesKeyword),
                        Some('o') => scan_chars!(stream, 'd', 'i', 'f', 'i', 'e', 'r')
                            .then_some(TokenKind::ModifierKeyword),
                        Some('u') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
                                    .then_some(TokenKind::MutableKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('n') => match stream.next() {
                        Some('e') => scan_chars!(stream, 'w').then_some(TokenKind::NewKeyword),
                        Some('u') => {
                            scan_chars!(stream, 'l', 'l').then_some(TokenKind::NullKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('o') => match stream.next() {
                        Some('f') => Some(TokenKind::OfKeyword),
                        Some('v') => scan_chars!(stream, 'e', 'r', 'r', 'i', 'd', 'e')
                            .then_some(TokenKind::OverrideKeyword),
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('p') => match stream.next() {
                        Some('a') => match stream.next() {
                            Some('r') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 't', 'i', 'a', 'l')
                                        .then_some(TokenKind::PartialKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('y') => scan_chars!(stream, 'a', 'b', 'l', 'e')
                                .then_some(TokenKind::PayableKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('r') => match stream.next() {
                            Some('a') => scan_chars!(stream, 'g', 'm', 'a')
                                .then_some(TokenKind::PragmaKeyword),
                            Some('i') => scan_chars!(stream, 'v', 'a', 't', 'e')
                                .then_some(TokenKind::PrivateKeyword),
                            Some('o') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 'm', 'i', 's', 'e')
                                        .then_some(TokenKind::PromiseKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('u') => match stream.next() {
                            Some('b') => scan_chars!(stream, 'l', 'i', 'c')
                                .then_some(TokenKind::PublicKeyword),
                            Some('r') => scan_chars!(stream, 'e').then_some(TokenKind::PureKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('r') => {
                        if scan_chars!(stream, 'e') {
                            match stream.next() {
                                Some('c') => scan_chars!(stream, 'e', 'i', 'v', 'e')
                                    .then_some(TokenKind::ReceiveKeyword),
                                Some('f') => {
                                    if self.version_is_at_least_0_5_0 {
                                        scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e')
                                            .then_some(TokenKind::ReferenceKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('l') => {
                                    scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e')
                                        .then_some(TokenKind::RelocatableKeyword)
                                }
                                Some('t') => {
                                    if scan_chars!(stream, 'u', 'r', 'n') {
                                        match stream.next() {
                                            Some('s') => Some(TokenKind::ReturnsKeyword),
                                            Some(_) => {
                                                stream.undo();
                                                Some(TokenKind::ReturnKeyword)
                                            }
                                            None => Some(TokenKind::ReturnKeyword),
                                        }
                                    } else {
                                        None
                                    }
                                }
                                Some('v') => scan_chars!(stream, 'e', 'r', 't')
                                    .then_some(TokenKind::RevertKeyword),
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            }
                        } else {
                            None
                        }
                    }
                    Some('s') => match stream.next() {
                        Some('e') => match stream.next() {
                            Some('a') => {
                                if self.version_is_at_least_0_5_0 {
                                    scan_chars!(stream, 'l', 'e', 'd')
                                        .then_some(TokenKind::SealedKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('c') => scan_chars!(stream, 'o', 'n', 'd', 's')
                                .then_some(TokenKind::SecondsKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('i') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 'z', 'e', 'o', 'f')
                                    .then_some(TokenKind::SizeofKeyword)
                            } else {
                                None
                            }
                        }
                        Some('o') => scan_chars!(stream, 'l', 'i', 'd', 'i', 't', 'y')
                            .then_some(TokenKind::SolidityKeyword),
                        Some('t') => match stream.next() {
                            Some('a') => scan_chars!(stream, 't', 'i', 'c')
                                .then_some(TokenKind::StaticKeyword),
                            Some('o') => scan_chars!(stream, 'r', 'a', 'g', 'e')
                                .then_some(TokenKind::StorageKeyword),
                            Some('r') => match stream.next() {
                                Some('i') => scan_chars!(stream, 'n', 'g')
                                    .then_some(TokenKind::StringKeyword),
                                Some('u') => scan_chars!(stream, 'c', 't')
                                    .then_some(TokenKind::StructKeyword),
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            },
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('u') => {
                            if self.version_is_at_least_0_5_0 {
                                scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's')
                                    .then_some(TokenKind::SupportsKeyword)
                            } else {
                                None
                            }
                        }
                        Some('w') => scan_chars!(stream, 'i', 't', 'c', 'h')
                            .then_some(TokenKind::SwitchKeyword),
                        Some('z') => {
                            scan_chars!(stream, 'a', 'b', 'o').then_some(TokenKind::SzaboKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('t') => match stream.next() {
                        Some('h') => {
                            scan_chars!(stream, 'r', 'o', 'w').then_some(TokenKind::ThrowKeyword)
                        }
                        Some('r') => match stream.next() {
                            Some('u') => scan_chars!(stream, 'e').then_some(TokenKind::TrueKeyword),
                            Some('y') => {
                                if self.version_is_at_least_0_6_0 {
                                    Some(TokenKind::TryKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('y') => {
                            if scan_chars!(stream, 'p', 'e') {
                                match stream.next() {
                                    Some('d') => {
                                        if self.version_is_at_least_0_5_0 {
                                            scan_chars!(stream, 'e', 'f')
                                                .then_some(TokenKind::TypedefKeyword)
                                        } else {
                                            None
                                        }
                                    }
                                    Some('o') => {
                                        scan_chars!(stream, 'f').then_some(TokenKind::TypeofKeyword)
                                    }
                                    Some(_) => {
                                        stream.undo();
                                        Some(TokenKind::TypeKeyword)
                                    }
                                    None => Some(TokenKind::TypeKeyword),
                                }
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('u') => match stream.next() {
                        Some('n') => {
                            if self.version_is_at_least_0_8_0 {
                                scan_chars!(stream, 'c', 'h', 'e', 'c', 'k', 'e', 'd')
                                    .then_some(TokenKind::UncheckedKeyword)
                            } else {
                                None
                            }
                        }
                        Some('s') => {
                            scan_chars!(stream, 'i', 'n', 'g').then_some(TokenKind::UsingKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('v') => match stream.next() {
                        Some('a') => scan_chars!(stream, 'r').then_some(TokenKind::VarKeyword),
                        Some('i') => match stream.next() {
                            Some('e') => scan_chars!(stream, 'w').then_some(TokenKind::ViewKeyword),
                            Some('r') => {
                                if self.version_is_at_least_0_6_0 {
                                    scan_chars!(stream, 't', 'u', 'a', 'l')
                                        .then_some(TokenKind::VirtualKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('w') => match stream.next() {
                        Some('e') => match stream.next() {
                            Some('e') => {
                                scan_chars!(stream, 'k', 's').then_some(TokenKind::WeeksKeyword)
                            }
                            Some('i') => Some(TokenKind::WeiKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('h') => {
                            scan_chars!(stream, 'i', 'l', 'e').then_some(TokenKind::WhileKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('y') => {
                        scan_chars!(stream, 'e', 'a', 'r', 's').then_some(TokenKind::YearsKeyword)
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                } {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(stream) {
                        furthest_position = stream.position();
                        longest_token = Some(kind);
                    }
                }
                stream.set_position(save);

                if let Some(kind) = match stream.next() {
                    Some('!') => match stream.next() {
                        Some('=') => Some(TokenKind::BangEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Bang)
                        }
                        None => Some(TokenKind::Bang),
                    },
                    Some('%') => match stream.next() {
                        Some('=') => Some(TokenKind::PercentEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Percent)
                        }
                        None => Some(TokenKind::Percent),
                    },
                    Some('&') => match stream.next() {
                        Some('&') => Some(TokenKind::AmpersandAmpersand),
                        Some('=') => Some(TokenKind::AmpersandEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Ampersand)
                        }
                        None => Some(TokenKind::Ampersand),
                    },
                    Some('(') => Some(TokenKind::OpenParen),
                    Some(')') => Some(TokenKind::CloseParen),
                    Some('*') => match stream.next() {
                        Some('*') => Some(TokenKind::AsteriskAsterisk),
                        Some('=') => Some(TokenKind::AsteriskEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Asterisk)
                        }
                        None => Some(TokenKind::Asterisk),
                    },
                    Some('+') => match stream.next() {
                        Some('+') => Some(TokenKind::PlusPlus),
                        Some('=') => Some(TokenKind::PlusEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Plus)
                        }
                        None => Some(TokenKind::Plus),
                    },
                    Some(',') => Some(TokenKind::Comma),
                    Some('-') => match stream.next() {
                        Some('-') => Some(TokenKind::MinusMinus),
                        Some('=') => Some(TokenKind::MinusEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Minus)
                        }
                        None => Some(TokenKind::Minus),
                    },
                    Some('.') => Some(TokenKind::Period),
                    Some('/') => match stream.next() {
                        Some('=') => Some(TokenKind::SlashEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Slash)
                        }
                        None => Some(TokenKind::Slash),
                    },
                    Some(':') => Some(TokenKind::Colon),
                    Some(';') => Some(TokenKind::Semicolon),
                    Some('<') => match stream.next() {
                        Some('<') => match stream.next() {
                            Some('=') => Some(TokenKind::LessThanLessThanEqual),
                            Some(_) => {
                                stream.undo();
                                Some(TokenKind::LessThanLessThan)
                            }
                            None => Some(TokenKind::LessThanLessThan),
                        },
                        Some('=') => Some(TokenKind::LessThanEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::LessThan)
                        }
                        None => Some(TokenKind::LessThan),
                    },
                    Some('=') => match stream.next() {
                        Some('=') => Some(TokenKind::EqualEqual),
                        Some('>') => Some(TokenKind::EqualGreaterThan),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Equal)
                        }
                        None => Some(TokenKind::Equal),
                    },
                    Some('>') => match stream.next() {
                        Some('=') => Some(TokenKind::GreaterThanEqual),
                        Some('>') => match stream.next() {
                            Some('=') => Some(TokenKind::GreaterThanGreaterThanEqual),
                            Some('>') => match stream.next() {
                                Some('=') => {
                                    Some(TokenKind::GreaterThanGreaterThanGreaterThanEqual)
                                }
                                Some(_) => {
                                    stream.undo();
                                    Some(TokenKind::GreaterThanGreaterThanGreaterThan)
                                }
                                None => Some(TokenKind::GreaterThanGreaterThanGreaterThan),
                            },
                            Some(_) => {
                                stream.undo();
                                Some(TokenKind::GreaterThanGreaterThan)
                            }
                            None => Some(TokenKind::GreaterThanGreaterThan),
                        },
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::GreaterThan)
                        }
                        None => Some(TokenKind::GreaterThan),
                    },
                    Some('?') => Some(TokenKind::QuestionMark),
                    Some('[') => Some(TokenKind::OpenBracket),
                    Some(']') => Some(TokenKind::CloseBracket),
                    Some('^') => match stream.next() {
                        Some('=') => Some(TokenKind::CaretEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Caret)
                        }
                        None => Some(TokenKind::Caret),
                    },
                    Some('{') => Some(TokenKind::OpenBrace),
                    Some('|') => match stream.next() {
                        Some('=') => Some(TokenKind::BarEqual),
                        Some('|') => Some(TokenKind::BarBar),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::Bar)
                        }
                        None => Some(TokenKind::Bar),
                    },
                    Some('}') => Some(TokenKind::CloseBrace),
                    Some('~') => Some(TokenKind::Tilde),
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = stream.position();
                    longest_token = Some(kind);
                }
                stream.set_position(save);

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
                                if self.$function(stream) && stream.position() > furthest_position {
                                    furthest_position = stream.position();
                                    longest_token = Some(TokenKind::$kind);
                                }
                                stream.set_position(save);
                            )*
                        };
                    }

                if let Some(kind) = scan_chars!(stream, 's', 'o', 'l', 'i', 'd', 'i', 't', 'y')
                    .then_some(TokenKind::SolidityKeyword)
                {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(stream) {
                        furthest_position = stream.position();
                        longest_token = Some(kind);
                    }
                }
                stream.set_position(save);

                if let Some(kind) = match stream.next() {
                    Some('-') => Some(TokenKind::Minus),
                    Some('.') => Some(TokenKind::Period),
                    Some('<') => match stream.next() {
                        Some('=') => Some(TokenKind::LessThanEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::LessThan)
                        }
                        None => Some(TokenKind::LessThan),
                    },
                    Some('=') => Some(TokenKind::Equal),
                    Some('>') => match stream.next() {
                        Some('=') => Some(TokenKind::GreaterThanEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::GreaterThan)
                        }
                        None => Some(TokenKind::GreaterThan),
                    },
                    Some('^') => Some(TokenKind::Caret),
                    Some('|') => scan_chars!(stream, '|').then_some(TokenKind::BarBar),
                    Some('~') => Some(TokenKind::Tilde),
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = stream.position();
                    longest_token = Some(kind);
                }
                stream.set_position(save);

                longest_match! {
                        { VersionPragmaValue = version_pragma_value }
                }
            }
            LexicalContext::YulBlock => {
                macro_rules! longest_match {
                        ($( { $kind:ident = $function:ident } )*) => {
                            $(
                                if self.$function(stream) && stream.position() > furthest_position {
                                    furthest_position = stream.position();
                                    longest_token = Some(TokenKind::$kind);
                                }
                                stream.set_position(save);
                            )*
                        };
                    }

                if let Some(kind) =
                    match stream.next() {
                        Some('b') => scan_chars!(stream, 'r', 'e', 'a', 'k')
                            .then_some(TokenKind::BreakKeyword),
                        Some('c') => match stream.next() {
                            Some('a') => {
                                scan_chars!(stream, 's', 'e').then_some(TokenKind::CaseKeyword)
                            }
                            Some('o') => scan_chars!(stream, 'n', 't', 'i', 'n', 'u', 'e')
                                .then_some(TokenKind::ContinueKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('d') => scan_chars!(stream, 'e', 'f', 'a', 'u', 'l', 't')
                            .then_some(TokenKind::DefaultKeyword),
                        Some('f') => match stream.next() {
                            Some('a') => scan_chars!(stream, 'l', 's', 'e')
                                .then_some(TokenKind::FalseKeyword),
                            Some('o') => scan_chars!(stream, 'r').then_some(TokenKind::ForKeyword),
                            Some('u') => scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                                .then_some(TokenKind::FunctionKeyword),
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some('h') => scan_chars!(stream, 'e', 'x').then_some(TokenKind::HexKeyword),
                        Some('i') => scan_chars!(stream, 'f').then_some(TokenKind::IfKeyword),
                        Some('l') => {
                            if scan_chars!(stream, 'e') {
                                match stream.next() {
                                    Some('a') => {
                                        if self.version_is_at_least_0_6_0 {
                                            scan_chars!(stream, 'v', 'e')
                                                .then_some(TokenKind::LeaveKeyword)
                                        } else {
                                            None
                                        }
                                    }
                                    Some('t') => Some(TokenKind::LetKeyword),
                                    Some(_) => {
                                        stream.undo();
                                        None
                                    }
                                    None => None,
                                }
                            } else {
                                None
                            }
                        }
                        Some('s') => scan_chars!(stream, 'w', 'i', 't', 'c', 'h')
                            .then_some(TokenKind::SwitchKeyword),
                        Some('t') => {
                            scan_chars!(stream, 'r', 'u', 'e').then_some(TokenKind::TrueKeyword)
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    }
                {
                    // Make sure that this is not the start of an identifier
                    if !self.identifier_part(stream) {
                        furthest_position = stream.position();
                        longest_token = Some(kind);
                    }
                }
                stream.set_position(save);

                if let Some(kind) = match stream.next() {
                    Some('(') => Some(TokenKind::OpenParen),
                    Some(')') => Some(TokenKind::CloseParen),
                    Some(',') => Some(TokenKind::Comma),
                    Some('-') => scan_chars!(stream, '>').then_some(TokenKind::MinusGreaterThan),
                    Some('.') => Some(TokenKind::Period),
                    Some(':') => scan_chars!(stream, '=').then_some(TokenKind::ColonEqual),
                    Some('{') => Some(TokenKind::OpenBrace),
                    Some('}') => Some(TokenKind::CloseBrace),
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                } {
                    furthest_position = stream.position();
                    longest_token = Some(kind);
                }
                stream.set_position(save);

                longest_match! {
                        { AsciiStringLiteral = ascii_string_literal }
                        { HexStringLiteral = hex_string_literal }
                        { YulDecimalLiteral = yul_decimal_literal }
                        { YulHexLiteral = yul_hex_literal }
                        { YulIdentifier = yul_identifier }
                }
            }
        }

        if longest_token.is_some() {
            stream.set_position(furthest_position);
        }

        longest_token
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
