use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{LiteralKind, Type};

use super::super::{
    DecimalNumberExpressionStruct, FunctionCallExpressionStruct, HexNumberExpressionStruct,
    StringExpression,
};

impl StringExpression {
    /// Returns the concatenated decoded string value as bytes.
    ///
    /// Handles all three variants:
    /// - `StringLiterals` — strips quotes, decodes escape sequences, returns
    ///   the decoded bytes
    /// - `HexStringLiterals` — strips `hex"..."`, decodes hex pairs to bytes
    /// - `UnicodeStringLiterals` — strips `unicode"..."`, decodes escape
    ///   sequences, returns UTF-8 bytes
    ///
    /// Escape sequences decoded for string and unicode-string variants:
    /// `\n`, `\r`, `\t`, `\'`, `\"`, `\\`, line continuations (`\<LF>`,
    /// `\<CR>`, `\<CRLF>`), `\xNN` (hex byte), and `\uNNNN` (Unicode code
    /// point, UTF-8 encoded).
    pub fn value(&self) -> Vec<u8> {
        // TODO: this implementation should live in the semantic crate to be
        // used by the typing pass. This is required for example to check for
        // valid implicit conversion of a string into a bytesXX array.
        let mut result = Vec::new();
        match self {
            StringExpression::StringLiterals(terminals) => {
                for terminal in terminals {
                    let content = strip_prefix_and_quotes(&terminal.text, "");
                    result.extend(decode_escape_sequences(content));
                }
            }
            StringExpression::HexStringLiterals(terminals) => {
                for terminal in terminals {
                    let content = strip_prefix_and_quotes(&terminal.text, "hex");
                    result.extend(decode_hex_string(content));
                }
            }
            StringExpression::UnicodeStringLiterals(terminals) => {
                for terminal in terminals {
                    let content = strip_prefix_and_quotes(&terminal.text, "unicode");
                    result.extend(decode_escape_sequences(content));
                }
            }
        }
        result
    }
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
        // Grammar guarantees at least one char after backslash; if input
        // is malformed we just stop.
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
                let h1 = chars.next().unwrap();
                let h2 = chars.next().unwrap();
                let hex: String = [h1, h2].iter().collect();
                out.push(u8::from_str_radix(&hex, 16).unwrap());
            }
            'u' => {
                let h1 = chars.next().unwrap();
                let h2 = chars.next().unwrap();
                let h3 = chars.next().unwrap();
                let h4 = chars.next().unwrap();
                let hex: String = [h1, h2, h3, h4].iter().collect();
                let code_point = u32::from_str_radix(&hex, 16).unwrap();
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

impl DecimalNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` for rationals
    /// that do not reduce to an integer after unit multiplication.
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }

    /// Returns the rational value of this literal, or `None` if the literal
    /// cannot be evaluated. Integer literals lift to a rational with
    /// denominator one.
    pub fn rational_value(&self) -> Option<BigRational> {
        rational_value_of_node(&self.semantic, self.ir_node.id())
    }
}

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }
}

/// Returns the integer value carried by a number-literal node's type, when
/// the type is an integer-shaped literal. Rational literals return `None`.
fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }) => {
            Some(value.clone())
        }
        _ => None,
    }
}

/// Returns the rational value carried by a number-literal node's type. Integer
/// literals are returned as a rational with denominator one.
fn rational_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigRational> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    match semantic.types().get_type_by_id(type_id) {
        Type::Literal(LiteralKind::Integer(value) | LiteralKind::HexInteger { value, .. }) => {
            Some(BigRational::from(value.clone()))
        }
        Type::Literal(LiteralKind::Rational(value)) => Some(value.clone()),
        _ => None,
    }
}

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            ir::Expression::ElementaryType(_) | ir::Expression::PayableKeyword => true,
            ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder().node_typing(terminal.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder().node_typing(mae.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            _ => false,
        }
    }
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
