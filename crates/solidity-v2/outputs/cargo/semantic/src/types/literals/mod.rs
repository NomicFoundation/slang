pub(crate) mod numbers;
pub(crate) mod strings;

pub use numbers::{value_of_decimal_literal, value_of_hex_literal};
pub use strings::{
    value_of_hex_string_literals, value_of_string_literals, value_of_unicode_string_literals,
};
