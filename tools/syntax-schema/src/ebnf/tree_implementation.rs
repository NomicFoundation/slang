use super::tree_interface::*;
impl comment::_S0 {
    pub fn new(
        ((slash_star, content), star_slash): (
            (FixedTerminal<2usize>, Box<comment::Content>),
            FixedTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            slash_star,
            content,
            star_slash,
        }
    }
}
impl comment::Content {
    pub fn new((_c2s, star_chars): (Vec<Box<comment::_C2>>, usize)) -> Self {
        Self { _c2s, star_chars }
    }
}
impl comment::_S3 {
    pub fn new((star_chars, _1): (usize, FixedTerminal<1usize>)) -> Self {
        Self { star_chars, _1 }
    }
}
impl grouped::_S0 {
    pub fn new(
        ((((open_paren_char, ignore_0), expression), ignore_1), close_paren_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), expression::N),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            expression,
            ignore_1,
            close_paren_char,
        }
    }
}
impl optional::_S0 {
    pub fn new(
        ((((open_bracket_char, ignore_0), expression), ignore_1), close_bracket_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), expression::N),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            expression,
            ignore_1,
            close_bracket_char,
        }
    }
}
impl repetition_separator::_S0 {
    pub fn new(
        ((slash_char, ignore), expression): ((FixedTerminal<1usize>, ignore::N), expression::N),
    ) -> Self {
        Self {
            slash_char,
            ignore,
            expression,
        }
    }
}
impl string_char::Escape {
    pub fn new(
        (backslash_char, quote_or_backslash_or_hex_escape): (
            FixedTerminal<1usize>,
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
    pub fn new(
        ((u_open_brace, _1), close_brace_char): (
            (FixedTerminal<2usize>, usize),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            u_open_brace,
            _1,
            close_brace_char,
        }
    }
}
impl repetition_prefix::_S0 {
    pub fn new(
        ((_c1, ignore), star_char): (
            (Box<repetition_prefix::_C1>, ignore::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            _c1,
            ignore,
            star_char,
        }
    }
}
impl repetition_prefix::_S6 {
    pub fn new(
        ((ellipsis_char, ignore), number): ((FixedTerminal<1usize>, ignore::N), number::N),
    ) -> Self {
        Self {
            ellipsis_char,
            ignore,
            number,
        }
    }
}
impl repetition_prefix::_S2 {
    pub fn new(
        ((number, ignore), _s4): ((number::N, ignore::N), Option<Box<repetition_prefix::_S4>>),
    ) -> Self {
        Self {
            number,
            ignore,
            _s4,
        }
    }
}
impl repetition_prefix::_S4 {
    pub fn new(
        ((ellipsis_char, ignore), number): ((FixedTerminal<1usize>, ignore::N), Option<number::N>),
    ) -> Self {
        Self {
            ellipsis_char,
            ignore,
            number,
        }
    }
}
impl raw_identifier::_S0 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}
impl single_char_string::_S0 {
    pub fn new(
        ((quote_char_0, string_char), quote_char_1): (
            (FixedTerminal<1usize>, string_char::N),
            FixedTerminal<1usize>,
        ),
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
        ((quote_char_0, string_chars), quote_char_1): (
            (FixedTerminal<1usize>, Vec<string_char::N>),
            FixedTerminal<1usize>,
        ),
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
            (
                (
                    (
                        ((((repetition_prefix, ignore_0), open_brace_char), ignore_1), expression),
                        ignore_2,
                    ),
                    repetition_separator,
                ),
                ignore_3,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (Option<repetition_prefix::N>, ignore::N),
                                    FixedTerminal<1usize>,
                                ),
                                ignore::N,
                            ),
                            expression::N,
                        ),
                        ignore::N,
                    ),
                    Option<repetition_separator::N>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            repetition_prefix,
            ignore_0,
            open_brace_char,
            ignore_1,
            expression,
            ignore_2,
            repetition_separator,
            ignore_3,
            close_brace_char,
        }
    }
}
impl identifier::_S1 {
    pub fn new(
        ((open_double_angle_char, raw_identifier), close_double_angle_char): (
            (FixedTerminal<1usize>, raw_identifier::N),
            FixedTerminal<1usize>,
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
        ((((single_char_string_0, ignore_0), ellipsis_char), ignore_1), single_char_string_1): (
            (
                ((single_char_string::N, ignore::N), FixedTerminal<1usize>),
                ignore::N,
            ),
            single_char_string::N,
        ),
    ) -> Self {
        Self {
            single_char_string_0,
            ignore_0,
            ellipsis_char,
            ignore_1,
            single_char_string_1,
        }
    }
}
impl negation::_S0 {
    pub fn new(
        ((not_char, ignore), primary): ((Option<FixedTerminal<1usize>>, ignore::N), primary::N),
    ) -> Self {
        Self {
            not_char,
            ignore,
            primary,
        }
    }
}
impl difference::_S0 {
    pub fn new(
        ((negation, ignore), _s2): ((negation::N, ignore::N), Option<Box<difference::_S2>>),
    ) -> Self {
        Self {
            negation,
            ignore,
            _s2,
        }
    }
}
impl difference::_S2 {
    pub fn new(
        ((minus_char, ignore), negation): ((FixedTerminal<1usize>, ignore::N), negation::N),
    ) -> Self {
        Self {
            minus_char,
            ignore,
            negation,
        }
    }
}
impl sequence::_S1 {
    pub fn new((difference, ignore): (difference::N, ignore::N)) -> Self {
        Self { difference, ignore }
    }
}
impl expression::_S0 {
    pub fn new((_s1s, _s2s): (Vec<Box<expression::_S1>>, Vec<Box<expression::_S2>>)) -> Self {
        Self { _s1s, _s2s }
    }
}
impl expression::_S2 {
    pub fn new((bar_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { bar_char, ignore }
    }
}
impl expression::_S1 {
    pub fn new((sequence, ignore): (sequence::N, ignore::N)) -> Self {
        Self { sequence, ignore }
    }
}
impl production::_S0 {
    pub fn new(
        (
            (((((identifier, ignore_0), equal_char), ignore_1), expression), ignore_2),
            semicolon_char,
        ): (
            (
                (
                    (
                        ((identifier::N, ignore::N), FixedTerminal<1usize>),
                        ignore::N,
                    ),
                    expression::N,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore_0,
            equal_char,
            ignore_1,
            expression,
            ignore_2,
            semicolon_char,
        }
    }
}
impl grammar::_S0 {
    pub fn new(
        ((((ignore_0, ignore_1), _s2s), ignore_2), end_marker): (
            (((ignore::N, ignore::N), Vec<Box<grammar::_S2>>), ignore::N),
            (),
        ),
    ) -> Self {
        Self {
            ignore_0,
            ignore_1,
            _s2s,
            ignore_2,
            end_marker,
        }
    }
}
impl grammar::_S2 {
    pub fn new((production, ignore): (production::N, ignore::N)) -> Self {
        Self { production, ignore }
    }
}
