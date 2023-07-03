// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::language::Language;
use super::stream::*;

impl Language {
    // «AbicoderKeyword» = "abicoder";

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
    // «AbstractKeyword» = "abstract";

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

    // «AddressKeyword» = "address";

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

    // «Ampersand» = "&";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '&'),
            scan_predicate!(stream, |c| c == '&' || c == '=')
        )
    }

    // «AmpersandAmpersand» = "&&";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand_ampersand(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '&')
    }

    // «AmpersandEqual» = "&=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ampersand_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '=')
    }

    // «AnonymousKeyword» = "anonymous";

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

    // «AsKeyword» = "as";

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

    // «AsciiEscape» = "n"
    //               | "r"
    //               | "t"
    //               | "'"
    //               | '"'
    //               | "\\"
    //               | "\n"
    //               | "\r";

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

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_ascii_string_literal(stream),
            self.double_quoted_ascii_string_literal(stream)
        )
    }

    // «AssemblyKeyword» = "assembly";

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

    // «Asterisk» = "*";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '*'),
            scan_predicate!(stream, |c| c == '*' || c == '=')
        )
    }

    // «AsteriskAsterisk» = "**";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk_asterisk(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '*')
    }

    // «AsteriskEqual» = "*=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '=')
    }

    // «Bang» = "!";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bang(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '!'), scan_chars!(stream, '='))
    }

    // «BangEqual» = "!=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bang_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '!', '=')
    }

    // «Bar» = "|";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '|'),
            scan_predicate!(stream, |c| c == '=' || c == '|')
        )
    }

    // «BarBar» = "||";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar_bar(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '|')
    }

    // «BarEqual» = "|=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bar_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '=')
    }

    // «BoolKeyword» = "bool";

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

    // «BreakKeyword» = "break";

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
    // «ByteType» = "byte";

    #[allow(dead_code, non_snake_case)]
    fn byte_type__0_4_11(&self, stream: &mut Stream) -> bool {
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
    pub(crate) fn byte_type__sparse_dispatch(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            None
        } else {
            Some(self.byte_type__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn byte_type(&self, stream: &mut Stream) -> bool {
        self.byte_type__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.5.0 *)
    // «CalldataKeyword» = "calldata";

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

    // «Caret» = "^";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn caret(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '^'), scan_chars!(stream, '='))
    }

    // «CaretEqual» = "^=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn caret_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '^', '=')
    }

    // «CaseKeyword» = "case";

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
    // «CatchKeyword» = "catch";

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

    // «CloseBrace» = "}";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_brace(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '}')
    }

    // «CloseBracket» = "]";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_bracket(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ']')
    }

    // «CloseParen» = ")";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn close_paren(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ')')
    }

    // «Colon» = ":";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn colon(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, ':'), scan_chars!(stream, '='))
    }

    // «ColonEqual» = ":=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn colon_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ':', '=')
    }

    // «Comma» = ",";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn comma(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ',')
    }

    // «ConstantKeyword» = "constant";

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
    // «ConstructorKeyword» = "constructor";

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

    // «ContinueKeyword» = "continue";

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

    // «ContractKeyword» = "contract";

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

    // «DaysKeyword» = "days";

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

    // «DecimalExponent» = ("e" | "E") "-"? «DecimalNumber»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn decimal_exponent(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_predicate!(stream, |c| c == 'E' || c == 'e'),
            scan_optional!(stream, scan_chars!(stream, '-')),
            self.decimal_number(stream)
        )
    }

    // (* v0.4.11 *)
    // «DecimalLiteral» = ((«DecimalNumber» ("." «DecimalNumber»?)?) | ("." «DecimalNumber»)) «DecimalExponent»?;

    #[allow(dead_code, non_snake_case)]
    fn decimal_literal__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_sequence!(
                    self.decimal_number(stream),
                    scan_optional!(
                        stream,
                        scan_sequence!(
                            scan_chars!(stream, '.'),
                            scan_optional!(stream, self.decimal_number(stream))
                        )
                    )
                ),
                scan_sequence!(scan_chars!(stream, '.'), self.decimal_number(stream))
            ),
            scan_optional!(stream, self.decimal_exponent(stream))
        )
    }

    // (* v0.5.0 *)
    // «DecimalLiteral» = ((«DecimalNumber» ("." «DecimalNumber»)?) | ("." «DecimalNumber»)) «DecimalExponent»?;

    #[allow(dead_code, non_snake_case)]
    fn decimal_literal__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_sequence!(
                    self.decimal_number(stream),
                    scan_optional!(
                        stream,
                        scan_sequence!(scan_chars!(stream, '.'), self.decimal_number(stream))
                    )
                ),
                scan_sequence!(scan_chars!(stream, '.'), self.decimal_number(stream))
            ),
            scan_optional!(stream, self.decimal_exponent(stream))
        )
    }

    pub(crate) fn decimal_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.decimal_literal__0_5_0(stream)
        } else {
            self.decimal_literal__0_4_11(stream)
        }
    }

    // «DecimalNumber» = ("0"…"9")+ ("_" ("0"…"9")+)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn decimal_number(&self, stream: &mut Stream) -> bool {
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

    // «DefaultKeyword» = "default";

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

    // «DeleteKeyword» = "delete";

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

    // «DoKeyword» = "do";

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

    // «DoubleQuotedAsciiStringLiteral» = '"' («EscapeSequence» | ((" "…"~") - ('"' | "\\")))* '"';

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

    // (* v0.7.0 *)
    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' («EscapeSequence» | !('"' | "\\" | "\n" | "\r"))* '"';

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

    // «ElseKeyword» = "else";

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
    // «EmitKeyword» = "emit";

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

    // «EndOfLine» = "\r"? "\n";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn end_of_line(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_optional!(stream, scan_chars!(stream, '\r')),
            scan_chars!(stream, '\n')
        )
    }

    // «EnumKeyword» = "enum";

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

    // «Equal» = "=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '='),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // «EqualEqual» = "==";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '=')
    }

    // «EqualGreaterThan» = "=>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equal_greater_than(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '>')
    }

    // «ErrorKeyword» = "error";

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

    // «EscapeSequence» = "\\" («AsciiEscape» | «HexByteEscape» | «UnicodeEscape»);

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

    // «EtherKeyword» = "ether";

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

    // «EventKeyword» = "event";

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

    // «Evmasm» = '"evmasm"';

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn evmasm(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '"', 'e', 'v', 'm', 'a', 's', 'm', '"')
    }

    // «ExperimentalKeyword» = "experimental";

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

    // «ExternalKeyword» = "external";

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

    // «FallbackKeyword» = "fallback";

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

    // «FalseKeyword» = "false";

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
    // «FinneyKeyword» = "finney";

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

    // «FixedBytesType» = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32");

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

    // «ForKeyword» = "for";

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

    // «FromKeyword» = "from";

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

    // «FunctionKeyword» = "function";

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

    // «GlobalKeyword» = "global";

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

    // «GreaterThan» = ">";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // «GreaterThanEqual» = ">=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '=')
    }

    // «GreaterThanGreaterThan» = ">>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    // «GreaterThanGreaterThanEqual» = ">>=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '>', '=')
    }

    // «GreaterThanGreaterThanGreaterThan» = ">>>";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>', '>'),
            scan_chars!(stream, '=')
        )
    }

    // «GreaterThanGreaterThanGreaterThanEqual» = ">>>=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn greater_than_greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '>', '>', '=')
    }

    // (* v0.6.11 *)
    // «GweiKeyword» = "gwei";

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

    // «HexByteEscape» = "x" «HexCharacter» «HexCharacter»;

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

    // «HexCharacter» = ("0"…"9") | ("a"…"f") | ("A"…"F");

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_character(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'F')
            || ('a' <= c && c <= 'f'))
    }

    // (* v0.4.11 *)
    // «HexLiteral» = "0" ("x" | "X") «HexCharacter»+ ("_" «HexCharacter»+)*;

    #[allow(dead_code, non_snake_case)]
    fn hex_literal__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '0'),
            scan_predicate!(stream, |c| c == 'X' || c == 'x'),
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
        )
    }

    // (* v0.5.0 *)
    // «HexLiteral» = "0x" «HexCharacter»+ ("_" «HexCharacter»+)*;

    #[allow(dead_code, non_snake_case)]
    fn hex_literal__0_5_0(&self, stream: &mut Stream) -> bool {
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
        )
    }

    pub(crate) fn hex_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.hex_literal__0_5_0(stream)
        } else {
            self.hex_literal__0_4_11(stream)
        }
    }

    // «HexStringLiteral» = "hex" (('"' «PossiblySeparatedPairsOfHexDigits»? '"') | ("'" «PossiblySeparatedPairsOfHexDigits»? "'"));

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x'),
            scan_choice!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '"'),
                    scan_optional!(stream, self.possibly_separated_pairs_of_hex_digits(stream)),
                    scan_chars!(stream, '"')
                ),
                scan_sequence!(
                    scan_chars!(stream, '\''),
                    scan_optional!(stream, self.possibly_separated_pairs_of_hex_digits(stream)),
                    scan_chars!(stream, '\'')
                )
            )
        )
    }

    // «HoursKeyword» = "hours";

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

    // «Identifier» = «RawIdentifier» - («NotAnIdentifierInAnyVersion» | «NotAnIdentifierInSomeVersions» | «FixedBytesType» | «SignedFixedType» | «UnsignedFixedType» | «SignedIntegerType» | «UnsignedIntegerType»);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.raw_identifier(stream),
            scan_choice!(
                stream,
                scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                        'd' + scan_chars!(stream, 'd', 'r', 'e', 's', 's'),
                        'f' + scan_chars!(stream, 't', 'e', 'r'),
                        'n' + scan_chars!(stream, 'o', 'n', 'y', 'm', 'o', 'u', 's'),
                        's' + scan_trie!(
                            stream,
                            EMPTY,
                            's' + scan_chars!(stream, 'e', 'm', 'b', 'l', 'y')
                        )
                    ),
                    'b' + scan_trie!(
                        stream,
                        'o' + scan_chars!(stream, 'o', 'l'),
                        'r' + scan_chars!(stream, 'e', 'a', 'k'),
                        'y' + scan_chars!(stream, 't', 'e')
                    ),
                    'c' + scan_trie!(
                        stream,
                        'a' + scan_trie!(
                            stream,
                            's' + scan_chars!(stream, 'e'),
                            't' + scan_chars!(stream, 'c', 'h')
                        ),
                        'o' + scan_sequence!(
                            scan_chars!(stream, 'n'),
                            scan_trie!(
                                stream,
                                's' + scan_chars!(stream, 't', 'a', 'n', 't'),
                                't' + scan_trie!(
                                    stream,
                                    'i' + scan_chars!(stream, 'n', 'u', 'e'),
                                    'r' + scan_chars!(stream, 'a', 'c', 't')
                                )
                            )
                        )
                    ),
                    'd' + scan_trie!(
                        stream,
                        ['o'],
                        'a' + scan_chars!(stream, 'y', 's'),
                        'e' + scan_trie!(
                            stream,
                            'f' + scan_chars!(stream, 'a', 'u', 'l', 't'),
                            'l' + scan_chars!(stream, 'e', 't', 'e')
                        )
                    ),
                    'e' + scan_trie!(
                        stream,
                        'l' + scan_chars!(stream, 's', 'e'),
                        'n' + scan_chars!(stream, 'u', 'm'),
                        't' + scan_chars!(stream, 'h', 'e', 'r'),
                        'v' + scan_chars!(stream, 'e', 'n', 't'),
                        'x' + scan_chars!(stream, 't', 'e', 'r', 'n', 'a', 'l')
                    ),
                    'f' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'l', 's', 'e'),
                        'i' + scan_chars!(stream, 'n', 'a', 'l'),
                        'o' + scan_chars!(stream, 'r'),
                        'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                    ),
                    'h' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 'x'),
                        'o' + scan_chars!(stream, 'u', 'r', 's')
                    ),
                    'i' + scan_trie!(
                        stream,
                        ['f' | 's'],
                        'm' + scan_chars!(stream, 'p', 'o', 'r', 't'),
                        'n' + scan_trie!(
                            stream,
                            EMPTY,
                            'd' + scan_chars!(stream, 'e', 'x', 'e', 'd'),
                            'l' + scan_chars!(stream, 'i', 'n', 'e'),
                            't' + scan_sequence!(
                                scan_chars!(stream, 'e', 'r'),
                                scan_trie!(
                                    stream,
                                    'f' + scan_chars!(stream, 'a', 'c', 'e'),
                                    'n' + scan_chars!(stream, 'a', 'l')
                                )
                            )
                        )
                    ),
                    'l' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 't'),
                        'i' + scan_chars!(stream, 'b', 'r', 'a', 'r', 'y')
                    ),
                    'm' + scan_trie!(
                        stream,
                        'a' + scan_trie!(
                            stream,
                            'p' + scan_chars!(stream, 'p', 'i', 'n', 'g'),
                            't' + scan_chars!(stream, 'c', 'h')
                        ),
                        'e' + scan_chars!(stream, 'm', 'o', 'r', 'y'),
                        'i' + scan_chars!(stream, 'n', 'u', 't', 'e', 's'),
                        'o' + scan_chars!(stream, 'd', 'i', 'f', 'i', 'e', 'r')
                    ),
                    'n' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 'w'),
                        'u' + scan_chars!(stream, 'l', 'l')
                    ),
                    'o' + scan_chars!(stream, 'f'),
                    'p' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'y', 'a', 'b', 'l', 'e'),
                        'r' + scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'g', 'm', 'a'),
                            'i' + scan_chars!(stream, 'v', 'a', 't', 'e')
                        ),
                        'u' + scan_trie!(
                            stream,
                            'b' + scan_chars!(stream, 'l', 'i', 'c'),
                            'r' + scan_chars!(stream, 'e')
                        )
                    ),
                    'r' + scan_sequence!(
                        scan_chars!(stream, 'e'),
                        scan_trie!(
                            stream,
                            'l' + scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'),
                            't' + scan_sequence!(
                                scan_chars!(stream, 'u', 'r', 'n'),
                                scan_trie!(stream, EMPTY, ['s'])
                            )
                        )
                    ),
                    's' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 'c', 'o', 'n', 'd', 's'),
                        't' + scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 't', 'i', 'c'),
                            'o' + scan_chars!(stream, 'r', 'a', 'g', 'e'),
                            'r' + scan_trie!(
                                stream,
                                'i' + scan_chars!(stream, 'n', 'g'),
                                'u' + scan_chars!(stream, 'c', 't')
                            )
                        ),
                        'w' + scan_chars!(stream, 'i', 't', 'c', 'h')
                    ),
                    't' + scan_trie!(
                        stream,
                        'h' + scan_chars!(stream, 'r', 'o', 'w'),
                        'r' + scan_trie!(stream, ['y'], 'u' + scan_chars!(stream, 'e')),
                        'y' + scan_sequence!(
                            scan_chars!(stream, 'p', 'e'),
                            scan_trie!(stream, EMPTY, 'o' + scan_chars!(stream, 'f'))
                        )
                    ),
                    'u' + scan_chars!(stream, 's', 'i', 'n', 'g'),
                    'v' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'r'),
                        'i' + scan_chars!(stream, 'e', 'w')
                    ),
                    'w' + scan_trie!(
                        stream,
                        'e' + scan_trie!(stream, ['i'], 'e' + scan_chars!(stream, 'k', 's')),
                        'h' + scan_chars!(stream, 'i', 'l', 'e')
                    ),
                    'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
                ),
                self.not_an_identifier_in_some_versions(stream),
                self.fixed_bytes_type(stream),
                self.signed_fixed_type(stream),
                self.unsigned_fixed_type(stream),
                self.signed_integer_type(stream),
                self.unsigned_integer_type(stream)
            )
        )
    }

    // «IdentifierPart» = «IdentifierStart» | ("0"…"9");

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_part(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    // «IdentifierStart» = "_" | "$" | ("a"…"z") | ("A"…"Z");

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_start(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    // «IfKeyword» = "if";

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
    // «ImmutableKeyword» = "immutable";

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

    // «ImportKeyword» = "import";

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

    // «IndexedKeyword» = "indexed";

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

    // «InterfaceKeyword» = "interface";

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

    // «InternalKeyword» = "internal";

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

    // «IsKeyword» = "is";

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

    // (* v0.6.0 *)
    // «LeaveKeyword» = "leave";

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

    // «LessThan» = "<";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<'),
            scan_predicate!(stream, |c| ('<' <= c && c <= '='))
        )
    }

    // «LessThanEqual» = "<=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '=')
    }

    // «LessThanLessThan» = "<<";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_less_than(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<', '<'),
            scan_chars!(stream, '=')
        )
    }

    // «LessThanLessThanEqual» = "<<=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn less_than_less_than_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '<', '=')
    }

    // «LetKeyword» = "let";

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

    // «LibraryKeyword» = "library";

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

    // «MappingKeyword» = "mapping";

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

    // «MemoryKeyword» = "memory";

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

    // «Minus» = "-";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '-'),
            scan_predicate!(stream, |c| c == '-' || ('=' <= c && c <= '>'))
        )
    }

    // «MinusEqual» = "-=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '=')
    }

    // «MinusGreaterThan» = "->";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_greater_than(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '>')
    }

    // «MinusMinus» = "--";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn minus_minus(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '-')
    }

    // «MinutesKeyword» = "minutes";

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

    // «ModifierKeyword» = "modifier";

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

    // «MultilineComment» = "/*" (!"*" | ("*" !"/"))* "*/";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn multiline_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '*'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    scan_predicate!(stream, |c| c != '*'),
                    scan_sequence!(
                        scan_chars!(stream, '*'),
                        scan_predicate!(stream, |c| c != '/')
                    )
                )
            ),
            scan_chars!(stream, '*', '/')
        )
    }

    // «NewKeyword» = "new";

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

    // «NotAnIdentifierInAnyVersion» = "abstract"
    //                               | "address"
    //                               | "after"
    //                               | "anonymous"
    //                               | "as"
    //                               | "assembly"
    //                               | "bool"
    //                               | "break"
    //                               | "byte"
    //                               | "case"
    //                               | "catch"
    //                               | "constant"
    //                               | "continue"
    //                               | "contract"
    //                               | "days"
    //                               | "default"
    //                               | "delete"
    //                               | "do"
    //                               | "else"
    //                               | "enum"
    //                               | "ether"
    //                               | "event"
    //                               | "external"
    //                               | "false"
    //                               | "final"
    //                               | "for"
    //                               | "function"
    //                               | "hex"
    //                               | "hours"
    //                               | "if"
    //                               | "import"
    //                               | "in"
    //                               | "indexed"
    //                               | "inline"
    //                               | "interface"
    //                               | "internal"
    //                               | "is"
    //                               | "let"
    //                               | "library"
    //                               | "mapping"
    //                               | "match"
    //                               | "memory"
    //                               | "minutes"
    //                               | "modifier"
    //                               | "new"
    //                               | "null"
    //                               | "of"
    //                               | "payable"
    //                               | "pragma"
    //                               | "private"
    //                               | "public"
    //                               | "pure"
    //                               | "relocatable"
    //                               | "return"
    //                               | "returns"
    //                               | "seconds"
    //                               | "static"
    //                               | "storage"
    //                               | "string"
    //                               | "struct"
    //                               | "switch"
    //                               | "throw"
    //                               | "true"
    //                               | "try"
    //                               | "type"
    //                               | "typeof"
    //                               | "using"
    //                               | "var"
    //                               | "view"
    //                               | "weeks"
    //                               | "wei"
    //                               | "while"
    //                               | "years";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn not_an_identifier_in_any_version(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                'd' + scan_chars!(stream, 'd', 'r', 'e', 's', 's'),
                'f' + scan_chars!(stream, 't', 'e', 'r'),
                'n' + scan_chars!(stream, 'o', 'n', 'y', 'm', 'o', 'u', 's'),
                's' + scan_trie!(
                    stream,
                    EMPTY,
                    's' + scan_chars!(stream, 'e', 'm', 'b', 'l', 'y')
                )
            ),
            'b' + scan_trie!(
                stream,
                'o' + scan_chars!(stream, 'o', 'l'),
                'r' + scan_chars!(stream, 'e', 'a', 'k'),
                'y' + scan_chars!(stream, 't', 'e')
            ),
            'c' + scan_trie!(
                stream,
                'a' + scan_trie!(
                    stream,
                    's' + scan_chars!(stream, 'e'),
                    't' + scan_chars!(stream, 'c', 'h')
                ),
                'o' + scan_sequence!(
                    scan_chars!(stream, 'n'),
                    scan_trie!(
                        stream,
                        's' + scan_chars!(stream, 't', 'a', 'n', 't'),
                        't' + scan_trie!(
                            stream,
                            'i' + scan_chars!(stream, 'n', 'u', 'e'),
                            'r' + scan_chars!(stream, 'a', 'c', 't')
                        )
                    )
                )
            ),
            'd' + scan_trie!(
                stream,
                ['o'],
                'a' + scan_chars!(stream, 'y', 's'),
                'e' + scan_trie!(
                    stream,
                    'f' + scan_chars!(stream, 'a', 'u', 'l', 't'),
                    'l' + scan_chars!(stream, 'e', 't', 'e')
                )
            ),
            'e' + scan_trie!(
                stream,
                'l' + scan_chars!(stream, 's', 'e'),
                'n' + scan_chars!(stream, 'u', 'm'),
                't' + scan_chars!(stream, 'h', 'e', 'r'),
                'v' + scan_chars!(stream, 'e', 'n', 't'),
                'x' + scan_chars!(stream, 't', 'e', 'r', 'n', 'a', 'l')
            ),
            'f' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'l', 's', 'e'),
                'i' + scan_chars!(stream, 'n', 'a', 'l'),
                'o' + scan_chars!(stream, 'r'),
                'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
            ),
            'h' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'x'),
                'o' + scan_chars!(stream, 'u', 'r', 's')
            ),
            'i' + scan_trie!(
                stream,
                ['f' | 's'],
                'm' + scan_chars!(stream, 'p', 'o', 'r', 't'),
                'n' + scan_trie!(
                    stream,
                    EMPTY,
                    'd' + scan_chars!(stream, 'e', 'x', 'e', 'd'),
                    'l' + scan_chars!(stream, 'i', 'n', 'e'),
                    't' + scan_sequence!(
                        scan_chars!(stream, 'e', 'r'),
                        scan_trie!(
                            stream,
                            'f' + scan_chars!(stream, 'a', 'c', 'e'),
                            'n' + scan_chars!(stream, 'a', 'l')
                        )
                    )
                )
            ),
            'l' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 't'),
                'i' + scan_chars!(stream, 'b', 'r', 'a', 'r', 'y')
            ),
            'm' + scan_trie!(
                stream,
                'a' + scan_trie!(
                    stream,
                    'p' + scan_chars!(stream, 'p', 'i', 'n', 'g'),
                    't' + scan_chars!(stream, 'c', 'h')
                ),
                'e' + scan_chars!(stream, 'm', 'o', 'r', 'y'),
                'i' + scan_chars!(stream, 'n', 'u', 't', 'e', 's'),
                'o' + scan_chars!(stream, 'd', 'i', 'f', 'i', 'e', 'r')
            ),
            'n' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'w'),
                'u' + scan_chars!(stream, 'l', 'l')
            ),
            'o' + scan_chars!(stream, 'f'),
            'p' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'y', 'a', 'b', 'l', 'e'),
                'r' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 'g', 'm', 'a'),
                    'i' + scan_chars!(stream, 'v', 'a', 't', 'e')
                ),
                'u' + scan_trie!(
                    stream,
                    'b' + scan_chars!(stream, 'l', 'i', 'c'),
                    'r' + scan_chars!(stream, 'e')
                )
            ),
            'r' + scan_sequence!(
                scan_chars!(stream, 'e'),
                scan_trie!(
                    stream,
                    'l' + scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e'),
                    't' + scan_sequence!(
                        scan_chars!(stream, 'u', 'r', 'n'),
                        scan_trie!(stream, EMPTY, ['s'])
                    )
                )
            ),
            's' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'c', 'o', 'n', 'd', 's'),
                't' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 't', 'i', 'c'),
                    'o' + scan_chars!(stream, 'r', 'a', 'g', 'e'),
                    'r' + scan_trie!(
                        stream,
                        'i' + scan_chars!(stream, 'n', 'g'),
                        'u' + scan_chars!(stream, 'c', 't')
                    )
                ),
                'w' + scan_chars!(stream, 'i', 't', 'c', 'h')
            ),
            't' + scan_trie!(
                stream,
                'h' + scan_chars!(stream, 'r', 'o', 'w'),
                'r' + scan_trie!(stream, ['y'], 'u' + scan_chars!(stream, 'e')),
                'y' + scan_sequence!(
                    scan_chars!(stream, 'p', 'e'),
                    scan_trie!(stream, EMPTY, 'o' + scan_chars!(stream, 'f'))
                )
            ),
            'u' + scan_chars!(stream, 's', 'i', 'n', 'g'),
            'v' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'r'),
                'i' + scan_chars!(stream, 'e', 'w')
            ),
            'w' + scan_trie!(
                stream,
                'e' + scan_trie!(stream, ['i'], 'e' + scan_chars!(stream, 'k', 's')),
                'h' + scan_chars!(stream, 'i', 'l', 'e')
            ),
            'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
        )
    }

    // (* v0.4.11 *)
    // «NotAnIdentifierInSomeVersions» = "finney" | "szabo";

    #[allow(dead_code, non_snake_case)]
    fn not_an_identifier_in_some_versions__0_4_11(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'f' + scan_chars!(stream, 'i', 'n', 'n', 'e', 'y'),
            's' + scan_chars!(stream, 'z', 'a', 'b', 'o')
        )
    }

    // (* v0.5.0 *)
    // «NotAnIdentifierInSomeVersions» = "finney"
    //                                 | "szabo"
    //                                 | "alias"
    //                                 | "apply"
    //                                 | "auto"
    //                                 | "calldata"
    //                                 | "constructor"
    //                                 | "copyof"
    //                                 | "define"
    //                                 | "emit"
    //                                 | "immutable"
    //                                 | "implements"
    //                                 | "macro"
    //                                 | "mutable"
    //                                 | "override"
    //                                 | "partial"
    //                                 | "promise"
    //                                 | "reference"
    //                                 | "sealed"
    //                                 | "sizeof"
    //                                 | "supports"
    //                                 | "typedef"
    //                                 | "unchecked";

    #[allow(dead_code, non_snake_case)]
    fn not_an_identifier_in_some_versions__0_5_0(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'l' + scan_chars!(stream, 'i', 'a', 's'),
                'p' + scan_chars!(stream, 'p', 'l', 'y'),
                'u' + scan_chars!(stream, 't', 'o')
            ),
            'c' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'l', 'l', 'd', 'a', 't', 'a'),
                'o' + scan_trie!(
                    stream,
                    'n' + scan_chars!(stream, 's', 't', 'r', 'u', 'c', 't', 'o', 'r'),
                    'p' + scan_chars!(stream, 'y', 'o', 'f')
                )
            ),
            'd' + scan_chars!(stream, 'e', 'f', 'i', 'n', 'e'),
            'e' + scan_chars!(stream, 'm', 'i', 't'),
            'f' + scan_chars!(stream, 'i', 'n', 'n', 'e', 'y'),
            'i' + scan_sequence!(
                scan_chars!(stream, 'm'),
                scan_trie!(
                    stream,
                    'm' + scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e'),
                    'p' + scan_chars!(stream, 'l', 'e', 'm', 'e', 'n', 't', 's')
                )
            ),
            'm' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'c', 'r', 'o'),
                'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
            ),
            'o' + scan_chars!(stream, 'v', 'e', 'r', 'r', 'i', 'd', 'e'),
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
                'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's'),
                'z' + scan_chars!(stream, 'a', 'b', 'o')
            ),
            't' + scan_chars!(stream, 'y', 'p', 'e', 'd', 'e', 'f'),
            'u' + scan_chars!(stream, 'n', 'c', 'h', 'e', 'c', 'k', 'e', 'd')
        )
    }

    // (* v0.6.0 *)
    // «NotAnIdentifierInSomeVersions» = "finney"
    //                                 | "szabo"
    //                                 | "alias"
    //                                 | "apply"
    //                                 | "auto"
    //                                 | "calldata"
    //                                 | "constructor"
    //                                 | "copyof"
    //                                 | "define"
    //                                 | "emit"
    //                                 | "immutable"
    //                                 | "implements"
    //                                 | "macro"
    //                                 | "mutable"
    //                                 | "override"
    //                                 | "partial"
    //                                 | "promise"
    //                                 | "reference"
    //                                 | "sealed"
    //                                 | "sizeof"
    //                                 | "supports"
    //                                 | "typedef"
    //                                 | "unchecked"
    //                                 | "fallback"
    //                                 | "receive"
    //                                 | "virtual";

    #[allow(dead_code, non_snake_case)]
    fn not_an_identifier_in_some_versions__0_6_0(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'l' + scan_chars!(stream, 'i', 'a', 's'),
                'p' + scan_chars!(stream, 'p', 'l', 'y'),
                'u' + scan_chars!(stream, 't', 'o')
            ),
            'c' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'l', 'l', 'd', 'a', 't', 'a'),
                'o' + scan_trie!(
                    stream,
                    'n' + scan_chars!(stream, 's', 't', 'r', 'u', 'c', 't', 'o', 'r'),
                    'p' + scan_chars!(stream, 'y', 'o', 'f')
                )
            ),
            'd' + scan_chars!(stream, 'e', 'f', 'i', 'n', 'e'),
            'e' + scan_chars!(stream, 'm', 'i', 't'),
            'f' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'l', 'l', 'b', 'a', 'c', 'k'),
                'i' + scan_chars!(stream, 'n', 'n', 'e', 'y')
            ),
            'i' + scan_sequence!(
                scan_chars!(stream, 'm'),
                scan_trie!(
                    stream,
                    'm' + scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e'),
                    'p' + scan_chars!(stream, 'l', 'e', 'm', 'e', 'n', 't', 's')
                )
            ),
            'm' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'c', 'r', 'o'),
                'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
            ),
            'o' + scan_chars!(stream, 'v', 'e', 'r', 'r', 'i', 'd', 'e'),
            'p' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'r', 't', 'i', 'a', 'l'),
                'r' + scan_chars!(stream, 'o', 'm', 'i', 's', 'e')
            ),
            'r' + scan_sequence!(
                scan_chars!(stream, 'e'),
                scan_trie!(
                    stream,
                    'c' + scan_chars!(stream, 'e', 'i', 'v', 'e'),
                    'f' + scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e')
                )
            ),
            's' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'a', 'l', 'e', 'd'),
                'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's'),
                'z' + scan_chars!(stream, 'a', 'b', 'o')
            ),
            't' + scan_chars!(stream, 'y', 'p', 'e', 'd', 'e', 'f'),
            'u' + scan_chars!(stream, 'n', 'c', 'h', 'e', 'c', 'k', 'e', 'd'),
            'v' + scan_chars!(stream, 'i', 'r', 't', 'u', 'a', 'l')
        )
    }

    // (* v0.7.0 *)
    // «NotAnIdentifierInSomeVersions» = "alias"
    //                                 | "apply"
    //                                 | "auto"
    //                                 | "calldata"
    //                                 | "constructor"
    //                                 | "copyof"
    //                                 | "define"
    //                                 | "emit"
    //                                 | "immutable"
    //                                 | "implements"
    //                                 | "macro"
    //                                 | "mutable"
    //                                 | "override"
    //                                 | "partial"
    //                                 | "promise"
    //                                 | "reference"
    //                                 | "sealed"
    //                                 | "sizeof"
    //                                 | "supports"
    //                                 | "typedef"
    //                                 | "unchecked"
    //                                 | "fallback"
    //                                 | "receive"
    //                                 | "virtual"
    //                                 | "gwei";

    #[allow(dead_code, non_snake_case)]
    fn not_an_identifier_in_some_versions__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_trie!(
            stream,
            'a' + scan_trie!(
                stream,
                'l' + scan_chars!(stream, 'i', 'a', 's'),
                'p' + scan_chars!(stream, 'p', 'l', 'y'),
                'u' + scan_chars!(stream, 't', 'o')
            ),
            'c' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'l', 'l', 'd', 'a', 't', 'a'),
                'o' + scan_trie!(
                    stream,
                    'n' + scan_chars!(stream, 's', 't', 'r', 'u', 'c', 't', 'o', 'r'),
                    'p' + scan_chars!(stream, 'y', 'o', 'f')
                )
            ),
            'd' + scan_chars!(stream, 'e', 'f', 'i', 'n', 'e'),
            'e' + scan_chars!(stream, 'm', 'i', 't'),
            'f' + scan_chars!(stream, 'a', 'l', 'l', 'b', 'a', 'c', 'k'),
            'g' + scan_chars!(stream, 'w', 'e', 'i'),
            'i' + scan_sequence!(
                scan_chars!(stream, 'm'),
                scan_trie!(
                    stream,
                    'm' + scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e'),
                    'p' + scan_chars!(stream, 'l', 'e', 'm', 'e', 'n', 't', 's')
                )
            ),
            'm' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'c', 'r', 'o'),
                'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
            ),
            'o' + scan_chars!(stream, 'v', 'e', 'r', 'r', 'i', 'd', 'e'),
            'p' + scan_trie!(
                stream,
                'a' + scan_chars!(stream, 'r', 't', 'i', 'a', 'l'),
                'r' + scan_chars!(stream, 'o', 'm', 'i', 's', 'e')
            ),
            'r' + scan_sequence!(
                scan_chars!(stream, 'e'),
                scan_trie!(
                    stream,
                    'c' + scan_chars!(stream, 'e', 'i', 'v', 'e'),
                    'f' + scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e')
                )
            ),
            's' + scan_trie!(
                stream,
                'e' + scan_chars!(stream, 'a', 'l', 'e', 'd'),
                'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's')
            ),
            't' + scan_chars!(stream, 'y', 'p', 'e', 'd', 'e', 'f'),
            'u' + scan_chars!(stream, 'n', 'c', 'h', 'e', 'c', 'k', 'e', 'd'),
            'v' + scan_chars!(stream, 'i', 'r', 't', 'u', 'a', 'l')
        )
    }

    pub(crate) fn not_an_identifier_in_some_versions(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.not_an_identifier_in_some_versions__0_7_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.not_an_identifier_in_some_versions__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.not_an_identifier_in_some_versions__0_5_0(stream)
        } else {
            self.not_an_identifier_in_some_versions__0_4_11(stream)
        }
    }

    // «OpenBrace» = "{";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_brace(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '{')
    }

    // «OpenBracket» = "[";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_bracket(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '[')
    }

    // «OpenParen» = "(";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn open_paren(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '(')
    }

    // «OverrideKeyword» = "override";

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

    // «PayableKeyword» = "payable";

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

    // «Percent» = "%";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn percent(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '%'), scan_chars!(stream, '='))
    }

    // «PercentEqual» = "%=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn percent_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '%', '=')
    }

    // «Period» = ".";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn period(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '.')
    }

    // «Plus» = "+";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '+'),
            scan_predicate!(stream, |c| c == '+' || c == '=')
        )
    }

    // «PlusEqual» = "+=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '=')
    }

    // «PlusPlus» = "++";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn plus_plus(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '+')
    }

    // «PossiblySeparatedPairsOfHexDigits» = «HexCharacter» «HexCharacter» ("_"? «HexCharacter» «HexCharacter»)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn possibly_separated_pairs_of_hex_digits(&self, stream: &mut Stream) -> bool {
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

    // «PragmaKeyword» = "pragma";

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

    // «PrivateKeyword» = "private";

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

    // «PublicKeyword» = "public";

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

    // «PureKeyword» = "pure";

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

    // «QuestionMark» = "?";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn question_mark(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '?')
    }

    // «RawIdentifier» = «IdentifierStart» «IdentifierPart»*;

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

    // «ReceiveKeyword» = "receive";

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

    // «ReturnKeyword» = "return";

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

    // «ReturnsKeyword» = "returns";

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

    // «RevertKeyword» = "revert";

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

    // «SecondsKeyword» = "seconds";

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

    // «Semicolon» = ";";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn semicolon(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ';')
    }

    // «SignedFixedType» = "fixed" (("0"…"9")+ "x" ("0"…"9")+)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn signed_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_sequence!(
                scan_chars!(stream, 'f', 'i', 'x', 'e', 'd'),
                scan_optional!(
                    stream,
                    scan_sequence!(
                        scan_one_or_more!(
                            stream,
                            scan_predicate!(stream, |c| ('0' <= c && c <= '9'))
                        ),
                        scan_chars!(stream, 'x'),
                        scan_one_or_more!(
                            stream,
                            scan_predicate!(stream, |c| ('0' <= c && c <= '9'))
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

    // «SignedIntegerType» = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;

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

    // «SingleLineComment» = "//" (!("\r" | "\n"))*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn single_line_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '/'),
            scan_zero_or_more!(stream, scan_predicate!(stream, |c| c != '\n' && c != '\r'))
        )
    }

    // «SingleQuotedAsciiStringLiteral» = "'" («EscapeSequence» | ((" "…"~") - ("'" | "\\")))* "'";

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

    // (* v0.7.0 *)
    // «SingleQuotedUnicodeStringLiteral» = "unicode'" («EscapeSequence» | !("'" | "\\" | "\n" | "\r"))* "'";

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

    // «Slash» = "/";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn slash(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '/'), scan_chars!(stream, '='))
    }

    // «SlashEqual» = "/=";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn slash_equal(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '/', '=')
    }

    // «SolidityKeyword» = "solidity";

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

    // «StorageKeyword» = "storage";

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

    // «StringKeyword» = "string";

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

    // «StructKeyword» = "struct";

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

    // «SwitchKeyword» = "switch";

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
    // «SzaboKeyword» = "szabo";

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
    // «ThrowKeyword» = "throw";

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

    // «Tilde» = "~";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tilde(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '~')
    }

    // «TrueKeyword» = "true";

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
    // «TryKeyword» = "try";

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
    // «TypeKeyword» = "type";

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
    // «UncheckedKeyword» = "unchecked";

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

    // «UnicodeEscape» = "u" «HexCharacter» «HexCharacter» «HexCharacter» «HexCharacter»;

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
    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral»;

    #[allow(dead_code, non_snake_case)]
    fn unicode_string_literal__0_7_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_unicode_string_literal(stream),
            self.double_quoted_unicode_string_literal(stream)
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

    // «UnsignedFixedType» = "u" «SignedFixedType»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(scan_chars!(stream, 'u'), self.signed_fixed_type(stream))
    }

    // «UnsignedIntegerType» = "u" «SignedIntegerType»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(scan_chars!(stream, 'u'), self.signed_integer_type(stream))
    }

    // «UsingKeyword» = "using";

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
    // «VarKeyword» = "var";

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

    // «VersionPragmaValue» = (("0"…"9") | "x" | "X" | "*")+;

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

    // «ViewKeyword» = "view";

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
    // «VirtualKeyword» = "virtual";

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

    // «WeeksKeyword» = "weeks";

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

    // «WeiKeyword» = "wei";

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

    // «WhileKeyword» = "while";

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

    // «Whitespace» = (" " | "\t")+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn whitespace(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\t' || c == ' '))
    }

    // (* v0.4.11 *)
    // «YearsKeyword» = "years";

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

    // «YulDecimalLiteral» = "0" | (("1"…"9") ("0"…"9")*);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '0'),
            scan_sequence!(
                scan_predicate!(stream, |c| ('1' <= c && c <= '9')),
                scan_zero_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
            )
        )
    }

    // «YulHexLiteral» = "0x" «HexCharacter»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_hex_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '0', 'x'),
            scan_one_or_more!(
                stream,
                scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                    || ('A' <= c && c <= 'F')
                    || ('a' <= c && c <= 'f'))
            )
        )
    }

    // «YulIdentifier» = «RawIdentifier» - («YulKeyword» | «YulReservedKeyword»);

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
    // «YulKeyword» = «BreakKeyword»
    //              | «CaseKeyword»
    //              | «ContinueKeyword»
    //              | «DefaultKeyword»
    //              | «FalseKeyword»
    //              | «ForKeyword»
    //              | «FunctionKeyword»
    //              | «IfKeyword»
    //              | «LetKeyword»
    //              | «SwitchKeyword»
    //              | «TrueKeyword»;

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
    // «YulKeyword» = «BreakKeyword»
    //              | «CaseKeyword»
    //              | «ContinueKeyword»
    //              | «DefaultKeyword»
    //              | «FalseKeyword»
    //              | «ForKeyword»
    //              | «FunctionKeyword»
    //              | «IfKeyword»
    //              | «LeaveKeyword»
    //              | «LetKeyword»
    //              | «SwitchKeyword»
    //              | «TrueKeyword»;

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

    // «YulReservedKeyword» = "hex";

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_reserved_keyword(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, 'h', 'e', 'x')
    }
}
