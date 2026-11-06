mod validate_string_literal;
#[path = "validate_syntax_version.generated.rs"]
mod validate_syntax_version;

pub(crate) use validate_string_literal::{
    validate_string_literal, validate_unicode_string_literal,
};
pub use validate_syntax_version::validate_syntax_version;
