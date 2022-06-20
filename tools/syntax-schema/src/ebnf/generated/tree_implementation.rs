use super::tree_interface::*;
impl<T: DefaultTest> DefaultTest for Box<T> {
    fn is_default(&self) -> bool {
        self.as_ref().is_default()
    }
}
impl<T> DefaultTest for Vec<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
impl<T> DefaultTest for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}
impl DefaultTest for () {
    fn is_default(&self) -> bool {
        true
    }
}
impl DefaultTest for usize {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
impl<const N: usize> DefaultTest for FixedTerminal<N> {
    fn is_default(&self) -> bool {
        true
    }
}

impl comment::_T0 {
    pub fn new(
        ((open_paren_star, content), star_close_paren): (
            (FixedTerminal<2usize>, Box<comment::Content>),
            FixedTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            open_paren_star,
            content,
            star_close_paren,
        }
    }
}

impl DefaultTest for comment::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_star.is_default()
            && self.content.is_default()
            && self.star_close_paren.is_default()
    }
}

impl comment::Content {
    pub fn new((_t2s, star_chars): (Vec<Box<comment::_T2>>, usize)) -> Self {
        Self { _t2s, star_chars }
    }
}

impl DefaultTest for comment::Content {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self.star_chars.is_default()
    }
}

impl DefaultTest for comment::_T2 {}

impl comment::_T3 {
    pub fn new((star_chars, _1): (usize, FixedTerminal<1usize>)) -> Self {
        Self { star_chars, _1 }
    }
}

impl DefaultTest for comment::_T3 {
    fn is_default(&self) -> bool {
        self.star_chars.is_default() && self._1.is_default()
    }
}

impl DefaultTest for ignore::_T1 {}

impl DefaultTest for string_char::_T0 {}

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

impl DefaultTest for string_char::Escape {}

impl DefaultTest for string_char::QuoteOrBackslashOrHexEscape {}

impl string_char::_T1 {
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

impl DefaultTest for string_char::_T1 {
    fn is_default(&self) -> bool {
        self.u_open_brace.is_default() && self._1.is_default() && self.close_brace_char.is_default()
    }
}

impl raw_identifier::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl DefaultTest for raw_identifier::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}

impl single_char_string::_T0 {
    pub fn new(
        ((quote_char_1, string_char), quote_char_2): (
            (FixedTerminal<1usize>, string_char::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            string_char,
            quote_char_2,
        }
    }
}

impl DefaultTest for single_char_string::_T0 {}

impl string::_T0 {
    pub fn new(
        ((quote_char_1, string_chars), quote_char_2): (
            (FixedTerminal<1usize>, Vec<string_char::N>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            string_chars,
            quote_char_2,
        }
    }
}

impl DefaultTest for string::_T0 {
    fn is_default(&self) -> bool {
        self.quote_char_1.is_default()
            && self.string_chars.is_default()
            && self.quote_char_2.is_default()
    }
}

impl grouped::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_1), expression), ignore_2), close_paren_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), expression::N),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            expression,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for grouped::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self.close_paren_char.is_default()
    }
}

impl optional::_T0 {
    pub fn new(
        ((((open_bracket_char, ignore_1), expression), ignore_2), close_bracket_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), expression::N),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_1,
            expression,
            ignore_2,
            close_bracket_char,
        }
    }
}

impl DefaultTest for optional::_T0 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl repetition_prefix::_T0 {
    pub fn new(
        ((_t1, ignore), star_char): (
            (Box<repetition_prefix::_T1>, ignore::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            _t1,
            ignore,
            star_char,
        }
    }
}

impl DefaultTest for repetition_prefix::_T0 {}

impl DefaultTest for repetition_prefix::_T1 {}

impl repetition_prefix::_T4 {
    pub fn new((ellipsis_char, number): (FixedTerminal<1usize>, number::N)) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}

impl DefaultTest for repetition_prefix::_T4 {
    fn is_default(&self) -> bool {
        self.ellipsis_char.is_default() && self.number.is_default()
    }
}

impl repetition_prefix::_T2 {
    pub fn new((number, _t3): (number::N, Option<Box<repetition_prefix::_T3>>)) -> Self {
        Self { number, _t3 }
    }
}

impl DefaultTest for repetition_prefix::_T2 {
    fn is_default(&self) -> bool {
        self.number.is_default() && self._t3.is_default()
    }
}

impl repetition_prefix::_T3 {
    pub fn new((ellipsis_char, number): (FixedTerminal<1usize>, Option<number::N>)) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}

impl DefaultTest for repetition_prefix::_T3 {
    fn is_default(&self) -> bool {
        self.ellipsis_char.is_default() && self.number.is_default()
    }
}

impl repetition_separator::_T0 {
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

impl DefaultTest for repetition_separator::_T0 {
    fn is_default(&self) -> bool {
        self.slash_char.is_default() && self.ignore.is_default() && self.expression.is_default()
    }
}

impl DefaultTest for identifier::_T0 {}

impl identifier::_T1 {
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

impl DefaultTest for identifier::_T1 {
    fn is_default(&self) -> bool {
        self.open_double_angle_char.is_default()
            && self.raw_identifier.is_default()
            && self.close_double_angle_char.is_default()
    }
}

impl char_range::_T0 {
    pub fn new(
        ((((single_char_string_1, ignore_1), ellipsis_char), ignore_2), single_char_string_2): (
            (
                ((single_char_string::N, ignore::N), FixedTerminal<1usize>),
                ignore::N,
            ),
            single_char_string::N,
        ),
    ) -> Self {
        Self {
            single_char_string_1,
            ignore_1,
            ellipsis_char,
            ignore_2,
            single_char_string_2,
        }
    }
}

impl DefaultTest for char_range::_T0 {}

impl repeated::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((repetition_prefix, ignore_1), open_brace_char), ignore_2), expression),
                        ignore_3,
                    ),
                    repetition_separator,
                ),
                ignore_4,
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
            ignore_1,
            open_brace_char,
            ignore_2,
            expression,
            ignore_3,
            repetition_separator,
            ignore_4,
            close_brace_char,
        }
    }
}

impl DefaultTest for repeated::_T0 {
    fn is_default(&self) -> bool {
        self.repetition_prefix.is_default()
            && self.ignore_1.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_2.is_default()
            && self.expression.is_default()
            && self.ignore_3.is_default()
            && self.repetition_separator.is_default()
            && self.ignore_4.is_default()
            && self.close_brace_char.is_default()
    }
}

impl DefaultTest for primary::_T0 {}

impl negation::_T0 {
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

impl DefaultTest for negation::_T0 {}

impl difference::_T0 {
    pub fn new(
        ((negation, ignore), _t1): ((negation::N, ignore::N), Option<Box<difference::_T1>>),
    ) -> Self {
        Self {
            negation,
            ignore,
            _t1,
        }
    }
}

impl DefaultTest for difference::_T0 {}

impl difference::_T1 {
    pub fn new((minus_char, negation): (FixedTerminal<1usize>, negation::N)) -> Self {
        Self {
            minus_char,
            negation,
        }
    }
}

impl DefaultTest for difference::_T1 {}

impl expression::_T0 {
    pub fn new((sequences, bar_chars): (Vec<sequence::N>, Vec<FixedTerminal<1usize>>)) -> Self {
        Self {
            sequences,
            bar_chars,
        }
    }
}

impl DefaultTest for expression::_T0 {
    fn is_default(&self) -> bool {
        self.sequences.is_default() && self.bar_chars.is_default()
    }
}

impl production::_T0 {
    pub fn new(
        (
            (((((identifier, ignore_1), equal_char), ignore_2), expression), ignore_3),
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
            ignore_1,
            equal_char,
            ignore_2,
            expression,
            ignore_3,
            semicolon_char,
        }
    }
}

impl DefaultTest for production::_T0 {}

impl grammar::_T0 {
    pub fn new(
        ((((ignore_1, ignore_2), productions), ignore_3), end_marker): (
            (((ignore::N, ignore::N), Vec<production::N>), ignore::N),
            (),
        ),
    ) -> Self {
        Self {
            ignore_1,
            ignore_2,
            productions,
            ignore_3,
            end_marker,
        }
    }
}

impl DefaultTest for grammar::_T0 {
    fn is_default(&self) -> bool {
        self.ignore_1.is_default()
            && self.ignore_2.is_default()
            && self.productions.is_default()
            && self.ignore_3.is_default()
            && self.end_marker.is_default()
    }
}
