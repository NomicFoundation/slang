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
        (((slash_star, _t2s), star_chars), star_slash): (
            ((FixedTerminal<2usize>, Vec<Box<comment::_T2>>), usize),
            FixedTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            slash_star,
            _t2s,
            star_chars,
            star_slash,
        }
    }
}

impl DefaultTest for comment::_T0 {
    fn is_default(&self) -> bool {
        self.slash_star.is_default()
            && self._t2s.is_default()
            && self.star_chars.is_default()
            && self.star_slash.is_default()
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

impl decimal_integer::_T0 {
    pub fn new(
        (_0, underscore_chars): (
            Vec<FixedTerminal<1usize>>,
            Vec<Option<FixedTerminal<1usize>>>,
        ),
    ) -> Self {
        Self {
            _0,
            underscore_chars,
        }
    }
}

impl DefaultTest for decimal_integer::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.underscore_chars.is_default()
    }
}

impl fixed_bytes_type::_T0 {
    pub fn new((bytes, _1): (FixedTerminal<5usize>, usize)) -> Self {
        Self { bytes, _1 }
    }
}

impl DefaultTest for fixed_bytes_type::_T0 {
    fn is_default(&self) -> bool {
        self.bytes.is_default() && self._1.is_default()
    }
}

impl fixed_type::_T0 {
    pub fn new((fixed, _t1): (FixedTerminal<5usize>, Option<Box<fixed_type::_T1>>)) -> Self {
        Self { fixed, _t1 }
    }
}

impl DefaultTest for fixed_type::_T0 {
    fn is_default(&self) -> bool {
        self.fixed.is_default() && self._t1.is_default()
    }
}

impl fixed_type::_T1 {
    pub fn new(
        ((((_0, _1), _2), _3), _4): (
            (
                ((FixedTerminal<1usize>, usize), FixedTerminal<1usize>),
                FixedTerminal<1usize>,
            ),
            usize,
        ),
    ) -> Self {
        Self { _0, _1, _2, _3, _4 }
    }
}

impl DefaultTest for fixed_type::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default()
            && self._1.is_default()
            && self._2.is_default()
            && self._3.is_default()
            && self._4.is_default()
    }
}

impl line_comment::_T0 {
    pub fn new((slash_slash, _1): (FixedTerminal<2usize>, usize)) -> Self {
        Self { slash_slash, _1 }
    }
}

impl DefaultTest for line_comment::_T0 {
    fn is_default(&self) -> bool {
        self.slash_slash.is_default() && self._1.is_default()
    }
}

impl pragma_directive::_T0 {
    pub fn new(
        ((pragma, not_semicolon_chars), semicolon_char): (
            (FixedTerminal<6usize>, usize),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            pragma,
            not_semicolon_chars,
            semicolon_char,
        }
    }
}

impl DefaultTest for pragma_directive::_T0 {
    fn is_default(&self) -> bool {
        self.pragma.is_default()
            && self.not_semicolon_chars.is_default()
            && self.semicolon_char.is_default()
    }
}

impl signed_integer_type::_T0 {
    pub fn new((int, _1): (FixedTerminal<3usize>, usize)) -> Self {
        Self { int, _1 }
    }
}

impl DefaultTest for signed_integer_type::_T0 {
    fn is_default(&self) -> bool {
        self.int.is_default() && self._1.is_default()
    }
}

impl DefaultTest for yul_decimal_number_literal::_T0 {}

impl yul_decimal_number_literal::_T1 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl DefaultTest for yul_decimal_number_literal::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}

impl yul_hex_literal::_T0 {
    pub fn new((zero_x, _1): (FixedTerminal<2usize>, usize)) -> Self {
        Self { zero_x, _1 }
    }
}

impl DefaultTest for yul_hex_literal::_T0 {
    fn is_default(&self) -> bool {
        self.zero_x.is_default() && self._1.is_default()
    }
}

impl decimal_exponent::_T0 {
    pub fn new(
        ((_0, minus_char), decimal_integer): (
            (FixedTerminal<1usize>, Option<FixedTerminal<1usize>>),
            decimal_integer::N,
        ),
    ) -> Self {
        Self {
            _0,
            minus_char,
            decimal_integer,
        }
    }
}

impl DefaultTest for decimal_exponent::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.minus_char.is_default() && self.decimal_integer.is_default()
    }
}

impl decimal_float::_T0 {
    pub fn new(
        ((decimal_integer_1, period_char), decimal_integer_2): (
            (Option<decimal_integer::N>, FixedTerminal<1usize>),
            decimal_integer::N,
        ),
    ) -> Self {
        Self {
            decimal_integer_1,
            period_char,
            decimal_integer_2,
        }
    }
}

impl DefaultTest for decimal_float::_T0 {
    fn is_default(&self) -> bool {
        self.decimal_integer_1.is_default()
            && self.period_char.is_default()
            && self.decimal_integer_2.is_default()
    }
}

impl hex_byte_escape::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl DefaultTest for hex_byte_escape::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}

impl hex_number::_T0 {
    pub fn new(
        ((zero_char, _1), _2): (
            (FixedTerminal<1usize>, FixedTerminal<1usize>),
            Box<hex_number::_T1>,
        ),
    ) -> Self {
        Self { zero_char, _1, _2 }
    }
}

impl DefaultTest for hex_number::_T0 {
    fn is_default(&self) -> bool {
        self.zero_char.is_default() && self._1.is_default() && self._2.is_default()
    }
}

impl hex_number::_T1 {
    pub fn new(
        (_0, underscore_chars): (
            Vec<FixedTerminal<1usize>>,
            Vec<Option<FixedTerminal<1usize>>>,
        ),
    ) -> Self {
        Self {
            _0,
            underscore_chars,
        }
    }
}

impl DefaultTest for hex_number::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.underscore_chars.is_default()
    }
}

impl DefaultTest for ignore::_T1 {}

impl possibly_separated_pairs_of_hex_digits::_T0 {
    pub fn new((_0, underscore_chars): (Vec<usize>, Vec<Option<FixedTerminal<1usize>>>)) -> Self {
        Self {
            _0,
            underscore_chars,
        }
    }
}

impl DefaultTest for possibly_separated_pairs_of_hex_digits::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.underscore_chars.is_default()
    }
}

impl ufixed_type::_T0 {
    pub fn new((_0, fixed_type): (FixedTerminal<1usize>, fixed_type::N)) -> Self {
        Self { _0, fixed_type }
    }
}

impl DefaultTest for ufixed_type::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.fixed_type.is_default()
    }
}

impl unicode_escape::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl DefaultTest for unicode_escape::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}

impl unsigned_integer_type::_T0 {
    pub fn new((_0, signed_integer_type): (FixedTerminal<1usize>, signed_integer_type::N)) -> Self {
        Self {
            _0,
            signed_integer_type,
        }
    }
}

impl DefaultTest for unsigned_integer_type::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.signed_integer_type.is_default()
    }
}

impl break_statement::_T0 {
    pub fn new(
        ((r#break, ignore), semicolon_char): (
            (FixedTerminal<5usize>, ignore::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#break,
            ignore,
            semicolon_char,
        }
    }
}

impl DefaultTest for break_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#break.is_default() && self.ignore.is_default() && self.semicolon_char.is_default()
    }
}

impl continue_statement::_T0 {
    pub fn new(
        ((r#continue, ignore), semicolon_char): (
            (FixedTerminal<8usize>, ignore::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#continue,
            ignore,
            semicolon_char,
        }
    }
}

impl DefaultTest for continue_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#continue.is_default() && self.ignore.is_default() && self.semicolon_char.is_default()
    }
}

impl decimal_number::_T0 {
    pub fn new(
        (_t1, decimal_exponent): (Box<decimal_number::_T1>, Option<decimal_exponent::N>),
    ) -> Self {
        Self {
            _t1,
            decimal_exponent,
        }
    }
}

impl DefaultTest for decimal_number::_T0 {}

impl DefaultTest for decimal_number::_T1 {}

impl DefaultTest for elementary_type::_T0 {}

impl escape_sequence::_T0 {
    pub fn new((backslash_char, _t1): (FixedTerminal<1usize>, Box<escape_sequence::_T1>)) -> Self {
        Self {
            backslash_char,
            _t1,
        }
    }
}

impl DefaultTest for escape_sequence::_T0 {}

impl DefaultTest for escape_sequence::_T1 {}

impl hex_string_literal::_T0 {
    pub fn new((hex, _t1): (FixedTerminal<3usize>, Box<hex_string_literal::_T1>)) -> Self {
        Self { hex, _t1 }
    }
}

impl DefaultTest for hex_string_literal::_T0 {}

impl DefaultTest for hex_string_literal::_T1 {}

impl hex_string_literal::_T3 {
    pub fn new(
        ((quote_char_1, possibly_separated_pairs_of_hex_digits), quote_char_2): (
            (
                FixedTerminal<1usize>,
                Option<possibly_separated_pairs_of_hex_digits::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            possibly_separated_pairs_of_hex_digits,
            quote_char_2,
        }
    }
}

impl DefaultTest for hex_string_literal::_T3 {
    fn is_default(&self) -> bool {
        self.quote_char_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.quote_char_2.is_default()
    }
}

impl hex_string_literal::_T2 {
    pub fn new(
        ((double_quote_char_1, possibly_separated_pairs_of_hex_digits), double_quote_char_2): (
            (
                FixedTerminal<1usize>,
                Option<possibly_separated_pairs_of_hex_digits::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            possibly_separated_pairs_of_hex_digits,
            double_quote_char_2,
        }
    }
}

impl DefaultTest for hex_string_literal::_T2 {
    fn is_default(&self) -> bool {
        self.double_quote_char_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.double_quote_char_2.is_default()
    }
}

impl DefaultTest for keyword::_T0 {}

impl positional_argument_list::_T0 {
    pub fn new(
        (expressions, comma_chars): (Vec<expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for positional_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.comma_chars.is_default()
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

impl unchecked_block::_T0 {
    pub fn new(
        ((unchecked, ignore), block): ((FixedTerminal<9usize>, ignore::N), block::N),
    ) -> Self {
        Self {
            unchecked,
            ignore,
            block,
        }
    }
}

impl DefaultTest for unchecked_block::_T0 {
    fn is_default(&self) -> bool {
        self.unchecked.is_default() && self.ignore.is_default() && self.block.is_default()
    }
}

impl double_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((double_quote_char_1, runs), double_quote_char_2): (
            (
                FixedTerminal<1usize>,
                Vec<Box<double_quoted_ascii_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            runs,
            double_quote_char_2,
        }
    }
}

impl DefaultTest for double_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.double_quote_char_1.is_default()
            && self.runs.is_default()
            && self.double_quote_char_2.is_default()
    }
}

impl DefaultTest for double_quoted_ascii_string_literal::Run {}

impl double_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_double_quote, runs), double_quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<double_quoted_unicode_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            runs,
            double_quote_char,
        }
    }
}

impl DefaultTest for double_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_double_quote.is_default()
            && self.runs.is_default()
            && self.double_quote_char.is_default()
    }
}

impl DefaultTest for double_quoted_unicode_string_literal::Run {}

impl DefaultTest for elementary_type_with_payable::_T0 {}

impl elementary_type_with_payable::_T1 {
    pub fn new((address, payable): (FixedTerminal<7usize>, Option<FixedTerminal<7usize>>)) -> Self {
        Self { address, payable }
    }
}

impl DefaultTest for elementary_type_with_payable::_T1 {
    fn is_default(&self) -> bool {
        self.address.is_default() && self.payable.is_default()
    }
}

impl DefaultTest for elementary_type_without_payable::_T0 {}

impl numeric_literal::_T0 {
    pub fn new(
        ((_t1, ignore), _2): ((Box<numeric_literal::_T1>, ignore::N), Option<usize>),
    ) -> Self {
        Self { _t1, ignore, _2 }
    }
}

impl DefaultTest for numeric_literal::_T0 {}

impl DefaultTest for numeric_literal::_T1 {}

impl DefaultTest for reserved_word::_T0 {}

impl single_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((quote_char_1, runs), quote_char_2): (
            (
                FixedTerminal<1usize>,
                Vec<Box<single_quoted_ascii_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            runs,
            quote_char_2,
        }
    }
}

impl DefaultTest for single_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.quote_char_1.is_default() && self.runs.is_default() && self.quote_char_2.is_default()
    }
}

impl DefaultTest for single_quoted_ascii_string_literal::Run {}

impl single_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_quote, runs), quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<single_quoted_unicode_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            runs,
            quote_char,
        }
    }
}

impl DefaultTest for single_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_quote.is_default() && self.runs.is_default() && self.quote_char.is_default()
    }
}

impl DefaultTest for single_quoted_unicode_string_literal::Run {}

impl DefaultTest for ascii_string_literal::_T0 {}

impl assembly_flags::_T0 {
    pub fn new(
        (
            (((open_paren_char, ignore_1), double_quoted_ascii_string_literals), ignore_2),
            close_paren_char,
        ): (
            (
                ((FixedTerminal<1usize>, ignore::N), Box<assembly_flags::_T1>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            double_quoted_ascii_string_literals,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for assembly_flags::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_1.is_default()
            && self.double_quoted_ascii_string_literals.is_default()
            && self.ignore_2.is_default()
            && self.close_paren_char.is_default()
    }
}

impl assembly_flags::_T1 {
    pub fn new(
        (double_quoted_ascii_string_literals, comma_chars): (
            Vec<double_quoted_ascii_string_literal::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            double_quoted_ascii_string_literals,
            comma_chars,
        }
    }
}

impl DefaultTest for assembly_flags::_T1 {
    fn is_default(&self) -> bool {
        self.double_quoted_ascii_string_literals.is_default() && self.comma_chars.is_default()
    }
}

impl DefaultTest for unicode_string_literal::_T0 {}

impl yul_function_call::_T0 {
    pub fn new(
        (
            (((((_t1, ignore_1), open_paren_char), ignore_2), yul_expressions), ignore_3),
            close_paren_char,
        ): (
            (
                (
                    (
                        (
                            (Box<yul_function_call::_T1>, ignore::N),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<yul_function_call::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            _t1,
            ignore_1,
            open_paren_char,
            ignore_2,
            yul_expressions,
            ignore_3,
            close_paren_char,
        }
    }
}

impl DefaultTest for yul_function_call::_T0 {}

impl yul_function_call::_T2 {
    pub fn new(
        (yul_expressions, comma_chars): (Vec<yul_expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            yul_expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for yul_function_call::_T2 {
    fn is_default(&self) -> bool {
        self.yul_expressions.is_default() && self.comma_chars.is_default()
    }
}

impl DefaultTest for yul_function_call::_T1 {}

impl yul_function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (((function, ignore_1), yul_identifier), ignore_2),
                                            open_paren_char,
                                        ),
                                        ignore_3,
                                    ),
                                    yul_identifiers,
                                ),
                                ignore_4,
                            ),
                            close_paren_char,
                        ),
                        ignore_5,
                    ),
                    _t2,
                ),
                ignore_6,
            ),
            yul_block,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                (
                                                    (FixedTerminal<8usize>, ignore::N),
                                                    yul_identifier::N,
                                                ),
                                                ignore::N,
                                            ),
                                            FixedTerminal<1usize>,
                                        ),
                                        ignore::N,
                                    ),
                                    Option<Box<yul_function_definition::_T1>>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<yul_function_definition::_T2>>,
                ),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            function,
            ignore_1,
            yul_identifier,
            ignore_2,
            open_paren_char,
            ignore_3,
            yul_identifiers,
            ignore_4,
            close_paren_char,
            ignore_5,
            _t2,
            ignore_6,
            yul_block,
        }
    }
}

impl DefaultTest for yul_function_definition::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.ignore_1.is_default()
            && self.yul_identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_3.is_default()
            && self.yul_identifiers.is_default()
            && self.ignore_4.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_5.is_default()
            && self._t2.is_default()
            && self.ignore_6.is_default()
            && self.yul_block.is_default()
    }
}

impl yul_function_definition::_T2 {
    pub fn new(
        (minus_greater, yul_identifiers): (
            FixedTerminal<2usize>,
            Box<yul_function_definition::_T3>,
        ),
    ) -> Self {
        Self {
            minus_greater,
            yul_identifiers,
        }
    }
}

impl DefaultTest for yul_function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.minus_greater.is_default() && self.yul_identifiers.is_default()
    }
}

impl yul_function_definition::_T3 {
    pub fn new(
        (yul_identifiers, comma_chars): (Vec<yul_identifier::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            yul_identifiers,
            comma_chars,
        }
    }
}

impl DefaultTest for yul_function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.yul_identifiers.is_default() && self.comma_chars.is_default()
    }
}

impl yul_function_definition::_T1 {
    pub fn new(
        (yul_identifiers, comma_chars): (Vec<yul_identifier::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            yul_identifiers,
            comma_chars,
        }
    }
}

impl DefaultTest for yul_function_definition::_T1 {
    fn is_default(&self) -> bool {
        self.yul_identifiers.is_default() && self.comma_chars.is_default()
    }
}

impl yul_path::_T0 {
    pub fn new(
        ((yul_identifier, ignore), _t2s): ((yul_identifier::N, ignore::N), Vec<Box<yul_path::_T2>>),
    ) -> Self {
        Self {
            yul_identifier,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for yul_path::_T0 {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default() && self.ignore.is_default() && self._t2s.is_default()
    }
}

impl yul_path::_T2 {
    pub fn new((period_char, _t3): (FixedTerminal<1usize>, Box<yul_path::_T3>)) -> Self {
        Self { period_char, _t3 }
    }
}

impl DefaultTest for yul_path::_T2 {}

impl DefaultTest for yul_path::_T3 {}

impl enum_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#enum, ignore_1), identifier), ignore_2), open_brace_char), ignore_3),
                    identifiers,
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
                                ((FixedTerminal<4usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Box<enum_definition::_T1>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#enum,
            ignore_1,
            identifier,
            ignore_2,
            open_brace_char,
            ignore_3,
            identifiers,
            ignore_4,
            close_brace_char,
        }
    }
}

impl DefaultTest for enum_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#enum.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_3.is_default()
            && self.identifiers.is_default()
            && self.ignore_4.is_default()
            && self.close_brace_char.is_default()
    }
}

impl enum_definition::_T1 {
    pub fn new(
        (identifiers, comma_chars): (Vec<identifier::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            identifiers,
            comma_chars,
        }
    }
}

impl DefaultTest for enum_definition::_T1 {
    fn is_default(&self) -> bool {
        self.identifiers.is_default() && self.comma_chars.is_default()
    }
}

impl identifier_path::_T0 {
    pub fn new(
        (identifiers, period_chars): (Vec<identifier::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            identifiers,
            period_chars,
        }
    }
}

impl DefaultTest for identifier_path::_T0 {
    fn is_default(&self) -> bool {
        self.identifiers.is_default() && self.period_chars.is_default()
    }
}

impl DefaultTest for literal::_T0 {}

impl named_argument::_T0 {
    pub fn new(
        ((((identifier, ignore_1), colon_char), ignore_2), expression): (
            (
                ((identifier::N, ignore::N), FixedTerminal<1usize>),
                ignore::N,
            ),
            expression::N,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore_1,
            colon_char,
            ignore_2,
            expression,
        }
    }
}

impl DefaultTest for named_argument::_T0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.colon_char.is_default()
            && self.ignore_2.is_default()
            && self.expression.is_default()
    }
}

impl parameter_declaration::_T0 {
    pub fn new(
        ((((type_name, ignore_1), _2), ignore_2), identifier): (
            (((type_name::N, ignore::N), Option<usize>), ignore::N),
            Option<identifier::N>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_1,
            _2,
            ignore_2,
            identifier,
        }
    }
}

impl DefaultTest for parameter_declaration::_T0 {}

impl selected_import::_T0 {
    pub fn new(
        ((identifier, ignore), _t1): (
            (identifier::N, ignore::N),
            Option<Box<selected_import::_T1>>,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore,
            _t1,
        }
    }
}

impl DefaultTest for selected_import::_T0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.ignore.is_default() && self._t1.is_default()
    }
}

impl selected_import::_T1 {
    pub fn new((r#as, identifier): (FixedTerminal<2usize>, identifier::N)) -> Self {
        Self { r#as, identifier }
    }
}

impl DefaultTest for selected_import::_T1 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}

impl user_defined_value_type_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#type, ignore_1), identifier), ignore_2), is), ignore_3),
                    elementary_type_with_payable,
                ),
                ignore_4,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<4usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            FixedTerminal<2usize>,
                        ),
                        ignore::N,
                    ),
                    elementary_type_with_payable::N,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#type,
            ignore_1,
            identifier,
            ignore_2,
            is,
            ignore_3,
            elementary_type_with_payable,
            ignore_4,
            semicolon_char,
        }
    }
}

impl DefaultTest for user_defined_value_type_definition::_T0 {}

impl DefaultTest for yul_literal::_T0 {}

impl mapping_type::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((mapping, ignore_1), open_paren_char), ignore_2), _t1), ignore_3),
                            equal_greater,
                        ),
                        ignore_4,
                    ),
                    type_name,
                ),
                ignore_5,
            ),
            close_paren_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((FixedTerminal<7usize>, ignore::N), FixedTerminal<1usize>),
                                        ignore::N,
                                    ),
                                    Box<mapping_type::_T1>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<2usize>,
                        ),
                        ignore::N,
                    ),
                    type_name::N,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            mapping,
            ignore_1,
            open_paren_char,
            ignore_2,
            _t1,
            ignore_3,
            equal_greater,
            ignore_4,
            type_name,
            ignore_5,
            close_paren_char,
        }
    }
}

impl DefaultTest for mapping_type::_T0 {}

impl DefaultTest for mapping_type::_T1 {}

impl named_argument_list::_T0 {
    pub fn new(
        ((((open_brace_char, ignore_1), named_arguments), ignore_2), close_brace_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Option<Box<named_argument_list::_T1>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_1,
            named_arguments,
            ignore_2,
            close_brace_char,
        }
    }
}

impl DefaultTest for named_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_1.is_default()
            && self.named_arguments.is_default()
            && self.ignore_2.is_default()
            && self.close_brace_char.is_default()
    }
}

impl named_argument_list::_T1 {
    pub fn new(
        (named_arguments, comma_chars): (Vec<named_argument::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            named_arguments,
            comma_chars,
        }
    }
}

impl DefaultTest for named_argument_list::_T1 {
    fn is_default(&self) -> bool {
        self.named_arguments.is_default() && self.comma_chars.is_default()
    }
}

impl non_empty_parameter_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_1), parameter_declarations), ignore_2), close_paren_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Box<non_empty_parameter_list::_T1>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            parameter_declarations,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for non_empty_parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_1.is_default()
            && self.parameter_declarations.is_default()
            && self.ignore_2.is_default()
            && self.close_paren_char.is_default()
    }
}

impl non_empty_parameter_list::_T1 {
    pub fn new(
        (parameter_declarations, comma_chars): (
            Vec<parameter_declaration::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            parameter_declarations,
            comma_chars,
        }
    }
}

impl DefaultTest for non_empty_parameter_list::_T1 {
    fn is_default(&self) -> bool {
        self.parameter_declarations.is_default() && self.comma_chars.is_default()
    }
}

impl override_specifier::_T0 {
    pub fn new(
        ((r#override, ignore), _t1): (
            (FixedTerminal<8usize>, ignore::N),
            Option<Box<override_specifier::_T1>>,
        ),
    ) -> Self {
        Self {
            r#override,
            ignore,
            _t1,
        }
    }
}

impl DefaultTest for override_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.r#override.is_default() && self.ignore.is_default() && self._t1.is_default()
    }
}

impl override_specifier::_T1 {
    pub fn new(
        ((open_paren_char, identifier_paths), close_paren_char): (
            (FixedTerminal<1usize>, Box<override_specifier::_T2>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            identifier_paths,
            close_paren_char,
        }
    }
}

impl DefaultTest for override_specifier::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.identifier_paths.is_default()
            && self.close_paren_char.is_default()
    }
}

impl override_specifier::_T2 {
    pub fn new(
        (identifier_paths, comma_chars): (Vec<identifier_path::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            identifier_paths,
            comma_chars,
        }
    }
}

impl DefaultTest for override_specifier::_T2 {
    fn is_default(&self) -> bool {
        self.identifier_paths.is_default() && self.comma_chars.is_default()
    }
}

impl parameter_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_1), parameter_declarations), ignore_2), close_paren_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Option<Box<parameter_list::_T1>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            parameter_declarations,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_1.is_default()
            && self.parameter_declarations.is_default()
            && self.ignore_2.is_default()
            && self.close_paren_char.is_default()
    }
}

impl parameter_list::_T1 {
    pub fn new(
        (parameter_declarations, comma_chars): (
            Vec<parameter_declaration::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            parameter_declarations,
            comma_chars,
        }
    }
}

impl DefaultTest for parameter_list::_T1 {
    fn is_default(&self) -> bool {
        self.parameter_declarations.is_default() && self.comma_chars.is_default()
    }
}

impl selecting_import_directive::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((open_brace_char, ignore_1), selected_imports), ignore_2),
                            close_brace_char,
                        ),
                        ignore_3,
                    ),
                    from,
                ),
                ignore_4,
            ),
            import_path,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (FixedTerminal<1usize>, ignore::N),
                                    Box<selecting_import_directive::_T1>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<4usize>,
                ),
                ignore::N,
            ),
            import_path::N,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_1,
            selected_imports,
            ignore_2,
            close_brace_char,
            ignore_3,
            from,
            ignore_4,
            import_path,
        }
    }
}

impl DefaultTest for selecting_import_directive::_T0 {}

impl selecting_import_directive::_T1 {
    pub fn new(
        (selected_imports, comma_chars): (Vec<selected_import::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            selected_imports,
            comma_chars,
        }
    }
}

impl DefaultTest for selecting_import_directive::_T1 {
    fn is_default(&self) -> bool {
        self.selected_imports.is_default() && self.comma_chars.is_default()
    }
}

impl simple_import_directive::_T0 {
    pub fn new(
        ((import_path, ignore), _t2s): (
            (import_path::N, ignore::N),
            Vec<Box<simple_import_directive::_T2>>,
        ),
    ) -> Self {
        Self {
            import_path,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for simple_import_directive::_T0 {}

impl simple_import_directive::_T2 {
    pub fn new((r#as, identifier): (FixedTerminal<2usize>, identifier::N)) -> Self {
        Self { r#as, identifier }
    }
}

impl DefaultTest for simple_import_directive::_T2 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}

impl star_import_directive::_T0 {
    pub fn new(
        (
            (((((((star_char, ignore_1), r#as), ignore_2), identifier), ignore_3), from), ignore_4),
            import_path,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<1usize>, ignore::N), FixedTerminal<2usize>),
                                ignore::N,
                            ),
                            identifier::N,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<4usize>,
                ),
                ignore::N,
            ),
            import_path::N,
        ),
    ) -> Self {
        Self {
            star_char,
            ignore_1,
            r#as,
            ignore_2,
            identifier,
            ignore_3,
            from,
            ignore_4,
            import_path,
        }
    }
}

impl DefaultTest for star_import_directive::_T0 {}

impl DefaultTest for yul_expression::_T0 {}

impl argument_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_1), _t1), ignore_2), close_paren_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Option<Box<argument_list::_T1>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            _t1,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_1.is_default()
            && self._t1.is_default()
            && self.ignore_2.is_default()
            && self.close_paren_char.is_default()
    }
}

impl DefaultTest for argument_list::_T1 {}

impl catch_clause::_T0 {
    pub fn new(
        ((((catch, ignore_1), _t1), ignore_2), block): (
            (
                (
                    (FixedTerminal<5usize>, ignore::N),
                    Option<Box<catch_clause::_T1>>,
                ),
                ignore::N,
            ),
            block::N,
        ),
    ) -> Self {
        Self {
            catch,
            ignore_1,
            _t1,
            ignore_2,
            block,
        }
    }
}

impl DefaultTest for catch_clause::_T0 {
    fn is_default(&self) -> bool {
        self.catch.is_default()
            && self.ignore_1.is_default()
            && self._t1.is_default()
            && self.ignore_2.is_default()
            && self.block.is_default()
    }
}

impl catch_clause::_T1 {
    pub fn new(
        (identifier, non_empty_parameter_list): (
            Option<identifier::N>,
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            identifier,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for catch_clause::_T1 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl function_type::_T0 {
    pub fn new(
        ((((((function, ignore_1), parameter_list), ignore_2), _4), ignore_3), _t2): (
            (
                (
                    (
                        ((FixedTerminal<8usize>, ignore::N), parameter_list::N),
                        ignore::N,
                    ),
                    Vec<usize>,
                ),
                ignore::N,
            ),
            Option<Box<function_type::_T2>>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_1,
            parameter_list,
            ignore_2,
            _4,
            ignore_3,
            _t2,
        }
    }
}

impl DefaultTest for function_type::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.ignore_1.is_default()
            && self.parameter_list.is_default()
            && self.ignore_2.is_default()
            && self._4.is_default()
            && self.ignore_3.is_default()
            && self._t2.is_default()
    }
}

impl function_type::_T2 {
    pub fn new(
        (returns, non_empty_parameter_list): (FixedTerminal<7usize>, non_empty_parameter_list::N),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for function_type::_T2 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl import_directive::_T0 {
    pub fn new(
        ((((import, ignore_1), _t1), ignore_2), semicolon_char): (
            (
                (
                    (FixedTerminal<6usize>, ignore::N),
                    Box<import_directive::_T1>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            import,
            ignore_1,
            _t1,
            ignore_2,
            semicolon_char,
        }
    }
}

impl DefaultTest for import_directive::_T0 {}

impl DefaultTest for import_directive::_T1 {}

impl DefaultTest for method_attribute::_T0 {}

impl DefaultTest for state_variable_attribute::_T0 {}

impl yul_assignment::_T0 {
    pub fn new(
        ((yul_path, ignore), _t1): ((yul_path::N, ignore::N), Box<yul_assignment::_T1>),
    ) -> Self {
        Self {
            yul_path,
            ignore,
            _t1,
        }
    }
}

impl DefaultTest for yul_assignment::_T0 {}

impl DefaultTest for yul_assignment::_T1 {}

impl yul_assignment::_T3 {
    pub fn new(
        ((_t5s, colon_equal), yul_function_call): (
            (Vec<Box<yul_assignment::_T5>>, FixedTerminal<2usize>),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            _t5s,
            colon_equal,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_assignment::_T3 {}

impl yul_assignment::_T5 {
    pub fn new((comma_char, yul_path): (FixedTerminal<1usize>, yul_path::N)) -> Self {
        Self {
            comma_char,
            yul_path,
        }
    }
}

impl DefaultTest for yul_assignment::_T5 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.yul_path.is_default()
    }
}

impl yul_assignment::_T2 {
    pub fn new((colon_equal, yul_expression): (FixedTerminal<2usize>, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}

impl DefaultTest for yul_assignment::_T2 {}

impl yul_for_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#for, ignore_1), yul_block_1), ignore_2), yul_expression), ignore_3),
                    yul_block_2,
                ),
                ignore_4,
            ),
            yul_block_3,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<3usize>, ignore::N), yul_block::N),
                                ignore::N,
                            ),
                            yul_expression::N,
                        ),
                        ignore::N,
                    ),
                    yul_block::N,
                ),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#for,
            ignore_1,
            yul_block_1,
            ignore_2,
            yul_expression,
            ignore_3,
            yul_block_2,
            ignore_4,
            yul_block_3,
        }
    }
}

impl DefaultTest for yul_for_statement::_T0 {}

impl yul_if_statement::_T0 {
    pub fn new(
        ((((r#if, ignore_1), yul_expression), ignore_2), yul_block): (
            (
                ((FixedTerminal<2usize>, ignore::N), yul_expression::N),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#if,
            ignore_1,
            yul_expression,
            ignore_2,
            yul_block,
        }
    }
}

impl DefaultTest for yul_if_statement::_T0 {}

impl yul_switch_statement::_T0 {
    pub fn new(
        ((((switch, ignore_1), yul_expression), ignore_2), _t1): (
            (
                ((FixedTerminal<6usize>, ignore::N), yul_expression::N),
                ignore::N,
            ),
            Box<yul_switch_statement::_T1>,
        ),
    ) -> Self {
        Self {
            switch,
            ignore_1,
            yul_expression,
            ignore_2,
            _t1,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T0 {}

impl DefaultTest for yul_switch_statement::_T1 {}

impl yul_switch_statement::_T6 {
    pub fn new((default, yul_block): (FixedTerminal<7usize>, yul_block::N)) -> Self {
        Self { default, yul_block }
    }
}

impl DefaultTest for yul_switch_statement::_T6 {
    fn is_default(&self) -> bool {
        self.default.is_default() && self.yul_block.is_default()
    }
}

impl yul_switch_statement::_T2 {
    pub fn new(
        (_t4s, _t5): (
            Vec<Box<yul_switch_statement::_T4>>,
            Option<Box<yul_switch_statement::_T5>>,
        ),
    ) -> Self {
        Self { _t4s, _t5 }
    }
}

impl DefaultTest for yul_switch_statement::_T2 {
    fn is_default(&self) -> bool {
        self._t4s.is_default() && self._t5.is_default()
    }
}

impl yul_switch_statement::_T5 {
    pub fn new((default, yul_block): (FixedTerminal<7usize>, yul_block::N)) -> Self {
        Self { default, yul_block }
    }
}

impl DefaultTest for yul_switch_statement::_T5 {
    fn is_default(&self) -> bool {
        self.default.is_default() && self.yul_block.is_default()
    }
}

impl yul_switch_statement::_T4 {
    pub fn new(
        ((case, yul_literal), yul_block): ((FixedTerminal<4usize>, yul_literal::N), yul_block::N),
    ) -> Self {
        Self {
            case,
            yul_literal,
            yul_block,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T4 {}

impl yul_variable_declaration::_T0 {
    pub fn new(
        ((((r#let, ignore_1), yul_identifier), ignore_2), _t1): (
            (
                ((FixedTerminal<3usize>, ignore::N), yul_identifier::N),
                ignore::N,
            ),
            Option<Box<yul_variable_declaration::_T1>>,
        ),
    ) -> Self {
        Self {
            r#let,
            ignore_1,
            yul_identifier,
            ignore_2,
            _t1,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T0 {
    fn is_default(&self) -> bool {
        self.r#let.is_default()
            && self.ignore_1.is_default()
            && self.yul_identifier.is_default()
            && self.ignore_2.is_default()
            && self._t1.is_default()
    }
}

impl DefaultTest for yul_variable_declaration::_T1 {}

impl yul_variable_declaration::_T3 {
    pub fn new(
        (_t4, _t5): (
            Option<Box<yul_variable_declaration::_T4>>,
            Option<Box<yul_variable_declaration::_T5>>,
        ),
    ) -> Self {
        Self { _t4, _t5 }
    }
}

impl DefaultTest for yul_variable_declaration::_T3 {
    fn is_default(&self) -> bool {
        self._t4.is_default() && self._t5.is_default()
    }
}

impl yul_variable_declaration::_T5 {
    pub fn new(
        (colon_equal, yul_function_call): (FixedTerminal<2usize>, yul_function_call::N),
    ) -> Self {
        Self {
            colon_equal,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T5 {}

impl yul_variable_declaration::_T4 {
    pub fn new((comma_char, yul_identifier): (FixedTerminal<1usize>, yul_identifier::N)) -> Self {
        Self {
            comma_char,
            yul_identifier,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.yul_identifier.is_default()
    }
}

impl yul_variable_declaration::_T2 {
    pub fn new((colon_equal, yul_expression): (FixedTerminal<2usize>, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T2 {}

impl inheritance_specifier::_T0 {
    pub fn new(
        ((identifier_path, ignore), argument_list): (
            (identifier_path::N, ignore::N),
            Option<argument_list::N>,
        ),
    ) -> Self {
        Self {
            identifier_path,
            ignore,
            argument_list,
        }
    }
}

impl DefaultTest for inheritance_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default()
            && self.ignore.is_default()
            && self.argument_list.is_default()
    }
}

impl modifier_invocation::_T0 {
    pub fn new(
        ((identifier_path, ignore), argument_list): (
            (identifier_path::N, ignore::N),
            Option<argument_list::N>,
        ),
    ) -> Self {
        Self {
            identifier_path,
            ignore,
            argument_list,
        }
    }
}

impl DefaultTest for modifier_invocation::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default()
            && self.ignore.is_default()
            && self.argument_list.is_default()
    }
}

impl type_name::_T0 {
    pub fn new(
        ((_t1, ignore), _t3s): ((Box<type_name::_T1>, ignore::N), Vec<Box<type_name::_T3>>),
    ) -> Self {
        Self { _t1, ignore, _t3s }
    }
}

impl DefaultTest for type_name::_T0 {}

impl type_name::_T3 {
    pub fn new(
        ((open_bracket_char, expression), close_bracket_char): (
            (FixedTerminal<1usize>, Option<expression::N>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            close_bracket_char,
        }
    }
}

impl DefaultTest for type_name::_T3 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expression.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl DefaultTest for type_name::_T1 {}

impl DefaultTest for yul_statement::_T0 {}

impl DefaultTest for constructor_attribute::_T0 {}

impl error_parameter::_T0 {
    pub fn new(
        ((type_name, ignore), identifier): ((type_name::N, ignore::N), Option<identifier::N>),
    ) -> Self {
        Self {
            type_name,
            ignore,
            identifier,
        }
    }
}

impl DefaultTest for error_parameter::_T0 {}

impl event_parameter::_T0 {
    pub fn new(
        ((((type_name, ignore_1), indexed), ignore_2), identifier): (
            (
                ((type_name::N, ignore::N), Option<FixedTerminal<7usize>>),
                ignore::N,
            ),
            Option<identifier::N>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_1,
            indexed,
            ignore_2,
            identifier,
        }
    }
}

impl DefaultTest for event_parameter::_T0 {}

impl DefaultTest for fallback_function_attribute::_T0 {}

impl DefaultTest for function_attribute::_T0 {}

impl inheritance_specifier_list::_T0 {
    pub fn new(
        ((is, ignore), inheritance_specifiers): (
            (FixedTerminal<2usize>, ignore::N),
            Box<inheritance_specifier_list::_T1>,
        ),
    ) -> Self {
        Self {
            is,
            ignore,
            inheritance_specifiers,
        }
    }
}

impl DefaultTest for inheritance_specifier_list::_T0 {
    fn is_default(&self) -> bool {
        self.is.is_default() && self.ignore.is_default() && self.inheritance_specifiers.is_default()
    }
}

impl inheritance_specifier_list::_T1 {
    pub fn new(
        (inheritance_specifiers, comma_chars): (
            Vec<inheritance_specifier::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            inheritance_specifiers,
            comma_chars,
        }
    }
}

impl DefaultTest for inheritance_specifier_list::_T1 {
    fn is_default(&self) -> bool {
        self.inheritance_specifiers.is_default() && self.comma_chars.is_default()
    }
}

impl DefaultTest for primary_expression::_T0 {}

impl primary_expression::_T6 {
    pub fn new(
        ((open_bracket_char, expressions), close_bracket_char): (
            (FixedTerminal<1usize>, Box<primary_expression::_T7>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expressions,
            close_bracket_char,
        }
    }
}

impl DefaultTest for primary_expression::_T6 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expressions.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl primary_expression::_T7 {
    pub fn new(
        (expressions, comma_chars): (Vec<expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for primary_expression::_T7 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.comma_chars.is_default()
    }
}

impl primary_expression::_T4 {
    pub fn new(
        ((open_paren_char, expressions), close_paren_char): (
            (FixedTerminal<1usize>, Box<primary_expression::_T5>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            expressions,
            close_paren_char,
        }
    }
}

impl DefaultTest for primary_expression::_T4 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.expressions.is_default()
            && self.close_paren_char.is_default()
    }
}

impl primary_expression::_T5 {
    pub fn new(
        (expressions, comma_chars): (Vec<Option<expression::N>>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for primary_expression::_T5 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.comma_chars.is_default()
    }
}

impl primary_expression::_T3 {
    pub fn new((new, type_name): (FixedTerminal<3usize>, type_name::N)) -> Self {
        Self { new, type_name }
    }
}

impl DefaultTest for primary_expression::_T3 {}

impl primary_expression::_T2 {
    pub fn new(
        (((r#type, open_paren_char), type_name), close_paren_char): (
            ((FixedTerminal<4usize>, FixedTerminal<1usize>), type_name::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#type,
            open_paren_char,
            type_name,
            close_paren_char,
        }
    }
}

impl DefaultTest for primary_expression::_T2 {}

impl primary_expression::_T1 {
    pub fn new((payable, argument_list): (FixedTerminal<7usize>, argument_list::N)) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}

impl DefaultTest for primary_expression::_T1 {
    fn is_default(&self) -> bool {
        self.payable.is_default() && self.argument_list.is_default()
    }
}

impl DefaultTest for receive_function_attribute::_T0 {}

impl struct_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#struct, ignore_1), identifier), ignore_2), open_brace_char), ignore_3),
                    _t2s,
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
                                ((FixedTerminal<6usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<struct_definition::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#struct,
            ignore_1,
            identifier,
            ignore_2,
            open_brace_char,
            ignore_3,
            _t2s,
            ignore_4,
            close_brace_char,
        }
    }
}

impl DefaultTest for struct_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#struct.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_3.is_default()
            && self._t2s.is_default()
            && self.ignore_4.is_default()
            && self.close_brace_char.is_default()
    }
}

impl struct_definition::_T2 {
    pub fn new(
        ((type_name, identifier), semicolon_char): (
            (type_name::N, identifier::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            type_name,
            identifier,
            semicolon_char,
        }
    }
}

impl DefaultTest for struct_definition::_T2 {}

impl using_directive::_T0 {
    pub fn new(
        (
            (
                (
                    (((((((using, ignore_1), _t1), ignore_2), r#for), ignore_3), _t4), ignore_4),
                    global,
                ),
                ignore_5,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (FixedTerminal<5usize>, ignore::N),
                                            Box<using_directive::_T1>,
                                        ),
                                        ignore::N,
                                    ),
                                    FixedTerminal<3usize>,
                                ),
                                ignore::N,
                            ),
                            Box<using_directive::_T4>,
                        ),
                        ignore::N,
                    ),
                    Option<FixedTerminal<6usize>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            using,
            ignore_1,
            _t1,
            ignore_2,
            r#for,
            ignore_3,
            _t4,
            ignore_4,
            global,
            ignore_5,
            semicolon_char,
        }
    }
}

impl DefaultTest for using_directive::_T0 {}

impl DefaultTest for using_directive::_T4 {}

impl DefaultTest for using_directive::_T1 {}

impl using_directive::_T2 {
    pub fn new(
        ((open_brace_char, identifier_paths), close_brace_char): (
            (FixedTerminal<1usize>, Box<using_directive::_T3>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            identifier_paths,
            close_brace_char,
        }
    }
}

impl DefaultTest for using_directive::_T2 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.identifier_paths.is_default()
            && self.close_brace_char.is_default()
    }
}

impl using_directive::_T3 {
    pub fn new(
        (identifier_paths, comma_chars): (Vec<identifier_path::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            identifier_paths,
            comma_chars,
        }
    }
}

impl DefaultTest for using_directive::_T3 {
    fn is_default(&self) -> bool {
        self.identifier_paths.is_default() && self.comma_chars.is_default()
    }
}

impl variable_declaration::_T0 {
    pub fn new(
        ((((type_name, ignore_1), _2), ignore_2), identifier): (
            (((type_name::N, ignore::N), Option<usize>), ignore::N),
            identifier::N,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_1,
            _2,
            ignore_2,
            identifier,
        }
    }
}

impl DefaultTest for variable_declaration::_T0 {}

impl yul_block::_T0 {
    pub fn new(
        ((((open_brace_char, ignore_1), yul_statements), ignore_2), close_brace_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), Vec<yul_statement::N>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_1,
            yul_statements,
            ignore_2,
            close_brace_char,
        }
    }
}

impl DefaultTest for yul_block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_1.is_default()
            && self.yul_statements.is_default()
            && self.ignore_2.is_default()
            && self.close_brace_char.is_default()
    }
}

impl assembly_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (((assembly, ignore_1), double_quote_evmasm_double_quote), ignore_2),
                    assembly_flags,
                ),
                ignore_3,
            ),
            yul_block,
        ): (
            (
                (
                    (
                        (
                            (FixedTerminal<8usize>, ignore::N),
                            Option<FixedTerminal<8usize>>,
                        ),
                        ignore::N,
                    ),
                    Option<assembly_flags::N>,
                ),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            assembly,
            ignore_1,
            double_quote_evmasm_double_quote,
            ignore_2,
            assembly_flags,
            ignore_3,
            yul_block,
        }
    }
}

impl DefaultTest for assembly_statement::_T0 {
    fn is_default(&self) -> bool {
        self.assembly.is_default()
            && self.ignore_1.is_default()
            && self.double_quote_evmasm_double_quote.is_default()
            && self.ignore_2.is_default()
            && self.assembly_flags.is_default()
            && self.ignore_3.is_default()
            && self.yul_block.is_default()
    }
}

impl DefaultTest for directive::_T0 {}

impl error_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                ((((error, ignore_1), identifier), ignore_2), open_paren_char),
                                ignore_3,
                            ),
                            error_parameters,
                        ),
                        ignore_4,
                    ),
                    close_paren_char,
                ),
                ignore_5,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((FixedTerminal<5usize>, ignore::N), identifier::N),
                                        ignore::N,
                                    ),
                                    FixedTerminal<1usize>,
                                ),
                                ignore::N,
                            ),
                            Option<Box<error_definition::_T1>>,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            error,
            ignore_1,
            identifier,
            ignore_2,
            open_paren_char,
            ignore_3,
            error_parameters,
            ignore_4,
            close_paren_char,
            ignore_5,
            semicolon_char,
        }
    }
}

impl DefaultTest for error_definition::_T0 {
    fn is_default(&self) -> bool {
        self.error.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_3.is_default()
            && self.error_parameters.is_default()
            && self.ignore_4.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_5.is_default()
            && self.semicolon_char.is_default()
    }
}

impl error_definition::_T1 {
    pub fn new(
        (error_parameters, comma_chars): (Vec<error_parameter::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            error_parameters,
            comma_chars,
        }
    }
}

impl DefaultTest for error_definition::_T1 {
    fn is_default(&self) -> bool {
        self.error_parameters.is_default() && self.comma_chars.is_default()
    }
}

impl event_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (((event, ignore_1), identifier), ignore_2),
                                            open_paren_char,
                                        ),
                                        ignore_3,
                                    ),
                                    event_parameters,
                                ),
                                ignore_4,
                            ),
                            close_paren_char,
                        ),
                        ignore_5,
                    ),
                    anonymous,
                ),
                ignore_6,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                ((FixedTerminal<5usize>, ignore::N), identifier::N),
                                                ignore::N,
                                            ),
                                            FixedTerminal<1usize>,
                                        ),
                                        ignore::N,
                                    ),
                                    Option<Box<event_definition::_T1>>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Option<FixedTerminal<9usize>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            event,
            ignore_1,
            identifier,
            ignore_2,
            open_paren_char,
            ignore_3,
            event_parameters,
            ignore_4,
            close_paren_char,
            ignore_5,
            anonymous,
            ignore_6,
            semicolon_char,
        }
    }
}

impl DefaultTest for event_definition::_T0 {
    fn is_default(&self) -> bool {
        self.event.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_3.is_default()
            && self.event_parameters.is_default()
            && self.ignore_4.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_5.is_default()
            && self.anonymous.is_default()
            && self.ignore_6.is_default()
            && self.semicolon_char.is_default()
    }
}

impl event_definition::_T1 {
    pub fn new(
        (event_parameters, comma_chars): (Vec<event_parameter::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            event_parameters,
            comma_chars,
        }
    }
}

impl DefaultTest for event_definition::_T1 {
    fn is_default(&self) -> bool {
        self.event_parameters.is_default() && self.comma_chars.is_default()
    }
}

impl index_access_expression::_T0 {
    pub fn new(
        ((primary_expression, ignore), _t2s): (
            (primary_expression::N, ignore::N),
            Vec<Box<index_access_expression::_T2>>,
        ),
    ) -> Self {
        Self {
            primary_expression,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for index_access_expression::_T0 {}

impl index_access_expression::_T2 {
    pub fn new(
        (((open_bracket_char, expression), _t3), close_bracket_char): (
            (
                (FixedTerminal<1usize>, Option<expression::N>),
                Option<Box<index_access_expression::_T3>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            _t3,
            close_bracket_char,
        }
    }
}

impl DefaultTest for index_access_expression::_T2 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expression.is_default()
            && self._t3.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl index_access_expression::_T3 {
    pub fn new((colon_char, expression): (FixedTerminal<1usize>, Option<expression::N>)) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}

impl DefaultTest for index_access_expression::_T3 {
    fn is_default(&self) -> bool {
        self.colon_char.is_default() && self.expression.is_default()
    }
}

impl variable_declaration_tuple::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((open_paren_char, ignore_1), comma_chars), ignore_2),
                            variable_declaration,
                        ),
                        ignore_3,
                    ),
                    _t3s,
                ),
                ignore_4,
            ),
            close_paren_char,
        ): (
            (
                (
                    (
                        (
                            (((FixedTerminal<1usize>, ignore::N), usize), ignore::N),
                            variable_declaration::N,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<variable_declaration_tuple::_T3>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_1,
            comma_chars,
            ignore_2,
            variable_declaration,
            ignore_3,
            _t3s,
            ignore_4,
            close_paren_char,
        }
    }
}

impl DefaultTest for variable_declaration_tuple::_T0 {}

impl variable_declaration_tuple::_T3 {
    pub fn new(
        (comma_char, variable_declaration): (
            FixedTerminal<1usize>,
            Option<variable_declaration::N>,
        ),
    ) -> Self {
        Self {
            comma_char,
            variable_declaration,
        }
    }
}

impl DefaultTest for variable_declaration_tuple::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.variable_declaration.is_default()
    }
}

impl member_access_expression::_T0 {
    pub fn new(
        ((index_access_expression, ignore), _t2s): (
            (index_access_expression::N, ignore::N),
            Vec<Box<member_access_expression::_T2>>,
        ),
    ) -> Self {
        Self {
            index_access_expression,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for member_access_expression::_T0 {}

impl member_access_expression::_T2 {
    pub fn new(
        (period_char, _t3): (FixedTerminal<1usize>, Box<member_access_expression::_T3>),
    ) -> Self {
        Self { period_char, _t3 }
    }
}

impl DefaultTest for member_access_expression::_T2 {}

impl DefaultTest for member_access_expression::_T3 {}

impl function_call_options_expression::_T0 {
    pub fn new(
        ((member_access_expression, ignore), _t2s): (
            (member_access_expression::N, ignore::N),
            Vec<Box<function_call_options_expression::_T2>>,
        ),
    ) -> Self {
        Self {
            member_access_expression,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T0 {}

impl function_call_options_expression::_T2 {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (
                FixedTerminal<1usize>,
                Box<function_call_options_expression::_T3>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T2 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.named_arguments.is_default()
            && self.close_brace_char.is_default()
    }
}

impl function_call_options_expression::_T3 {
    pub fn new(
        (named_arguments, comma_chars): (Vec<named_argument::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            named_arguments,
            comma_chars,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T3 {
    fn is_default(&self) -> bool {
        self.named_arguments.is_default() && self.comma_chars.is_default()
    }
}

impl function_call_expression::_T0 {
    pub fn new(
        ((function_call_options_expression, ignore), argument_lists): (
            (function_call_options_expression::N, ignore::N),
            Vec<argument_list::N>,
        ),
    ) -> Self {
        Self {
            function_call_options_expression,
            ignore,
            argument_lists,
        }
    }
}

impl DefaultTest for function_call_expression::_T0 {}

impl unary_prefix_expression::_T0 {
    pub fn new(
        ((_0, ignore), function_call_expression): (
            (Option<usize>, ignore::N),
            function_call_expression::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore,
            function_call_expression,
        }
    }
}

impl DefaultTest for unary_prefix_expression::_T0 {}

impl unary_suffix_expression::_T0 {
    pub fn new(
        ((unary_prefix_expression, ignore), _2): (
            (unary_prefix_expression::N, ignore::N),
            Option<FixedTerminal<2usize>>,
        ),
    ) -> Self {
        Self {
            unary_prefix_expression,
            ignore,
            _2,
        }
    }
}

impl DefaultTest for unary_suffix_expression::_T0 {}

impl exponentiation_expression::_T0 {
    pub fn new(
        (unary_suffix_expressions, star_stars): (
            Vec<unary_suffix_expression::N>,
            Vec<FixedTerminal<2usize>>,
        ),
    ) -> Self {
        Self {
            unary_suffix_expressions,
            star_stars,
        }
    }
}

impl DefaultTest for exponentiation_expression::_T0 {
    fn is_default(&self) -> bool {
        self.unary_suffix_expressions.is_default() && self.star_stars.is_default()
    }
}

impl mul_div_mod_expression::_T0 {
    pub fn new(
        (exponentiation_expressions, _1): (
            Vec<exponentiation_expression::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            exponentiation_expressions,
            _1,
        }
    }
}

impl DefaultTest for mul_div_mod_expression::_T0 {
    fn is_default(&self) -> bool {
        self.exponentiation_expressions.is_default() && self._1.is_default()
    }
}

impl add_sub_expression::_T0 {
    pub fn new(
        (mul_div_mod_expressions, _1): (Vec<mul_div_mod_expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            mul_div_mod_expressions,
            _1,
        }
    }
}

impl DefaultTest for add_sub_expression::_T0 {
    fn is_default(&self) -> bool {
        self.mul_div_mod_expressions.is_default() && self._1.is_default()
    }
}

impl shift_expression::_T0 {
    pub fn new((add_sub_expressions, _1): (Vec<add_sub_expression::N>, Vec<usize>)) -> Self {
        Self {
            add_sub_expressions,
            _1,
        }
    }
}

impl DefaultTest for shift_expression::_T0 {
    fn is_default(&self) -> bool {
        self.add_sub_expressions.is_default() && self._1.is_default()
    }
}

impl bit_and_expression::_T0 {
    pub fn new(
        (shift_expressions, ampersand_chars): (
            Vec<shift_expression::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            shift_expressions,
            ampersand_chars,
        }
    }
}

impl DefaultTest for bit_and_expression::_T0 {
    fn is_default(&self) -> bool {
        self.shift_expressions.is_default() && self.ampersand_chars.is_default()
    }
}

impl bit_x_or_expression::_T0 {
    pub fn new(
        (bit_and_expressions, caret_chars): (
            Vec<bit_and_expression::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            bit_and_expressions,
            caret_chars,
        }
    }
}

impl DefaultTest for bit_x_or_expression::_T0 {
    fn is_default(&self) -> bool {
        self.bit_and_expressions.is_default() && self.caret_chars.is_default()
    }
}

impl bit_or_expression::_T0 {
    pub fn new(
        (bit_x_or_expressions, bar_chars): (
            Vec<bit_x_or_expression::N>,
            Vec<FixedTerminal<1usize>>,
        ),
    ) -> Self {
        Self {
            bit_x_or_expressions,
            bar_chars,
        }
    }
}

impl DefaultTest for bit_or_expression::_T0 {
    fn is_default(&self) -> bool {
        self.bit_x_or_expressions.is_default() && self.bar_chars.is_default()
    }
}

impl order_comparison_expression::_T0 {
    pub fn new((bit_or_expressions, _1): (Vec<bit_or_expression::N>, Vec<usize>)) -> Self {
        Self {
            bit_or_expressions,
            _1,
        }
    }
}

impl DefaultTest for order_comparison_expression::_T0 {
    fn is_default(&self) -> bool {
        self.bit_or_expressions.is_default() && self._1.is_default()
    }
}

impl equality_comparison_expression::_T0 {
    pub fn new(
        (order_comparison_expressions, _1): (
            Vec<order_comparison_expression::N>,
            Vec<FixedTerminal<2usize>>,
        ),
    ) -> Self {
        Self {
            order_comparison_expressions,
            _1,
        }
    }
}

impl DefaultTest for equality_comparison_expression::_T0 {
    fn is_default(&self) -> bool {
        self.order_comparison_expressions.is_default() && self._1.is_default()
    }
}

impl and_expression::_T0 {
    pub fn new(
        (equality_comparison_expressions, ampersand_ampersands): (
            Vec<equality_comparison_expression::N>,
            Vec<FixedTerminal<2usize>>,
        ),
    ) -> Self {
        Self {
            equality_comparison_expressions,
            ampersand_ampersands,
        }
    }
}

impl DefaultTest for and_expression::_T0 {
    fn is_default(&self) -> bool {
        self.equality_comparison_expressions.is_default() && self.ampersand_ampersands.is_default()
    }
}

impl or_expression::_T0 {
    pub fn new(
        (and_expressions, bar_bars): (Vec<and_expression::N>, Vec<FixedTerminal<2usize>>),
    ) -> Self {
        Self {
            and_expressions,
            bar_bars,
        }
    }
}

impl DefaultTest for or_expression::_T0 {
    fn is_default(&self) -> bool {
        self.and_expressions.is_default() && self.bar_bars.is_default()
    }
}

impl conditional_expression::_T0 {
    pub fn new(
        ((or_expression, ignore), _t1): (
            (or_expression::N, ignore::N),
            Option<Box<conditional_expression::_T1>>,
        ),
    ) -> Self {
        Self {
            or_expression,
            ignore,
            _t1,
        }
    }
}

impl DefaultTest for conditional_expression::_T0 {
    fn is_default(&self) -> bool {
        self.or_expression.is_default() && self.ignore.is_default() && self._t1.is_default()
    }
}

impl conditional_expression::_T1 {
    pub fn new(
        (((question_char, expression_1), colon_char), expression_2): (
            (
                (FixedTerminal<1usize>, expression::N),
                FixedTerminal<1usize>,
            ),
            expression::N,
        ),
    ) -> Self {
        Self {
            question_char,
            expression_1,
            colon_char,
            expression_2,
        }
    }
}

impl DefaultTest for conditional_expression::_T1 {
    fn is_default(&self) -> bool {
        self.question_char.is_default()
            && self.expression_1.is_default()
            && self.colon_char.is_default()
            && self.expression_2.is_default()
    }
}

impl assignment_expression::_T0 {
    pub fn new(
        (conditional_expressions, _1): (Vec<conditional_expression::N>, Vec<usize>),
    ) -> Self {
        Self {
            conditional_expressions,
            _1,
        }
    }
}

impl DefaultTest for assignment_expression::_T0 {
    fn is_default(&self) -> bool {
        self.conditional_expressions.is_default() && self._1.is_default()
    }
}

impl expression::_T0 {
    pub fn new(
        (conditional_expressions, _1): (Vec<conditional_expression::N>, Vec<usize>),
    ) -> Self {
        Self {
            conditional_expressions,
            _1,
        }
    }
}

impl DefaultTest for expression::_T0 {
    fn is_default(&self) -> bool {
        self.conditional_expressions.is_default() && self._1.is_default()
    }
}

impl constant_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((type_name, ignore_1), constant), ignore_2), identifier), ignore_3),
                            equal_char,
                        ),
                        ignore_4,
                    ),
                    expression,
                ),
                ignore_5,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((type_name::N, ignore::N), FixedTerminal<8usize>),
                                        ignore::N,
                                    ),
                                    identifier::N,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
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
            type_name,
            ignore_1,
            constant,
            ignore_2,
            identifier,
            ignore_3,
            equal_char,
            ignore_4,
            expression,
            ignore_5,
            semicolon_char,
        }
    }
}

impl DefaultTest for constant_definition::_T0 {}

impl do_while_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((((r#do, ignore_1), statement), ignore_2), r#while),
                                        ignore_3,
                                    ),
                                    open_paren_char,
                                ),
                                ignore_4,
                            ),
                            expression,
                        ),
                        ignore_5,
                    ),
                    close_paren_char,
                ),
                ignore_6,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                ((FixedTerminal<2usize>, ignore::N), statement::N),
                                                ignore::N,
                                            ),
                                            FixedTerminal<5usize>,
                                        ),
                                        ignore::N,
                                    ),
                                    FixedTerminal<1usize>,
                                ),
                                ignore::N,
                            ),
                            expression::N,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#do,
            ignore_1,
            statement,
            ignore_2,
            r#while,
            ignore_3,
            open_paren_char,
            ignore_4,
            expression,
            ignore_5,
            close_paren_char,
            ignore_6,
            semicolon_char,
        }
    }
}

impl DefaultTest for do_while_statement::_T0 {}

impl emit_statement::_T0 {
    pub fn new(
        ((((((emit, ignore_1), expression), ignore_2), argument_list), ignore_3), semicolon_char): (
            (
                (
                    (
                        ((FixedTerminal<4usize>, ignore::N), expression::N),
                        ignore::N,
                    ),
                    argument_list::N,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            emit,
            ignore_1,
            expression,
            ignore_2,
            argument_list,
            ignore_3,
            semicolon_char,
        }
    }
}

impl DefaultTest for emit_statement::_T0 {
    fn is_default(&self) -> bool {
        self.emit.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self.argument_list.is_default()
            && self.ignore_3.is_default()
            && self.semicolon_char.is_default()
    }
}

impl expression_statement::_T0 {
    pub fn new(
        ((expression, ignore), semicolon_char): ((expression::N, ignore::N), FixedTerminal<1usize>),
    ) -> Self {
        Self {
            expression,
            ignore,
            semicolon_char,
        }
    }
}

impl DefaultTest for expression_statement::_T0 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self.ignore.is_default() && self.semicolon_char.is_default()
    }
}

impl if_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                ((((r#if, ignore_1), open_paren_char), ignore_2), expression),
                                ignore_3,
                            ),
                            close_paren_char,
                        ),
                        ignore_4,
                    ),
                    statement,
                ),
                ignore_5,
            ),
            _t1,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((FixedTerminal<2usize>, ignore::N), FixedTerminal<1usize>),
                                        ignore::N,
                                    ),
                                    expression::N,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    statement::N,
                ),
                ignore::N,
            ),
            Option<Box<if_statement::_T1>>,
        ),
    ) -> Self {
        Self {
            r#if,
            ignore_1,
            open_paren_char,
            ignore_2,
            expression,
            ignore_3,
            close_paren_char,
            ignore_4,
            statement,
            ignore_5,
            _t1,
        }
    }
}

impl DefaultTest for if_statement::_T0 {}

impl if_statement::_T1 {
    pub fn new((r#else, statement): (FixedTerminal<4usize>, statement::N)) -> Self {
        Self { r#else, statement }
    }
}

impl DefaultTest for if_statement::_T1 {}

impl return_statement::_T0 {
    pub fn new(
        ((((r#return, ignore_1), expression), ignore_2), semicolon_char): (
            (
                ((FixedTerminal<6usize>, ignore::N), Option<expression::N>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#return,
            ignore_1,
            expression,
            ignore_2,
            semicolon_char,
        }
    }
}

impl DefaultTest for return_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#return.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self.semicolon_char.is_default()
    }
}

impl revert_statement::_T0 {
    pub fn new(
        ((((((revert , ignore_1) , expression) , ignore_2) , argument_list) , ignore_3) , semicolon_char) : ((((((FixedTerminal < 6usize > , ignore :: N) , expression :: N) , ignore :: N) , argument_list :: N) , ignore :: N) , FixedTerminal < 1usize >),
    ) -> Self {
        Self {
            revert,
            ignore_1,
            expression,
            ignore_2,
            argument_list,
            ignore_3,
            semicolon_char,
        }
    }
}

impl DefaultTest for revert_statement::_T0 {
    fn is_default(&self) -> bool {
        self.revert.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self.argument_list.is_default()
            && self.ignore_3.is_default()
            && self.semicolon_char.is_default()
    }
}

impl state_variable_declaration::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((type_name, ignore_1), state_variable_attributes), ignore_2),
                            identifier,
                        ),
                        ignore_3,
                    ),
                    _t2,
                ),
                ignore_4,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                ((type_name::N, ignore::N), Vec<state_variable_attribute::N>),
                                ignore::N,
                            ),
                            identifier::N,
                        ),
                        ignore::N,
                    ),
                    Option<Box<state_variable_declaration::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_1,
            state_variable_attributes,
            ignore_2,
            identifier,
            ignore_3,
            _t2,
            ignore_4,
            semicolon_char,
        }
    }
}

impl DefaultTest for state_variable_declaration::_T0 {}

impl state_variable_declaration::_T2 {
    pub fn new((equal_char, expression): (FixedTerminal<1usize>, expression::N)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}

impl DefaultTest for state_variable_declaration::_T2 {
    fn is_default(&self) -> bool {
        self.equal_char.is_default() && self.expression.is_default()
    }
}

impl try_statement::_T0 {
    pub fn new(
        (
            (((((((r#try, ignore_1), expression), ignore_2), _t1), ignore_3), block), ignore_4),
            catch_clauses,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<3usize>, ignore::N), expression::N),
                                ignore::N,
                            ),
                            Option<Box<try_statement::_T1>>,
                        ),
                        ignore::N,
                    ),
                    block::N,
                ),
                ignore::N,
            ),
            Vec<catch_clause::N>,
        ),
    ) -> Self {
        Self {
            r#try,
            ignore_1,
            expression,
            ignore_2,
            _t1,
            ignore_3,
            block,
            ignore_4,
            catch_clauses,
        }
    }
}

impl DefaultTest for try_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#try.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
            && self.ignore_2.is_default()
            && self._t1.is_default()
            && self.ignore_3.is_default()
            && self.block.is_default()
            && self.ignore_4.is_default()
            && self.catch_clauses.is_default()
    }
}

impl try_statement::_T1 {
    pub fn new(
        (returns, non_empty_parameter_list): (FixedTerminal<7usize>, non_empty_parameter_list::N),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for try_statement::_T1 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl variable_declaration_statement::_T0 {
    pub fn new(
        ((_t1, ignore), semicolon_char): (
            (Box<variable_declaration_statement::_T1>, ignore::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            _t1,
            ignore,
            semicolon_char,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T0 {}

impl DefaultTest for variable_declaration_statement::_T1 {}

impl variable_declaration_statement::_T4 {
    pub fn new(
        ((variable_declaration_tuple, equal_char), expression): (
            (variable_declaration_tuple::N, FixedTerminal<1usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            variable_declaration_tuple,
            equal_char,
            expression,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T4 {}

impl variable_declaration_statement::_T2 {
    pub fn new(
        (variable_declaration, _t3): (
            variable_declaration::N,
            Option<Box<variable_declaration_statement::_T3>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            _t3,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T2 {}

impl variable_declaration_statement::_T3 {
    pub fn new((equal_char, expression): (FixedTerminal<1usize>, expression::N)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T3 {
    fn is_default(&self) -> bool {
        self.equal_char.is_default() && self.expression.is_default()
    }
}

impl while_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#while, ignore_1), open_paren_char), ignore_2), expression), ignore_3),
                    close_paren_char,
                ),
                ignore_4,
            ),
            statement,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<5usize>, ignore::N), FixedTerminal<1usize>),
                                ignore::N,
                            ),
                            expression::N,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            statement::N,
        ),
    ) -> Self {
        Self {
            r#while,
            ignore_1,
            open_paren_char,
            ignore_2,
            expression,
            ignore_3,
            close_paren_char,
            ignore_4,
            statement,
        }
    }
}

impl DefaultTest for while_statement::_T0 {}

impl DefaultTest for simple_statement::_T0 {}

impl for_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((((r#for, ignore_1), open_paren_char), ignore_2), _t1),
                                        ignore_3,
                                    ),
                                    _t2,
                                ),
                                ignore_4,
                            ),
                            expression,
                        ),
                        ignore_5,
                    ),
                    close_paren_char,
                ),
                ignore_6,
            ),
            statement,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                (
                                                    (FixedTerminal<3usize>, ignore::N),
                                                    FixedTerminal<1usize>,
                                                ),
                                                ignore::N,
                                            ),
                                            Box<for_statement::_T1>,
                                        ),
                                        ignore::N,
                                    ),
                                    Box<for_statement::_T2>,
                                ),
                                ignore::N,
                            ),
                            Option<expression::N>,
                        ),
                        ignore::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            statement::N,
        ),
    ) -> Self {
        Self {
            r#for,
            ignore_1,
            open_paren_char,
            ignore_2,
            _t1,
            ignore_3,
            _t2,
            ignore_4,
            expression,
            ignore_5,
            close_paren_char,
            ignore_6,
            statement,
        }
    }
}

impl DefaultTest for for_statement::_T0 {}

impl DefaultTest for for_statement::_T2 {}

impl DefaultTest for for_statement::_T1 {}

impl DefaultTest for statement::_T0 {}

impl block::_T0 {
    pub fn new(
        ((((open_brace_char, ignore_1), _t2s), ignore_2), close_brace_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), Vec<Box<block::_T2>>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_1,
            _t2s,
            ignore_2,
            close_brace_char,
        }
    }
}

impl DefaultTest for block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_1.is_default()
            && self._t2s.is_default()
            && self.ignore_2.is_default()
            && self.close_brace_char.is_default()
    }
}

impl DefaultTest for block::_T2 {}

impl constructor_definition::_T0 {
    pub fn new(
        (
            (
                ((((constructor, ignore_1), parameter_list), ignore_2), constructor_attributes),
                ignore_3,
            ),
            block,
        ): (
            (
                (
                    (
                        ((FixedTerminal<11usize>, ignore::N), parameter_list::N),
                        ignore::N,
                    ),
                    Vec<constructor_attribute::N>,
                ),
                ignore::N,
            ),
            block::N,
        ),
    ) -> Self {
        Self {
            constructor,
            ignore_1,
            parameter_list,
            ignore_2,
            constructor_attributes,
            ignore_3,
            block,
        }
    }
}

impl DefaultTest for constructor_definition::_T0 {
    fn is_default(&self) -> bool {
        self.constructor.is_default()
            && self.ignore_1.is_default()
            && self.parameter_list.is_default()
            && self.ignore_2.is_default()
            && self.constructor_attributes.is_default()
            && self.ignore_3.is_default()
            && self.block.is_default()
    }
}

impl fallback_function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((fallback, ignore_1), parameter_list), ignore_2),
                            fallback_function_attributes,
                        ),
                        ignore_3,
                    ),
                    _t2,
                ),
                ignore_4,
            ),
            _t3,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<8usize>, ignore::N), parameter_list::N),
                                ignore::N,
                            ),
                            Vec<fallback_function_attribute::N>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<fallback_function_definition::_T2>>,
                ),
                ignore::N,
            ),
            Box<fallback_function_definition::_T3>,
        ),
    ) -> Self {
        Self {
            fallback,
            ignore_1,
            parameter_list,
            ignore_2,
            fallback_function_attributes,
            ignore_3,
            _t2,
            ignore_4,
            _t3,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T0 {}

impl DefaultTest for fallback_function_definition::_T3 {}

impl fallback_function_definition::_T2 {
    pub fn new(
        (returns, non_empty_parameter_list): (FixedTerminal<7usize>, non_empty_parameter_list::N),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((function, ignore_1), _t1), ignore_2), parameter_list), ignore_3),
                            function_attributes,
                        ),
                        ignore_4,
                    ),
                    _t3,
                ),
                ignore_5,
            ),
            _t4,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (FixedTerminal<8usize>, ignore::N),
                                            Box<function_definition::_T1>,
                                        ),
                                        ignore::N,
                                    ),
                                    parameter_list::N,
                                ),
                                ignore::N,
                            ),
                            Vec<function_attribute::N>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<function_definition::_T3>>,
                ),
                ignore::N,
            ),
            Box<function_definition::_T4>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_1,
            _t1,
            ignore_2,
            parameter_list,
            ignore_3,
            function_attributes,
            ignore_4,
            _t3,
            ignore_5,
            _t4,
        }
    }
}

impl DefaultTest for function_definition::_T0 {}

impl DefaultTest for function_definition::_T4 {}

impl function_definition::_T3 {
    pub fn new(
        (returns, non_empty_parameter_list): (FixedTerminal<7usize>, non_empty_parameter_list::N),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl DefaultTest for function_definition::_T1 {}

impl modifier_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((modifier, ignore_1), identifier), ignore_2), parameter_list), ignore_3),
                    method_attributes,
                ),
                ignore_4,
            ),
            _t2,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<8usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            Option<parameter_list::N>,
                        ),
                        ignore::N,
                    ),
                    Vec<method_attribute::N>,
                ),
                ignore::N,
            ),
            Box<modifier_definition::_T2>,
        ),
    ) -> Self {
        Self {
            modifier,
            ignore_1,
            identifier,
            ignore_2,
            parameter_list,
            ignore_3,
            method_attributes,
            ignore_4,
            _t2,
        }
    }
}

impl DefaultTest for modifier_definition::_T0 {}

impl DefaultTest for modifier_definition::_T2 {}

impl receive_function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((receive, ignore_1), open_paren_char), ignore_2), close_paren_char),
                        ignore_3,
                    ),
                    receive_function_attributes,
                ),
                ignore_4,
            ),
            _t2,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<7usize>, ignore::N), FixedTerminal<1usize>),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<receive_function_attribute::N>,
                ),
                ignore::N,
            ),
            Box<receive_function_definition::_T2>,
        ),
    ) -> Self {
        Self {
            receive,
            ignore_1,
            open_paren_char,
            ignore_2,
            close_paren_char,
            ignore_3,
            receive_function_attributes,
            ignore_4,
            _t2,
        }
    }
}

impl DefaultTest for receive_function_definition::_T0 {}

impl DefaultTest for receive_function_definition::_T2 {}

impl DefaultTest for contract_body_element::_T0 {}

impl contract_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (((r#abstract, ignore_1), contract), ignore_2),
                                            identifier,
                                        ),
                                        ignore_3,
                                    ),
                                    inheritance_specifier_list,
                                ),
                                ignore_4,
                            ),
                            open_brace_char,
                        ),
                        ignore_5,
                    ),
                    contract_body_elements,
                ),
                ignore_6,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                (
                                                    (Option<FixedTerminal<8usize>>, ignore::N),
                                                    FixedTerminal<8usize>,
                                                ),
                                                ignore::N,
                                            ),
                                            identifier::N,
                                        ),
                                        ignore::N,
                                    ),
                                    Option<inheritance_specifier_list::N>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<contract_body_element::N>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#abstract,
            ignore_1,
            contract,
            ignore_2,
            identifier,
            ignore_3,
            inheritance_specifier_list,
            ignore_4,
            open_brace_char,
            ignore_5,
            contract_body_elements,
            ignore_6,
            close_brace_char,
        }
    }
}

impl DefaultTest for contract_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#abstract.is_default()
            && self.ignore_1.is_default()
            && self.contract.is_default()
            && self.ignore_2.is_default()
            && self.identifier.is_default()
            && self.ignore_3.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.ignore_4.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_5.is_default()
            && self.contract_body_elements.is_default()
            && self.ignore_6.is_default()
            && self.close_brace_char.is_default()
    }
}

impl interface_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (((interface, ignore_1), identifier), ignore_2),
                                    inheritance_specifier_list,
                                ),
                                ignore_3,
                            ),
                            open_brace_char,
                        ),
                        ignore_4,
                    ),
                    contract_body_elements,
                ),
                ignore_5,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((FixedTerminal<9usize>, ignore::N), identifier::N),
                                        ignore::N,
                                    ),
                                    Option<inheritance_specifier_list::N>,
                                ),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<contract_body_element::N>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            interface,
            ignore_1,
            identifier,
            ignore_2,
            inheritance_specifier_list,
            ignore_3,
            open_brace_char,
            ignore_4,
            contract_body_elements,
            ignore_5,
            close_brace_char,
        }
    }
}

impl DefaultTest for interface_definition::_T0 {
    fn is_default(&self) -> bool {
        self.interface.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.ignore_3.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_4.is_default()
            && self.contract_body_elements.is_default()
            && self.ignore_5.is_default()
            && self.close_brace_char.is_default()
    }
}

impl library_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((library, ignore_1), identifier), ignore_2), open_brace_char), ignore_3),
                    contract_body_elements,
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
                                ((FixedTerminal<7usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<contract_body_element::N>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            library,
            ignore_1,
            identifier,
            ignore_2,
            open_brace_char,
            ignore_3,
            contract_body_elements,
            ignore_4,
            close_brace_char,
        }
    }
}

impl DefaultTest for library_definition::_T0 {
    fn is_default(&self) -> bool {
        self.library.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_3.is_default()
            && self.contract_body_elements.is_default()
            && self.ignore_4.is_default()
            && self.close_brace_char.is_default()
    }
}

impl DefaultTest for definition::_T0 {}

impl source_unit::_T0 {
    pub fn new(
        ((((ignore_1, ignore_2), _t2s), ignore_3), end_marker): (
            (
                ((ignore::N, ignore::N), Vec<Box<source_unit::_T2>>),
                ignore::N,
            ),
            (),
        ),
    ) -> Self {
        Self {
            ignore_1,
            ignore_2,
            _t2s,
            ignore_3,
            end_marker,
        }
    }
}

impl DefaultTest for source_unit::_T0 {
    fn is_default(&self) -> bool {
        self.ignore_1.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.end_marker.is_default()
    }
}

impl DefaultTest for source_unit::_T2 {}
