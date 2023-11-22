use itertools::Itertools;
use quote::ToTokens;
use std::fmt::Display;
use syn::{
    parse::{Parse, ParseStream},
    Data, DeriveInput, Error, Fields, FieldsNamed, Ident, Result, Variant,
};

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
    fn parse(input: ParseStream) -> Result<Self> {
        let input = DeriveInput::parse(input)?;

        return Self::from_syn(input);
    }
}

impl InputItem {
    fn from_syn(input: DeriveInput) -> Result<Self> {
        return Ok(match input.data {
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
                    .map(|variant| InputVariant::from_syn(variant))
                    .try_collect()?,
            },
            Data::Union(data) => error(data.union_token, "Unions are not supported.")?,
        });
    }
}

pub struct InputVariant {
    pub name: Ident,
    pub fields: Option<Vec<InputField>>,
}

impl InputVariant {
    fn from_syn(input: Variant) -> Result<Self> {
        return Ok(match input.fields {
            Fields::Named(fields) => InputVariant {
                name: input.ident,
                fields: Some(InputField::from_syn(fields)?),
            },
            Fields::Unit => InputVariant {
                name: input.ident,
                fields: None,
            },
            Fields::Unnamed(fields) => error(fields, "Unnamed fields are not supported.")?,
        });
    }
}

pub struct InputField {
    pub name: Ident,
}

impl InputField {
    fn from_syn(input: FieldsNamed) -> Result<Vec<Self>> {
        return input
            .named
            .into_iter()
            .map(|field| match field.ident {
                Some(name) => Ok(Self { name }),
                None => error(field, "Unnamed fields are not supported."),
            })
            .try_collect();
    }
}

fn error<T>(spanned: impl ToTokens, message: impl Display) -> Result<T> {
    return Err(Error::new_spanned(spanned, message));
}
