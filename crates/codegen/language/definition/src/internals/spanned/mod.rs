use crate::internals::{ErrorsCollection, ParseInputTokens, Result, WriteOutputTokens};
use proc_macro2::{Span, TokenStream};
use std::cmp::Ordering;
use syn::parse::ParseStream;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    span: Span,
    value: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, value: T) -> Self {
        return Self { span, value };
    }

    pub fn span(&self) -> Span {
        return self.span;
    }

    pub fn into_value(self) -> T {
        return self.value;
    }
}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl<T> std::fmt::Display for Spanned<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value.fmt(f);
    }
}

impl<T: Eq> Eq for Spanned<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.value.hash(state);
    }
}

impl<T: Ord> Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.value.cmp(&other.value);
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.value.partial_cmp(&other.value);
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

impl<T: WriteOutputTokens> WriteOutputTokens for Spanned<T> {
    fn write_output_tokens(&self) -> TokenStream {
        // 'Spanned' is removed from macro output, leaving only the inner value:
        return self.value.write_output_tokens();
    }
}
