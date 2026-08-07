use slang_solidity_v2_common::utils::{decode_escape_sequences, decode_hex_string, strip_string_literal_prefix_and_quotes};
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

#[cfg(test)]
mod tests {
    use super::{decode_escape_sequences, decode_hex_string};

    // ----- decode_hex_string ------

    #[test]
    fn decode_hex_string_no_underscores() {
        assert_eq!(decode_hex_string("303132"), b"012");
    }

    #[test]
    fn decode_hex_string_with_underscores() {
        assert_eq!(decode_hex_string("30_31_32"), b"012");
    }

    // ----- decode_escape_sequences -----

    #[test]
    fn decode_empty() {
        assert_eq!(decode_escape_sequences(""), Vec::<u8>::new());
    }

    #[test]
    fn decode_no_escapes() {
        assert_eq!(decode_escape_sequences("hello"), b"hello");
    }

    #[test]
    fn decode_ascii_escapes() {
        assert_eq!(decode_escape_sequences(r"\n"), b"\n");
        assert_eq!(decode_escape_sequences(r"\r"), b"\r");
        assert_eq!(decode_escape_sequences(r"\t"), b"\t");
        assert_eq!(decode_escape_sequences(r"\'"), b"'");
        assert_eq!(decode_escape_sequences(r#"\""#), b"\"");
        assert_eq!(decode_escape_sequences(r"\\"), b"\\");
    }

    #[test]
    fn decode_mixed_text_and_escapes() {
        assert_eq!(
            decode_escape_sequences(r"hello\tworld\n"),
            b"hello\tworld\n",
        );
    }

    #[test]
    fn decode_multiple_ascii_escapes() {
        assert_eq!(
            decode_escape_sequences(r#"\t\n\r\'\"\\"#),
            &[0x09, 0x0a, 0x0d, 0x27, 0x22, 0x5c]
        );
    }

    #[test]
    fn decode_line_continuation_lf() {
        // Backslash followed by raw LF → empty.
        assert_eq!(decode_escape_sequences("a\\\nb"), b"ab");
    }

    #[test]
    fn decode_line_continuation_cr() {
        // Backslash followed by raw CR → empty.
        assert_eq!(decode_escape_sequences("a\\\rb"), b"ab");
    }

    #[test]
    fn decode_line_continuation_crlf() {
        // Backslash followed by raw CRLF → empty (CR and LF both consumed).
        assert_eq!(decode_escape_sequences("a\\\r\nb"), b"ab");
    }

    #[test]
    fn decode_hex_byte_escape() {
        assert_eq!(decode_escape_sequences(r"\x41"), b"A");
        assert_eq!(decode_escape_sequences(r"\xff"), &[0xffu8]);
        assert_eq!(decode_escape_sequences(r"\x00"), &[0x00u8]);
    }

    #[test]
    fn decode_unicode_escape_ascii() {
        // U+0041 → 'A' (single UTF-8 byte).
        assert_eq!(decode_escape_sequences(r"\u0041"), b"A");
    }

    #[test]
    fn decode_unicode_escape_multibyte() {
        // U+00E9 é → 0xC3 0xA9.
        assert_eq!(decode_escape_sequences(r"\u00e9"), &[0xC3, 0xA9]);
        // U+2713 ✓ → 0xE2 0x9C 0x93.
        assert_eq!(decode_escape_sequences(r"\u2713"), &[0xE2, 0x9C, 0x93]);
    }

    #[test]
    fn decode_unicode_escape_combined_multibyte() {
        // Dollar sign
        assert_eq!(decode_escape_sequences(r"aaa\u0024aaa"), b"aaa$aaa");
        // Cent
        assert_eq!(decode_escape_sequences(r"aaa\u00A2aaa"), b"aaa\xc2\xa2aaa");
        // Euro
        assert_eq!(
            decode_escape_sequences(r"aaa\u20ACaaa"),
            b"aaa\xe2\x82\xacaaa"
        );
        // All combined
        assert_eq!(
            decode_escape_sequences(r"\u0024\u00A2\u20AC"),
            b"$\xc2\xa2\xe2\x82\xac"
        );
    }

    #[test]
    fn decode_unicode_escape_surrogate_skipped() {
        // Lone surrogate 0xD800 is not a valid Unicode scalar → skipped.
        assert_eq!(decode_escape_sequences(r"\ud800"), Vec::<u8>::new());
    }

    #[test]
    fn decode_raw_non_ascii_passthrough() {
        // Raw multibyte input (legal in unicode strings) passes through as
        // its UTF-8 encoding.
        assert_eq!(decode_escape_sequences("ñ"), &[0xC3, 0xB1]);
    }
}
