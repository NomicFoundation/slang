// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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
    // Built-in:
    UNRECOGNIZED,
    MISSING,

    // Generated:
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// ```ebnf
    /// DELIMITED_IDENTIFIER = «DELIMITED_IDENTIFIER_START» «DELIMITED_IDENTIFIER_PART»*;
    /// ```
    DelimitedIdentifier,
    /// ```ebnf
    /// END_OF_LINE = "\r"? "\n";
    /// ```
    EndOfLine,
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER»;
    /// ```
    Identifier,
    /// ```ebnf
    /// MULTI_LINE_COMMENT = "/*" (!"*" | "*")* "*/";
    /// ```
    MultiLineComment,
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (!("\r" "\n"))*;
    /// ```
    SingleLineComment,
    /// ```ebnf
    /// STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    /// ```
    StringLiteral,
    /// ```ebnf
    /// TREE_KEYWORD = "tree";
    /// ```
    TreeKeyword,
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {
    fn is_trivia(&self) -> bool {
        matches!(self, |Self::EndOfLine| Self::MultiLineComment
            | Self::SingleLineComment
            | Self::Whitespace)
    }

    fn is_valid(&self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}
