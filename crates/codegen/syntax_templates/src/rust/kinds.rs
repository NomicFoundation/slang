use serde::Serialize;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
pub enum TokenKind {
    XXX,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
pub enum RuleKind {
    _SEQUENCE,
    _DELIMITEDBY,
    _TERMINATEDBY,
    XXX,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
pub enum ProductionKind {
    XXX,
}
