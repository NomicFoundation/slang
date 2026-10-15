/// Strips a prefix and removes quotes from a parsed string literal. Prefix can
/// be the empty string. Panics if the string is not quoted or the prefix
/// doesn't match.
pub fn strip_string_literal_prefix_and_quotes<'a>(text: &'a str, prefix: &str) -> &'a str {
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
        .expect("string literal prefix mismatch or not quoted")
}

/// Strips quotes from a parsed string literal. Panics if the string is not
/// quoted.
pub fn strip_string_literal_quotes(text: &str) -> &str {
    strip_string_literal_prefix_and_quotes(text, "")
}

pub fn decode_escape_sequences(content: &str) -> Vec<u8> {
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
            // The parser doesn't guarantee exactly 2 characters after `\x`,
            // if less than 2 are present we defensively pass them verbatim
            'x' => match take_hex_digits(&mut chars, 2) {
                Ok(byte) => out.push(u8::try_from(byte).unwrap()),
                Err(digits) => {
                    out.push(b'x');
                    out.extend_from_slice(digits.as_bytes());
                }
            },
            // Like `\x`, the parser doesn't guarantee what follows a `\u`
            'u' => {
                let code_point = match take_hex_digits(&mut chars, 4) {
                    Ok(code_point) => code_point,
                    Err(digits) => {
                        out.push(b'u');
                        out.extend_from_slice(digits.as_bytes());
                        continue;
                    }
                };
                match char::from_u32(code_point) {
                    Some(ch) => out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes()),
                    // Encode surrogates manually using the three-byte UTF-8 formula.
                    // UTF-8 forbids surrogate code points, so these bytes can be
                    // stored in the output Vec<u8> but cannot form a Rust String.
                    None => out.extend(
                        [
                            0xE0 | (code_point >> 12),
                            0x80 | ((code_point >> 6) & 0x3F),
                            0x80 | (code_point & 0x3F),
                        ]
                        .map(|byte| u8::try_from(byte).unwrap()),
                    ),
                }
            }
            other => {
                // The grammar allows unknown escape chars, pass
                // them through verbatim as a defensive fallback.
                out.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out
}

/// Takes the `count` hex digits of an escape sequence, and their value. An escape that is cut
/// short, which no solc version accepts but the lexer still takes, gives back the digits it has.
fn take_hex_digits(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    count: usize,
) -> Result<u32, String> {
    let mut value = 0;
    let mut digits = String::new();
    for _ in 0..count {
        match chars.next_if(char::is_ascii_hexdigit) {
            Some(digit) => {
                value = (value << 4) | digit.to_digit(16).unwrap();
                digits.push(digit);
            }
            None => return Err(digits),
        }
    }
    Ok(value)
}

pub fn decode_hex_string(content: &str) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::{
        decode_escape_sequences, decode_hex_string, strip_string_literal_prefix_and_quotes,
    };

    #[test]
    fn strip_double_quotes_no_prefix() {
        assert_eq!(
            strip_string_literal_prefix_and_quotes(r#""hello""#, ""),
            "hello"
        );
    }

    #[test]
    fn strip_single_quotes_no_prefix() {
        assert_eq!(
            strip_string_literal_prefix_and_quotes("'hello'", ""),
            "hello"
        );
    }

    #[test]
    fn strip_hex_prefix_double_quotes() {
        assert_eq!(
            strip_string_literal_prefix_and_quotes(r#"hex"414243""#, "hex"),
            "414243"
        );
    }

    #[test]
    fn strip_hex_prefix_single_quotes() {
        assert_eq!(
            strip_string_literal_prefix_and_quotes("hex'414243'", "hex"),
            "414243"
        );
    }

    #[test]
    fn strip_unicode_prefix() {
        assert_eq!(
            strip_string_literal_prefix_and_quotes(r#"unicode"ñ""#, "unicode"),
            "ñ"
        );
    }

    #[test]
    fn strip_empty_content() {
        assert_eq!(strip_string_literal_prefix_and_quotes(r#""""#, ""), "");
    }

    #[test]
    #[should_panic(expected = "string literal prefix mismatch or not quoted")]
    fn strip_panics_when_not_quoted() {
        strip_string_literal_prefix_and_quotes("unquoted", "");
    }

    #[test]
    #[should_panic(expected = "string literal prefix mismatch or not quoted")]
    fn strip_panics_on_prefix_mismatch() {
        strip_string_literal_prefix_and_quotes("hex'1234'", "unicode");
    }

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
    fn decode_unicode_escape_surrogate() {
        // Lone surrogate 0xD800 → 0xED 0xA0 0x80.
        assert_eq!(decode_escape_sequences(r"\ud800"), &[0xED, 0xA0, 0x80]);
    }

    #[test]
    fn decode_unicode_escape_surrogate_pair() {
        // Each surrogate is encoded independently, producing six bytes.
        assert_eq!(
            decode_escape_sequences(r"\ud83d\ude00"),
            &[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80]
        );
    }

    #[test]
    fn decode_invalid_escapes_without_panicking() {
        assert_eq!(decode_escape_sequences(r"\q"), b"q");
        assert_eq!(decode_escape_sequences(r"\x4"), b"x4");
        assert_eq!(decode_escape_sequences(r"\x4g"), b"x4g");
        assert_eq!(decode_escape_sequences(r"\u00e"), b"u00e");
        assert_eq!(decode_escape_sequences(r"\u"), b"u");
    }

    #[test]
    fn decode_raw_non_ascii_passthrough() {
        // Raw multibyte input (legal in unicode strings) passes through as
        // its UTF-8 encoding.
        assert_eq!(decode_escape_sequences("ñ"), &[0xC3, 0xB1]);
    }
}
