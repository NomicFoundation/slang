use slang_solidity_v2_ir::ir;

/// Decodes the concatenated value of a collection of `StringLiteral` to its raw bytes.
///
/// Strips quotes and decodes escape sequences.
pub fn value_of_string_literals(literals: &[ir::StringLiteral]) -> Vec<u8> {
    let mut result = Vec::new();
    for literal in literals {
        let content = strip_prefix_and_quotes(&literal.text, "");
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
        let content = strip_prefix_and_quotes(&literal.text, "hex");
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
        let content = strip_prefix_and_quotes(&literal.text, "unicode");
        result.extend(decode_escape_sequences(content));
    }
    result
}

fn strip_prefix_and_quotes<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.strip_prefix(prefix)
        .and_then(|stripped| {
            stripped
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| {
                    stripped
                        .strip_prefix('\'')
                        .and_then(|s| s.strip_suffix('\''))
                })
        })
        .expect("string prefix mismatch or not quoted")
}

fn decode_hex_string(content: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(content.len() / 2);
    let mut i = 0usize;
    while i < content.len() {
        // Decode pairs of hex digits skipping over underscore separators
        if content.as_bytes()[i] == b'_' {
            i += 1;
        }
        // Parser grammar guarantees that we have at least 2 more digits
        result.push(u8::from_str_radix(&content[i..i + 2], 16).unwrap());
        i += 2;
    }
    result
}

fn decode_escape_sequences(content: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len());
    let mut buf = [0u8; 4];
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        // Parser guarantees at least one char after backslash according to
        // grammar definition.
        let next = chars.next().expect("unterminated escape sequence");
        match next {
            'n' => out.push(b'\n'),
            'r' => out.push(b'\r'),
            't' => out.push(b'\t'),
            '\'' => out.push(b'\''),
            '"' => out.push(b'"'),
            '\\' => out.push(b'\\'),
            // Line continuation: `\<CR>`, `\<CRLF>`, `\<LF>` all decode to empty.
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
            }
            '\n' => {}
            'x' => {
                let h1 = u8::try_from(chars.next().unwrap().to_digit(16).unwrap()).unwrap();
                let h2 = u8::try_from(chars.next().unwrap().to_digit(16).unwrap()).unwrap();
                out.push((h1 << 4) | h2);
            }
            'u' => {
                let h1 = chars.next().unwrap().to_digit(16).unwrap();
                let h2 = chars.next().unwrap().to_digit(16).unwrap();
                let h3 = chars.next().unwrap().to_digit(16).unwrap();
                let h4 = chars.next().unwrap().to_digit(16).unwrap();
                let code_point = (h1 << 12) | (h2 << 8) | (h3 << 4) | h4;
                // `\uNNNN` code points in the surrogate range (0xD800..=0xDFFF)
                // are not valid Unicode scalars; skip them silently.
                if let Some(ch) = char::from_u32(code_point) {
                    out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                }
                // TODO(validation): emit an error/warning if the unicode scalar
                // is not valid
            }
            other => {
                // Grammar should prevent unknown escape chars, but pass
                // them through verbatim as a defensive fallback.
                out.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{decode_escape_sequences, decode_hex_string, strip_prefix_and_quotes};

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

    // ----- strip_prefix_and_quotes -----

    #[test]
    fn strip_double_quotes_no_prefix() {
        assert_eq!(strip_prefix_and_quotes(r#""hello""#, ""), "hello");
    }

    #[test]
    fn strip_single_quotes_no_prefix() {
        assert_eq!(strip_prefix_and_quotes("'hello'", ""), "hello");
    }

    #[test]
    fn strip_hex_prefix_double_quotes() {
        assert_eq!(strip_prefix_and_quotes(r#"hex"414243""#, "hex"), "414243");
    }

    #[test]
    fn strip_hex_prefix_single_quotes() {
        assert_eq!(strip_prefix_and_quotes("hex'414243'", "hex"), "414243");
    }

    #[test]
    fn strip_unicode_prefix() {
        assert_eq!(strip_prefix_and_quotes(r#"unicode"ñ""#, "unicode"), "ñ");
    }

    #[test]
    fn strip_empty_content() {
        assert_eq!(strip_prefix_and_quotes(r#""""#, ""), "");
    }

    #[test]
    #[should_panic(expected = "string prefix mismatch or not quoted")]
    fn strip_panics_when_not_quoted() {
        strip_prefix_and_quotes("unquoted", "");
    }

    #[test]
    #[should_panic(expected = "string prefix mismatch or not quoted")]
    fn strip_panics_on_prefix_mismatch() {
        strip_prefix_and_quotes("hex'1234'", "unicode");
    }
}
