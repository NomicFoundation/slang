#[allow(unused_imports)]
use super::builder;
#[allow(unused_imports)]
use crate::chumsky::combinators::NomicParser;
use chumsky::{prelude::*, Parser};

pub type ErrorType = Simple<char>;
