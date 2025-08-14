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
    PrecedenceHelper, SeparatedHelper, SequenceHelper, ZeroOrMoreHelper,
};
use crate::parser::scanner_macros::{
    scan_char_range, scan_chars, scan_choice, scan_keyword_choice, scan_none_of,
    scan_not_followed_by, scan_one_or_more, scan_optional, scan_sequence, scan_zero_or_more,
};
use crate::parser::ParseOutput;
use crate::utils::LanguageFacts;

/// Parses `CodegenRuntime` source code. `Parser` must be initialized with a specific
/// language version that's supported by Slang. See [`LanguageFacts`] to determine what language
/// versions are available.
///
/// ```
/// use slang_solidity::parser::Parser;
/// use slang_solidity::utils::LanguageFacts;
///
/// // Initialize parser
/// let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
/// // Get source code to parse
/// let source = "contract AContract { }";
///
/// // Parse the entire file, then get a cursor to start navigating the parsed CST.
/// // Any parse errors will be reflected by error nodes in the tree. Errors can also
/// // be listed with `output.errors()`.
/// let output = parser.parse_file_contents(&source);
/// let mut cursor = output.create_tree_cursor();
/// ```
#[derive(Debug)]
pub struct Parser {
    language_version: Version,
}

/// Errors that may occur when initializing a [`Parser`].
#[derive(thiserror::Error, Debug)]
pub enum ParserInitializationError {
    /// Tried to initialize a [`Parser`] with a version that is not supported for `CodegenRuntime`.
    /// See [`LanguageFacts::ALL_VERSIONS`] for a complete list of supported versions.
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
}

impl Parser {
    /// Create a new `CodegenRuntime` parser that supports the specified language version.
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

    /// Returns the `CodegenRuntime` version that this parser supports.
    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    /// Parse the contents of an entire `CodegenRuntime` source file.
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
