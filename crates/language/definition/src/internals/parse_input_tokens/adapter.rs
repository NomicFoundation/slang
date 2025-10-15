use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

use crate::internals::{ErrorsCollection, ParseInputTokens, Result};
use crate::model::SpannedLanguage;

pub(crate) struct ParseAdapter;

impl ParseAdapter {
    pub fn parse(input: TokenStream) -> Result<ParseOutput> {
        Ok(syn::parse2(input)?)
    }
}

pub(crate) struct ParseOutput {
    pub language: SpannedLanguage,
    pub errors: ErrorsCollection,
}

/// A wrapper around [`syn::parse::Parse`] to convert to/from our own error types.
impl Parse for ParseOutput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut errors = ErrorsCollection::new();

        let language = SpannedLanguage::parse_named_value(input, &mut errors)?;

        Ok(Self { language, errors })
    }
}
