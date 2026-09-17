use serde::Serialize;

/// The kind of a member that can take part in overriding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum OverridableKind {
    Function,
    Modifier,
    PublicStateVariable,
}

impl OverridableKind {
    /// The kind as it reads in a diagnostic message (e.g. `modifier`).
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Modifier => "modifier",
            Self::PublicStateVariable => "public state variable",
        }
    }
}
