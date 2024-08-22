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

#[cfg(feature = "__private_napi_interfaces")]
use napi_derive::napi;
use semver::Version;

use crate::cst;
use crate::cst::{
    EdgeLabel, IsLexicalContext, LexicalContext, LexicalContextType, NonterminalKind, TerminalKind,
};
use crate::language::lexer::{KeywordScan, Lexer, ScannedTerminal};
use crate::language::parser_support::{
    ChoiceHelper, OneOrMoreHelper, OptionalHelper, ParserContext, ParserFunction, ParserResult,
    PrecedenceHelper, SeparatedHelper, SequenceHelper, TerminalAcceptanceThreshold,
    ZeroOrMoreHelper,
};
use crate::language::scanner_macros::{
    scan_char_range, scan_chars, scan_choice, scan_keyword_choice, scan_none_of,
    scan_not_followed_by, scan_one_or_more, scan_optional, scan_sequence, scan_zero_or_more,
};
#[cfg(feature = "__private_napi_interfaces")]
use crate::napi_interface::parse_output::ParseOutput as NAPIParseOutput;
use crate::parse_output::ParseOutput;

#[derive(Debug)]
#[cfg_attr(feature = "__private_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version: Version,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),

    #[cfg(feature = "__private_napi_interfaces")]
    #[error("Invalid semantic version '{0}'.")]
    InvalidSemanticVersion(String),
}

#[cfg(feature = "__private_napi_interfaces")]
impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}

impl Language {
    pub const SUPPORTED_VERSIONS: &'static [Version] = &[];

    pub const ROOT_KIND: NonterminalKind = NonterminalKind::Stub1;

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

    pub fn parse(&self, kind: NonterminalKind, input: &str) -> ParseOutput {
        unreachable!("Attempting to parse in stubs: {kind}: {input}")
    }
}

impl Lexer for Language {
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

#[cfg(feature = "__private_napi_interfaces")]
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
        js_name = "rootKind",
        ts_return_type = "cst.NonterminalKind",
        catch_unwind
    )]
    pub fn root_kind_napi() -> NonterminalKind {
        Self::ROOT_KIND
    }

    #[napi(
        js_name = "parse",
        ts_return_type = "parse_output.ParseOutput",
        catch_unwind
    )]
    pub fn parse_napi(
        &self,
        #[napi(ts_arg_type = "cst.NonterminalKind")] kind: NonterminalKind,
        input: String,
    ) -> NAPIParseOutput {
        self.parse(kind, input.as_str()).into()
    }
}
