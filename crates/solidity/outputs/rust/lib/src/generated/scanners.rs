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
macro_rules! scan_delimited_by {
    ($stream:ident, [$($open:literal),*], $expr:expr, [$($close:literal),*]) => {
        scan_chars!($stream, $($open),*) && ($expr) && scan_chars!($stream, $($close),*)
    };
}

#[allow(unused_macros)]
macro_rules! scan_separated_by {
    ($stream:ident, $expr:expr, [$($separator:literal),*]) => {
        loop {
            if !($expr) { break false };
            let save = $stream.position();
            if !(scan_chars!($stream, $($separator),*)) {
                $stream.set_position(save);
                break true
            }
        }
    };
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

impl Language {
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

    // «BooleanLiteral» = "true" | "false" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_boolean_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_trie!(
                stream,
                'f' + scan_chars!(stream, 'a', 'l', 's', 'e'),
                't' + scan_chars!(stream, 'r', 'u', 'e')
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
            scan_separated_by!(
                stream,
                scan_one_or_more!(stream, scan_predicate!(stream, |c| ('0' <= c && c <= '9'))),
                ['_']
            )
        }
    }

    // «DoubleQuotedAsciiStringLiteral» = '"' { «EscapeSequence» | '\u{20}'…'~' - ( '"' | '\\' ) } '"' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_delimited_by!(
                stream,
                ['"'],
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
                ['"']
            )
        }
    }

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { «EscapeSequence» | ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } '"' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_delimited_by!(
                stream,
                ['u', 'n', 'i', 'c', 'o', 'd', 'e', '"'],
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
                ['"']
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

    // «FixedBytesType» = "bytes" ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32" ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        {
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
                scan_separated_by!(
                    stream,
                    scan_one_or_more!(
                        stream,
                        scan_predicate!(stream, |c| ('0' <= c && c <= '9')
                            || ('A' <= c && c <= 'F')
                            || ('a' <= c && c <= 'f'))
                    ),
                    ['_']
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
                    scan_delimited_by!(
                        stream,
                        ['"'],
                        scan_optional!(
                            stream,
                            self.scan_possibly_separated_pairs_of_hex_digits(stream)
                        ),
                        ['"']
                    ),
                    scan_delimited_by!(
                        stream,
                        ['\''],
                        scan_optional!(
                            stream,
                            self.scan_possibly_separated_pairs_of_hex_digits(stream)
                        ),
                        ['\'']
                    )
                )
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

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | "abstract" | "address" | "anonymous" | "as" | "assembly" | "bool" | "break" | "calldata" | "catch" | "constant" | "constructor" | "continue" | "contract" | "delete" | "do" | "else" | "emit" | "enum" | "event" | "external" | "fallback" | "false" | "fixed" | "for" | "function" | "hex" | "if" | "immutable" | "import" | "indexed" | "interface" | "internal" | "is" | "library" | "mapping" | "memory" | "modifier" | "new" | "override" | "payable" | "pragma" | "private" | "public" | "pure" | "receive" | "return" | "returns" | "storage" | "string" | "struct" | "true" | "try" | "type" | "ufixed" | "unchecked" | "using" | "view" | "virtual" | "while" ;

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
                    'd' + scan_trie!(
                        stream,
                        'a' + scan_chars!(stream, 'y', 's'),
                        'e' + scan_sequence!(
                            scan_chars!(stream, 'f'),
                            scan_trie!(
                                stream,
                                'a' + scan_chars!(stream, 'u', 'l', 't'),
                                'i' + scan_chars!(stream, 'n', 'e')
                            )
                        )
                    ),
                    'e' + scan_chars!(stream, 't', 'h', 'e', 'r'),
                    'f' + scan_sequence!(
                        scan_chars!(stream, 'i', 'n'),
                        scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'l'),
                            'n' + scan_chars!(stream, 'e', 'y')
                        )
                    ),
                    'g' + scan_chars!(stream, 'w', 'e', 'i'),
                    'h' + scan_chars!(stream, 'o', 'u', 'r', 's'),
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
                        'i' + scan_chars!(stream, 'n', 'u', 't', 'e', 's'),
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
                        'e' + scan_trie!(
                            stream,
                            'a' + scan_chars!(stream, 'l', 'e', 'd'),
                            'c' + scan_chars!(stream, 'o', 'n', 'd', 's')
                        ),
                        'i' + scan_chars!(stream, 'z', 'e', 'o', 'f'),
                        't' + scan_chars!(stream, 'a', 't', 'i', 'c'),
                        'u' + scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's'),
                        'w' + scan_chars!(stream, 'i', 't', 'c', 'h'),
                        'z' + scan_chars!(stream, 'a', 'b', 'o')
                    ),
                    't' + scan_sequence!(
                        scan_chars!(stream, 'y', 'p', 'e'),
                        scan_trie!(
                            stream,
                            'd' + scan_chars!(stream, 'e', 'f'),
                            'o' + scan_chars!(stream, 'f')
                        )
                    ),
                    'v' + scan_chars!(stream, 'a', 'r'),
                    'w' + scan_sequence!(
                        scan_chars!(stream, 'e'),
                        scan_trie!(stream, ['i'], 'e' + scan_chars!(stream, 'k', 's'))
                    ),
                    'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
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
                    'd' + scan_trie!(stream, ['o'], 'e' + scan_chars!(stream, 'l', 'e', 't', 'e')),
                    'e' + scan_trie!(
                        stream,
                        'l' + scan_chars!(stream, 's', 'e'),
                        'm' + scan_chars!(stream, 'i', 't'),
                        'n' + scan_chars!(stream, 'u', 'm'),
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
                        'i' + scan_chars!(stream, 'x', 'e', 'd'),
                        'o' + scan_chars!(stream, 'r'),
                        'u' + scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n')
                    ),
                    'h' + scan_chars!(stream, 'e', 'x'),
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
                    's' + scan_sequence!(
                        scan_chars!(stream, 't'),
                        scan_trie!(
                            stream,
                            'o' + scan_chars!(stream, 'r', 'a', 'g', 'e'),
                            'r' + scan_trie!(
                                stream,
                                'i' + scan_chars!(stream, 'n', 'g'),
                                'u' + scan_chars!(stream, 'c', 't')
                            )
                        )
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
                    'w' + scan_chars!(stream, 'h', 'i', 'l', 'e')
                )
            )
        }
    }

    // «MultilineComment» = "/*" { ¬'*' | '*' ¬'/' } "*/" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_multiline_comment(&self, stream: &mut Stream) -> bool {
        {
            scan_delimited_by!(
                stream,
                ['/', '*'],
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
                ['*', '/']
            )
        }
    }

    // «NumberUnit» = "days" | "ether" | "finney" | "gwei" | "hours" | "minutes" | "seconds" | "szabo" | "weeks" | "wei" | "years" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_number_unit(&self, stream: &mut Stream) -> bool {
        {
            scan_trie!(
                stream,
                'd' + scan_chars!(stream, 'a', 'y', 's'),
                'e' + scan_chars!(stream, 't', 'h', 'e', 'r'),
                'f' + scan_chars!(stream, 'i', 'n', 'n', 'e', 'y'),
                'g' + scan_chars!(stream, 'w', 'e', 'i'),
                'h' + scan_chars!(stream, 'o', 'u', 'r', 's'),
                'm' + scan_chars!(stream, 'i', 'n', 'u', 't', 'e', 's'),
                's' + scan_trie!(
                    stream,
                    'e' + scan_chars!(stream, 'c', 'o', 'n', 'd', 's'),
                    'z' + scan_chars!(stream, 'a', 'b', 'o')
                ),
                'w' + scan_sequence!(
                    scan_chars!(stream, 'e'),
                    scan_trie!(stream, ['i'], 'e' + scan_chars!(stream, 'k', 's'))
                ),
                'y' + scan_chars!(stream, 'e', 'a', 'r', 's')
            )
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

    // «SignedFixedType» = "fixed" [ 1…{ '0'…'9' } 'x' 1…{ '0'…'9' } ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_signed_fixed_type(&self, stream: &mut Stream) -> bool {
        {
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
            )
        }
    }

    // «SignedIntegerType» = "int" [ '8' | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256" ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_signed_integer_type(&self, stream: &mut Stream) -> bool {
        {
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
            scan_delimited_by!(
                stream,
                ['\''],
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
                ['\'']
            )
        }
    }

    // «SingleQuotedUnicodeStringLiteral» = "unicode\'" { «EscapeSequence» | ¬( "\'" | '\\' | '\u{a}' | '\u{d}' ) } "\'" ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        {
            scan_delimited_by!(
                stream,
                ['u', 'n', 'i', 'c', 'o', 'd', 'e', '\''],
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
                ['\'']
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

    // «Whitespace» = 1…{ '\u{20}' | '\u{9}' } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn scan_whitespace(&self, stream: &mut Stream) -> bool {
        {
            scan_one_or_more!(stream, scan_predicate!(stream, |c| c == '\t' || c == ' '))
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

    // «YulKeyword» = «BooleanLiteral» | "break" | "case" | "continue" | "default" | "for" | "function" | "hex" | "if" | "leave" | "let" | "switch" ;

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
