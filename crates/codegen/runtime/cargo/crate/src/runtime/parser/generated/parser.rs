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

use semver::Version;

use crate::cst;
use crate::cst::{
    EdgeLabel, IsLexicalContext, LexicalContext, LexicalContextType, NonterminalKind, TerminalKind,
};
use crate::parser::lexer::{KeywordScan, Lexer, ScannedTerminal};
use crate::parser::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, SeparatedHelper, SequenceHelper, TerminalAcceptanceThreshold,
    ZeroOrMoreHelper,
};
use crate::parser::scanner_macros::{
    scan_char_range, scan_chars, scan_choice, scan_keyword_choice, scan_none_of,
    scan_not_followed_by, scan_one_or_more, scan_optional, scan_sequence, scan_zero_or_more,
};
use crate::parser::ParseOutput;
use crate::utils::LanguageFacts;

#[derive(Debug)]
pub struct Parser {
    language_version: Version,
}

#[derive(thiserror::Error, Debug)]
pub enum ParserInitializationError {
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
}

impl Parser {
    pub fn create(
        language_version: Version,
    ) -> std::result::Result<Self, ParserInitializationError> {
        if LanguageFacts::ALL_VERSIONS
            .binary_search(&language_version)
            .is_ok()
        {
            Ok(Self { language_version })
        } else {
            Err(ParserInitializationError::UnsupportedLanguageVersion(
                language_version,
            ))
        }
    }

    pub fn infer_language_version(input: &str) -> Vec<Version> {
        crate::extensions::semver::infer_language_version(input)
    }

    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    pub fn parse_file_contents(&self, input: &str) -> ParseOutput {
        self.parse_nonterminal(NonterminalKind::Stub1, input)
    }
    pub fn parse_nonterminal(&self, kind: NonterminalKind, input: &str) -> ParseOutput {
        unreachable!("Attempting to parse in stubs: {kind}: {input}")
    }
}

impl Lexer for Parser {
    fn leading_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        unreachable!("Invoking leading_trivia in stubs: {input:#?}")
    }

    fn trailing_trivia(&self, input: &mut ParserContext<'_>) -> ParserResult {
        unreachable!("Invoking trailing_trivia in stubs: {input:#?}")
    }

    fn delimiters<LexCtx: IsLexicalContext>() -> &'static [(TerminalKind, TerminalKind)] {
        unreachable!("Invoking delimiters in stubs.")
    }

    fn next_terminal<LexCtx: IsLexicalContext>(
        &self,
        input: &mut ParserContext<'_>,
    ) -> Option<ScannedTerminal> {
        unreachable!("Invoking next_terminal in stubs: {input:#?}")
    }
}
