// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]

/// Represents different kinds of nonterminal nodes in the syntax tree.
/// These are nodes that can have child nodes and represent higher-level language constructs.
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
    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
