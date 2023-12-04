use crate::internals::{
    parse_input_tokens::ParseHelpers, Error, ErrorsCollection, ParseInputTokens, Result, Spanned,
};
use indexmap::{IndexMap, IndexSet};
use proc_macro2::Ident;
use semver::Version;
use std::{fmt::Debug, rc::Rc};
use syn::{parse::ParseStream, LitBool, LitChar, LitStr};

impl ParseInputTokens for bool {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitBool>(input)?;

        Ok(literal.value())
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Box<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let value = T::parse_value(input, errors)?;

        Ok(value.into())
    }
}

impl ParseInputTokens for char {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitChar>(input)?;

        Ok(literal.value())
    }
}

impl<K: ParseInputTokens + std::hash::Hash + Eq, V: ParseInputTokens> ParseInputTokens
    for IndexMap<K, V>
{
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        ParseHelpers::map(input, errors)
    }
}

impl<T: ParseInputTokens + std::hash::Hash + Ord> ParseInputTokens for IndexSet<Spanned<T>> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let sequence: Vec<Spanned<T>> = ParseHelpers::sequence(input, errors)?;

        let mut set = Self::new();

        for item in sequence {
            if set.contains(&item) {
                errors.add(&item, &Errors::DuplicateEntry);
            }

            set.insert(item);
        }

        Ok(set)
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Option<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::parse_value(input, errors)?))
        }
    }

    fn parse_field(name: &str, input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        match ParseHelpers::syn::<Ident>(&input.fork()) {
            Ok(key) if key == name => Ok(Some(ParseHelpers::field(name, input, errors)?)),
            _ => Ok(None),
        }
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Rc<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let value = T::parse_value(input, errors)?;

        Ok(value.into())
    }
}

impl ParseInputTokens for String {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitStr>(input)?;
        let value = literal.value();

        if value.is_empty() {
            errors.add(&literal, &Errors::EmptyString);
        }

        Ok(value)
    }
}

impl ParseInputTokens for usize {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<syn::LitInt>(input)?;

        literal.base10_parse::<usize>().map_err(Error::from_syn)
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Vec<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        ParseHelpers::sequence(input, errors)
    }
}

impl ParseInputTokens for Version {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitStr>(input)?;

        match Self::parse(&literal.value()) {
            Ok(version) => Ok(version),
            Err(error) => {
                errors.add(&literal, &error);

                Ok(Version::new(0, 0, 0))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Set entries must be unique.")]
    DuplicateEntry,
    #[error("Expected a non-empty string.")]
    EmptyString,
}
