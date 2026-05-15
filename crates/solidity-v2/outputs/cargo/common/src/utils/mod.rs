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

#[cfg(test)]
mod tests {
    use super::strip_string_literal_prefix_and_quotes;

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
}
