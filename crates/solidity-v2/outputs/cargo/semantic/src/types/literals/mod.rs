pub(crate) mod numbers;
pub(crate) mod strings;
pub(crate) mod yul;

pub use numbers::value_of_hex_number_expression;
pub use strings::{
    value_of_hex_string_literals, value_of_string_literals, value_of_unicode_string_literals,
};
pub use yul::yul_literal_value;
