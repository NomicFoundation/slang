pub mod cst;
pub mod cursor;
pub mod query;
pub mod text_index;
pub mod tree;

pub trait Kind:
    Sized
    + Copy
    + Clone
    + PartialEq
    + Eq
    + std::fmt::Display
    + std::fmt::Debug
    + serde::Serialize
    + for<'a> std::convert::TryFrom<&'a str, Error = strum::ParseError>
{
}

impl<T> Kind for T where
    T: Sized
        + Copy
        + Clone
        + PartialEq
        + Eq
        + std::fmt::Display
        + std::fmt::Debug
        + serde::Serialize
        + for<'a> std::convert::TryFrom<&'a str, Error = strum::ParseError>
{
}

pub trait KindTypes: Clone + PartialEq {
    type NonTerminalKind: Kind;
    type TerminalKind: Kind;
    type LabelKind: Kind;
}
