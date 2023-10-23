use crate::{
    internals::{
        parse_input_tokens::ParseHelpers, Error, ErrorsCollection, ParseInputTokens, Result,
        Spanned,
    },
    Identifier,
};
use indexmap::{IndexMap, IndexSet};
use infra_utils::paths::PathExtensions;
use proc_macro2::Ident;
use semver::Version;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    rc::Rc,
};
use syn::{parse::ParseStream, LitBool, LitChar, LitStr};

impl ParseInputTokens for bool {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitBool>(input)?;

        return Ok(literal.value());
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Box<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let value = T::parse_value(input, errors)?;

        return Ok(value.into());
    }
}

impl ParseInputTokens for char {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitChar>(input)?;

        return Ok(literal.value());
    }
}

impl ParseInputTokens for Identifier {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let value = ParseHelpers::syn::<Ident>(input)?.to_string();

        return Ok(value.into());
    }
}

impl<K: ParseInputTokens + std::hash::Hash + std::cmp::Eq, V: ParseInputTokens> ParseInputTokens
    for IndexMap<Spanned<K>, Spanned<V>>
{
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        return ParseHelpers::map(input, errors);
    }
}

impl<T: ParseInputTokens + std::hash::Hash + std::cmp::Ord> ParseInputTokens
    for IndexSet<Spanned<T>>
{
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let sequence: Vec<Spanned<T>> = ParseHelpers::sequence(input, errors)?;

        let mut set = Self::new();

        for item in sequence {
            if set.contains(&item) {
                errors.add(&item, &Errors::DuplicateEntry);
            }

            set.insert(item);
        }

        return Ok(set);
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Option<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        if input.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(T::parse_value(input, errors)?));
        }
    }

    fn parse_field(name: &str, input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        match ParseHelpers::syn::<Ident>(&input.fork()) {
            Ok(key) if key.to_string() == name => {
                return Ok(Some(ParseHelpers::field(name, input, errors)?));
            }
            _ => {
                return Ok(None);
            }
        };
    }
}

impl ParseInputTokens for PathBuf {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let literal = &ParseHelpers::syn::<LitStr>(input)?;
        let value = literal.value();
        let full_path = Path::repo_path(value);

        if !full_path.exists() {
            errors.add(literal, &Errors::PathNotFound(&full_path));
        }

        return Ok(full_path);
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Rc<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let value = T::parse_value(input, errors)?;

        return Ok(value.into());
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Spanned<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let span = input.span();
        let value = T::parse_value(input, errors)?;

        return Ok(Self::new(span, value));
    }

    fn parse_named_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let span = input.span();
        let value = ParseInputTokens::parse_named_value(input, errors)?;

        return Ok(Self::new(span, value));
    }
}

impl ParseInputTokens for String {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitStr>(input)?;
        let value = literal.value();

        if value.is_empty() {
            errors.add(&literal, &Errors::EmptyString);
        }

        return Ok(value);
    }
}

impl ParseInputTokens for usize {
    fn parse_value(input: ParseStream, _: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<syn::LitInt>(input)?;

        return literal
            .base10_parse::<usize>()
            .map_err(|error| Error::from_syn(error));
    }
}

impl<T: ParseInputTokens> ParseInputTokens for Vec<T> {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        return ParseHelpers::sequence(input, errors);
    }
}

impl ParseInputTokens for Version {
    fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> Result<Self> {
        let literal = ParseHelpers::syn::<LitStr>(input)?;

        match Self::parse(&literal.value()) {
            Ok(version) => {
                return Ok(version);
            }
            Err(error) => {
                errors.add(&literal, &error);

                return Ok(Version::new(0, 0, 0));
            }
        };
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Set entries must be unique.")]
    DuplicateEntry,
    #[error("Expected a non-empty string.")]
    EmptyString,
    #[error("Path not found: {0}")]
    PathNotFound(&'err PathBuf),
}
