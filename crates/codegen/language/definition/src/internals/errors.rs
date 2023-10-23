use proc_macro2::{Span, TokenStream};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

/// Our own proxy for [syn::Error] since the latter does not expose the underlying sub-errors.
#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn fatal<T>(has_span: &impl HasSpan, message: &impl Display) -> Result<T> {
        return Err(Error {
            message: message.to_string(),
            span: has_span.span(),
        });
    }

    pub fn from_syn(error: syn::Error) -> Self {
        return Self {
            message: error.to_string(),
            span: error.span(),
        };
    }

    pub fn to_syn(&self) -> syn::Error {
        return syn::Error::new(self.span, &self.message);
    }

    pub fn to_compile_error(&self) -> TokenStream {
        return syn::Error::new(self.span, &self.message).to_compile_error();
    }
}

#[derive(Debug)]
pub struct ErrorsCollection {
    errors: Vec<Error>,
}

impl ErrorsCollection {
    pub fn new() -> Self {
        return Self { errors: vec![] };
    }

    pub fn has_errors(&self) -> bool {
        return !self.errors.is_empty();
    }

    pub fn add(&mut self, has_span: &impl HasSpan, message: &impl Display) {
        self.errors.push(Error {
            message: message.to_string(),
            span: has_span.span(),
        });
    }

    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn to_compile_errors(&self) -> TokenStream {
        return self
            .errors
            .iter()
            .map(|error| error.to_compile_error())
            .collect();
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

impl<T> HasSpan for crate::internals::Spanned<T> {
    fn span(&self) -> Span {
        return self.span();
    }
}

impl<T: syn::spanned::Spanned> HasSpan for T {
    fn span(&self) -> Span {
        return self.span();
    }
}
