pub mod cst;
pub mod cursor;
pub mod query;
pub mod text_index;

pub trait AbstractKind: Sized + std::fmt::Debug + Copy + PartialEq + Eq + serde::Serialize {
    fn as_static_str(&self) -> &'static str;
    fn try_from_str(str: &str) -> Result<Self, String>;
}

impl<T> AbstractKind for T
where
    T: Sized
        + std::fmt::Debug
        + Copy
        + Eq
        + serde::Serialize
        + for<'a> std::convert::TryFrom<&'a str>
        + std::convert::Into<&'static str>,
{
    fn as_static_str(&self) -> &'static str {
        (*self).into()
    }
    fn try_from_str(str: &str) -> Result<Self, String> {
        match Self::try_from(str) {
            Ok(val) => Ok(val),
            Err(_) => Err(format!(
                "Failed to convert \"{str}\" to {}",
                std::any::type_name::<Self>()
            )),
        }
    }
}

pub trait TerminalKind: AbstractKind {
    fn is_trivia(&self) -> bool {
        false
    }

    /// Returns whether the terminal is valid, i.e. does not represent missing or invalid syntax.
    fn is_valid(&self) -> bool {
        true
    }
}

pub trait NonterminalKind: AbstractKind {}

pub trait EdgeLabel: AbstractKind {}

pub trait KindTypes: std::fmt::Debug + Clone + PartialEq {
    type NonterminalKind: NonterminalKind;
    type TerminalKind: TerminalKind;
    type EdgeLabel: EdgeLabel;
}
