pub mod cst;
pub mod cursor;
pub mod query;
pub mod text_index;

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

pub trait TerminalKind: Kind {
    fn is_trivia(&self) -> bool;
}

pub trait NonTerminalKind: Kind {}

pub trait EdgeKind: Kind {}

pub trait KindTypes: Clone + PartialEq {
    type NonTerminalKind: NonTerminalKind;
    type TerminalKind: TerminalKind;
    type EdgeKind: EdgeKind;
}
