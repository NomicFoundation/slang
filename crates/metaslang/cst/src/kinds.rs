pub trait BaseKind: Sized + std::fmt::Debug + Copy + PartialEq + Eq + serde::Serialize {
    fn as_static_str(&self) -> &'static str;
    fn try_from_str(str: &str) -> Result<Self, String>;
}

impl<T> BaseKind for T
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

/// Useful extension methods for working with terminals and terminal kinds.
pub trait TerminalKindExtensions: BaseKind {
    /// Returns `true` if the terminal is an identifier token.
    fn is_identifier(self) -> bool {
        false
    }

    /// Returns `true` if the terminal is a trivia token. i.e. whitespace, comments, etc... 
    fn is_trivia(self) -> bool {
        false
    }

    /// Returns `true` if the terminal is a valid token in the language grammar.
    fn is_valid(self) -> bool {
        true
    }
}

pub trait NonterminalKindExtensions: BaseKind {}

pub trait EdgeLabelExtensions: BaseKind + Default {}

pub trait KindTypes: std::fmt::Debug + Clone + PartialEq {
    type NonterminalKind: NonterminalKindExtensions;
    type TerminalKind: TerminalKindExtensions;
    type EdgeLabel: EdgeLabelExtensions;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeKind<T: KindTypes> {
    /// Represents different kinds of nonterminal nodes in the syntax tree.
    /// These are nodes that can have child nodes and represent higher-level language constructs.
    Nonterminal(T::NonterminalKind),
    /// Represents different kinds of terminal nodes in the syntax tree.
    /// These are leaf nodes that represent actual tokens in the source code.
    Terminal(T::TerminalKind),
}

impl<T: KindTypes> From<NodeKind<T>> for &'static str {
    fn from(val: NodeKind<T>) -> Self {
        match val {
            NodeKind::Nonterminal(t) => t.as_static_str(),
            NodeKind::Terminal(t) => t.as_static_str(),
        }
    }
}

impl<T: KindTypes> std::fmt::Display for NodeKind<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Nonterminal(t) => {
                write!(f, "{}", t.as_static_str())
            }
            NodeKind::Terminal(t) => {
                write!(f, "{}", t.as_static_str())
            }
        }
    }
}
