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
pub enum EdgeLabel {
    // Built-in:
    Root,
    Unrecognized,
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,

    // Generated:
    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::EdgeLabelExtensions for EdgeLabel {}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self::Root
    }
}
