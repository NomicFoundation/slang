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
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported Testlang language version '{0}'.")]
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
            Ok(Self { version })
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
                choice.finish(input)
            })
        })
        .with_kind(RuleKind::LeadingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn literal(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.parse_token_with_trivia::<LexicalContextType::Default>(
                input,
                TokenKind::StringLiteral,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::Literal)
    }

    #[allow(unused_assignments, unused_parens)]
    fn separated_identifiers(&self, input: &mut ParserContext<'_>) -> ParserResult {
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
        .with_kind(RuleKind::SeparatedIdentifiers)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, input: &mut ParserContext<'_>) -> ParserResult {
        self.source_unit_members(input)
            .with_name(FieldName::Members)
            .with_kind(RuleKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_member(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree(input);
            choice.consider(input, result)?;
            let result = self.separated_identifiers(input);
            choice.consider(input, result)?;
            let result = self.literal(input);
            choice.consider(input, result)?;
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
    fn tree(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            seq.elem(
                SequenceHelper::run(|mut seq| {
                    seq.elem_named(
                        FieldName::Keyword,
                        self.parse_token_with_trivia::<LexicalContextType::Tree>(
                            input,
                            TokenKind::TreeKeyword,
                        ),
                    )?;
                    seq.elem_named(
                        FieldName::Name,
                        OptionalHelper::transform(
                            self.parse_token_with_trivia::<LexicalContextType::Tree>(
                                input,
                                TokenKind::Identifier,
                            ),
                        ),
                    )?;
                    seq.elem_named(FieldName::Node, self.tree_node(input))?;
                    seq.finish()
                })
                .recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                    input,
                    self,
                    TokenKind::Semicolon,
                    RecoverFromNoMatch::No,
                ),
            )?;
            seq.elem_named(
                FieldName::Semicolon,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TokenKind::Semicolon,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::Tree)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node(&self, input: &mut ParserContext<'_>) -> ParserResult {
        SequenceHelper::run(|mut seq| {
            let mut delim_guard = input.open_delim(TokenKind::CloseBracket);
            let input = delim_guard.ctx();
            seq.elem_named(
                FieldName::OpenBracket,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TokenKind::OpenBracket,
                ),
            )?;
            seq.elem(
                self.tree_node_children(input)
                    .with_name(FieldName::Members)
                    .recover_until_with_nested_delims::<_, LexicalContextType::Tree>(
                        input,
                        self,
                        TokenKind::CloseBracket,
                        RecoverFromNoMatch::Yes,
                    ),
            )?;
            seq.elem_named(
                FieldName::CloseBracket,
                self.parse_token_with_trivia::<LexicalContextType::Tree>(
                    input,
                    TokenKind::CloseBracket,
                ),
            )?;
            seq.finish()
        })
        .with_kind(RuleKind::TreeNode)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_child(&self, input: &mut ParserContext<'_>) -> ParserResult {
        ChoiceHelper::run(input, |mut choice, input| {
            let result = self.tree_node(input);
            choice.consider(input, result)?;
            let result = self.parse_token_with_trivia::<LexicalContextType::Tree>(
                input,
                TokenKind::DelimitedIdentifier,
            );
            choice.consider(input, result)?;
            choice.finish(input)
        })
        .with_name(FieldName::Variant)
        .with_kind(RuleKind::TreeNodeChild)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tree_node_children(&self, input: &mut ParserContext<'_>) -> ParserResult {
        OneOrMoreHelper::run(input, |input| {
            self.tree_node_child(input).with_name(FieldName::Item)
        })
        .with_kind(RuleKind::TreeNodeChildren)
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

    // Keyword scanners

    pub fn parse(&self, kind: RuleKind, input: &str) -> ParseOutput {
        match kind {
            | RuleKind::LeadingTrivia => Self::leading_trivia.parse(self, input, false),
            | RuleKind::Literal => Self::literal.parse(self, input, true),
            | RuleKind::SeparatedIdentifiers => {
                Self::separated_identifiers.parse(self, input, true)
            }
            | RuleKind::SourceUnit => Self::source_unit.parse(self, input, true),
            | RuleKind::SourceUnitMember => Self::source_unit_member.parse(self, input, true),
            | RuleKind::SourceUnitMembers => Self::source_unit_members.parse(self, input, true),
            | RuleKind::TrailingTrivia => Self::trailing_trivia.parse(self, input, false),
            | RuleKind::Tree => Self::tree.parse(self, input, true),
            | RuleKind::TreeNode => Self::tree_node.parse(self, input, true),
            | RuleKind::TreeNodeChild => Self::tree_node_child.parse(self, input, true),
            | RuleKind::TreeNodeChildren => Self::tree_node_children.parse(self, input, true),
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
            | LexicalContext::Default => &[],
            | LexicalContext::Tree => &[(TokenKind::OpenBracket, TokenKind::CloseBracket)],
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
            | LexicalContext::Default => {
                if let Some(kind) = if scan_chars!(input, '.') {
                    Some(TokenKind::Period)
                } else {
                    None
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
                        | _ if input.position() < furthest_position => KeywordScan::Absent,
                        | value => value,
                    };

                    input.set_position(furthest_position);
                    return Some(ScannedToken::IdentifierOrKeyword {
                        identifier,
                        kw: kw_scan,
                    });
                }
            }
            | LexicalContext::Tree => {
                if let Some(kind) = match input.next() {
                    | Some(';') => Some(TokenKind::Semicolon),
                    | Some('[') => Some(TokenKind::OpenBracket),
                    | Some(']') => Some(TokenKind::CloseBracket),
                    | Some(_) => {
                        input.undo();
                        None
                    }
                    | None => None,
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
                    longest_token.filter(|tok| [TokenKind::Identifier].contains(tok))
                {
                    let kw_scan = if scan_chars!(input, 't', 'r', 'e', 'e') {
                        KeywordScan::Reserved(TokenKind::TreeKeyword)
                    } else {
                        KeywordScan::Absent
                    };
                    let kw_scan = match kw_scan {
                        // Strict prefix; we need to match the whole identifier to promote
                        | _ if input.position() < furthest_position => KeywordScan::Absent,
                        | value => value,
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
            | Some(token) => {
                input.set_position(furthest_position);
                Some(ScannedToken::Single(token))
            }
            // Skip a character if possible and if we didn't recognize a token
            | None if input.peek().is_some() => {
                let _ = input.next();
                Some(ScannedToken::Single(TokenKind::SKIPPED))
            }
            | None => None,
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
