// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs:68:22). Please don't edit by hand.

#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
    Clone,
    Copy,
)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
pub enum TerminalKind {
    UNRECOGNIZED,
    MISSING,

    /// Represents a node with kind `Bang`, having the following structure:
    ///
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// Represents a node with kind `CloseBracket`, having the following structure:
    ///
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// Represents a node with kind `DelimitedIdentifier`, having the following structure:
    ///
    /// ```ebnf
    /// DELIMITED_IDENTIFIER = «DELIMITED_IDENTIFIER_START» «DELIMITED_IDENTIFIER_PART»*;
    /// ```
    DelimitedIdentifier,
    /// Represents a node with kind `EndOfLine`, having the following structure:
    ///
    /// ```ebnf
    /// END_OF_LINE = "\r"? "\n";
    /// ```
    EndOfLine,
    /// Represents a node with kind `Identifier`, having the following structure:
    ///
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER»;
    /// ```
    Identifier,
    /// Represents a node with kind `MultiLineComment`, having the following structure:
    ///
    /// ```ebnf
    /// MULTI_LINE_COMMENT = "/*" (?!"*") (!"*" | ("*" (?!"/")))* "*/";
    /// ```
    MultiLineComment,
    /// Represents a node with kind `OpenBracket`, having the following structure:
    ///
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// Represents a node with kind `Period`, having the following structure:
    ///
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// Represents a node with kind `Plus`, having the following structure:
    ///
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// Represents a node with kind `Semicolon`, having the following structure:
    ///
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// Represents a node with kind `SingleLineComment`, having the following structure:
    ///
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (?!"/") (!("\r" | "\n"))*;
    /// ```
    SingleLineComment,
    /// Represents a node with kind `StringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
    /// ```
    StringLiteral,
    /// Represents a node with kind `TreeKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// TREE_KEYWORD = "tree";
    /// ```
    TreeKeyword,
    /// Represents a node with kind `Whitespace`, having the following structure:
    ///
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {
    fn is_identifier(self) -> bool {
        matches!(
                self,| Self::Identifier)
    }

    fn is_trivia(self) -> bool {
        matches!(self, |Self::EndOfLine| Self::MultiLineComment
            | Self::SingleLineComment
            | Self::Whitespace)
    }

    fn is_valid(self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}
