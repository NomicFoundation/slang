use crate::internals::{Error, ErrorsCollection, ParseInputTokens, Result, Spanned};
use indexmap::IndexMap;
use proc_macro2::{extra::DelimSpan, Delimiter, Ident, TokenStream};
use std::fmt::Debug;
use syn::{braced, bracketed, parenthesized, parse::ParseStream, Token};

pub struct ParseHelpers;

impl ParseHelpers {
    pub fn syn<T: syn::parse::Parse>(input: ParseStream) -> Result<T> {
        return input.parse::<T>().map_err(|error| {
            return Error::from_syn(error);
        });
    }

    pub fn delimited<T>(
        delimiter: Delimiter,
        input: ParseStream,
        inner_parser: impl FnOnce(DelimSpan, ParseStream) -> Result<T>,
    ) -> Result<T> {
        return delimited(delimiter, input, inner_parser).map_err(|error| {
            return Error::from_syn(error);
        });
    }

    pub fn sequence<T: ParseInputTokens>(
        input: ParseStream,
        errors: &mut ErrorsCollection,
    ) -> Result<Vec<T>> {
        match Self::delimited(Delimiter::Bracket, input, |_, inner_input| {
            let mut result = Vec::new();

            while !inner_input.is_empty() {
                result.push(ParseInputTokens::parse_named_value(inner_input, errors)?);

                if !inner_input.is_empty() {
                    let comma = Self::syn::<Token![,]>(inner_input)?;

                    if inner_input.is_empty() {
                        errors.add(&comma, &Errors::TrailingComma);
                    }
                }
            }

            return Ok(result);
        }) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                errors.push(error);
                return Ok(vec![]);
            }
        };
    }

    pub fn map<K: ParseInputTokens + std::hash::Hash + Eq, V: ParseInputTokens>(
        input: ParseStream,
        errors: &mut ErrorsCollection,
    ) -> Result<IndexMap<Spanned<K>, Spanned<V>>> {
        match Self::delimited(Delimiter::Parenthesis, input, |_, inner_input| {
            let mut result = IndexMap::new();

            while !inner_input.is_empty() {
                let key = ParseInputTokens::parse_named_value(inner_input, errors)?;

                Self::syn::<Token![=]>(inner_input)?;

                let value = ParseInputTokens::parse_named_value(inner_input, errors)?;

                if !inner_input.is_empty() {
                    let comma = Self::syn::<Token![,]>(inner_input)?;

                    if inner_input.is_empty() {
                        errors.add(&comma, &Errors::TrailingComma);
                    }
                }

                if result.contains_key(&key) {
                    errors.add(&key, &Errors::DuplicateMapKey);
                } else {
                    result.insert(key, value);
                }
            }

            return Ok(result);
        }) {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                errors.push(error);
                return Ok(IndexMap::new());
            }
        }
    }

    pub fn field<T: ParseInputTokens>(
        name: &str,
        input: ParseStream,
        errors: &mut ErrorsCollection,
    ) -> Result<T> {
        let span = input.span();
        match Self::syn::<Ident>(input) {
            Ok(key) if key == name => {}
            _ => return Error::fatal(&span, &Errors::ExpectedField(name)),
        };

        Self::syn::<Token![=]>(input)?;

        let value = ParseInputTokens::parse_named_value(input, errors)?;

        if !input.is_empty() {
            let comma = Self::syn::<Token![,]>(input)?;

            if input.is_empty() {
                errors.add(&comma, &Errors::TrailingComma);
            }
        }

        return Ok(value);
    }
}

/// A wrapper to convert error types in callsites, since the macros below requires returning [syn::Result].
fn delimited<T>(
    delimiter: Delimiter,
    input: ParseStream,
    inner_parser: impl FnOnce(DelimSpan, ParseStream) -> Result<T>,
) -> syn::Result<T> {
    let inner_input;
    let span = match delimiter {
        Delimiter::Brace => braced!(inner_input in input).span,
        Delimiter::Bracket => bracketed!(inner_input in input).span,
        Delimiter::Parenthesis => parenthesized!(inner_input in input).span,
        Delimiter::None => {
            return Err(syn::Error::new(input.span(), "Expected a delimited block."));
        }
    };

    return inner_parser(span, &inner_input).map_err(|error| {
        // consume the rest of 'inner_input' so that we don't end up
        // reporting an extra/unnecessary "unexpected token" error:
        inner_input.parse::<TokenStream>().ok();

        return error.to_syn();
    });
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Expected field: '{0}'.")]
    ExpectedField(&'err str),
    #[error("Duplicate map key.")]
    DuplicateMapKey,
    #[error("Trailing commas are not allowed.")]
    TrailingComma,
}
