mod adapter;
mod external_types;
mod helpers;

pub(crate) use adapter::*;
pub(crate) use helpers::*;
use syn::parse::ParseStream;

use crate::internals::{ErrorsCollection, Result};

pub trait ParseInputTokens: Sized {
    /// Main parser entrypoint, and should be implemented by all types.
    fn parse_value(input: ParseStream<'_>, errors: &mut ErrorsCollection) -> Result<Self>;

    /// Allows named types (structs) to parse the type name before its body.
    /// By default, it will parse the value directly if not overriden.
    fn parse_named_value(input: ParseStream<'_>, errors: &mut ErrorsCollection) -> Result<Self> {
        Self::parse_value(input, errors)
    }

    /// Allows implementations (like `Option<T>`) to modify the parsing logic,
    /// by checking if the field exists before attempting to parse it.
    fn parse_field(
        name: &str,
        input: ParseStream<'_>,
        errors: &mut ErrorsCollection,
    ) -> Result<Self> {
        ParseHelpers::field(name, input, errors)
    }
}
