use serde::Serialize;
use strum_macros::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum TokenKind {
    XXX,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum RuleKind {
    _SEQUENCE,
    _DELIMITEDBY,
    _TERMINATEDBY,
    XXX,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display,
)]
pub enum ProductionKind {
    XXX,
}
