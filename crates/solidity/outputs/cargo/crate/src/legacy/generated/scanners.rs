// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::language::Language;
use super::stream::*;

impl Language {
    // ABICODER_KEYWORD = "abicoder";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn abicoder_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 'b', 'i', 'c', 'o', 'd', 'e', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.6.0 *)
    // ABSTRACT_KEYWORD = "abstract";

    #[allow(dead_code, non_snake_case)]
    fn abstract_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 'b', 's', 't', 'r', 'a', 'c', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn abstract_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.abstract_keyword__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn abstract_keyword(&self, stream: &mut Stream) -> bool {
        self.abstract_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ADDRESS_KEYWORD = "address";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn address_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 'd', 'd', 'r', 'e', 's', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // AMPERSAND = "&";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '&'),
            scan_predicate!(stream, |c| c == '&' || c == '=')
        )
    }

    // AMPERSAND_AMPERSAND = "&&";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand_ampersand(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '&')
    }

    // AMPERSAND_EQUAL = "&=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '=')
    }

    // ANONYMOUS_KEYWORD = "anonymous";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn anonymous_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 'n', 'o', 'n', 'y', 'm', 'o', 'u', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // AS_KEYWORD = "as";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn as_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «ASCII_ESCAPE» = "n" | "r" | "t" | "'" | '"' | "\\" | "\n" | "\r";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ascii_escape(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '\n'
            || c == '\r'
            || c == '"'
            || c == '\''
            || c == '\\'
            || c == 'n'
            || c == 'r'
            || c == 't')
    }

    // ASCII_STRING_LITERAL = «SINGLE_QUOTED_ASCII_STRING_LITERAL»
    //                      | «DOUBLE_QUOTED_ASCII_STRING_LITERAL»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_choice!(
                stream,
                self.single_quoted_ascii_string_literal(stream),
                self.double_quoted_ascii_string_literal(stream)
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // ASSEMBLY_KEYWORD = "assembly";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn assembly_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'a', 's', 's', 'e', 'm', 'b', 'l', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // ASTERISK = "*";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '*'),
            scan_predicate!(stream, |c| c == '*' || c == '=')
        )
    }

    // ASTERISK_ASTERISK = "**";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk_asterisk(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '*')
    }

    // ASTERISK_EQUAL = "*=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '=')
    }

    // BANG = "!";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bang(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '!'), scan_chars!(stream, '='))
    }

    // BANG_EQUAL = "!=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bang_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '!', '=')
    }

    // BAR = "|";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '|'),
            scan_predicate!(stream, |c| c == '=' || c == '|')
        )
    }

    // BAR_BAR = "||";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar_bar(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '|')
    }

    // BAR_EQUAL = "|=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '=')
    }

    // BOOL_KEYWORD = "bool";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bool_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'b', 'o', 'o', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // BREAK_KEYWORD = "break";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn break_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'b', 'r', 'e', 'a', 'k'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.11 *)
    // BYTE_KEYWORD = "byte";

    #[allow(dead_code, non_snake_case)]
    fn byte_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'b', 'y', 't', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn byte_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            None
        } else {
            Some(self.byte_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn byte_keyword(&self, stream: &mut Stream) -> bool {
        self.byte_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.5.0 *)
    // CALLDATA_KEYWORD = "calldata";

    #[allow(dead_code, non_snake_case)]
    fn calldata_keyword__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'a', 'l', 'l', 'd', 'a', 't', 'a'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn calldata_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            Some(self.calldata_keyword__0_5_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn calldata_keyword(&self, stream: &mut Stream) -> bool {
        self.calldata_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // CARET = "^";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn caret(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '^'), scan_chars!(stream, '='))
    }

    // CARET_EQUAL = "^=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn caret_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '^', '=')
    }

    // CASE_KEYWORD = "case";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn case_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'a', 's', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.6.0 *)
    // CATCH_KEYWORD = "catch";

    #[allow(dead_code, non_snake_case)]
    fn catch_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'a', 't', 'c', 'h'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn catch_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.catch_keyword__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn catch_keyword(&self, stream: &mut Stream) -> bool {
        self.catch_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // CLOSE_BRACE = "}";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_brace(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '}')
    }

    // CLOSE_BRACKET = "]";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_bracket(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ']')
    }

    // CLOSE_PAREN = ")";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_paren(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ')')
    }

    // COLON = ":";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn colon(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, ':'), scan_chars!(stream, '='))
    }

    // COLON_EQUAL = ":=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn colon_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ':', '=')
    }

    // COMMA = ",";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn comma(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ',')
    }

    // CONSTANT_KEYWORD = "constant";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn constant_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'o', 'n', 's', 't', 'a', 'n', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.22 *)
    // CONSTRUCTOR_KEYWORD = "constructor";

    #[allow(dead_code, non_snake_case)]
    fn constructor_keyword__0_4_22(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'o', 'n', 's', 't', 'r', 'u', 'c', 't', 'o', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn constructor_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.constructor_keyword__0_4_22(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn constructor_keyword(&self, stream: &mut Stream) -> bool {
        self.constructor_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // CONTINUE_KEYWORD = "continue";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn continue_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'o', 'n', 't', 'i', 'n', 'u', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // CONTRACT_KEYWORD = "contract";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn contract_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'c', 'o', 'n', 't', 'r', 'a', 'c', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // DAYS_KEYWORD = "days";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn days_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'd', 'a', 'y', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «DECIMAL_DIGITS» = "0"…"9"+ ("_" "0"…"9"+)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn decimal_digits(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9'))),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '_'),
                    scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
                )
            )
        )
    }

    // «DECIMAL_EXPONENT» = ("e" | "E") "-"? «DECIMAL_DIGITS»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn decimal_exponent(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_predicate!(stream, |c| c == 'E' || c == 'e'),
            scan_optional!(stream, scan_chars!(stream, '-')),
            self.decimal_digits(stream)
        )
    }

    // (* v0.4.11 *)
    // DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»?)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;

    #[allow(dead_code, non_snake_case)]
    fn decimal_literal__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_choice!(
                    stream,
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(
                                scan_chars!(stream, '.'),
                                scan_optional!(stream, self.decimal_digits(stream))
                            )
                        )
                    ),
                    scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
                ),
                scan_optional!(stream, self.decimal_exponent(stream))
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.5.0 *)
    // DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;

    #[allow(dead_code, non_snake_case)]
    fn decimal_literal__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_choice!(
                    stream,
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
                        )
                    ),
                    scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
                ),
                scan_optional!(stream, self.decimal_exponent(stream))
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    pub(crate) fn decimal_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.decimal_literal__0_5_0(stream)
        } else {
            self.decimal_literal__0_4_11(stream)
        }
    }

    // DEFAULT_KEYWORD = "default";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn default_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'd', 'e', 'f', 'a', 'u', 'l', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // DELETE_KEYWORD = "delete";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn delete_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'd', 'e', 'l', 'e', 't', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // DO_KEYWORD = "do";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn do_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'd', 'o'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «DOUBLE_QUOTED_ASCII_STRING_LITERAL» = '"' («ESCAPE_SEQUENCE» | (" "…"~" - ('"' | "\\")))* '"';

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    scan_predicate!(stream, |c| (' ' <= c && c <= '!')
                        || ('#' <= c && c <= '[')
                        || (']' <= c && c <= '~'))
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    // «DOUBLE_QUOTED_HEX_STRING_LITERAL» = "hex" '"' «HEX_STRING_CONTENTS»? '"';

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn double_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x'),
            scan_chars!(stream, '"'),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '"')
        )
    }

    // (* v0.7.0 *)
    // «DOUBLE_QUOTED_UNICODE_STRING_LITERAL» = 'unicode"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\n" | "\r"))* '"';

    #[allow(dead_code, non_snake_case)]
    fn double_quoted_unicode_string_literal__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    scan_predicate!(stream, |c| c != '\n' && c != '\r' && c != '"' && c != '\\')
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn double_quoted_unicode_string_literal__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            Some(self.double_quoted_unicode_string_literal__0_7_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.double_quoted_unicode_string_literal__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ELSE_KEYWORD = "else";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn else_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'l', 's', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.21 *)
    // EMIT_KEYWORD = "emit";

    #[allow(dead_code, non_snake_case)]
    fn emit_keyword__0_4_21(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'm', 'i', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn emit_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_4_21 {
            Some(self.emit_keyword__0_4_21(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn emit_keyword(&self, stream: &mut Stream) -> bool {
        self.emit_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // END_OF_LINE = "\r"? "\n";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn end_of_line(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_optional!(stream, scan_chars!(stream, '\r')),
            scan_chars!(stream, '\n')
        )
    }

    // ENUM_KEYWORD = "enum";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn enum_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'n', 'u', 'm'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // EQUAL = "=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '='),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // EQUAL_EQUAL = "==";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '=')
    }

    // EQUAL_GREATER_THAN = "=>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal_greater_than(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '>')
    }

    // ERROR_KEYWORD = "error";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn error_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'r', 'r', 'o', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «ESCAPE_SEQUENCE» = "\\" («ASCII_ESCAPE» | «HEX_BYTE_ESCAPE» | «UNICODE_ESCAPE»);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn escape_sequence(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\\'),
            scan_choice!(
                stream,
                scan_trie!(stream, ['\n' | '\r' | '"' | '\'' | '\\' | 'n' | 'r' | 't']),
                self.hex_byte_escape(stream),
                self.unicode_escape(stream)
            )
        )
    }

    // ETHER_KEYWORD = "ether";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ether_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 't', 'h', 'e', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // EVENT_KEYWORD = "event";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn event_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'v', 'e', 'n', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // EXPERIMENTAL_KEYWORD = "experimental";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn experimental_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // EXTERNAL_KEYWORD = "external";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn external_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'e', 'x', 't', 'e', 'r', 'n', 'a', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // FALLBACK_KEYWORD = "fallback";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn fallback_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'a', 'l', 'l', 'b', 'a', 'c', 'k'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // FALSE_KEYWORD = "false";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn false_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'a', 'l', 's', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.11 *)
    // FINNEY_KEYWORD = "finney";

    #[allow(dead_code, non_snake_case)]
    fn finney_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'i', 'n', 'n', 'e', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn finney_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            None
        } else {
            Some(self.finney_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn finney_keyword(&self, stream: &mut Stream) -> bool {
        self.finney_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // FIXED_BYTES_TYPE = "bytes" «FIXED_BYTES_TYPE_SIZE»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'b', 'y', 't', 'e', 's'),
                scan_trie!(
                    stream,
                    ['4' | '5' | '6' | '7' | '8' | '9'],
                    '1' + scan_trie!(
                        stream,
                        EMPTY,
                        ['0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9']
                    ),
                    '2' + scan_trie!(
                        stream,
                        EMPTY,
                        ['0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9']
                    ),
                    '3' + scan_trie!(stream, EMPTY, ['0' | '1' | '2'])
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «FIXED_BYTES_TYPE_SIZE» = "1"
    //                         | "2"
    //                         | "3"
    //                         | "4"
    //                         | "5"
    //                         | "6"
    //                         | "7"
    //                         | "8"
    //                         | "9"
    //                         | "10"
    //                         | "11"
    //                         | "12"
    //                         | "13"
    //                         | "14"
    //                         | "15"
    //                         | "16"
    //                         | "17"
    //                         | "18"
    //                         | "19"
    //                         | "20"
    //                         | "21"
    //                         | "22"
    //                         | "23"
    //                         | "24"
    //                         | "25"
    //                         | "26"
    //                         | "27"
    //                         | "28"
    //                         | "29"
    //                         | "30"
    //                         | "31"
    //                         | "32";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn fixed_bytes_type_size(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            ['4' | '5' | '6' | '7' | '8' | '9'],
            '1' + scan_trie!(
                stream,
                EMPTY,
                ['0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9']
            ),
            '2' + scan_trie!(
                stream,
                EMPTY,
                ['0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9']
            ),
            '3' + scan_trie!(stream, EMPTY, ['0' | '1' | '2'])
        )
    }

    // «FIXED_TYPE_SIZE» = "0"…"9"+ "x" "0"…"9"+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn fixed_type_size(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9'))),
            scan_chars!(stream, 'x'),
            scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
        )
    }

    // FOR_KEYWORD = "for";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn for_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'o', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // FROM_KEYWORD = "from";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn from_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'r', 'o', 'm'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // FUNCTION_KEYWORD = "function";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'f', 'u', 'n', 'c', 't', 'i', 'o', 'n'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // GLOBAL_KEYWORD = "global";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn global_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'g', 'l', 'o', 'b', 'a', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // GREATER_THAN = ">";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // GREATER_THAN_EQUAL = ">=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '=')
    }

    // GREATER_THAN_GREATER_THAN = ">>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // GREATER_THAN_GREATER_THAN_EQUAL = ">>=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '>', '=')
    }

    // GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>', '>'),
            scan_chars!(stream, '=')
        )
    }

    // GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '>', '>', '=')
    }

    // (* v0.6.11 *)
    // GWEI_KEYWORD = "gwei";

    #[allow(dead_code, non_snake_case)]
    fn gwei_keyword__0_6_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'g', 'w', 'e', 'i'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn gwei_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_11 {
            Some(self.gwei_keyword__0_6_11(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn gwei_keyword(&self, stream: &mut Stream) -> bool {
        self.gwei_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «HEX_BYTE_ESCAPE» = "x" «HEX_CHARACTER» «HEX_CHARACTER»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_byte_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'x'),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f'))
        )
    }

    // «HEX_CHARACTER» = "0"…"9" | "a"…"f" | "A"…"F";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_character(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'F')
            || ('a' <= c && c <= 'f'))
    }

    // (* v0.4.11 *)
    // HEX_LITERAL = ("0x" | "0X") «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;

    #[allow(dead_code, non_snake_case)]
    fn hex_literal__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_sequence!(scan_chars!(stream, '0'), scan_trie!(stream, ['X' | 'x'])),
                scan_one_or_more!(
                    stream,
                    scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                        || ('A' <= c && c <= 'F')
                        || ('a' <= c && c <= 'f'))
                ),
                scan_zero_or_more!(
                    stream,
                    scan_sequence!(
                        scan_chars!(stream, '_'),
                        scan_one_or_more!(
                            stream,
                            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                                || ('A' <= c && c <= 'F')
                                || ('a' <= c && c <= 'f'))
                        )
                    )
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.5.0 *)
    // HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;

    #[allow(dead_code, non_snake_case)]
    fn hex_literal__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, '0', 'x'),
                scan_one_or_more!(
                    stream,
                    scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                        || ('A' <= c && c <= 'F')
                        || ('a' <= c && c <= 'f'))
                ),
                scan_zero_or_more!(
                    stream,
                    scan_sequence!(
                        scan_chars!(stream, '_'),
                        scan_one_or_more!(
                            stream,
                            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                                || ('A' <= c && c <= 'F')
                                || ('a' <= c && c <= 'f'))
                        )
                    )
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    pub(crate) fn hex_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.hex_literal__0_5_0(stream)
        } else {
            self.hex_literal__0_4_11(stream)
        }
    }

    // «HEX_STRING_CONTENTS» = «HEX_CHARACTER» «HEX_CHARACTER» ("_"? «HEX_CHARACTER» «HEX_CHARACTER»)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_string_contents(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_optional!(stream, scan_chars!(stream, '_')),
                    scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                        || ('A' <= c && c <= 'F')
                        || ('a' <= c && c <= 'f')),
                    scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                        || ('A' <= c && c <= 'F')
                        || ('a' <= c && c <= 'f'))
                )
            )
        )
    }

    // HEX_STRING_LITERAL = «SINGLE_QUOTED_HEX_STRING_LITERAL»
    //                    | «DOUBLE_QUOTED_HEX_STRING_LITERAL»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_choice!(
                stream,
                self.single_quoted_hex_string_literal(stream),
                self.double_quoted_hex_string_literal(stream)
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // HOURS_KEYWORD = "hours";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hours_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'h', 'o', 'u', 'r', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.11 *)
    // IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION»);

    #[allow(dead_code, non_snake_case)]
    fn identifier__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.raw_identifier(stream),
            scan_choice!(
                stream,
                self.keyword_in_any_version(stream),
                self.keyword_in_some_version(stream),
                scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                        'f' + scan_chars!(stream, 't', 'e', 'r')
                    ),
                    'b' + scan_chars!(stream, 'y', 't', 'e'),
                    'c' + scan_chars!(stream, 'a', 't', 'c', 'h'),
                    'f' + scan_sequence!(
                        scan_chars!(stream, 'i', 'n'),
                        scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'l'),
                            'n' + scan_chars!(stream, 'e', 'y')
                        )
                    ),
                    'h' + scan_chars!(stream, 'e', 'x'),
                    'i' + scan_sequence!(
                        scan_chars!(stream, 'n'),
                        scan_trie!(stream, EMPTY, 'l' + scan_chars!(stream, 'i', 'n', 'e'))
                    ),
                    'm' + scan_chars!(stream, 'a', 't', 'c', 'h'),
                    'n' + scan_chars!(stream, 'u', 'l', 'l'),
                    'o' + scan_chars!(stream, 'f'),
                    'r' + scan_chars!(stream, 'e', 'l', 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'),
                    's' + scan_trie!(
                        stream,
                        't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                        'z' + scan_chars!(stream, 'a', 'b', 'o')
                    ),
                    't' + scan_trie!(
                        stream,
                        'h' + scan_chars!(stream, 'r', 'o', 'w'),
                        'r' + scan_chars!(stream, 'y'),
                        'y' + scan_sequence!(
                            scan_chars!(stream, 'p', 'e'),
                            scan_trie!(stream, EMPTY, 'o' + scan_chars!(stream, 'f'))
                        )
                    ),
                    'v' + scan_chars!(stream, 'a', 'r'),
                    'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
                )
            )
        )
    }

    // (* v0.5.0 *)
    // IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION» | «RESERVED_WORD_IN_SOME_VERSION»);

    #[allow(dead_code, non_snake_case)]
    fn identifier__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.raw_identifier(stream),
            scan_choice!(
                stream,
                self.keyword_in_any_version(stream),
                self.keyword_in_some_version(stream),
                scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                        'f' + scan_chars!(stream, 't', 'e', 'r')
                    ),
                    'b' + scan_chars!(stream, 'y', 't', 'e'),
                    'c' + scan_chars!(stream, 'a', 't', 'c', 'h'),
                    'f' + scan_sequence!(
                        scan_chars!(stream, 'i', 'n'),
                        scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'l'),
                            'n' + scan_chars!(stream, 'e', 'y')
                        )
                    ),
                    'h' + scan_chars!(stream, 'e', 'x'),
                    'i' + scan_sequence!(
                        scan_chars!(stream, 'n'),
                        scan_trie!(stream, EMPTY, 'l' + scan_chars!(stream, 'i', 'n', 'e'))
                    ),
                    'm' + scan_chars!(stream, 'a', 't', 'c', 'h'),
                    'n' + scan_chars!(stream, 'u', 'l', 'l'),
                    'o' + scan_chars!(stream, 'f'),
                    'r' + scan_chars!(stream, 'e', 'l', 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'),
                    's' + scan_trie!(
                        stream,
                        't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                        'z' + scan_chars!(stream, 'a', 'b', 'o')
                    ),
                    't' + scan_trie!(
                        stream,
                        'h' + scan_chars!(stream, 'r', 'o', 'w'),
                        'r' + scan_chars!(stream, 'y'),
                        'y' + scan_sequence!(
                            scan_chars!(stream, 'p', 'e'),
                            scan_trie!(stream, EMPTY, 'o' + scan_chars!(stream, 'f'))
                        )
                    ),
                    'v' + scan_chars!(stream, 'a', 'r'),
                    'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
                ),
                self.reserved_word_in_some_version(stream)
            )
        )
    }

    pub(crate) fn identifier(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.identifier__0_5_0(stream)
        } else {
            self.identifier__0_4_11(stream)
        }
    }

    // «IDENTIFIER_PART» = «IDENTIFIER_START» | "0"…"9";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_part(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    // «IDENTIFIER_START» = "_" | "$" | "a"…"z" | "A"…"Z";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_start(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    // IF_KEYWORD = "if";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn if_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'f'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.6.5 *)
    // IMMUTABLE_KEYWORD = "immutable";

    #[allow(dead_code, non_snake_case)]
    fn immutable_keyword__0_6_5(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'm', 'm', 'u', 't', 'a', 'b', 'l', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn immutable_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_5 {
            Some(self.immutable_keyword__0_6_5(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn immutable_keyword(&self, stream: &mut Stream) -> bool {
        self.immutable_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // IMPORT_KEYWORD = "import";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn import_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'm', 'p', 'o', 'r', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // INDEXED_KEYWORD = "indexed";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn indexed_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'n', 'd', 'e', 'x', 'e', 'd'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «INTEGER_TYPE_SIZE» = "8"
    //                     | "16"
    //                     | "24"
    //                     | "32"
    //                     | "40"
    //                     | "48"
    //                     | "56"
    //                     | "64"
    //                     | "72"
    //                     | "80"
    //                     | "88"
    //                     | "96"
    //                     | "104"
    //                     | "112"
    //                     | "120"
    //                     | "128"
    //                     | "136"
    //                     | "144"
    //                     | "152"
    //                     | "160"
    //                     | "168"
    //                     | "176"
    //                     | "184"
    //                     | "192"
    //                     | "200"
    //                     | "208"
    //                     | "216"
    //                     | "224"
    //                     | "232"
    //                     | "240"
    //                     | "248"
    //                     | "256";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn integer_type_size(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            '1' + scan_trie!(
                stream,
                '0' + scan_chars!(stream, '4'),
                '1' + scan_chars!(stream, '2'),
                '2' + scan_trie!(stream, ['0' | '8']),
                '3' + scan_chars!(stream, '6'),
                '4' + scan_chars!(stream, '4'),
                '5' + scan_chars!(stream, '2'),
                '6' + scan_trie!(stream, EMPTY, ['0' | '8']),
                '7' + scan_chars!(stream, '6'),
                '8' + scan_chars!(stream, '4'),
                '9' + scan_chars!(stream, '2')
            ),
            '2' + scan_trie!(
                stream,
                '0' + scan_trie!(stream, ['0' | '8']),
                '1' + scan_chars!(stream, '6'),
                '2' + scan_chars!(stream, '4'),
                '3' + scan_chars!(stream, '2'),
                '4' + scan_trie!(stream, EMPTY, ['0' | '8']),
                '5' + scan_chars!(stream, '6')
            ),
            '3' + scan_chars!(stream, '2'),
            '4' + scan_trie!(stream, ['0' | '8']),
            '5' + scan_chars!(stream, '6'),
            '6' + scan_chars!(stream, '4'),
            '7' + scan_chars!(stream, '2'),
            '8' + scan_trie!(stream, EMPTY, ['0' | '8']),
            '9' + scan_chars!(stream, '6')
        )
    }

    // INTERFACE_KEYWORD = "interface";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn interface_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'n', 't', 'e', 'r', 'f', 'a', 'c', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // INTERNAL_KEYWORD = "internal";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn internal_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 'n', 't', 'e', 'r', 'n', 'a', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // IS_KEYWORD = "is";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn is_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'i', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «KEYWORD_IN_ANY_VERSION» = FIXED_BYTES_TYPE
    //                          | SIGNED_FIXED_TYPE
    //                          | UNSIGNED_FIXED_TYPE
    //                          | SIGNED_INTEGER_TYPE
    //                          | UNSIGNED_INTEGER_TYPE
    //                          | ADDRESS_KEYWORD
    //                          | ANONYMOUS_KEYWORD
    //                          | AS_KEYWORD
    //                          | ASSEMBLY_KEYWORD
    //                          | BOOL_KEYWORD
    //                          | BREAK_KEYWORD
    //                          | CASE_KEYWORD
    //                          | CONSTANT_KEYWORD
    //                          | CONTINUE_KEYWORD
    //                          | CONTRACT_KEYWORD
    //                          | DAYS_KEYWORD
    //                          | DEFAULT_KEYWORD
    //                          | DELETE_KEYWORD
    //                          | DO_KEYWORD
    //                          | ELSE_KEYWORD
    //                          | ENUM_KEYWORD
    //                          | ETHER_KEYWORD
    //                          | EVENT_KEYWORD
    //                          | EXTERNAL_KEYWORD
    //                          | FALSE_KEYWORD
    //                          | FOR_KEYWORD
    //                          | FUNCTION_KEYWORD
    //                          | HOURS_KEYWORD
    //                          | IF_KEYWORD
    //                          | IMPORT_KEYWORD
    //                          | INDEXED_KEYWORD
    //                          | INTERFACE_KEYWORD
    //                          | INTERNAL_KEYWORD
    //                          | IS_KEYWORD
    //                          | LET_KEYWORD
    //                          | LIBRARY_KEYWORD
    //                          | MAPPING_KEYWORD
    //                          | MEMORY_KEYWORD
    //                          | MINUTES_KEYWORD
    //                          | MODIFIER_KEYWORD
    //                          | NEW_KEYWORD
    //                          | PAYABLE_KEYWORD
    //                          | PRAGMA_KEYWORD
    //                          | PRIVATE_KEYWORD
    //                          | PUBLIC_KEYWORD
    //                          | PURE_KEYWORD
    //                          | RETURN_KEYWORD
    //                          | RETURNS_KEYWORD
    //                          | SECONDS_KEYWORD
    //                          | STORAGE_KEYWORD
    //                          | STRING_KEYWORD
    //                          | STRUCT_KEYWORD
    //                          | SWITCH_KEYWORD
    //                          | TRUE_KEYWORD
    //                          | USING_KEYWORD
    //                          | VIEW_KEYWORD
    //                          | WEEKS_KEYWORD
    //                          | WEI_KEYWORD
    //                          | WHILE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn keyword_in_any_version(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.fixed_bytes_type(stream),
            self.signed_fixed_type(stream),
            self.unsigned_fixed_type(stream),
            self.signed_integer_type(stream),
            self.unsigned_integer_type(stream),
            self.address_keyword(stream),
            self.anonymous_keyword(stream),
            self.as_keyword(stream),
            self.assembly_keyword(stream),
            self.bool_keyword(stream),
            self.break_keyword(stream),
            self.case_keyword(stream),
            self.constant_keyword(stream),
            self.continue_keyword(stream),
            self.contract_keyword(stream),
            self.days_keyword(stream),
            self.default_keyword(stream),
            self.delete_keyword(stream),
            self.do_keyword(stream),
            self.else_keyword(stream),
            self.enum_keyword(stream),
            self.ether_keyword(stream),
            self.event_keyword(stream),
            self.external_keyword(stream),
            self.false_keyword(stream),
            self.for_keyword(stream),
            self.function_keyword(stream),
            self.hours_keyword(stream),
            self.if_keyword(stream),
            self.import_keyword(stream),
            self.indexed_keyword(stream),
            self.interface_keyword(stream),
            self.internal_keyword(stream),
            self.is_keyword(stream),
            self.let_keyword(stream),
            self.library_keyword(stream),
            self.mapping_keyword(stream),
            self.memory_keyword(stream),
            self.minutes_keyword(stream),
            self.modifier_keyword(stream),
            self.new_keyword(stream),
            self.payable_keyword(stream),
            self.pragma_keyword(stream),
            self.private_keyword(stream),
            self.public_keyword(stream),
            self.pure_keyword(stream),
            self.return_keyword(stream),
            self.returns_keyword(stream),
            self.seconds_keyword(stream),
            self.storage_keyword(stream),
            self.string_keyword(stream),
            self.struct_keyword(stream),
            self.switch_keyword(stream),
            self.true_keyword(stream),
            self.using_keyword(stream),
            self.view_keyword(stream),
            self.weeks_keyword(stream),
            self.wei_keyword(stream),
            self.while_keyword(stream)
        )
    }

    // (* v0.4.11 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | FINNEY_KEYWORD
    //                           | SZABO_KEYWORD
    //                           | THROW_KEYWORD
    //                           | VAR_KEYWORD
    //                           | YEARS_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.finney_keyword(stream),
            self.szabo_keyword(stream),
            self.throw_keyword(stream),
            self.var_keyword(stream),
            self.years_keyword(stream)
        )
    }

    // (* v0.5.0 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | FINNEY_KEYWORD
    //                           | SZABO_KEYWORD
    //                           | CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.finney_keyword(stream),
            self.szabo_keyword(stream),
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream)
        )
    }

    // (* v0.5.3 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | FINNEY_KEYWORD
    //                           | SZABO_KEYWORD
    //                           | CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD
    //                           | TYPE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_5_3(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.finney_keyword(stream),
            self.szabo_keyword(stream),
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream),
            self.type_keyword(stream)
        )
    }

    // (* v0.6.0 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | FINNEY_KEYWORD
    //                           | SZABO_KEYWORD
    //                           | CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD
    //                           | TYPE_KEYWORD
    //                           | ABSTRACT_KEYWORD
    //                           | CATCH_KEYWORD
    //                           | FALLBACK_KEYWORD
    //                           | RECEIVE_KEYWORD
    //                           | TRY_KEYWORD
    //                           | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.finney_keyword(stream),
            self.szabo_keyword(stream),
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream),
            self.type_keyword(stream),
            self.abstract_keyword(stream),
            self.catch_keyword(stream),
            self.fallback_keyword(stream),
            self.receive_keyword(stream),
            self.try_keyword(stream),
            self.virtual_keyword(stream)
        )
    }

    // (* v0.6.5 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | FINNEY_KEYWORD
    //                           | SZABO_KEYWORD
    //                           | CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD
    //                           | TYPE_KEYWORD
    //                           | ABSTRACT_KEYWORD
    //                           | CATCH_KEYWORD
    //                           | FALLBACK_KEYWORD
    //                           | RECEIVE_KEYWORD
    //                           | TRY_KEYWORD
    //                           | VIRTUAL_KEYWORD
    //                           | IMMUTABLE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_6_5(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.finney_keyword(stream),
            self.szabo_keyword(stream),
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream),
            self.type_keyword(stream),
            self.abstract_keyword(stream),
            self.catch_keyword(stream),
            self.fallback_keyword(stream),
            self.receive_keyword(stream),
            self.try_keyword(stream),
            self.virtual_keyword(stream),
            self.immutable_keyword(stream)
        )
    }

    // (* v0.7.0 *)
    // «KEYWORD_IN_SOME_VERSION» = BYTE_KEYWORD
    //                           | CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD
    //                           | TYPE_KEYWORD
    //                           | ABSTRACT_KEYWORD
    //                           | CATCH_KEYWORD
    //                           | FALLBACK_KEYWORD
    //                           | RECEIVE_KEYWORD
    //                           | TRY_KEYWORD
    //                           | VIRTUAL_KEYWORD
    //                           | IMMUTABLE_KEYWORD
    //                           | GWEI_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.byte_keyword(stream),
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream),
            self.type_keyword(stream),
            self.abstract_keyword(stream),
            self.catch_keyword(stream),
            self.fallback_keyword(stream),
            self.receive_keyword(stream),
            self.try_keyword(stream),
            self.virtual_keyword(stream),
            self.immutable_keyword(stream),
            self.gwei_keyword(stream)
        )
    }

    // (* v0.8.0 *)
    // «KEYWORD_IN_SOME_VERSION» = CALLDATA_KEYWORD
    //                           | CONSTRUCTOR_KEYWORD
    //                           | EMIT_KEYWORD
    //                           | OVERRIDE_KEYWORD
    //                           | TYPE_KEYWORD
    //                           | ABSTRACT_KEYWORD
    //                           | CATCH_KEYWORD
    //                           | FALLBACK_KEYWORD
    //                           | RECEIVE_KEYWORD
    //                           | TRY_KEYWORD
    //                           | VIRTUAL_KEYWORD
    //                           | IMMUTABLE_KEYWORD
    //                           | GWEI_KEYWORD
    //                           | UNCHECKED_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn keyword_in_some_version__0_8_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.calldata_keyword(stream),
            self.constructor_keyword(stream),
            self.emit_keyword(stream),
            self.override_keyword(stream),
            self.type_keyword(stream),
            self.abstract_keyword(stream),
            self.catch_keyword(stream),
            self.fallback_keyword(stream),
            self.receive_keyword(stream),
            self.try_keyword(stream),
            self.virtual_keyword(stream),
            self.immutable_keyword(stream),
            self.gwei_keyword(stream),
            self.unchecked_keyword(stream)
        )
    }

    pub(crate) fn keyword_in_some_version(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.keyword_in_some_version__0_8_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.keyword_in_some_version__0_7_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_5 {
            self.keyword_in_some_version__0_6_5(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.keyword_in_some_version__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_3 {
            self.keyword_in_some_version__0_5_3(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.keyword_in_some_version__0_5_0(stream)
        } else {
            self.keyword_in_some_version__0_4_11(stream)
        }
    }

    // (* v0.6.0 *)
    // LEAVE_KEYWORD = "leave";

    #[allow(dead_code, non_snake_case)]
    fn leave_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'l', 'e', 'a', 'v', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn leave_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.leave_keyword__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn leave_keyword(&self, stream: &mut Stream) -> bool {
        self.leave_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // LESS_THAN = "<";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<'),
            scan_predicate!(stream, |c| ('<' <= c && c <= '='))
        )
    }

    // LESS_THAN_EQUAL = "<=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '=')
    }

    // LESS_THAN_LESS_THAN = "<<";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_less_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<', '<'),
            scan_chars!(stream, '=')
        )
    }

    // LESS_THAN_LESS_THAN_EQUAL = "<<=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_less_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '<', '=')
    }

    // LET_KEYWORD = "let";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn let_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'l', 'e', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // LIBRARY_KEYWORD = "library";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn library_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'l', 'i', 'b', 'r', 'a', 'r', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // MAPPING_KEYWORD = "mapping";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn mapping_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'm', 'a', 'p', 'p', 'i', 'n', 'g'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // MEMORY_KEYWORD = "memory";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn memory_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'm', 'e', 'm', 'o', 'r', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // MINUS = "-";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '-'),
            scan_predicate!(stream, |c| c == '-' || ('=' <= c && c <= '>'))
        )
    }

    // MINUS_EQUAL = "-=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '=')
    }

    // MINUS_GREATER_THAN = "->";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_greater_than(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '>')
    }

    // MINUS_MINUS = "--";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_minus(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '-')
    }

    // MINUTES_KEYWORD = "minutes";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minutes_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'm', 'i', 'n', 'u', 't', 'e', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // MODIFIER_KEYWORD = "modifier";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn modifier_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'm', 'o', 'd', 'i', 'f', 'i', 'e', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // MULTILINE_COMMENT = "/" "*" (!"*" | "*")* "*" "/";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn multiline_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/'),
            scan_chars!(stream, '*'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    scan_predicate!(stream, |c| c != '*'),
                    scan_not_followed_by!(
                        stream,
                        scan_chars!(stream, '*'),
                        scan_chars!(stream, '/')
                    )
                )
            ),
            scan_chars!(stream, '*'),
            scan_chars!(stream, '/')
        )
    }

    // NEW_KEYWORD = "new";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn new_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'n', 'e', 'w'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // OPEN_BRACE = "{";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_brace(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '{')
    }

    // OPEN_BRACKET = "[";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_bracket(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '[')
    }

    // OPEN_PAREN = "(";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_paren(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '(')
    }

    // OVERRIDE_KEYWORD = "override";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn override_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'o', 'v', 'e', 'r', 'r', 'i', 'd', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // PAYABLE_KEYWORD = "payable";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn payable_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'p', 'a', 'y', 'a', 'b', 'l', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // PERCENT = "%";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn percent(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '%'), scan_chars!(stream, '='))
    }

    // PERCENT_EQUAL = "%=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn percent_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '%', '=')
    }

    // PERIOD = ".";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn period(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '.')
    }

    // PLUS = "+";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '+'),
            scan_predicate!(stream, |c| c == '+' || c == '=')
        )
    }

    // PLUS_EQUAL = "+=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '=')
    }

    // PLUS_PLUS = "++";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus_plus(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '+')
    }

    // PRAGMA_KEYWORD = "pragma";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn pragma_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'p', 'r', 'a', 'g', 'm', 'a'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // PRIVATE_KEYWORD = "private";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn private_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'p', 'r', 'i', 'v', 'a', 't', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // PUBLIC_KEYWORD = "public";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn public_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'p', 'u', 'b', 'l', 'i', 'c'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // PURE_KEYWORD = "pure";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn pure_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'p', 'u', 'r', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // QUESTION_MARK = "?";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn question_mark(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '?')
    }

    // «RAW_IDENTIFIER» = «IDENTIFIER_START» «IDENTIFIER_PART»*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn raw_identifier(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_predicate!(stream, |c| c == '$'
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z')),
            scan_zero_or_more!(
                stream,
                scan_predicate!(stream, |c| c == '$'
                    || ('0' <= c && c <= '9')
                    || ('A' <= c && c <= 'Z')
                    || c == '_'
                    || ('a' <= c && c <= 'z'))
            )
        )
    }

    // RECEIVE_KEYWORD = "receive";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn receive_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'r', 'e', 'c', 'e', 'i', 'v', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // «RESERVED_WORD_IN_ANY_VERSION» = "abstract"
    //                                | "after"
    //                                | "byte"
    //                                | "catch"
    //                                | "final"
    //                                | "finney"
    //                                | "hex"
    //                                | "in"
    //                                | "inline"
    //                                | "match"
    //                                | "null"
    //                                | "of"
    //                                | "relocatable"
    //                                | "static"
    //                                | "szabo"
    //                                | "throw"
    //                                | "try"
    //                                | "type"
    //                                | "typeof"
    //                                | "var"
    //                                | "years";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn reserved_word_in_any_version(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                'f' + scan_chars!(stream, 't', 'e', 'r')
            ),
            'b' + scan_chars!(stream, 'y', 't', 'e'),
            'c' + scan_chars!(stream, 'a', 't', 'c', 'h'),
            'f' + scan_sequence!(
                scan_chars!(stream, 'i', 'n'),
                scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 'l'),
                    'n' + scan_chars!(stream, 'e', 'y')
                )
            ),
            'h' + scan_chars!(stream, 'e', 'x'),
            'i' + scan_sequence!(
                scan_chars!(stream, 'n'),
                scan_trie!(stream, EMPTY, 'l' + scan_chars!(stream, 'i', 'n', 'e'))
            ),
            'm' + scan_chars!(stream, 'a', 't', 'c', 'h'),
            'n' + scan_chars!(stream, 'u', 'l', 'l'),
            'o' + scan_chars!(stream, 'f'),
            'r' + scan_chars!(stream, 'e', 'l', 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'),
            's' + scan_trie!(
                stream,
                't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                'z' + scan_chars!(stream, 'a', 'b', 'o')
            ),
            't' + scan_trie!(
                stream,
                'h' + scan_chars!(stream, 'r', 'o', 'w'),
                'r' + scan_chars!(stream, 'y'),
                'y' + scan_sequence!(
                    scan_chars!(stream, 'p', 'e'),
                    scan_trie!(stream, EMPTY, 'o' + scan_chars!(stream, 'f'))
                )
            ),
            'v' + scan_chars!(stream, 'a', 'r'),
            'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
        )
    }

    // (* v0.5.0 *)
    // «RESERVED_WORD_IN_SOME_VERSION» = "alias"
    //                                 | "apply"
    //                                 | "auto"
    //                                 | "copyof"
    //                                 | "define"
    //                                 | "implements"
    //                                 | "macro"
    //                                 | "mutable"
    //                                 | "partial"
    //                                 | "promise"
    //                                 | "reference"
    //                                 | "sealed"
    //                                 | "sizeof"
    //                                 | "supports"
    //                                 | "typedef";

    #[allow(dead_code, non_snake_case)]
    fn reserved_word_in_some_version__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'l' + scan_chars!(stream, 'i', 'a', 's'),
                'p' + scan_chars!(stream, 'p', 'l', 'y'),
                'u' + scan_chars!(stream, 't', 'o')
            ),
            'c' + scan_chars!(stream, 'o', 'p', 'y', 'o', 'f'),
            'd' + scan_chars!(stream, 'e', 'f', 'i', 'n', 'e'),
            'i' + scan_chars!(stream, 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 's'),
            'm' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'c', 'r', 'o'),
                'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
            ),
            'p' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'r', 't', 'i', 'a', 'l'),
                'r' + scan_chars!(stream, 'o', 'm', 'i', 's', 'e')
            ),
            'r' + scan_chars!(stream, 'e', 'f', 'e', 'r', 'e', 'n', 'c', 'e'),
            's' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'a', 'l', 'e', 'd'),
                'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's')
            ),
            't' + scan_chars!(stream, 'y', 'p', 'e', 'd', 'e', 'f')
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn reserved_word_in_some_version__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            Some(self.reserved_word_in_some_version__0_5_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn reserved_word_in_some_version(&self, stream: &mut Stream) -> bool {
        self.reserved_word_in_some_version__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // RETURN_KEYWORD = "return";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn return_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // RETURNS_KEYWORD = "returns";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn returns_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // REVERT_KEYWORD = "revert";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn revert_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'r', 'e', 'v', 'e', 'r', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // SECONDS_KEYWORD = "seconds";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn seconds_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 'e', 'c', 'o', 'n', 'd', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // SEMICOLON = ";";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn semicolon(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ';')
    }

    // SIGNED_FIXED_TYPE = "fixed" «FIXED_TYPE_SIZE»?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn signed_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'f', 'i', 'x', 'e', 'd'),
                scan_optional!(stream, self.fixed_type_size(stream))
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // SIGNED_INTEGER_TYPE = "int" «INTEGER_TYPE_SIZE»?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn signed_integer_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'i', 'n', 't'),
                scan_optional!(
                    stream,
                    scan_trie!(
                        stream,
                        '1' + scan_trie!(
                            stream,
                            '0' + scan_chars!(stream, '4'),
                            '1' + scan_chars!(stream, '2'),
                            '2' + scan_trie!(stream, ['0' | '8']),
                            '3' + scan_chars!(stream, '6'),
                            '4' + scan_chars!(stream, '4'),
                            '5' + scan_chars!(stream, '2'),
                            '6' + scan_trie!(stream, EMPTY, ['0' | '8']),
                            '7' + scan_chars!(stream, '6'),
                            '8' + scan_chars!(stream, '4'),
                            '9' + scan_chars!(stream, '2')
                        ),
                        '2' + scan_trie!(
                            stream,
                            '0' + scan_trie!(stream, ['0' | '8']),
                            '1' + scan_chars!(stream, '6'),
                            '2' + scan_chars!(stream, '4'),
                            '3' + scan_chars!(stream, '2'),
                            '4' + scan_trie!(stream, EMPTY, ['0' | '8']),
                            '5' + scan_chars!(stream, '6')
                        ),
                        '3' + scan_chars!(stream, '2'),
                        '4' + scan_trie!(stream, ['0' | '8']),
                        '5' + scan_chars!(stream, '6'),
                        '6' + scan_chars!(stream, '4'),
                        '7' + scan_chars!(stream, '2'),
                        '8' + scan_trie!(stream, EMPTY, ['0' | '8']),
                        '9' + scan_chars!(stream, '6')
                    )
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // SINGLE_LINE_COMMENT = "//" (!("\r" | "\n"))*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn single_line_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '/'),
            scan_zero_or_more!(stream, scan_predicate!(stream, |c| c != '\n' && c != '\r'))
        )
    }

    // «SINGLE_QUOTED_ASCII_STRING_LITERAL» = "'" («ESCAPE_SEQUENCE» | (" "…"~" - ("'" | "\\")))* "'";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn single_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    scan_predicate!(stream, |c| (' ' <= c && c <= '&')
                        || ('(' <= c && c <= '[')
                        || (']' <= c && c <= '~'))
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    // «SINGLE_QUOTED_HEX_STRING_LITERAL» = "hex" "'" «HEX_STRING_CONTENTS»? "'";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn single_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x'),
            scan_chars!(stream, '\''),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '\'')
        )
    }

    // (* v0.7.0 *)
    // «SINGLE_QUOTED_UNICODE_STRING_LITERAL» = "unicode'" («ESCAPE_SEQUENCE» | !("'" | "\\" | "\n" | "\r"))* "'";

    #[allow(dead_code, non_snake_case)]
    fn single_quoted_unicode_string_literal__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    scan_predicate!(stream, |c| c != '\n' && c != '\r' && c != '\'' && c != '\\')
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn single_quoted_unicode_string_literal__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            Some(self.single_quoted_unicode_string_literal__0_7_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.single_quoted_unicode_string_literal__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // SLASH = "/";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn slash(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '/'), scan_chars!(stream, '='))
    }

    // SLASH_EQUAL = "/=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn slash_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '/', '=')
    }

    // SOLIDITY_KEYWORD = "solidity";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn solidity_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 'o', 'l', 'i', 'd', 'i', 't', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // STORAGE_KEYWORD = "storage";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn storage_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 't', 'o', 'r', 'a', 'g', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // STRING_KEYWORD = "string";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn string_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 't', 'r', 'i', 'n', 'g'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // STRUCT_KEYWORD = "struct";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn struct_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 't', 'r', 'u', 'c', 't'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // SWITCH_KEYWORD = "switch";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn switch_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 'w', 'i', 't', 'c', 'h'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.11 *)
    // SZABO_KEYWORD = "szabo";

    #[allow(dead_code, non_snake_case)]
    fn szabo_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 's', 'z', 'a', 'b', 'o'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn szabo_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            None
        } else {
            Some(self.szabo_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn szabo_keyword(&self, stream: &mut Stream) -> bool {
        self.szabo_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // THROW_KEYWORD = "throw";

    #[allow(dead_code, non_snake_case)]
    fn throw_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 't', 'h', 'r', 'o', 'w'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn throw_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.throw_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn throw_keyword(&self, stream: &mut Stream) -> bool {
        self.throw_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TILDE = "~";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tilde(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '~')
    }

    // TRUE_KEYWORD = "true";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn true_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 't', 'r', 'u', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.6.0 *)
    // TRY_KEYWORD = "try";

    #[allow(dead_code, non_snake_case)]
    fn try_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 't', 'r', 'y'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn try_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.try_keyword__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn try_keyword(&self, stream: &mut Stream) -> bool {
        self.try_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.5.3 *)
    // TYPE_KEYWORD = "type";

    #[allow(dead_code, non_snake_case)]
    fn type_keyword__0_5_3(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 't', 'y', 'p', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn type_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            Some(self.type_keyword__0_5_3(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn type_keyword(&self, stream: &mut Stream) -> bool {
        self.type_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.0 *)
    // UNCHECKED_KEYWORD = "unchecked";

    #[allow(dead_code, non_snake_case)]
    fn unchecked_keyword__0_8_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'u', 'n', 'c', 'h', 'e', 'c', 'k', 'e', 'd'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn unchecked_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            Some(self.unchecked_keyword__0_8_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn unchecked_keyword(&self, stream: &mut Stream) -> bool {
        self.unchecked_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «UNICODE_ESCAPE» = "u" «HEX_CHARACTER» «HEX_CHARACTER» «HEX_CHARACTER» «HEX_CHARACTER»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unicode_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f')),
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f'))
        )
    }

    // (* v0.7.0 *)
    // UNICODE_STRING_LITERAL = «SINGLE_QUOTED_UNICODE_STRING_LITERAL»
    //                        | «DOUBLE_QUOTED_UNICODE_STRING_LITERAL»;

    #[allow(dead_code, non_snake_case)]
    fn unicode_string_literal__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_choice!(
                stream,
                self.single_quoted_unicode_string_literal(stream),
                self.double_quoted_unicode_string_literal(stream)
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn unicode_string_literal__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            Some(self.unicode_string_literal__0_7_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.unicode_string_literal__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // UNSIGNED_FIXED_TYPE = "ufixed" «FIXED_TYPE_SIZE»?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'u', 'f', 'i', 'x', 'e', 'd'),
                scan_optional!(stream, self.fixed_type_size(stream))
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // UNSIGNED_INTEGER_TYPE = "uint" «INTEGER_TYPE_SIZE»?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'u', 'i', 'n', 't'),
                scan_optional!(
                    stream,
                    scan_trie!(
                        stream,
                        '1' + scan_trie!(
                            stream,
                            '0' + scan_chars!(stream, '4'),
                            '1' + scan_chars!(stream, '2'),
                            '2' + scan_trie!(stream, ['0' | '8']),
                            '3' + scan_chars!(stream, '6'),
                            '4' + scan_chars!(stream, '4'),
                            '5' + scan_chars!(stream, '2'),
                            '6' + scan_trie!(stream, EMPTY, ['0' | '8']),
                            '7' + scan_chars!(stream, '6'),
                            '8' + scan_chars!(stream, '4'),
                            '9' + scan_chars!(stream, '2')
                        ),
                        '2' + scan_trie!(
                            stream,
                            '0' + scan_trie!(stream, ['0' | '8']),
                            '1' + scan_chars!(stream, '6'),
                            '2' + scan_chars!(stream, '4'),
                            '3' + scan_chars!(stream, '2'),
                            '4' + scan_trie!(stream, EMPTY, ['0' | '8']),
                            '5' + scan_chars!(stream, '6')
                        ),
                        '3' + scan_chars!(stream, '2'),
                        '4' + scan_trie!(stream, ['0' | '8']),
                        '5' + scan_chars!(stream, '6'),
                        '6' + scan_chars!(stream, '4'),
                        '7' + scan_chars!(stream, '2'),
                        '8' + scan_trie!(stream, EMPTY, ['0' | '8']),
                        '9' + scan_chars!(stream, '6')
                    )
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // USING_KEYWORD = "using";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn using_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'u', 's', 'i', 'n', 'g'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.4.11 *)
    // VAR_KEYWORD = "var";

    #[allow(dead_code, non_snake_case)]
    fn var_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'v', 'a', 'r'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn var_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.var_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn var_keyword(&self, stream: &mut Stream) -> bool {
        self.var_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // VERSION_PRAGMA_VALUE = ("0"…"9" | "x" | "X" | "*")+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_value(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_predicate!(stream, |c| c == '*'
                || ('0' <= c && c <= '9')
                || c == 'X'
                || c == 'x')
        )
    }

    // VIEW_KEYWORD = "view";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn view_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'v', 'i', 'e', 'w'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // (* v0.6.0 *)
    // VIRTUAL_KEYWORD = "virtual";

    #[allow(dead_code, non_snake_case)]
    fn virtual_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'v', 'i', 'r', 't', 'u', 'a', 'l'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn virtual_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.virtual_keyword__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn virtual_keyword(&self, stream: &mut Stream) -> bool {
        self.virtual_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // WEEKS_KEYWORD = "weeks";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn weeks_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'w', 'e', 'e', 'k', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // WEI_KEYWORD = "wei";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn wei_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'w', 'e', 'i'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // WHILE_KEYWORD = "while";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn while_keyword(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'w', 'h', 'i', 'l', 'e'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // WHITESPACE = (" " | "\t")+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn whitespace(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\t' || c == ' '))
    }

    // (* v0.4.11 *)
    // YEARS_KEYWORD = "years";

    #[allow(dead_code, non_snake_case)]
    fn years_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, 'y', 'e', 'a', 'r', 's'),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    #[allow(non_snake_case)]
    pub(crate) fn years_keyword__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.years_keyword__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn years_keyword(&self, stream: &mut Stream) -> bool {
        self.years_keyword__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // YUL_DECIMAL_LITERAL = "0" | ("1"…"9" "0"…"9"*);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_choice!(
                stream,
                scan_chars!(stream, '0'),
                scan_sequence!(
                    scan_predicate!(stream, |c| ('1' <= c && c <= '9')),
                    scan_zero_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_hex_literal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, '0', 'x'),
                scan_one_or_more!(
                    stream,
                    scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                        || ('A' <= c && c <= 'F')
                        || ('a' <= c && c <= 'f'))
                )
            ),
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        )
    }

    // YUL_IDENTIFIER = «RAW_IDENTIFIER» - («YUL_KEYWORD» | «YUL_RESERVED_WORD»);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_identifier(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.raw_identifier(stream),
            scan_choice!(
                stream,
                self.yul_keyword(stream),
                scan_chars!(stream, 'h', 'e', 'x')
            )
        )
    }

    // (* v0.4.11 *)
    // «YUL_KEYWORD» = BREAK_KEYWORD
    //               | CASE_KEYWORD
    //               | CONTINUE_KEYWORD
    //               | DEFAULT_KEYWORD
    //               | FALSE_KEYWORD
    //               | FOR_KEYWORD
    //               | FUNCTION_KEYWORD
    //               | IF_KEYWORD
    //               | LET_KEYWORD
    //               | SWITCH_KEYWORD
    //               | TRUE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn yul_keyword__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.break_keyword(stream),
            self.case_keyword(stream),
            self.continue_keyword(stream),
            self.default_keyword(stream),
            self.false_keyword(stream),
            self.for_keyword(stream),
            self.function_keyword(stream),
            self.if_keyword(stream),
            self.let_keyword(stream),
            self.switch_keyword(stream),
            self.true_keyword(stream)
        )
    }

    // (* v0.6.0 *)
    // «YUL_KEYWORD» = BREAK_KEYWORD
    //               | CASE_KEYWORD
    //               | CONTINUE_KEYWORD
    //               | DEFAULT_KEYWORD
    //               | FALSE_KEYWORD
    //               | FOR_KEYWORD
    //               | FUNCTION_KEYWORD
    //               | IF_KEYWORD
    //               | LEAVE_KEYWORD
    //               | LET_KEYWORD
    //               | SWITCH_KEYWORD
    //               | TRUE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn yul_keyword__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.break_keyword(stream),
            self.case_keyword(stream),
            self.continue_keyword(stream),
            self.default_keyword(stream),
            self.false_keyword(stream),
            self.for_keyword(stream),
            self.function_keyword(stream),
            self.if_keyword(stream),
            self.leave_keyword(stream),
            self.let_keyword(stream),
            self.switch_keyword(stream),
            self.true_keyword(stream)
        )
    }

    pub(crate) fn yul_keyword(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.yul_keyword__0_6_0(stream)
        } else {
            self.yul_keyword__0_4_11(stream)
        }
    }

    // «YUL_RESERVED_WORD» = "hex";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_reserved_word(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, 'h', 'e', 'x')
    }
}
