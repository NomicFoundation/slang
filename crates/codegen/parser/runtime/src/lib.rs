#![allow(dead_code)]

#[macro_use]
mod parser_support;

pub mod cst;
pub mod cursor;
pub mod kinds;
pub(crate) mod lexer;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[cfg(feature = "slang_napi_interfaces")]
pub mod napi_interface;

// TODO(#863): replace with the same hierarchy as the product crate:
mod user_defined {
    pub mod query {
        pub struct UserDefinedQueriesImpl;

        impl crate::query::UserDefinedQueries for UserDefinedQueriesImpl {
            // Empty Stub
        }
    }
}
