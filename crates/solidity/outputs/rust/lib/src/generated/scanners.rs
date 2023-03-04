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
    // «AbicoderKeyword» = "abicoder" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_abicoder_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «AbstractKeyword» = "abstract" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_abstract_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «AddressKeyword» = "address" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_address_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Ampersand» = '&' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ampersand(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '&'),
                scan_predicate!(stream, |c| c == '&' || c == '=')
            )
        }
    }

    // «AmpersandAmpersand» = "&&" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ampersand_ampersand(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '&', '&')
        }
    }

    // «AmpersandEqual» = "&=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ampersand_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '&', '=')
        }
    }

    // «AnonymousKeyword» = "anonymous" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_anonymous_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «AsKeyword» = "as" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_as_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «AsciiEscape» = 'n' | 'r' | 't' | "\'" | '"' | '\\' | '\u{a}' | '\u{d}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ascii_escape(&self, stream: &mut Stream) -> bool {
        {
            scan_predicate!(stream, |c| c == '\n'
                || c == '\r'
                || c == '"'
                || c == '\''
                || c == '\\'
                || c == 'n'
                || c == 'r'
                || c == 't')
        }
    }

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_choice!(
                stream,
                self.scan_single_quoted_ascii_string_literal(stream),
                self.scan_double_quoted_ascii_string_literal(stream)
            )
        }
    }

    // «AssemblyKeyword» = "assembly" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_assembly_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Bang» = '!' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bang(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(stream, scan_chars!(stream, '!'), scan_chars!(stream, '='))
        }
    }

    // «BangEqual» = "!=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bang_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '!', '=')
        }
    }

    // «Bar» = '|' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bar(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '|'),
                scan_predicate!(stream, |c| c == '=' || c == '|')
            )
        }
    }

    // «BarBar» = "||" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bar_bar(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '|', '|')
        }
    }

    // «BarEqual» = "|=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bar_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '|', '=')
        }
    }

    // «BoolKeyword» = "bool" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_bool_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «BreakKeyword» = "break" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_break_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «CalldataKeyword» = "calldata" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_calldata_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Caret» = '^' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_caret(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(stream, scan_chars!(stream, '^'), scan_chars!(stream, '='))
        }
    }

    // «CaretEqual» = "^=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_caret_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '^', '=')
        }
    }

    // «CaseKeyword» = "case" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_case_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «CatchKeyword» = "catch" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_catch_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «CloseBrace» = '}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_close_brace(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '}')
        }
    }

    // «CloseBracket» = ']' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_close_bracket(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, ']')
        }
    }

    // «CloseParen» = ')' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_close_paren(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, ')')
        }
    }

    // «Colon» = ':' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_colon(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(stream, scan_chars!(stream, ':'), scan_chars!(stream, '='))
        }
    }

    // «ColonEqual» = ":=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_colon_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, ':', '=')
        }
    }

    // «Comma» = ',' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_comma(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, ',')
        }
    }

    // «ConstantKeyword» = "constant" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_constant_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ConstructorKeyword» = "constructor" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_constructor_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ContinueKeyword» = "continue" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_continue_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ContractKeyword» = "contract" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_contract_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DaysKeyword» = "days" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_days_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalNumber» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_decimal_exponent(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_predicate!(stream, |c| c == 'E' || c == 'e'),
                scan_optional!(stream, scan_chars!(stream, '-')),
                self.scan_decimal_number(stream)
            )
        }
    }

    // «DecimalLiteral» = ( «DecimalNumber» [ '.' «DecimalNumber» ] | '.' «DecimalNumber» ) [ «DecimalExponent» ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_decimal_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_choice!(
                    stream,
                    scan_sequence!(
                        self.scan_decimal_number(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(
                                scan_chars!(stream, '.'),
                                self.scan_decimal_number(stream)
                            )
                        )
                    ),
                    scan_sequence!(scan_chars!(stream, '.'), self.scan_decimal_number(stream))
                ),
                scan_optional!(stream, self.scan_decimal_exponent(stream))
            )
        }
    }

    // «DecimalNumber» = 1…{ '0'…'9' } { '_' 1…{ '0'…'9' } } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_decimal_number(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9'))),
                scan_zero_or_more!(
                    stream,
                    scan_sequence!(
                        scan_chars!(stream, '_'),
                        scan_one_or_more!(
                            stream,
                            scan_predicate!(stream, |c| ('0' <= c && c <= '9'))
                        )
                    )
                )
            )
        }
    }

    // «DefaultKeyword» = "default" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_default_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DeleteKeyword» = "delete" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_delete_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DoKeyword» = "do" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_do_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' { «EscapeSequence» | '\u{20}'…'~' - ( '"' | '\\' ) } '"' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { «EscapeSequence» | ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } '"' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.scan_escape_sequence(stream),
                        scan_predicate!(stream, |c| c != '\n'
                            && c != '\r'
                            && c != '"'
                            && c != '\\')
                    )
                ),
                scan_chars!(stream, '"')
            )
        }
    }

    // «ElseKeyword» = "else" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_else_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «EmitKeyword» = "emit" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_emit_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «EndOfLine» = 1…{ '\u{d}' | '\u{a}' } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_end_of_line(&self, stream: &mut Stream) -> bool {
        {
            scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\n' || c == '\r'))
        }
    }

    // «EnumKeyword» = "enum" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_enum_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Equal» = '=' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '='),
                scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
            )
        }
    }

    // «EqualEqual» = "==" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_equal_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '=', '=')
        }
    }

    // «EqualGreaterThan» = "=>" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_equal_greater_than(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '=', '>')
        }
    }

    // «ErrorKeyword» = "error" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_error_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_escape_sequence(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «EtherKeyword» = "ether" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_ether_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «EventKeyword» = "event" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_event_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Evmasm» = '"evmasm"' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_evmasm(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '"', 'e', 'v', 'm', 'a', 's', 'm', '"')
        }
    }

    // «ExperimentalKeyword» = "experimental" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_experimental_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ExternalKeyword» = "external" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_external_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FallbackKeyword» = "fallback" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_fallback_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FalseKeyword» = "false" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_false_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FinneyKeyword» = "finney" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_finney_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FixedBytesType» = "bytes" ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32" ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ForKeyword» = "for" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_for_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FromKeyword» = "from" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_from_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «FunctionKeyword» = "function" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_function_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «GlobalKeyword» = "global" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_global_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «GreaterThan» = '>' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '>'),
                scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
            )
        }
    }

    // «GreaterThanEqual» = ">=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '>', '=')
        }
    }

    // «GreaterThanGreaterThan» = ">>" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '>', '>'),
                scan_predicate!(stream, |c| ('=' <= c && c <= '>'))
            )
        }
    }

    // «GreaterThanGreaterThanEqual» = ">>=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than_greater_than_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '>', '>', '=')
        }
    }

    // «GreaterThanGreaterThanGreaterThan» = ">>>" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than_greater_than_greater_than(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '>', '>', '>'),
                scan_chars!(stream, '=')
            )
        }
    }

    // «GreaterThanGreaterThanGreaterThanEqual» = ">>>=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_greater_than_greater_than_greater_than_equal(
        &self,
        stream: &mut Stream,
    ) -> bool {
        {
            scan_chars!(stream, '>', '>', '>', '=')
        }
    }

    // «GweiKeyword» = "gwei" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_gwei_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «HexByteEscape» = 'x' 2…2{ «HexCharacter» } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_hex_byte_escape(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_hex_character(&self, stream: &mut Stream) -> bool {
        {
            scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'F')
                || ('a' <= c && c <= 'f'))
        }
    }

    // «HexLiteral» = "0x" 1…{ «HexCharacter» } { '_' 1…{ «HexCharacter» } } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_hex_literal(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «HexStringLiteral» = "hex" ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | "\'" [ «PossiblySeparatedPairsOfHexDigits» ] "\'" ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_hex_string_literal(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «HoursKeyword» = "hours" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_hours_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Identifier» = «RawIdentifier» - «Keyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_identifier(&self, stream: &mut Stream) -> bool {
        {
            scan_difference!(
                stream,
                self.scan_raw_identifier(stream),
                self.scan_keyword(stream)
            )
        }
    }

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_identifier_part(&self, stream: &mut Stream) -> bool {
        {
            scan_predicate!(stream, |c| c == '$'
                || ('0' <= c && c <= '9')
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        }
    }

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_identifier_start(&self, stream: &mut Stream) -> bool {
        {
            scan_predicate!(stream, |c| c == '$'
                || ('A' <= c && c <= 'Z')
                || c == '_'
                || ('a' <= c && c <= 'z'))
        }
    }

    // «IfKeyword» = "if" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_if_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ImmutableKeyword» = "immutable" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_immutable_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ImportKeyword» = "import" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_import_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «IndexedKeyword» = "indexed" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_indexed_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «InterfaceKeyword» = "interface" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_interface_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «InternalKeyword» = "internal" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_internal_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «IsKeyword» = "is" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_is_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Keyword» = "true" | "false" | «FixedBytesType» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | "days" | "ether" | "finney" | "gwei" | "hours" | "minutes" | "seconds" | "szabo" | "weeks" | "wei" | "years" | "abstract" | "address" | "anonymous" | "as" | "assembly" | "bool" | "break" | "calldata" | "catch" | "constant" | "constructor" | "continue" | "contract" | "delete" | "do" | "else" | "emit" | "enum" | "event" | "external" | "fallback" | "false" | "fixed" | "for" | "function" | "hex" | "if" | "immutable" | "import" | "indexed" | "interface" | "internal" | "is" | "library" | "mapping" | "memory" | "modifier" | "new" | "override" | "payable" | "pragma" | "private" | "public" | "pure" | "receive" | "return" | "returns" | "storage" | "string" | "struct" | "true" | "try" | "type" | "ufixed" | "unchecked" | "using" | "view" | "virtual" | "while" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_keyword(&self, stream: &mut Stream) -> bool {
        {
            scan_choice!(
                stream,
                scan_trie!(
                    stream,
                    'f' + scan_chars!(stream, 'a', 'l', 's', 'e'),
                    't' + scan_chars!(stream, 'r', 'u', 'e')
                ),
                self.scan_fixed_bytes_type(stream),
                scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'f' + scan_chars!(stream, 't', 'e', 'r'),
                        'l' + scan_chars!(stream, 'i', 'a', 's'),
                        'p' + scan_chars!(stream, 'p', 'l', 'y'),
                        'u' + scan_chars!(stream, 't', 'o')
                    ),
                    'b' + scan_chars!(stream, 'y', 't', 'e'),
                    'c' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 's', 'e'),
                        'o' + scan_chars!(stream, 'p', 'y', 'o', 'f')
                    ),
                    'd' + scan_sequence!(
                        scan_chars!(stream, 'e', 'f'),
                        scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'u', 'l', 't'),
                            'i' + scan_chars!(stream, 'n', 'e')
                        )
                    ),
                    'f' + scan_chars!(stream, 'i', 'n', 'a', 'l'),
                    'i' + scan_trie!(
                        stream,
                        'm' + scan_chars!(stream, 'p', 'l', 'e', 'm', 'e', 'n', 't', 's'),
                        'n' + scan_trie!(stream, EMPTY, 'l' + scan_chars!(stream, 'i', 'n', 'e'))
                    ),
                    'l' + scan_chars!(stream, 'e', 't'),
                    'm' + scan_trie!(
                        stream,
                        'a' + scan_trie!(
                            stream,
                            'c' + scan_chars!(stream, 'r', 'o'),
                            't' + scan_chars!(stream, 'c', 'h')
                        ),
                        'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
                    ),
                    'n' + scan_chars!(stream, 'u', 'l', 'l'),
                    'o' + scan_chars!(stream, 'f'),
                    'p' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'r', 't', 'i', 'a', 'l'),
                        'r' + scan_chars!(stream, 'o', 'm', 'i', 's', 'e')
                    ),
                    'r' + scan_sequence!(
                        scan_chars!(stream, 'e'),
                        scan_trie!(
                            stream,
                            'f' + scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e'),
                            'l' + scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e')
                        )
                    ),
                    's' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 'a', 'l', 'e', 'd'),
                        'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                        't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                        'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's'),
                        'w' + scan_chars!(stream, 'i', 't', 'c', 'h')
                    ),
                    't' + scan_sequence!(
                        scan_chars!(stream, 'y', 'p', 'e'),
                        scan_trie!(
                            stream,
                            'd' + scan_chars!(stream, 'e', 'f'),
                            'o' + scan_chars!(stream, 'f')
                        )
                    ),
                    'v' + scan_chars!(stream, 'a', 'r')
                ),
                self.scan_signed_integer_type(stream),
                self.scan_unsigned_integer_type(stream),
                scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'b' + scan_chars!(stream, 's', 't', 'r', 'a', 'c', 't'),
                        'd' + scan_chars!(stream, 'd', 'r', 'e', 's', 's'),
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
                        'r' + scan_chars!(stream, 'e', 'a', 'k')
                    ),
                    'c' + scan_trie!(
                        stream,
                        'a' + scan_trie!(
                            stream,
                            'l' + scan_chars!(stream, 'l', 'd', 'a', 't', 'a'),
                            't' + scan_chars!(stream, 'c', 'h')
                        ),
                        'o' + scan_sequence!(
                            scan_chars!(stream, 'n'),
                            scan_trie!(
                                stream,
                                's' + scan_sequence!(
                                    scan_chars!(stream, 't'),
                                    scan_trie!(
                                        stream,
                                        'a' + scan_chars!(stream, 'n', 't'),
                                        'r' + scan_chars!(stream, 'u', 'c', 't', 'o', 'r')
                                    )
                                ),
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
                        'e' + scan_chars!(stream, 'l', 'e', 't', 'e')
                    ),
                    'e' + scan_trie!(
                        stream,
                        'l' + scan_chars!(stream, 's', 'e'),
                        'm' + scan_chars!(stream, 'i', 't'),
                        'n' + scan_chars!(stream, 'u', 'm'),
                        't' + scan_chars!(stream, 'h', 'e', 'r'),
                        'v' + scan_chars!(stream, 'e', 'n', 't'),
                        'x' + scan_chars!(stream, 't', 'e', 'r', 'n', 'a', 'l')
                    ),
                    'f' + scan_trie!(
                        stream,
                        'a' + scan_sequence!(
                            scan_chars!(stream, 'l'),
                            scan_trie!(
                                stream,
                                'l' + scan_chars!(stream, 'b', 'a', 'c', 'k'),
                                's' + scan_chars!(stream, 'e')
                            )
                        ),
                        'i' + scan_trie!(
                            stream,
                            'n' + scan_chars!(stream, 'n', 'e', 'y'),
                            'x' + scan_chars!(stream, 'e', 'd')
                        ),
                        'o' + scan_chars!(stream, 'r'),
                        'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                    ),
                    'g' + scan_chars!(stream, 'w', 'e', 'i'),
                    'h' + scan_trie!(
                        stream,
                        'e' + scan_chars!(stream, 'x'),
                        'o' + scan_chars!(stream, 'u', 'r', 's')
                    ),
                    'i' + scan_trie!(
                        stream,
                        ['f' | 's'],
                        'm' + scan_trie!(
                            stream,
                            'm' + scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e'),
                            'p' + scan_chars!(stream, 'o', 'r', 't')
                        ),
                        'n' + scan_trie!(
                            stream,
                            'd' + scan_chars!(stream, 'e', 'x', 'e', 'd'),
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
                    'l' + scan_chars!(stream, 'i', 'b', 'r', 'a', 'r', 'y'),
                    'm' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'p', 'p', 'i', 'n', 'g'),
                        'e' + scan_chars!(stream, 'm', 'o', 'r', 'y'),
                        'i' + scan_chars!(stream, 'n', 'u', 't', 'e', 's'),
                        'o' + scan_chars!(stream, 'd', 'i', 'f', 'i', 'e', 'r')
                    ),
                    'n' + scan_chars!(stream, 'e', 'w'),
                    'o' + scan_chars!(stream, 'v', 'e', 'r', 'r', 'i', 'd', 'e'),
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
                            'c' + scan_chars!(stream, 'e', 'i', 'v', 'e'),
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
                            'o' + scan_chars!(stream, 'r', 'a', 'g', 'e'),
                            'r' + scan_trie!(
                                stream,
                                'i' + scan_chars!(stream, 'n', 'g'),
                                'u' + scan_chars!(stream, 'c', 't')
                            )
                        ),
                        'z' + scan_chars!(stream, 'a', 'b', 'o')
                    ),
                    't' + scan_trie!(
                        stream,
                        'r' + scan_trie!(stream, ['y'], 'u' + scan_chars!(stream, 'e')),
                        'y' + scan_chars!(stream, 'p', 'e')
                    ),
                    'u' + scan_trie!(
                        stream,
                        'f' + scan_chars!(stream, 'i', 'x', 'e', 'd'),
                        'n' + scan_chars!(stream, 'c', 'h', 'e', 'c', 'k', 'e', 'd'),
                        's' + scan_chars!(stream, 'i', 'n', 'g')
                    ),
                    'v' + scan_sequence!(
                        scan_chars!(stream, 'i'),
                        scan_trie!(
                            stream,
                            'e' + scan_chars!(stream, 'w'),
                            'r' + scan_chars!(stream, 't', 'u', 'a', 'l')
                        )
                    ),
                    'w' + scan_trie!(
                        stream,
                        'e' + scan_trie!(stream, ['i'], 'e' + scan_chars!(stream, 'k', 's')),
                        'h' + scan_chars!(stream, 'i', 'l', 'e')
                    ),
                    'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
                )
            )
        }
    }

    // «LeaveKeyword» = "leave" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_leave_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «LessThan» = '<' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_less_than(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '<'),
                scan_predicate!(stream, |c| ('<' <= c && c <= '='))
            )
        }
    }

    // «LessThanEqual» = "<=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_less_than_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '<', '=')
        }
    }

    // «LessThanLessThan» = "<<" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_less_than_less_than(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '<', '<'),
                scan_chars!(stream, '=')
            )
        }
    }

    // «LessThanLessThanEqual» = "<<=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_less_than_less_than_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '<', '<', '=')
        }
    }

    // «LetKeyword» = "let" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_let_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «LibraryKeyword» = "library" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_library_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «MappingKeyword» = "mapping" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_mapping_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «MemoryKeyword» = "memory" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_memory_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Minus» = '-' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_minus(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '-'),
                scan_predicate!(stream, |c| c == '-' || ('=' <= c && c <= '>'))
            )
        }
    }

    // «MinusEqual» = "-=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_minus_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '-', '=')
        }
    }

    // «MinusGreaterThan» = "->" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_minus_greater_than(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '-', '>')
        }
    }

    // «MinusMinus» = "--" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_minus_minus(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '-', '-')
        }
    }

    // «MinutesKeyword» = "minutes" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_minutes_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ModifierKeyword» = "modifier" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_modifier_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «MultilineComment» = "/*" { ¬'*' | '*' ¬'/' } "*/" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_multiline_comment(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «NewKeyword» = "new" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_new_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «OpenBrace» = '{' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_open_brace(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '{')
        }
    }

    // «OpenBracket» = '[' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_open_bracket(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '[')
        }
    }

    // «OpenParen» = '(' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_open_paren(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '(')
        }
    }

    // «OverrideKeyword» = "override" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_override_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «PayableKeyword» = "payable" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_payable_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Percent» = '%' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_percent(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(stream, scan_chars!(stream, '%'), scan_chars!(stream, '='))
        }
    }

    // «PercentEqual» = "%=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_percent_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '%', '=')
        }
    }

    // «Period» = '.' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_period(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '.')
        }
    }

    // «Plus» = '+' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_plus(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '+'),
                scan_predicate!(stream, |c| c == '+' || c == '=')
            )
        }
    }

    // «PlusEqual» = "+=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_plus_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '+', '=')
        }
    }

    // «PlusPlus» = "++" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_plus_plus(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '+', '+')
        }
    }

    // «PossiblySeparatedPairsOfHexDigits» = 2…2{ «HexCharacter» } { [ '_' ] 2…2{ «HexCharacter» } } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_possibly_separated_pairs_of_hex_digits(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «PragmaKeyword» = "pragma" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_pragma_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «PrivateKeyword» = "private" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_private_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «PublicKeyword» = "public" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_public_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «PureKeyword» = "pure" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_pure_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «QuestionMark» = '?' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_question_mark(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '?')
        }
    }

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_raw_identifier(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ReceiveKeyword» = "receive" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_receive_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ReservedKeyword» = "after" | "alias" | "apply" | "auto" | "byte" | "case" | "copyof" | "default" | "define" | "final" | "implements" | "in" | "inline" | "let" | "macro" | "match" | "mutable" | "null" | "of" | "partial" | "promise" | "reference" | "relocatable" | "sealed" | "sizeof" | "static" | "supports" | "switch" | "typedef" | "typeof" | "var" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_reserved_keyword(&self, stream: &mut Stream) -> bool {
        {
            scan_trie!(
                stream,
                'a' + scan_trie!(
                    stream,
                    'f' + scan_chars!(stream, 't', 'e', 'r'),
                    'l' + scan_chars!(stream, 'i', 'a', 's'),
                    'p' + scan_chars!(stream, 'p', 'l', 'y'),
                    'u' + scan_chars!(stream, 't', 'o')
                ),
                'b' + scan_chars!(stream, 'y', 't', 'e'),
                'c' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 's', 'e'),
                    'o' + scan_chars!(stream, 'p', 'y', 'o', 'f')
                ),
                'd' + scan_sequence!(
                    scan_chars!(stream, 'e', 'f'),
                    scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'u', 'l', 't'),
                        'i' + scan_chars!(stream, 'n', 'e')
                    )
                ),
                'f' + scan_chars!(stream, 'i', 'n', 'a', 'l'),
                'i' + scan_trie!(
                    stream,
                    'm' + scan_chars!(stream, 'p', 'l', 'e', 'm', 'e', 'n', 't', 's'),
                    'n' + scan_trie!(stream, EMPTY, 'l' + scan_chars!(stream, 'i', 'n', 'e'))
                ),
                'l' + scan_chars!(stream, 'e', 't'),
                'm' + scan_trie!(
                    stream,
                    'a' + scan_trie!(
                        stream,
                        'c' + scan_chars!(stream, 'r', 'o'),
                        't' + scan_chars!(stream, 'c', 'h')
                    ),
                    'u' + scan_chars!(stream, 't', 'a', 'b', 'l', 'e')
                ),
                'n' + scan_chars!(stream, 'u', 'l', 'l'),
                'o' + scan_chars!(stream, 'f'),
                'p' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 'r', 't', 'i', 'a', 'l'),
                    'r' + scan_chars!(stream, 'o', 'm', 'i', 's', 'e')
                ),
                'r' + scan_sequence!(
                    scan_chars!(stream, 'e'),
                    scan_trie!(
                        stream,
                        'f' + scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e'),
                        'l' + scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e')
                    )
                ),
                's' + scan_trie!(
                    stream,
                    'e' + scan_chars!(stream, 'a', 'l', 'e', 'd'),
                    'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                    't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                    'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's'),
                    'w' + scan_chars!(stream, 'i', 't', 'c', 'h')
                ),
                't' + scan_sequence!(
                    scan_chars!(stream, 'y', 'p', 'e'),
                    scan_trie!(
                        stream,
                        'd' + scan_chars!(stream, 'e', 'f'),
                        'o' + scan_chars!(stream, 'f')
                    )
                ),
                'v' + scan_chars!(stream, 'a', 'r')
            )
        }
    }

    // «ReturnKeyword» = "return" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_return_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «ReturnsKeyword» = "returns" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_returns_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «RevertKeyword» = "revert" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_revert_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SecondsKeyword» = "seconds" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_seconds_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Semicolon» = ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_semicolon(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, ';')
        }
    }

    // «SignedFixedType» = "fixed" [ 1…{ '0'…'9' } 'x' 1…{ '0'…'9' } ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_signed_fixed_type(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SignedIntegerType» = "int" [ '8' | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256" ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_signed_integer_type(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SingleLineComment» = "//" { ¬( '\u{d}' | '\u{a}' ) } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_single_line_comment(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_chars!(stream, '/', '/'),
                scan_zero_or_more!(stream, scan_predicate!(stream, |c| c != '\n' && c != '\r'))
            )
        }
    }

    // «SingleQuotedAsciiStringLiteral» = "\'" { «EscapeSequence» | '\u{20}'…'~' - ( "\'" | '\\' ) } "\'" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_single_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SingleQuotedUnicodeStringLiteral» = "unicode\'" { «EscapeSequence» | ¬( "\'" | '\\' | '\u{a}' | '\u{d}' ) } "\'" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.scan_escape_sequence(stream),
                        scan_predicate!(stream, |c| c != '\n'
                            && c != '\r'
                            && c != '\''
                            && c != '\\')
                    )
                ),
                scan_chars!(stream, '\'')
            )
        }
    }

    // «Slash» = '/' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_slash(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(stream, scan_chars!(stream, '/'), scan_chars!(stream, '='))
        }
    }

    // «SlashEqual» = "/=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_slash_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '/', '=')
        }
    }

    // «SolidityKeyword» = "solidity" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_solidity_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Star» = '*' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_star(&self, stream: &mut Stream) -> bool {
        {
            scan_not_followed_by!(
                stream,
                scan_chars!(stream, '*'),
                scan_predicate!(stream, |c| c == '*' || c == '=')
            )
        }
    }

    // «StarEqual» = "*=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_star_equal(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '*', '=')
        }
    }

    // «StarStar» = "**" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_star_star(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '*', '*')
        }
    }

    // «StorageKeyword» = "storage" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_storage_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «StringKeyword» = "string" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_string_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «StructKeyword» = "struct" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_struct_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SwitchKeyword» = "switch" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_switch_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «SzaboKeyword» = "szabo" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_szabo_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Tilde» = '~' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_tilde(&self, stream: &mut Stream) -> bool {
        {
            scan_chars!(stream, '~')
        }
    }

    // «TrueKeyword» = "true" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_true_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «TryKeyword» = "try" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_try_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «TypeKeyword» = "type" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_type_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «UncheckedKeyword» = "unchecked" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_unchecked_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «UnicodeEscape» = 'u' 4…4{ «HexCharacter» } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_unicode_escape(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_choice!(
                stream,
                self.scan_single_quoted_unicode_string_literal(stream),
                self.scan_double_quoted_unicode_string_literal(stream)
            )
        }
    }

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_chars!(stream, 'u'),
                self.scan_signed_fixed_type(stream)
            )
        }
    }

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        {
            scan_sequence!(
                scan_chars!(stream, 'u'),
                self.scan_signed_integer_type(stream)
            )
        }
    }

    // «UsingKeyword» = "using" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_using_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | "<=" | ">=" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_version_pragma_operator(&self, stream: &mut Stream) -> bool {
        {
            scan_trie!(
                stream,
                ['=' | '^' | '~'],
                '<' + scan_trie!(stream, EMPTY, ['=']),
                '>' + scan_trie!(stream, EMPTY, ['='])
            )
        }
    }

    // «VersionPragmaValue» = 1…{ '0'…'9' | 'x' | 'X' | '*' } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_version_pragma_value(&self, stream: &mut Stream) -> bool {
        {
            scan_one_or_more!(
                stream,
                scan_predicate!(stream, |c| c == '*'
                    || ('0' <= c && c <= '9')
                    || c == 'X'
                    || c == 'x')
            )
        }
    }

    // «ViewKeyword» = "view" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_view_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «VirtualKeyword» = "virtual" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_virtual_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «WeeksKeyword» = "weeks" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_weeks_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «WeiKeyword» = "wei" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_wei_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «WhileKeyword» = "while" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_while_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «Whitespace» = 1…{ '\u{20}' | '\u{9}' } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_whitespace(&self, stream: &mut Stream) -> bool {
        {
            scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\t' || c == ' '))
        }
    }

    // «YearsKeyword» = "years" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_years_keyword(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «YulDecimalLiteral» = '0' | '1'…'9' { '0'…'9' } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_choice!(
                stream,
                scan_chars!(stream, '0'),
                scan_sequence!(
                    scan_predicate!(stream, |c| ('1' <= c && c <= '9')),
                    scan_zero_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9')))
                )
            )
        }
    }

    // «YulHexLiteral» = "0x" 1…{ «HexCharacter» } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_yul_hex_literal(&self, stream: &mut Stream) -> bool {
        {
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
    }

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_yul_identifier(&self, stream: &mut Stream) -> bool {
        {
            scan_difference!(
                stream,
                self.scan_raw_identifier(stream),
                scan_trie!(
                    stream,
                    'b' + scan_chars!(stream, 'r', 'e', 'a', 'k'),
                    'c' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 's', 'e'),
                        'o' + scan_chars!(stream, 'n', 't', 'i', 'n', 'u', 'e')
                    ),
                    'd' + scan_chars!(stream, 'e', 'f', 'a', 'u', 'l', 't'),
                    'f' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'l', 's', 'e'),
                        'o' + scan_chars!(stream, 'r'),
                        'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                    ),
                    'h' + scan_chars!(stream, 'e', 'x'),
                    'i' + scan_chars!(stream, 'f'),
                    'l' + scan_sequence!(
                        scan_chars!(stream, 'e'),
                        scan_trie!(stream, ['t'], 'a' + scan_chars!(stream, 'v', 'e'))
                    ),
                    's' + scan_chars!(stream, 'w', 'i', 't', 'c', 'h'),
                    't' + scan_chars!(stream, 'r', 'u', 'e')
                )
            )
        }
    }

    // «YulKeyword» = "false" | "true" | "break" | "case" | "continue" | "default" | "for" | "function" | "hex" | "if" | "leave" | "let" | "switch" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_yul_keyword(&self, stream: &mut Stream) -> bool {
        {
            scan_trie!(
                stream,
                'b' + scan_chars!(stream, 'r', 'e', 'a', 'k'),
                'c' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 's', 'e'),
                    'o' + scan_chars!(stream, 'n', 't', 'i', 'n', 'u', 'e')
                ),
                'd' + scan_chars!(stream, 'e', 'f', 'a', 'u', 'l', 't'),
                'f' + scan_trie!(
                    stream,
                    'a' + scan_chars!(stream, 'l', 's', 'e'),
                    'o' + scan_chars!(stream, 'r'),
                    'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                ),
                'h' + scan_chars!(stream, 'e', 'x'),
                'i' + scan_chars!(stream, 'f'),
                'l' + scan_sequence!(
                    scan_chars!(stream, 'e'),
                    scan_trie!(stream, ['t'], 'a' + scan_chars!(stream, 'v', 'e'))
                ),
                's' + scan_chars!(stream, 'w', 'i', 't', 'c', 'h'),
                't' + scan_chars!(stream, 'r', 'u', 'e')
            )
        }
    }
}
