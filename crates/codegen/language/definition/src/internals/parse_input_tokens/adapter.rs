use crate::{
    internals::{Error, ErrorsCollection, ParseInputTokens, Result},
    model::SpannedLanguage,
};
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

pub(crate) struct ParseAdapter;

impl ParseAdapter {
    pub fn parse(input: TokenStream) -> Result<ParseOutput> {
        return syn::parse2(input).map_err(Error::from_syn);
    }
}

pub(crate) struct ParseOutput {
    pub language: SpannedLanguage,
    pub errors: ErrorsCollection,
}

/// A wrapper around [syn::parse::Parse] to convert to/from our own error types.
impl Parse for ParseOutput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = ErrorsCollection::new();

        match SpannedLanguage::parse_named_value(input, &mut errors) {
            Ok(language) => {
                return Ok(Self { language, errors });
            }
            Err(error) => {
                return Err(error.to_syn());
            }
        };
    }
}
