use std::fmt::Display;

use itertools::Itertools;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Ident, Result, Type, Variant};

pub enum InputItem {
    Struct {
        name: Ident,
        fields: Vec<InputField>,
    },
    Enum {
        name: Ident,
        variants: Vec<InputVariant>,
    },
}

impl Parse for InputItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let input = DeriveInput::parse(input)?;

        Self::from_syn(input)
    }
}

impl InputItem {
    fn from_syn(input: DeriveInput) -> Result<Self> {
        Ok(match input.data {
            Data::Struct(data) => InputItem::Struct {
                name: input.ident,
                fields: match data.fields {
                    Fields::Named(fields) => InputField::from_syn(fields)?,
                    _ => error(&data.fields, "Only named fields are supported.")?,
                },
            },
            Data::Enum(data) => InputItem::Enum {
                name: input.ident,
                variants: data
                    .variants
                    .into_iter()
                    .map(InputVariant::from_syn)
                    .try_collect()?,
            },
            Data::Union(data) => error(data.union_token, "Unions are not supported.")?,
        })
    }
}

pub struct InputVariant {
    pub name: Ident,
    pub fields: Option<Vec<InputField>>,
}

impl InputVariant {
    fn from_syn(input: Variant) -> Result<Self> {
        Ok(match input.fields {
            Fields::Named(fields) => InputVariant {
                name: input.ident,
                fields: Some(InputField::from_syn(fields)?),
            },
            Fields::Unit => InputVariant {
                name: input.ident,
                fields: None,
            },
            Fields::Unnamed(fields) => error(fields, "Unnamed fields are not supported.")?,
        })
    }
}

pub struct InputField {
    pub name: Ident,
    pub r#type: Type,
}

impl InputField {
    fn from_syn(input: FieldsNamed) -> Result<Vec<Self>> {
        input
            .named
            .into_iter()
            .map(|field| match field.ident {
                Some(name) => Ok(Self {
                    name,
                    r#type: field.ty,
                }),
                None => error(field, "Unnamed fields are not supported."),
            })
            .try_collect()
    }
}

pub fn add_spanned_prefix(input: impl Display) -> String {
    format!("Spanned{input}")
}

pub fn strip_spanned_prefix(input: String) -> String {
    match input.strip_prefix("Spanned") {
        Some(suffix) if !suffix.is_empty() => suffix.to_owned(),
        _ => input,
    }
}

fn error<T>(spanned: impl ToTokens, message: impl Display) -> Result<T> {
    Err(Error::new_spanned(spanned, message))
}
