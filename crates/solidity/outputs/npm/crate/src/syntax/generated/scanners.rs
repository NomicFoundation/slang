// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::language::*;

#[allow(unused_macros)]
macro_rules! scan_predicate {
    ($stream:ident, $predicate:expr) => {
        if let Some(c) = $stream.next() {
            if $predicate(c) {
                true
            } else {
                $stream.undo();
                false
            }
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_chars {
    ($stream:ident, $($char:literal),+) => {
        if $( $stream.next() == Some($char) )&&* {
            true
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_trie {
    ($stream:ident, EMPTY, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        ({
            match $stream.next() {
                $($(Some($terminal))|* => true,)?
                $(Some($prefix) => $subtree,)*
                _ => { $stream.undo(); true }
            }
        })
    };
    ($stream:ident, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        match $stream.next() {
            $($(Some($terminal))|* => true,)?
            $(Some($prefix) => $subtree,)*
            _ => { $stream.undo(); false }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_sequence {
    ($($expr:expr),*) => {
        $(($expr))&&*
    };
}

#[allow(unused_macros)]
macro_rules! scan_choice {
    ($stream:ident, $($expr:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                if ($expr) { break true }
                $stream.set_position(save);
            )*
            break false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_zero_or_more {
    ($stream:ident, $expr:expr) => {
        loop {
            let save = $stream.position();
            if !($expr) {
                $stream.set_position(save);
                break true;
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_one_or_more {
    ($stream:ident, $expr:expr) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < 1 {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_repeated {
    ($stream:ident, $expr:expr, $min:literal, $max:literal) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < $min {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
            if count == $max {
                break true;
            }
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_optional {
    ($stream:ident, $expr:expr) => {{
        let save = $stream.position();
        if !($expr) {
            $stream.set_position(save)
        }
        true
    }};
}

#[allow(unused_macros)]
macro_rules! scan_difference {
    ($stream:ident, $minuend:expr, $subtrahend:expr) => {{
        let start = $stream.position();
        ($minuend)
            && ({
                let end = $stream.position();
                $stream.set_position(start);
                if ($subtrahend) && (end == $stream.position()) {
                    false
                } else {
                    $stream.set_position(end);
                    true
                }
            })
    }};
}

#[allow(unused_macros)]
macro_rules! scan_not_followed_by {
    ($stream:ident, $expression:expr, $not_followed_by:expr) => {
        ($expression)
            && ({
                let end = $stream.position();
                let following = $not_followed_by;
                $stream.set_position(end);
                !following
            })
    };
}

impl Language {
    // «AbicoderKeyword» = "abicoder";

    #[allow(unused_assignments, unused_parens)]
    fn scan_abicoder_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_abicoder_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_abicoder_keyword_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «AbstractKeyword» = "abstract";

    #[allow(unused_assignments, unused_parens)]
    fn scan_abstract_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_abstract_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_abstract_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_abstract_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_abstract_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_abstract_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_abstract_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «AddressKeyword» = "address";

    #[allow(unused_assignments, unused_parens)]
    fn scan_address_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_address_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_address_keyword_0_4_11(stream)
    }

    // «Ampersand» = "&";

    #[allow(unused_assignments, unused_parens)]
    fn scan_ampersand_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '&'),
            scan_predicate!(stream, |c| c == '&' || c == '=')
        )
    }

    #[inline]
    pub(crate) fn scan_ampersand(&self, stream: &mut Stream) -> bool {
        self.scan_ampersand_0_4_11(stream)
    }

    // «AmpersandAmpersand» = "&&";

    #[allow(unused_assignments, unused_parens)]
    fn scan_ampersand_ampersand_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '&')
    }

    #[inline]
    pub(crate) fn scan_ampersand_ampersand(&self, stream: &mut Stream) -> bool {
        self.scan_ampersand_ampersand_0_4_11(stream)
    }

    // «AmpersandEqual» = "&=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_ampersand_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '&', '=')
    }

    #[inline]
    pub(crate) fn scan_ampersand_equal(&self, stream: &mut Stream) -> bool {
        self.scan_ampersand_equal_0_4_11(stream)
    }

    // «AnonymousKeyword» = "anonymous";

    #[allow(unused_assignments, unused_parens)]
    fn scan_anonymous_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_anonymous_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_anonymous_keyword_0_4_11(stream)
    }

    // «AsKeyword» = "as";

    #[allow(unused_assignments, unused_parens)]
    fn scan_as_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_as_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_as_keyword_0_4_11(stream)
    }

    // «AsciiEscape» = "n"
    //               | "r"
    //               | "t"
    //               | "'"
    //               | '"'
    //               | "\\"
    //               | "\n"
    //               | "\r";

    #[allow(unused_assignments, unused_parens)]
    fn scan_ascii_escape_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '\n'
            || c == '\r'
            || c == '"'
            || c == '\''
            || c == '\\'
            || c == 'n'
            || c == 'r'
            || c == 't')
    }

    #[inline]
    pub(crate) fn scan_ascii_escape(&self, stream: &mut Stream) -> bool {
        self.scan_ascii_escape_0_4_11(stream)
    }

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral»;

    #[allow(unused_assignments, unused_parens)]
    fn scan_ascii_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.scan_single_quoted_ascii_string_literal(stream),
            self.scan_double_quoted_ascii_string_literal(stream)
        )
    }

    #[inline]
    pub(crate) fn scan_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_ascii_string_literal_0_4_11(stream)
    }

    // «AssemblyKeyword» = "assembly";

    #[allow(unused_assignments, unused_parens)]
    fn scan_assembly_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_assembly_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_assembly_keyword_0_4_11(stream)
    }

    // «Bang» = "!";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bang_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '!'), scan_chars!(stream, '='))
    }

    #[inline]
    pub(crate) fn scan_bang(&self, stream: &mut Stream) -> bool {
        self.scan_bang_0_4_11(stream)
    }

    // «BangEqual» = "!=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bang_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '!', '=')
    }

    #[inline]
    pub(crate) fn scan_bang_equal(&self, stream: &mut Stream) -> bool {
        self.scan_bang_equal_0_4_11(stream)
    }

    // «Bar» = "|";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bar_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '|'),
            scan_predicate!(stream, |c| c == '=' || c == '|')
        )
    }

    #[inline]
    pub(crate) fn scan_bar(&self, stream: &mut Stream) -> bool {
        self.scan_bar_0_4_11(stream)
    }

    // «BarBar» = "||";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bar_bar_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '|')
    }

    #[inline]
    pub(crate) fn scan_bar_bar(&self, stream: &mut Stream) -> bool {
        self.scan_bar_bar_0_4_11(stream)
    }

    // «BarEqual» = "|=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bar_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '|', '=')
    }

    #[inline]
    pub(crate) fn scan_bar_equal(&self, stream: &mut Stream) -> bool {
        self.scan_bar_equal_0_4_11(stream)
    }

    // «BoolKeyword» = "bool";

    #[allow(unused_assignments, unused_parens)]
    fn scan_bool_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_bool_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_bool_keyword_0_4_11(stream)
    }

    // «BreakKeyword» = "break";

    #[allow(unused_assignments, unused_parens)]
    fn scan_break_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_break_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_break_keyword_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «ByteType» = "byte";

    #[allow(unused_assignments, unused_parens)]
    fn scan_byte_type_0_4_11(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_byte_type(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            None
        } else {
            Some(self.scan_byte_type_0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_byte_type(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_byte_type(stream)
    }

    #[inline]
    pub(crate) fn scan_byte_type(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_byte_type(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.5.0 *)
    // «CalldataKeyword» = "calldata";

    #[allow(unused_assignments, unused_parens)]
    fn scan_calldata_keyword_0_5_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_calldata_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            Some(self.scan_calldata_keyword_0_5_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_calldata_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_calldata_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_calldata_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_calldata_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «Caret» = "^";

    #[allow(unused_assignments, unused_parens)]
    fn scan_caret_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '^'), scan_chars!(stream, '='))
    }

    #[inline]
    pub(crate) fn scan_caret(&self, stream: &mut Stream) -> bool {
        self.scan_caret_0_4_11(stream)
    }

    // «CaretEqual» = "^=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_caret_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '^', '=')
    }

    #[inline]
    pub(crate) fn scan_caret_equal(&self, stream: &mut Stream) -> bool {
        self.scan_caret_equal_0_4_11(stream)
    }

    // «CaseKeyword» = "case";

    #[allow(unused_assignments, unused_parens)]
    fn scan_case_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_case_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_case_keyword_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «CatchKeyword» = "catch";

    #[allow(unused_assignments, unused_parens)]
    fn scan_catch_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_catch_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_catch_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_catch_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_catch_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_catch_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_catch_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «CloseBrace» = "}";

    #[allow(unused_assignments, unused_parens)]
    fn scan_close_brace_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '}')
    }

    #[inline]
    pub(crate) fn scan_close_brace(&self, stream: &mut Stream) -> bool {
        self.scan_close_brace_0_4_11(stream)
    }

    // «CloseBracket» = "]";

    #[allow(unused_assignments, unused_parens)]
    fn scan_close_bracket_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ']')
    }

    #[inline]
    pub(crate) fn scan_close_bracket(&self, stream: &mut Stream) -> bool {
        self.scan_close_bracket_0_4_11(stream)
    }

    // «CloseParen» = ")";

    #[allow(unused_assignments, unused_parens)]
    fn scan_close_paren_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ')')
    }

    #[inline]
    pub(crate) fn scan_close_paren(&self, stream: &mut Stream) -> bool {
        self.scan_close_paren_0_4_11(stream)
    }

    // «Colon» = ":";

    #[allow(unused_assignments, unused_parens)]
    fn scan_colon_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, ':'), scan_chars!(stream, '='))
    }

    #[inline]
    pub(crate) fn scan_colon(&self, stream: &mut Stream) -> bool {
        self.scan_colon_0_4_11(stream)
    }

    // «ColonEqual» = ":=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_colon_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ':', '=')
    }

    #[inline]
    pub(crate) fn scan_colon_equal(&self, stream: &mut Stream) -> bool {
        self.scan_colon_equal_0_4_11(stream)
    }

    // «Comma» = ",";

    #[allow(unused_assignments, unused_parens)]
    fn scan_comma_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ',')
    }

    #[inline]
    pub(crate) fn scan_comma(&self, stream: &mut Stream) -> bool {
        self.scan_comma_0_4_11(stream)
    }

    // «ConstantKeyword» = "constant";

    #[allow(unused_assignments, unused_parens)]
    fn scan_constant_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_constant_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_constant_keyword_0_4_11(stream)
    }

    // (* v0.4.22 *)
    // «ConstructorKeyword» = "constructor";

    #[allow(unused_assignments, unused_parens)]
    fn scan_constructor_keyword_0_4_22(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_constructor_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.scan_constructor_keyword_0_4_22(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_constructor_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_constructor_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_constructor_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_constructor_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «ContinueKeyword» = "continue";

    #[allow(unused_assignments, unused_parens)]
    fn scan_continue_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_continue_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_continue_keyword_0_4_11(stream)
    }

    // «ContractKeyword» = "contract";

    #[allow(unused_assignments, unused_parens)]
    fn scan_contract_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_contract_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_contract_keyword_0_4_11(stream)
    }

    // «DaysKeyword» = "days";

    #[allow(unused_assignments, unused_parens)]
    fn scan_days_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_days_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_days_keyword_0_4_11(stream)
    }

    // «DecimalExponent» = ("e" | "E") "-"? «DecimalNumber»;

    #[allow(unused_assignments, unused_parens)]
    fn scan_decimal_exponent_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_predicate!(stream, |c| c == 'E' || c == 'e'),
            scan_optional!(stream, scan_chars!(stream, '-')),
            self.scan_decimal_number(stream)
        )
    }

    #[inline]
    pub(crate) fn scan_decimal_exponent(&self, stream: &mut Stream) -> bool {
        self.scan_decimal_exponent_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «DecimalLiteral» = ((«DecimalNumber» ("." «DecimalNumber»?)?) | ("." «DecimalNumber»)) «DecimalExponent»?;

    #[allow(unused_assignments, unused_parens)]
    fn scan_decimal_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_sequence!(
                    self.scan_decimal_number(stream),
                    scan_optional!(
                        stream,
                        scan_sequence!(
                            scan_chars!(stream, '.'),
                            scan_optional!(stream, self.scan_decimal_number(stream))
                        )
                    )
                ),
                scan_sequence!(scan_chars!(stream, '.'), self.scan_decimal_number(stream))
            ),
            scan_optional!(stream, self.scan_decimal_exponent(stream))
        )
    }

    // (* v0.5.0 *)
    // «DecimalLiteral» = ((«DecimalNumber» ("." «DecimalNumber»)?) | ("." «DecimalNumber»)) «DecimalExponent»?;

    #[allow(unused_assignments, unused_parens)]
    fn scan_decimal_literal_0_5_0(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_sequence!(
                    self.scan_decimal_number(stream),
                    scan_optional!(
                        stream,
                        scan_sequence!(scan_chars!(stream, '.'), self.scan_decimal_number(stream))
                    )
                ),
                scan_sequence!(scan_chars!(stream, '.'), self.scan_decimal_number(stream))
            ),
            scan_optional!(stream, self.scan_decimal_exponent(stream))
        )
    }

    fn dispatch_scan_decimal_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.scan_decimal_literal_0_5_0(stream)
        } else {
            self.scan_decimal_literal_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn scan_decimal_literal(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_decimal_literal(stream)
    }

    // «DecimalNumber» = ("0"…"9")+ ("_" ("0"…"9")+)*;

    #[allow(unused_assignments, unused_parens)]
    fn scan_decimal_number_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_decimal_number(&self, stream: &mut Stream) -> bool {
        self.scan_decimal_number_0_4_11(stream)
    }

    // «DefaultKeyword» = "default";

    #[allow(unused_assignments, unused_parens)]
    fn scan_default_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_default_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_default_keyword_0_4_11(stream)
    }

    // «DeleteKeyword» = "delete";

    #[allow(unused_assignments, unused_parens)]
    fn scan_delete_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_delete_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_delete_keyword_0_4_11(stream)
    }

    // «DoKeyword» = "do";

    #[allow(unused_assignments, unused_parens)]
    fn scan_do_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_do_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_do_keyword_0_4_11(stream)
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' («EscapeSequence» | ((" "…"~") - ('"' | "\\")))* '"';

    #[allow(unused_assignments, unused_parens)]
    fn scan_double_quoted_ascii_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.scan_escape_sequence(stream),
                    scan_predicate!(stream, |c| (' ' <= c && c <= '!')
                        || ('#' <= c && c <= '[')
                        || (']' <= c && c <= '~'))
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    #[inline]
    pub(crate) fn scan_double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_double_quoted_ascii_string_literal_0_4_11(stream)
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' («EscapeSequence» | !('"' | "\\" | "\n" | "\r"))* '"';

    #[allow(unused_assignments, unused_parens)]
    fn scan_double_quoted_unicode_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.scan_escape_sequence(stream),
                    scan_predicate!(stream, |c| c != '\n' && c != '\r' && c != '"' && c != '\\')
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    #[inline]
    pub(crate) fn scan_double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_double_quoted_unicode_string_literal_0_4_11(stream)
    }

    // «ElseKeyword» = "else";

    #[allow(unused_assignments, unused_parens)]
    fn scan_else_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_else_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_else_keyword_0_4_11(stream)
    }

    // (* v0.4.21 *)
    // «EmitKeyword» = "emit";

    #[allow(unused_assignments, unused_parens)]
    fn scan_emit_keyword_0_4_21(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_emit_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_4_21 {
            Some(self.scan_emit_keyword_0_4_21(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_emit_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_emit_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_emit_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_emit_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «EndOfLine» = "\r"? "\n";

    #[allow(unused_assignments, unused_parens)]
    fn scan_end_of_line_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_optional!(stream, scan_chars!(stream, '\r')),
            scan_chars!(stream, '\n')
        )
    }

    #[inline]
    pub(crate) fn scan_end_of_line(&self, stream: &mut Stream) -> bool {
        self.scan_end_of_line_0_4_11(stream)
    }

    // «EnumKeyword» = "enum";

    #[allow(unused_assignments, unused_parens)]
    fn scan_enum_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_enum_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_enum_keyword_0_4_11(stream)
    }

    // «Equal» = "=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '='),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    #[inline]
    pub(crate) fn scan_equal(&self, stream: &mut Stream) -> bool {
        self.scan_equal_0_4_11(stream)
    }

    // «EqualEqual» = "==";

    #[allow(unused_assignments, unused_parens)]
    fn scan_equal_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '=')
    }

    #[inline]
    pub(crate) fn scan_equal_equal(&self, stream: &mut Stream) -> bool {
        self.scan_equal_equal_0_4_11(stream)
    }

    // «EqualGreaterThan» = "=>";

    #[allow(unused_assignments, unused_parens)]
    fn scan_equal_greater_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '=', '>')
    }

    #[inline]
    pub(crate) fn scan_equal_greater_than(&self, stream: &mut Stream) -> bool {
        self.scan_equal_greater_than_0_4_11(stream)
    }

    // «ErrorKeyword» = "error";

    #[allow(unused_assignments, unused_parens)]
    fn scan_error_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_error_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_error_keyword_0_4_11(stream)
    }

    // «EscapeSequence» = "\\" («AsciiEscape» | «HexByteEscape» | «UnicodeEscape»);

    #[allow(unused_assignments, unused_parens)]
    fn scan_escape_sequence_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\\'),
            scan_choice!(
                stream,
                scan_trie!(stream, ['\n' | '\r' | '"' | '\'' | '\\' | 'n' | 'r' | 't']),
                self.scan_hex_byte_escape(stream),
                self.scan_unicode_escape(stream)
            )
        )
    }

    #[inline]
    pub(crate) fn scan_escape_sequence(&self, stream: &mut Stream) -> bool {
        self.scan_escape_sequence_0_4_11(stream)
    }

    // «EtherKeyword» = "ether";

    #[allow(unused_assignments, unused_parens)]
    fn scan_ether_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_ether_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_ether_keyword_0_4_11(stream)
    }

    // «EventKeyword» = "event";

    #[allow(unused_assignments, unused_parens)]
    fn scan_event_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_event_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_event_keyword_0_4_11(stream)
    }

    // «Evmasm» = '"evmasm"';

    #[allow(unused_assignments, unused_parens)]
    fn scan_evmasm_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '"', 'e', 'v', 'm', 'a', 's', 'm', '"')
    }

    #[inline]
    pub(crate) fn scan_evmasm(&self, stream: &mut Stream) -> bool {
        self.scan_evmasm_0_4_11(stream)
    }

    // «ExperimentalKeyword» = "experimental";

    #[allow(unused_assignments, unused_parens)]
    fn scan_experimental_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_experimental_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_experimental_keyword_0_4_11(stream)
    }

    // «ExternalKeyword» = "external";

    #[allow(unused_assignments, unused_parens)]
    fn scan_external_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_external_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_external_keyword_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «FallbackKeyword» = "fallback";

    #[allow(unused_assignments, unused_parens)]
    fn scan_fallback_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_fallback_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_fallback_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_fallback_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_fallback_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_fallback_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_fallback_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «FalseKeyword» = "false";

    #[allow(unused_assignments, unused_parens)]
    fn scan_false_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_false_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_false_keyword_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «FinneyKeyword» = "finney";

    #[allow(unused_assignments, unused_parens)]
    fn scan_finney_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_finney_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            None
        } else {
            Some(self.scan_finney_keyword_0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_finney_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_finney_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_finney_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_finney_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «FixedBytesType» = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32");

    #[allow(unused_assignments, unused_parens)]
    fn scan_fixed_bytes_type_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        self.scan_fixed_bytes_type_0_4_11(stream)
    }

    // «ForKeyword» = "for";

    #[allow(unused_assignments, unused_parens)]
    fn scan_for_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_for_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_for_keyword_0_4_11(stream)
    }

    // «FromKeyword» = "from";

    #[allow(unused_assignments, unused_parens)]
    fn scan_from_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_from_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_from_keyword_0_4_11(stream)
    }

    // «FunctionKeyword» = "function";

    #[allow(unused_assignments, unused_parens)]
    fn scan_function_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_function_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_function_keyword_0_4_11(stream)
    }

    // «GlobalKeyword» = "global";

    #[allow(unused_assignments, unused_parens)]
    fn scan_global_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_global_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_global_keyword_0_4_11(stream)
    }

    // «GreaterThan» = ">";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    #[inline]
    pub(crate) fn scan_greater_than(&self, stream: &mut Stream) -> bool {
        self.scan_greater_than_0_4_11(stream)
    }

    // «GreaterThanEqual» = ">=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '=')
    }

    #[inline]
    pub(crate) fn scan_greater_than_equal(&self, stream: &mut Stream) -> bool {
        self.scan_greater_than_equal_0_4_11(stream)
    }

    // «GreaterThanGreaterThan» = ">>";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_greater_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>'),
            scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
        )
    }

    #[inline]
    pub(crate) fn scan_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        self.scan_greater_than_greater_than_0_4_11(stream)
    }

    // «GreaterThanGreaterThanEqual» = ">>=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_greater_than_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '>', '>', '=')
    }

    #[inline]
    pub(crate) fn scan_greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        self.scan_greater_than_greater_than_equal_0_4_11(stream)
    }

    // «GreaterThanGreaterThanGreaterThan» = ">>>";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_greater_than_greater_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '>', '>', '>'),
            scan_chars!(stream, '=')
        )
    }

    #[inline]
    pub(crate) fn scan_greater_than_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        self.scan_greater_than_greater_than_greater_than_0_4_11(stream)
    }

    // «GreaterThanGreaterThanGreaterThanEqual» = ">>>=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_greater_than_greater_than_greater_than_equal_0_4_11(
        &self,
        stream: &mut Stream,
    ) -> bool {
        scan_chars!(stream, '>', '>', '>', '=')
    }

    #[inline]
    pub(crate) fn scan_greater_than_greater_than_greater_than_equal(
        &self,
        stream: &mut Stream,
    ) -> bool {
        self.scan_greater_than_greater_than_greater_than_equal_0_4_11(stream)
    }

    // (* v0.6.11 *)
    // «GweiKeyword» = "gwei";

    #[allow(unused_assignments, unused_parens)]
    fn scan_gwei_keyword_0_6_11(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_gwei_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_11 {
            Some(self.scan_gwei_keyword_0_6_11(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_gwei_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_gwei_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_gwei_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_gwei_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «HexByteEscape» = "x" «HexCharacter»{2,2};

    #[allow(unused_assignments, unused_parens)]
    fn scan_hex_byte_escape_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'x'),
            scan_repeated!(
                stream,
                scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                    || ('A' <= c && c <= 'F')
                    || ('a' <= c && c <= 'f')),
                2usize,
                2usize
            )
        )
    }

    #[inline]
    pub(crate) fn scan_hex_byte_escape(&self, stream: &mut Stream) -> bool {
        self.scan_hex_byte_escape_0_4_11(stream)
    }

    // «HexCharacter» = ("0"…"9") | ("a"…"f") | ("A"…"F");

    #[allow(unused_assignments, unused_parens)]
    fn scan_hex_character_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'F')
            || ('a' <= c && c <= 'f'))
    }

    #[inline]
    pub(crate) fn scan_hex_character(&self, stream: &mut Stream) -> bool {
        self.scan_hex_character_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «HexLiteral» = "0" ("x" | "X") «HexCharacter»+ ("_" «HexCharacter»+)*;

    #[allow(unused_assignments, unused_parens)]
    fn scan_hex_literal_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_hex_literal_0_5_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_hex_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.scan_hex_literal_0_5_0(stream)
        } else {
            self.scan_hex_literal_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn scan_hex_literal(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_hex_literal(stream)
    }

    // «HexStringLiteral» = "hex" (('"' «PossiblySeparatedPairsOfHexDigits»? '"') | ("'" «PossiblySeparatedPairsOfHexDigits»? "'"));

    #[allow(unused_assignments, unused_parens)]
    fn scan_hex_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x'),
            scan_choice!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '"'),
                    scan_optional!(
                        stream,
                        self.scan_possibly_separated_pairs_of_hex_digits(stream)
                    ),
                    scan_chars!(stream, '"')
                ),
                scan_sequence!(
                    scan_chars!(stream, '\''),
                    scan_optional!(
                        stream,
                        self.scan_possibly_separated_pairs_of_hex_digits(stream)
                    ),
                    scan_chars!(stream, '\'')
                )
            )
        )
    }

    #[inline]
    pub(crate) fn scan_hex_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_hex_string_literal_0_4_11(stream)
    }

    // «HoursKeyword» = "hours";

    #[allow(unused_assignments, unused_parens)]
    fn scan_hours_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_hours_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_hours_keyword_0_4_11(stream)
    }

    // «Identifier» = «RawIdentifier» - («NotAnIdentifierInAnyVersion» | «NotAnIdentifierInSomeVersions» | «FixedBytesType» | «SignedFixedType» | «UnsignedFixedType» | «SignedIntegerType» | «UnsignedIntegerType»);

    #[allow(unused_assignments, unused_parens)]
    fn scan_identifier_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.scan_raw_identifier(stream),
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
                self.scan_not_an_identifier_in_some_versions(stream),
                self.scan_fixed_bytes_type(stream),
                self.scan_signed_fixed_type(stream),
                self.scan_unsigned_fixed_type(stream),
                self.scan_signed_integer_type(stream),
                self.scan_unsigned_integer_type(stream)
            )
        )
    }

    #[inline]
    pub(crate) fn scan_identifier(&self, stream: &mut Stream) -> bool {
        self.scan_identifier_0_4_11(stream)
    }

    // «IdentifierPart» = «IdentifierStart» | ("0"…"9");

    #[allow(unused_assignments, unused_parens)]
    fn scan_identifier_part_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('0' <= c && c <= '9')
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    #[inline]
    pub(crate) fn scan_identifier_part(&self, stream: &mut Stream) -> bool {
        self.scan_identifier_part_0_4_11(stream)
    }

    // «IdentifierStart» = "_" | "$" | ("a"…"z") | ("A"…"Z");

    #[allow(unused_assignments, unused_parens)]
    fn scan_identifier_start_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_predicate!(stream, |c| c == '$'
            || ('A' <= c && c <= 'Z')
            || c == '_'
            || ('a' <= c && c <= 'z'))
    }

    #[inline]
    pub(crate) fn scan_identifier_start(&self, stream: &mut Stream) -> bool {
        self.scan_identifier_start_0_4_11(stream)
    }

    // «IfKeyword» = "if";

    #[allow(unused_assignments, unused_parens)]
    fn scan_if_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_if_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_if_keyword_0_4_11(stream)
    }

    // (* v0.6.4 *)
    // «ImmutableKeyword» = "immutable";

    #[allow(unused_assignments, unused_parens)]
    fn scan_immutable_keyword_0_6_4(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_immutable_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_4 {
            Some(self.scan_immutable_keyword_0_6_4(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_immutable_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_immutable_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_immutable_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_immutable_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «ImportKeyword» = "import";

    #[allow(unused_assignments, unused_parens)]
    fn scan_import_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_import_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_import_keyword_0_4_11(stream)
    }

    // «IndexedKeyword» = "indexed";

    #[allow(unused_assignments, unused_parens)]
    fn scan_indexed_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_indexed_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_indexed_keyword_0_4_11(stream)
    }

    // «InterfaceKeyword» = "interface";

    #[allow(unused_assignments, unused_parens)]
    fn scan_interface_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_interface_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_interface_keyword_0_4_11(stream)
    }

    // «InternalKeyword» = "internal";

    #[allow(unused_assignments, unused_parens)]
    fn scan_internal_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_internal_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_internal_keyword_0_4_11(stream)
    }

    // «IsKeyword» = "is";

    #[allow(unused_assignments, unused_parens)]
    fn scan_is_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_is_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_is_keyword_0_4_11(stream)
    }

    // «LeaveKeyword» = "leave";

    #[allow(unused_assignments, unused_parens)]
    fn scan_leave_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_leave_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_leave_keyword_0_4_11(stream)
    }

    // «LessThan» = "<";

    #[allow(unused_assignments, unused_parens)]
    fn scan_less_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<'),
            scan_predicate!(stream, |c| ('<' <= c && c <= '='))
        )
    }

    #[inline]
    pub(crate) fn scan_less_than(&self, stream: &mut Stream) -> bool {
        self.scan_less_than_0_4_11(stream)
    }

    // «LessThanEqual» = "<=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_less_than_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '=')
    }

    #[inline]
    pub(crate) fn scan_less_than_equal(&self, stream: &mut Stream) -> bool {
        self.scan_less_than_equal_0_4_11(stream)
    }

    // «LessThanLessThan» = "<<";

    #[allow(unused_assignments, unused_parens)]
    fn scan_less_than_less_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '<', '<'),
            scan_chars!(stream, '=')
        )
    }

    #[inline]
    pub(crate) fn scan_less_than_less_than(&self, stream: &mut Stream) -> bool {
        self.scan_less_than_less_than_0_4_11(stream)
    }

    // «LessThanLessThanEqual» = "<<=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_less_than_less_than_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '<', '<', '=')
    }

    #[inline]
    pub(crate) fn scan_less_than_less_than_equal(&self, stream: &mut Stream) -> bool {
        self.scan_less_than_less_than_equal_0_4_11(stream)
    }

    // «LetKeyword» = "let";

    #[allow(unused_assignments, unused_parens)]
    fn scan_let_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_let_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_let_keyword_0_4_11(stream)
    }

    // «LibraryKeyword» = "library";

    #[allow(unused_assignments, unused_parens)]
    fn scan_library_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_library_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_library_keyword_0_4_11(stream)
    }

    // «MappingKeyword» = "mapping";

    #[allow(unused_assignments, unused_parens)]
    fn scan_mapping_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_mapping_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_mapping_keyword_0_4_11(stream)
    }

    // «MemoryKeyword» = "memory";

    #[allow(unused_assignments, unused_parens)]
    fn scan_memory_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_memory_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_memory_keyword_0_4_11(stream)
    }

    // «Minus» = "-";

    #[allow(unused_assignments, unused_parens)]
    fn scan_minus_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '-'),
            scan_predicate!(stream, |c| c == '-' || ('=' <= c && c <= '>'))
        )
    }

    #[inline]
    pub(crate) fn scan_minus(&self, stream: &mut Stream) -> bool {
        self.scan_minus_0_4_11(stream)
    }

    // «MinusEqual» = "-=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_minus_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '=')
    }

    #[inline]
    pub(crate) fn scan_minus_equal(&self, stream: &mut Stream) -> bool {
        self.scan_minus_equal_0_4_11(stream)
    }

    // «MinusGreaterThan» = "->";

    #[allow(unused_assignments, unused_parens)]
    fn scan_minus_greater_than_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '>')
    }

    #[inline]
    pub(crate) fn scan_minus_greater_than(&self, stream: &mut Stream) -> bool {
        self.scan_minus_greater_than_0_4_11(stream)
    }

    // «MinusMinus» = "--";

    #[allow(unused_assignments, unused_parens)]
    fn scan_minus_minus_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '-', '-')
    }

    #[inline]
    pub(crate) fn scan_minus_minus(&self, stream: &mut Stream) -> bool {
        self.scan_minus_minus_0_4_11(stream)
    }

    // «MinutesKeyword» = "minutes";

    #[allow(unused_assignments, unused_parens)]
    fn scan_minutes_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_minutes_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_minutes_keyword_0_4_11(stream)
    }

    // «ModifierKeyword» = "modifier";

    #[allow(unused_assignments, unused_parens)]
    fn scan_modifier_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_modifier_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_modifier_keyword_0_4_11(stream)
    }

    // «MultilineComment» = "/*" (!"*" | ("*" !"/"))* "*/";

    #[allow(unused_assignments, unused_parens)]
    fn scan_multiline_comment_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_multiline_comment(&self, stream: &mut Stream) -> bool {
        self.scan_multiline_comment_0_4_11(stream)
    }

    // «NewKeyword» = "new";

    #[allow(unused_assignments, unused_parens)]
    fn scan_new_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_new_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_new_keyword_0_4_11(stream)
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_not_an_identifier_in_any_version_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_not_an_identifier_in_any_version(&self, stream: &mut Stream) -> bool {
        self.scan_not_an_identifier_in_any_version_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «NotAnIdentifierInSomeVersions» = "finney" | "szabo";

    #[allow(unused_assignments, unused_parens)]
    fn scan_not_an_identifier_in_some_versions_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_not_an_identifier_in_some_versions_0_5_0(&self, stream: &mut Stream) -> bool {
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_not_an_identifier_in_some_versions_0_6_0(&self, stream: &mut Stream) -> bool {
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_not_an_identifier_in_some_versions_0_7_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_not_an_identifier_in_some_versions(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.scan_not_an_identifier_in_some_versions_0_7_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.scan_not_an_identifier_in_some_versions_0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.scan_not_an_identifier_in_some_versions_0_5_0(stream)
        } else {
            self.scan_not_an_identifier_in_some_versions_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn scan_not_an_identifier_in_some_versions(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_not_an_identifier_in_some_versions(stream)
    }

    // «OpenBrace» = "{";

    #[allow(unused_assignments, unused_parens)]
    fn scan_open_brace_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '{')
    }

    #[inline]
    pub(crate) fn scan_open_brace(&self, stream: &mut Stream) -> bool {
        self.scan_open_brace_0_4_11(stream)
    }

    // «OpenBracket» = "[";

    #[allow(unused_assignments, unused_parens)]
    fn scan_open_bracket_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '[')
    }

    #[inline]
    pub(crate) fn scan_open_bracket(&self, stream: &mut Stream) -> bool {
        self.scan_open_bracket_0_4_11(stream)
    }

    // «OpenParen» = "(";

    #[allow(unused_assignments, unused_parens)]
    fn scan_open_paren_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '(')
    }

    #[inline]
    pub(crate) fn scan_open_paren(&self, stream: &mut Stream) -> bool {
        self.scan_open_paren_0_4_11(stream)
    }

    // «OverrideKeyword» = "override";

    #[allow(unused_assignments, unused_parens)]
    fn scan_override_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_override_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_override_keyword_0_4_11(stream)
    }

    // «PayableKeyword» = "payable";

    #[allow(unused_assignments, unused_parens)]
    fn scan_payable_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_payable_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_payable_keyword_0_4_11(stream)
    }

    // «Percent» = "%";

    #[allow(unused_assignments, unused_parens)]
    fn scan_percent_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '%'), scan_chars!(stream, '='))
    }

    #[inline]
    pub(crate) fn scan_percent(&self, stream: &mut Stream) -> bool {
        self.scan_percent_0_4_11(stream)
    }

    // «PercentEqual» = "%=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_percent_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '%', '=')
    }

    #[inline]
    pub(crate) fn scan_percent_equal(&self, stream: &mut Stream) -> bool {
        self.scan_percent_equal_0_4_11(stream)
    }

    // «Period» = ".";

    #[allow(unused_assignments, unused_parens)]
    fn scan_period_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '.')
    }

    #[inline]
    pub(crate) fn scan_period(&self, stream: &mut Stream) -> bool {
        self.scan_period_0_4_11(stream)
    }

    // «Plus» = "+";

    #[allow(unused_assignments, unused_parens)]
    fn scan_plus_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '+'),
            scan_predicate!(stream, |c| c == '+' || c == '=')
        )
    }

    #[inline]
    pub(crate) fn scan_plus(&self, stream: &mut Stream) -> bool {
        self.scan_plus_0_4_11(stream)
    }

    // «PlusEqual» = "+=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_plus_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '=')
    }

    #[inline]
    pub(crate) fn scan_plus_equal(&self, stream: &mut Stream) -> bool {
        self.scan_plus_equal_0_4_11(stream)
    }

    // «PlusPlus» = "++";

    #[allow(unused_assignments, unused_parens)]
    fn scan_plus_plus_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '+', '+')
    }

    #[inline]
    pub(crate) fn scan_plus_plus(&self, stream: &mut Stream) -> bool {
        self.scan_plus_plus_0_4_11(stream)
    }

    // «PossiblySeparatedPairsOfHexDigits» = «HexCharacter»{2,2} ("_"? «HexCharacter»{2,2})*;

    #[allow(unused_assignments, unused_parens)]
    fn scan_possibly_separated_pairs_of_hex_digits_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_repeated!(
                stream,
                scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                    || ('A' <= c && c <= 'F')
                    || ('a' <= c && c <= 'f')),
                2usize,
                2usize
            ),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_optional!(stream, scan_chars!(stream, '_')),
                    scan_repeated!(
                        stream,
                        scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                            || ('A' <= c && c <= 'F')
                            || ('a' <= c && c <= 'f')),
                        2usize,
                        2usize
                    )
                )
            )
        )
    }

    #[inline]
    pub(crate) fn scan_possibly_separated_pairs_of_hex_digits(&self, stream: &mut Stream) -> bool {
        self.scan_possibly_separated_pairs_of_hex_digits_0_4_11(stream)
    }

    // «PragmaKeyword» = "pragma";

    #[allow(unused_assignments, unused_parens)]
    fn scan_pragma_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_pragma_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_pragma_keyword_0_4_11(stream)
    }

    // «PrivateKeyword» = "private";

    #[allow(unused_assignments, unused_parens)]
    fn scan_private_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_private_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_private_keyword_0_4_11(stream)
    }

    // «PublicKeyword» = "public";

    #[allow(unused_assignments, unused_parens)]
    fn scan_public_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_public_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_public_keyword_0_4_11(stream)
    }

    // «PureKeyword» = "pure";

    #[allow(unused_assignments, unused_parens)]
    fn scan_pure_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_pure_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_pure_keyword_0_4_11(stream)
    }

    // «QuestionMark» = "?";

    #[allow(unused_assignments, unused_parens)]
    fn scan_question_mark_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '?')
    }

    #[inline]
    pub(crate) fn scan_question_mark(&self, stream: &mut Stream) -> bool {
        self.scan_question_mark_0_4_11(stream)
    }

    // «RawIdentifier» = «IdentifierStart» «IdentifierPart»*;

    #[allow(unused_assignments, unused_parens)]
    fn scan_raw_identifier_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_raw_identifier(&self, stream: &mut Stream) -> bool {
        self.scan_raw_identifier_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «ReceiveKeyword» = "receive";

    #[allow(unused_assignments, unused_parens)]
    fn scan_receive_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_receive_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_receive_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_receive_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_receive_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_receive_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_receive_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «ReturnKeyword» = "return";

    #[allow(unused_assignments, unused_parens)]
    fn scan_return_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_return_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_return_keyword_0_4_11(stream)
    }

    // «ReturnsKeyword» = "returns";

    #[allow(unused_assignments, unused_parens)]
    fn scan_returns_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_returns_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_returns_keyword_0_4_11(stream)
    }

    // «RevertKeyword» = "revert";

    #[allow(unused_assignments, unused_parens)]
    fn scan_revert_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_revert_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_revert_keyword_0_4_11(stream)
    }

    // «SecondsKeyword» = "seconds";

    #[allow(unused_assignments, unused_parens)]
    fn scan_seconds_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_seconds_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_seconds_keyword_0_4_11(stream)
    }

    // «Semicolon» = ";";

    #[allow(unused_assignments, unused_parens)]
    fn scan_semicolon_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, ';')
    }

    #[inline]
    pub(crate) fn scan_semicolon(&self, stream: &mut Stream) -> bool {
        self.scan_semicolon_0_4_11(stream)
    }

    // «SignedFixedType» = "fixed" (("0"…"9")+ "x" ("0"…"9")+)?;

    #[allow(unused_assignments, unused_parens)]
    fn scan_signed_fixed_type_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_signed_fixed_type(&self, stream: &mut Stream) -> bool {
        self.scan_signed_fixed_type_0_4_11(stream)
    }

    // «SignedIntegerType» = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;

    #[allow(unused_assignments, unused_parens)]
    fn scan_signed_integer_type_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_signed_integer_type(&self, stream: &mut Stream) -> bool {
        self.scan_signed_integer_type_0_4_11(stream)
    }

    // «SingleLineComment» = "//" (!("\r" | "\n"))*;

    #[allow(unused_assignments, unused_parens)]
    fn scan_single_line_comment_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '/'),
            scan_zero_or_more!(stream, scan_predicate!(stream, |c| c != '\n' && c != '\r'))
        )
    }

    #[inline]
    pub(crate) fn scan_single_line_comment(&self, stream: &mut Stream) -> bool {
        self.scan_single_line_comment_0_4_11(stream)
    }

    // «SingleQuotedAsciiStringLiteral» = "'" («EscapeSequence» | ((" "…"~") - ("'" | "\\")))* "'";

    #[allow(unused_assignments, unused_parens)]
    fn scan_single_quoted_ascii_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.scan_escape_sequence(stream),
                    scan_predicate!(stream, |c| (' ' <= c && c <= '&')
                        || ('(' <= c && c <= '[')
                        || (']' <= c && c <= '~'))
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    #[inline]
    pub(crate) fn scan_single_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_single_quoted_ascii_string_literal_0_4_11(stream)
    }

    // «SingleQuotedUnicodeStringLiteral» = "unicode'" («EscapeSequence» | !("'" | "\\" | "\n" | "\r"))* "'";

    #[allow(unused_assignments, unused_parens)]
    fn scan_single_quoted_unicode_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.scan_escape_sequence(stream),
                    scan_predicate!(stream, |c| c != '\n' && c != '\r' && c != '\'' && c != '\\')
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    #[inline]
    pub(crate) fn scan_single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_single_quoted_unicode_string_literal_0_4_11(stream)
    }

    // «Slash» = "/";

    #[allow(unused_assignments, unused_parens)]
    fn scan_slash_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(stream, scan_chars!(stream, '/'), scan_chars!(stream, '='))
    }

    #[inline]
    pub(crate) fn scan_slash(&self, stream: &mut Stream) -> bool {
        self.scan_slash_0_4_11(stream)
    }

    // «SlashEqual» = "/=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_slash_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '/', '=')
    }

    #[inline]
    pub(crate) fn scan_slash_equal(&self, stream: &mut Stream) -> bool {
        self.scan_slash_equal_0_4_11(stream)
    }

    // «SolidityKeyword» = "solidity";

    #[allow(unused_assignments, unused_parens)]
    fn scan_solidity_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_solidity_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_solidity_keyword_0_4_11(stream)
    }

    // «Star» = "*";

    #[allow(unused_assignments, unused_parens)]
    fn scan_star_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_not_followed_by!(
            stream,
            scan_chars!(stream, '*'),
            scan_predicate!(stream, |c| c == '*' || c == '=')
        )
    }

    #[inline]
    pub(crate) fn scan_star(&self, stream: &mut Stream) -> bool {
        self.scan_star_0_4_11(stream)
    }

    // «StarEqual» = "*=";

    #[allow(unused_assignments, unused_parens)]
    fn scan_star_equal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '=')
    }

    #[inline]
    pub(crate) fn scan_star_equal(&self, stream: &mut Stream) -> bool {
        self.scan_star_equal_0_4_11(stream)
    }

    // «StarStar» = "**";

    #[allow(unused_assignments, unused_parens)]
    fn scan_star_star_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '*', '*')
    }

    #[inline]
    pub(crate) fn scan_star_star(&self, stream: &mut Stream) -> bool {
        self.scan_star_star_0_4_11(stream)
    }

    // «StorageKeyword» = "storage";

    #[allow(unused_assignments, unused_parens)]
    fn scan_storage_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_storage_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_storage_keyword_0_4_11(stream)
    }

    // «StringKeyword» = "string";

    #[allow(unused_assignments, unused_parens)]
    fn scan_string_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_string_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_string_keyword_0_4_11(stream)
    }

    // «StructKeyword» = "struct";

    #[allow(unused_assignments, unused_parens)]
    fn scan_struct_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_struct_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_struct_keyword_0_4_11(stream)
    }

    // «SwitchKeyword» = "switch";

    #[allow(unused_assignments, unused_parens)]
    fn scan_switch_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_switch_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_switch_keyword_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «SzaboKeyword» = "szabo";

    #[allow(unused_assignments, unused_parens)]
    fn scan_szabo_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_szabo_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            None
        } else {
            Some(self.scan_szabo_keyword_0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_szabo_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_szabo_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_szabo_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_szabo_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «ThrowKeyword» = "throw";

    #[allow(unused_assignments, unused_parens)]
    fn scan_throw_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_throw_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_throw_keyword_0_4_11(stream)
    }

    // «Tilde» = "~";

    #[allow(unused_assignments, unused_parens)]
    fn scan_tilde_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, '~')
    }

    #[inline]
    pub(crate) fn scan_tilde(&self, stream: &mut Stream) -> bool {
        self.scan_tilde_0_4_11(stream)
    }

    // «TrueKeyword» = "true";

    #[allow(unused_assignments, unused_parens)]
    fn scan_true_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_true_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_true_keyword_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «TryKeyword» = "try";

    #[allow(unused_assignments, unused_parens)]
    fn scan_try_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_try_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_try_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_try_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_try_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_try_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_try_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.5.3 *)
    // «TypeKeyword» = "type";

    #[allow(unused_assignments, unused_parens)]
    fn scan_type_keyword_0_5_3(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_type_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            Some(self.scan_type_keyword_0_5_3(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_type_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_type_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_type_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_type_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.0 *)
    // «UncheckedKeyword» = "unchecked";

    #[allow(unused_assignments, unused_parens)]
    fn scan_unchecked_keyword_0_8_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_unchecked_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            Some(self.scan_unchecked_keyword_0_8_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_unchecked_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_unchecked_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_unchecked_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_unchecked_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «UnicodeEscape» = "u" «HexCharacter»{4,4};

    #[allow(unused_assignments, unused_parens)]
    fn scan_unicode_escape_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            scan_repeated!(
                stream,
                scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                    || ('A' <= c && c <= 'F')
                    || ('a' <= c && c <= 'f')),
                4usize,
                4usize
            )
        )
    }

    #[inline]
    pub(crate) fn scan_unicode_escape(&self, stream: &mut Stream) -> bool {
        self.scan_unicode_escape_0_4_11(stream)
    }

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral»;

    #[allow(unused_assignments, unused_parens)]
    fn scan_unicode_string_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.scan_single_quoted_unicode_string_literal(stream),
            self.scan_double_quoted_unicode_string_literal(stream)
        )
    }

    #[inline]
    pub(crate) fn scan_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        self.scan_unicode_string_literal_0_4_11(stream)
    }

    // «UnsignedFixedType» = "u" «SignedFixedType»;

    #[allow(unused_assignments, unused_parens)]
    fn scan_unsigned_fixed_type_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            self.scan_signed_fixed_type(stream)
        )
    }

    #[inline]
    pub(crate) fn scan_unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        self.scan_unsigned_fixed_type_0_4_11(stream)
    }

    // «UnsignedIntegerType» = "u" «SignedIntegerType»;

    #[allow(unused_assignments, unused_parens)]
    fn scan_unsigned_integer_type_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            self.scan_signed_integer_type(stream)
        )
    }

    #[inline]
    pub(crate) fn scan_unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        self.scan_unsigned_integer_type_0_4_11(stream)
    }

    // «UsingKeyword» = "using";

    #[allow(unused_assignments, unused_parens)]
    fn scan_using_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_using_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_using_keyword_0_4_11(stream)
    }

    // (* v0.4.11 *)
    // «VarKeyword» = "var";

    #[allow(unused_assignments, unused_parens)]
    fn scan_var_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_var_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.scan_var_keyword_0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_var_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_var_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_var_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_var_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «VersionPragmaValue» = (("0"…"9") | "x" | "X" | "*")+;

    #[allow(unused_assignments, unused_parens)]
    fn scan_version_pragma_value_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_predicate!(stream, |c| c == '*'
                || ('0' <= c && c <= '9')
                || c == 'X'
                || c == 'x')
        )
    }

    #[inline]
    pub(crate) fn scan_version_pragma_value(&self, stream: &mut Stream) -> bool {
        self.scan_version_pragma_value_0_4_11(stream)
    }

    // «ViewKeyword» = "view";

    #[allow(unused_assignments, unused_parens)]
    fn scan_view_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_view_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_view_keyword_0_4_11(stream)
    }

    // (* v0.6.0 *)
    // «VirtualKeyword» = "virtual";

    #[allow(unused_assignments, unused_parens)]
    fn scan_virtual_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
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

    fn dispatch_scan_virtual_keyword(&self, stream: &mut Stream) -> Option<bool> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.scan_virtual_keyword_0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn maybe_scan_virtual_keyword(&self, stream: &mut Stream) -> Option<bool> {
        self.dispatch_scan_virtual_keyword(stream)
    }

    #[inline]
    pub(crate) fn scan_virtual_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_virtual_keyword(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «WeeksKeyword» = "weeks";

    #[allow(unused_assignments, unused_parens)]
    fn scan_weeks_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_weeks_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_weeks_keyword_0_4_11(stream)
    }

    // «WeiKeyword» = "wei";

    #[allow(unused_assignments, unused_parens)]
    fn scan_wei_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_wei_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_wei_keyword_0_4_11(stream)
    }

    // «WhileKeyword» = "while";

    #[allow(unused_assignments, unused_parens)]
    fn scan_while_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_while_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_while_keyword_0_4_11(stream)
    }

    // «Whitespace» = (" " | "\t")+;

    #[allow(unused_assignments, unused_parens)]
    fn scan_whitespace_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\t' || c == ' '))
    }

    #[inline]
    pub(crate) fn scan_whitespace(&self, stream: &mut Stream) -> bool {
        self.scan_whitespace_0_4_11(stream)
    }

    // «YearsKeyword» = "years";

    #[allow(unused_assignments, unused_parens)]
    fn scan_years_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_years_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_years_keyword_0_4_11(stream)
    }

    // «YulDecimalLiteral» = "0" | (("1"…"9") ("0"…"9")*);

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_decimal_literal_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '0'),
            scan_sequence!(
                scan_predicate!(stream, |c| ('1' <= c && c <= '9')),
                scan_zero_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
            )
        )
    }

    #[inline]
    pub(crate) fn scan_yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        self.scan_yul_decimal_literal_0_4_11(stream)
    }

    // «YulHexLiteral» = "0x" «HexCharacter»+;

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_hex_literal_0_4_11(&self, stream: &mut Stream) -> bool {
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

    #[inline]
    pub(crate) fn scan_yul_hex_literal(&self, stream: &mut Stream) -> bool {
        self.scan_yul_hex_literal_0_4_11(stream)
    }

    // «YulIdentifier» = «RawIdentifier» - («YulKeyword» | «YulReservedKeyword»);

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_identifier_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_difference!(
            stream,
            self.scan_raw_identifier(stream),
            scan_choice!(
                stream,
                self.scan_yul_keyword(stream),
                scan_chars!(stream, 'h', 'e', 'x')
            )
        )
    }

    #[inline]
    pub(crate) fn scan_yul_identifier(&self, stream: &mut Stream) -> bool {
        self.scan_yul_identifier_0_4_11(stream)
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.scan_break_keyword(stream),
            self.scan_case_keyword(stream),
            self.scan_continue_keyword(stream),
            self.scan_default_keyword(stream),
            self.scan_false_keyword(stream),
            self.scan_for_keyword(stream),
            self.scan_function_keyword(stream),
            self.scan_if_keyword(stream),
            self.scan_let_keyword(stream),
            self.scan_switch_keyword(stream),
            self.scan_true_keyword(stream)
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

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_keyword_0_6_0(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.scan_break_keyword(stream),
            self.scan_case_keyword(stream),
            self.scan_continue_keyword(stream),
            self.scan_default_keyword(stream),
            self.scan_false_keyword(stream),
            self.scan_for_keyword(stream),
            self.scan_function_keyword(stream),
            self.scan_if_keyword(stream),
            self.scan_leave_keyword(stream),
            self.scan_let_keyword(stream),
            self.scan_switch_keyword(stream),
            self.scan_true_keyword(stream)
        )
    }

    fn dispatch_scan_yul_keyword(&self, stream: &mut Stream) -> bool {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.scan_yul_keyword_0_6_0(stream)
        } else {
            self.scan_yul_keyword_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn scan_yul_keyword(&self, stream: &mut Stream) -> bool {
        self.dispatch_scan_yul_keyword(stream)
    }

    // «YulReservedKeyword» = "hex";

    #[allow(unused_assignments, unused_parens)]
    fn scan_yul_reserved_keyword_0_4_11(&self, stream: &mut Stream) -> bool {
        scan_chars!(stream, 'h', 'e', 'x')
    }

    #[inline]
    pub(crate) fn scan_yul_reserved_keyword(&self, stream: &mut Stream) -> bool {
        self.scan_yul_reserved_keyword_0_4_11(stream)
    }
}
