use crate::{
    internals::{Error, ErrorsCollection, ParseInputTokens, Result},
    model::Language,
};
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

pub struct ParseAdapter;

impl ParseAdapter {
    pub fn parse(input: TokenStream) -> Result<ParseOutput> {
        return syn::parse2(input).map_err(|error| Error::from_syn(error));
    }
}

pub struct ParseOutput {
    pub language: Language,
    pub errors: ErrorsCollection,
}

/// A wrapper around [syn::parse::Parse] to convert to/from our own error types.
impl Parse for ParseOutput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = ErrorsCollection::new();

        match Language::parse_named_value(input, &mut errors) {
            Ok(language) => {
                return Ok(Self { language, errors });
            }
            Err(error) => {
                return Err(error.to_syn());
            }
        };
    }
}
