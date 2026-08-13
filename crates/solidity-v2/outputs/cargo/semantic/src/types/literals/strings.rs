use slang_solidity_v2_common::utils::strings::{
    decode_escape_sequences, decode_hex_string, strip_string_literal_prefix_and_quotes,
};
use slang_solidity_v2_ir::ir;

/// Decodes the concatenated value of a collection of `StringLiteral` to its raw bytes.
///
/// Strips quotes and decodes escape sequences.
pub fn value_of_string_literals(literals: &[ir::StringLiteral]) -> Vec<u8> {
    let mut result = Vec::new();
    for literal in literals {
        let content = strip_string_literal_prefix_and_quotes(&literal.text, "");
        result.extend(decode_escape_sequences(content));
    }
    result
}

/// Decodes the concatenated value of a collection of `HexStringLiteral` to its raw bytes.
///
/// Strips prefix and quotes.
pub fn value_of_hex_string_literals(literals: &[ir::HexStringLiteral]) -> Vec<u8> {
    let mut result = Vec::new();
    for literal in literals {
        let content = strip_string_literal_prefix_and_quotes(&literal.text, "hex");
        result.extend(decode_hex_string(content));
    }
    result
}

/// Decodes the concatenated value of a collection of `UnicodeStringLiteral` to
/// its raw bytes.
///
/// Strips prefix and quotes, and decodes escape sequences.
pub fn value_of_unicode_string_literals(literals: &[ir::UnicodeStringLiteral]) -> Vec<u8> {
    let mut result = Vec::new();
    for literal in literals {
        let content = strip_string_literal_prefix_and_quotes(&literal.text, "unicode");
        result.extend(decode_escape_sequences(content));
    }
    result
}
