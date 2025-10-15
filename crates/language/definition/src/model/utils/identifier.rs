use std::fmt;
use std::ops::Deref;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::parse::ParseStream;
use syn::Ident;

use crate::internals::{
    ErrorsCollection, ParseHelpers, ParseInputTokens, Result, WriteOutputTokens,
};

/// A wrapper type to make sure the DSL token is written as an identifier instead of a string literal.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier {
    value: String,
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            value: Deserialize::deserialize(deserializer)?,
        })
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl ParseInputTokens for Identifier {
    fn parse_value(input: ParseStream<'_>, _: &mut ErrorsCollection) -> Result<Self> {
        let value = ParseHelpers::syn::<Ident>(input)?.to_string();

        Ok(value.into())
    }
}

impl WriteOutputTokens for Identifier {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::string(self);

        quote! {
            #value.into()
        }
    }
}

impl quote::IdentFragment for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}
