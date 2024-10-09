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
pub enum NonterminalKind {
    AdditionExpression,
    Expression,
    Literal,
    MemberAccessExpression,
    NegationExpression,
    SeparatedIdentifiers,
    SourceUnit,
    SourceUnitMember,
    SourceUnitMembers,
    Tree,
    TreeNode,
    TreeNodeChild,
    TreeNodeChildren,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
