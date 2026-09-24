use slang_solidity_v2_common::utils::strings::{
    decode_escape_sequences, decode_hex_string, strip_string_literal_prefix_and_quotes,
};
use slang_solidity_v2_ir::ir;

/// Decodes the concatenated value of a collection of `StringLiteral` to its raw bytes.
///
/// Strips quotes and decodes escape sequences.
pub fn value_of_string_literals(literals: &[ir::StringLiteral]) -> Vec<u8> {
    let decode = |literal: &ir::StringLiteral| {
        decode_escape_sequences(strip_string_literal_prefix_and_quotes(&literal.text, ""))
    };
    // A lone literal (the usual case) keeps its decoded buffer, avoiding a copy.
    if let [literal] = literals {
        return decode(literal);
    }
    literals.iter().flat_map(decode).collect()
}

/// Decodes the concatenated value of a collection of `HexStringLiteral` to its raw bytes.
///
/// Strips prefix and quotes.
pub fn value_of_hex_string_literals(literals: &[ir::HexStringLiteral]) -> Vec<u8> {
    let decode = |literal: &ir::HexStringLiteral| {
        decode_hex_string(strip_string_literal_prefix_and_quotes(&literal.text, "hex"))
    };
    // A lone literal (the usual case) keeps its decoded buffer, avoiding a copy.
    if let [literal] = literals {
        return decode(literal);
    }
    literals.iter().flat_map(decode).collect()
}

/// Decodes the concatenated value of a collection of `UnicodeStringLiteral` to
/// its raw bytes.
///
/// Strips prefix and quotes, and decodes escape sequences.
pub fn value_of_unicode_string_literals(literals: &[ir::UnicodeStringLiteral]) -> Vec<u8> {
    let decode = |literal: &ir::UnicodeStringLiteral| {
        decode_escape_sequences(strip_string_literal_prefix_and_quotes(
            &literal.text,
            "unicode",
        ))
    };
    // A lone literal (the usual case) keeps its decoded buffer, avoiding a copy.
    if let [literal] = literals {
        return decode(literal);
    }
    literals.iter().flat_map(decode).collect()
}
