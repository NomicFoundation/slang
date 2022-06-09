use super::tree_interface::*;
impl comment::_S0 {
    pub fn new(
        ((slash_star, content), star_slash): ((usize, Box<comment::Content>), usize),
    ) -> Self {
        Self {
            slash_star,
            content,
            star_slash,
        }
    }
}
impl comment::Content {
    pub fn new((_c2s, star_chars): (Vec<Box<comment::_C2>>, Vec<char>)) -> Self {
        Self { _c2s, star_chars }
    }
}
impl comment::_S3 {
    pub fn new((star_chars, _1): (Vec<char>, char)) -> Self {
        Self { star_chars, _1 }
    }
}
impl grouped::_S0 {
    pub fn new(
        ((open_paren_char, expression), close_paren_char): ((char, expression::N), char),
    ) -> Self {
        Self {
            open_paren_char,
            expression,
            close_paren_char,
        }
    }
}
impl optional::_S0 {
    pub fn new(
        ((open_bracket_char, expression), close_bracket_char): ((char, expression::N), char),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            close_bracket_char,
        }
    }
}
impl repetition_separator::_S0 {
    pub fn new((slash_char, expression): (char, expression::N)) -> Self {
        Self {
            slash_char,
            expression,
        }
    }
}
impl string_char::Escape {
    pub fn new(
        (backslash_char, quote_or_backslash_or_hex_escape): (
            char,
            Box<string_char::QuoteOrBackslashOrHexEscape>,
        ),
    ) -> Self {
        Self {
            backslash_char,
            quote_or_backslash_or_hex_escape,
        }
    }
}
impl string_char::_S1 {
    pub fn new(((u_open_brace, _1), close_brace_char): ((usize, Vec<char>), char)) -> Self {
        Self {
            u_open_brace,
            _1,
            close_brace_char,
        }
    }
}
impl repetition_prefix::_S0 {
    pub fn new((_c1, star_char): (Box<repetition_prefix::_C1>, char)) -> Self {
        Self { _c1, star_char }
    }
}
impl repetition_prefix::_S6 {
    pub fn new((ellipsis_char, number): (char, number::N)) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}
impl repetition_prefix::_S2 {
    pub fn new((number, _s4): (number::N, Option<Box<repetition_prefix::_S4>>)) -> Self {
        Self { number, _s4 }
    }
}
impl repetition_prefix::_S4 {
    pub fn new((ellipsis_char, number): (char, Option<number::N>)) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}
impl raw_identifier::_S0 {
    pub fn new((_0, _1): (char, Vec<char>)) -> Self {
        Self { _0, _1 }
    }
}
impl single_char_string::_S0 {
    pub fn new(
        ((quote_char_0, string_char), quote_char_1): ((char, string_char::N), char),
    ) -> Self {
        Self {
            quote_char_0,
            string_char,
            quote_char_1,
        }
    }
}
impl string::_S0 {
    pub fn new(
        ((quote_char_0, string_chars), quote_char_1): ((char, Vec<string_char::N>), char),
    ) -> Self {
        Self {
            quote_char_0,
            string_chars,
            quote_char_1,
        }
    }
}
impl repeated::_S0 {
    pub fn new(
        (
            (((repetition_prefix, open_brace_char), expression), repetition_separator),
            close_brace_char,
        ): (
            (
                ((Option<repetition_prefix::N>, char), expression::N),
                Option<repetition_separator::N>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            repetition_prefix,
            open_brace_char,
            expression,
            repetition_separator,
            close_brace_char,
        }
    }
}
impl identifier::_S1 {
    pub fn new(
        ((open_double_angle_char, raw_identifier), close_double_angle_char): (
            (char, raw_identifier::N),
            char,
        ),
    ) -> Self {
        Self {
            open_double_angle_char,
            raw_identifier,
            close_double_angle_char,
        }
    }
}
impl char_range::_S0 {
    pub fn new(
        ((single_char_string_0, ellipsis_char), single_char_string_1): (
            (single_char_string::N, char),
            single_char_string::N,
        ),
    ) -> Self {
        Self {
            single_char_string_0,
            ellipsis_char,
            single_char_string_1,
        }
    }
}
impl negation::_S0 {
    pub fn new((not_char, primary): (Option<char>, primary::N)) -> Self {
        Self { not_char, primary }
    }
}
impl difference::_S0 {
    pub fn new((negation, _s2): (negation::N, Option<Box<difference::_S2>>)) -> Self {
        Self { negation, _s2 }
    }
}
impl difference::_S2 {
    pub fn new((minus_char, negation): (char, negation::N)) -> Self {
        Self {
            minus_char,
            negation,
        }
    }
}
impl expression::_S0 {
    pub fn new((sequences, bar_chars): (Vec<sequence::N>, Vec<char>)) -> Self {
        Self {
            sequences,
            bar_chars,
        }
    }
}
impl production::_S0 {
    pub fn new(
        (((identifier, equal_char), expression), semicolon_char): (
            ((identifier::N, char), expression::N),
            char,
        ),
    ) -> Self {
        Self {
            identifier,
            equal_char,
            expression,
            semicolon_char,
        }
    }
}
impl grammar::_S0 {
    pub fn new(((ignore, productions), end_marker): ((ignore::N, Vec<production::N>), ())) -> Self {
        Self {
            ignore,
            productions,
            end_marker,
        }
    }
}
