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
pub enum TerminalKind {
    // Built-in:
    UNRECOGNIZED,
    MISSING,

    // Generated:
    /// BANG = "!";
    Bang,
    /// CLOSE_BRACKET = "]";
    CloseBracket,
    /// DELIMITED_IDENTIFIER = «DELIMITED_IDENTIFIER_START» «DELIMITED_IDENTIFIER_PART»*;
    DelimitedIdentifier,
    /// END_OF_LINE = "\r"? "\n";
    EndOfLine,
    /// IDENTIFIER = «RAW_IDENTIFIER»;
    Identifier,
    /// MULTI_LINE_COMMENT = "/*" (!"*" | "*")* "*/";
    MultiLineComment,
    /// OPEN_BRACKET = "[";
    OpenBracket,
    /// PERIOD = ".";
    Period,
    /// PLUS = "+";
    Plus,
    /// SEMICOLON = ";";
    Semicolon,
    /// SINGLE_LINE_COMMENT = "//" (!("\r" "\n"))*;
    SingleLineComment,
    /// STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    StringLiteral,
    /// TREE_KEYWORD = "tree";
    TreeKeyword,
    /// WHITESPACE = (" " | "\t")+;
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
