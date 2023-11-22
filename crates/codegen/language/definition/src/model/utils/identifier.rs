use serde::Serialize;
use std::ops::Deref;

/// A wrapper type to make sure the DSL token is written as an identifier instead of a string literal.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier {
    value: String,
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value.fmt(f);
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        return Self { value };
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        return Self {
            value: value.to_owned(),
        };
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return self.value.serialize(serializer);
    }
}
