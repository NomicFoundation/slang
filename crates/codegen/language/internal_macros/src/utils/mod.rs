use quote::ToTokens;
use std::fmt::Display;
use syn::{Error, Result};

pub fn error<T>(spanned: impl ToTokens, message: impl Display) -> Result<T> {
    return Err(Error::new_spanned(spanned, message));
}
