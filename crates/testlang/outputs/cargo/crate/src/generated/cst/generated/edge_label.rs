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
)]
#[strum(serialize_all = "snake_case")]
#[derive(Clone, Copy)]
/// Represents the different types of relationships between nodes in the syntax tree.
pub enum EdgeLabel {
    /// Represents a child node with the label `root`.
    Root,
    /// Represents a child node with the label `unrecognized`.
    Unrecognized,
    /// Represents a child node with the label `missing`.
    Missing,
    /// Represents a child node with the label `item`.
    Item,
    /// Represents a child node with the label `variant`.
    Variant,
    /// Represents a child node with the label `separator`.
    Separator,
    /// Represents a child node with the label `operand`.
    Operand,
    /// Represents a child node with the label `left_operand`.
    LeftOperand,
    /// Represents a child node with the label `right_operand`.
    RightOperand,
    /// Represents a child node with the label `leading_trivia`.
    LeadingTrivia,
    /// Represents a child node with the label `trailing_trivia`.
    TrailingTrivia,

    /// Represents a child node with the label `close_bracket`.
    CloseBracket,
    /// Represents a child node with the label `keyword`.
    Keyword,
    /// Represents a child node with the label `member`.
    Member,
    /// Represents a child node with the label `members`.
    Members,
    /// Represents a child node with the label `name`.
    Name,
    /// Represents a child node with the label `node`.
    Node,
    /// Represents a child node with the label `open_bracket`.
    OpenBracket,
    /// Represents a child node with the label `operator`.
    Operator,
    /// Represents a child node with the label `period`.
    Period,
    /// Represents a child node with the label `semicolon`.
    Semicolon,
}

impl crate::cst::EdgeLabelExtensions for EdgeLabel {}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self::Root
    }
}
