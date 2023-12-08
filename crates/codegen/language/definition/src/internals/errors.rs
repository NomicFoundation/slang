use std::fmt::Display;

use proc_macro2::{Span, TokenStream};

pub type Result<T> = std::result::Result<T, Error>;

/// Our own proxy for [`syn::Error`] since the latter does not expose the underlying sub-errors.
#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn fatal<T>(has_span: &impl HasSpan, message: &impl Display) -> Result<T> {
        Err(Error {
            message: message.to_string(),
            span: has_span.span(),
        })
    }

    pub fn from_syn(error: &syn::Error) -> Self {
        Self {
            message: error.to_string(),
            span: error.span(),
        }
    }

    pub fn to_syn(&self) -> syn::Error {
        syn::Error::new(self.span, &self.message)
    }

    pub fn to_compile_error(&self) -> TokenStream {
        syn::Error::new(self.span, &self.message).to_compile_error()
    }
}

impl From<syn::Error> for Error {
    fn from(error: syn::Error) -> Self {
        Self::from_syn(&error)
    }
}

impl From<Error> for syn::Error {
    fn from(error: Error) -> Self {
        Error::to_syn(&error)
    }
}

#[derive(Debug)]
pub struct ErrorsCollection {
    errors: Vec<Error>,
}

impl ErrorsCollection {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
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
        self.errors.iter().map(Error::to_compile_error).collect()
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

impl<T> HasSpan for crate::internals::Spanned<T> {
    fn span(&self) -> Span {
        self.span()
    }
}

impl<T: syn::spanned::Spanned> HasSpan for T {
    fn span(&self) -> Span {
        self.span()
    }
}
