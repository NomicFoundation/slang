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
        (expressions, underscore_chars): (
            Vec<FixedTerminal<1usize>>,
            Vec<Option<FixedTerminal<1usize>>>,
        ),
    ) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}

impl DefaultTest for decimal_integer::_T0 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.underscore_chars.is_default()
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
        ((decimal_integer_0, period_char), decimal_integer_1): (
            (Option<decimal_integer::N>, FixedTerminal<1usize>),
            decimal_integer::N,
        ),
    ) -> Self {
        Self {
            decimal_integer_0,
            period_char,
            decimal_integer_1,
        }
    }
}

impl DefaultTest for decimal_float::_T0 {
    fn is_default(&self) -> bool {
        self.decimal_integer_0.is_default()
            && self.period_char.is_default()
            && self.decimal_integer_1.is_default()
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
        (expressions, underscore_chars): (
            Vec<FixedTerminal<1usize>>,
            Vec<Option<FixedTerminal<1usize>>>,
        ),
    ) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}

impl DefaultTest for hex_number::_T1 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.underscore_chars.is_default()
    }
}

impl DefaultTest for ignore::_T1 {}

impl possibly_separated_pairs_of_hex_digits::_T0 {
    pub fn new(
        (expressions, underscore_chars): (Vec<usize>, Vec<Option<FixedTerminal<1usize>>>),
    ) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}

impl DefaultTest for possibly_separated_pairs_of_hex_digits::_T0 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.underscore_chars.is_default()
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
        ((quote_char_0, possibly_separated_pairs_of_hex_digits), quote_char_1): (
            (
                FixedTerminal<1usize>,
                Option<possibly_separated_pairs_of_hex_digits::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_0,
            possibly_separated_pairs_of_hex_digits,
            quote_char_1,
        }
    }
}

impl DefaultTest for hex_string_literal::_T3 {
    fn is_default(&self) -> bool {
        self.quote_char_0.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.quote_char_1.is_default()
    }
}

impl hex_string_literal::_T2 {
    pub fn new(
        ((double_quote_char_0, possibly_separated_pairs_of_hex_digits), double_quote_char_1): (
            (
                FixedTerminal<1usize>,
                Option<possibly_separated_pairs_of_hex_digits::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_char_0,
            possibly_separated_pairs_of_hex_digits,
            double_quote_char_1,
        }
    }
}

impl DefaultTest for hex_string_literal::_T2 {
    fn is_default(&self) -> bool {
        self.double_quote_char_0.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.double_quote_char_1.is_default()
    }
}

impl DefaultTest for keyword::_T0 {}

impl positional_argument_list::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<positional_argument_list::_T1>>,
            Vec<Box<positional_argument_list::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for positional_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl positional_argument_list::_T2 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for positional_argument_list::_T2 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl positional_argument_list::_T1 {
    pub fn new((expression, ignore): (expression::N, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}

impl DefaultTest for positional_argument_list::_T1 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self.ignore.is_default()
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
        ((double_quote_char_0, runs), double_quote_char_1): (
            (
                FixedTerminal<1usize>,
                Vec<Box<double_quoted_ascii_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_char_0,
            runs,
            double_quote_char_1,
        }
    }
}

impl DefaultTest for double_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.double_quote_char_0.is_default()
            && self.runs.is_default()
            && self.double_quote_char_1.is_default()
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
    pub fn new(
        ((address, ignore), payable): (
            (FixedTerminal<7usize>, ignore::N),
            Option<FixedTerminal<7usize>>,
        ),
    ) -> Self {
        Self {
            address,
            ignore,
            payable,
        }
    }
}

impl DefaultTest for elementary_type_with_payable::_T1 {
    fn is_default(&self) -> bool {
        self.address.is_default() && self.ignore.is_default() && self.payable.is_default()
    }
}

impl DefaultTest for elementary_type_without_payable::_T0 {}

impl numeric_literal::_T0 {
    pub fn new(
        ((_t1, ignore), _1): ((Box<numeric_literal::_T1>, ignore::N), Option<usize>),
    ) -> Self {
        Self { _t1, ignore, _1 }
    }
}

impl DefaultTest for numeric_literal::_T0 {}

impl DefaultTest for numeric_literal::_T1 {}

impl DefaultTest for reserved_word::_T0 {}

impl single_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((quote_char_0, runs), quote_char_1): (
            (
                FixedTerminal<1usize>,
                Vec<Box<single_quoted_ascii_string_literal::Run>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_0,
            runs,
            quote_char_1,
        }
    }
}

impl DefaultTest for single_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.quote_char_0.is_default() && self.runs.is_default() && self.quote_char_1.is_default()
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
        ((((open_paren_char, ignore_0), _t2s), ignore_1), close_paren_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), Box<assembly_flags::_T1>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _t2s,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for assembly_flags::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t2s.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl assembly_flags::_T1 {
    pub fn new(
        (_t2s, _t3s): (Vec<Box<assembly_flags::_T2>>, Vec<Box<assembly_flags::_T3>>),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for assembly_flags::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl assembly_flags::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for assembly_flags::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl assembly_flags::_T2 {
    pub fn new(
        (double_quoted_ascii_string_literal, ignore): (
            double_quoted_ascii_string_literal::N,
            ignore::N,
        ),
    ) -> Self {
        Self {
            double_quoted_ascii_string_literal,
            ignore,
        }
    }
}

impl DefaultTest for assembly_flags::_T2 {
    fn is_default(&self) -> bool {
        self.double_quoted_ascii_string_literal.is_default() && self.ignore.is_default()
    }
}

impl DefaultTest for unicode_string_literal::_T0 {}

impl yul_function_call::_T0 {
    pub fn new(
        ((((((_t1, ignore_0), open_paren_char), ignore_1), _t3s), ignore_2), close_paren_char): (
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
            ignore_0,
            open_paren_char,
            ignore_1,
            _t3s,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for yul_function_call::_T0 {}

impl yul_function_call::_T2 {
    pub fn new(
        (_t3s, _t4s): (
            Vec<Box<yul_function_call::_T3>>,
            Vec<Box<yul_function_call::_T4>>,
        ),
    ) -> Self {
        Self { _t3s, _t4s }
    }
}

impl DefaultTest for yul_function_call::_T2 {
    fn is_default(&self) -> bool {
        self._t3s.is_default() && self._t4s.is_default()
    }
}

impl yul_function_call::_T4 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for yul_function_call::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl yul_function_call::_T3 {
    pub fn new((yul_expression, ignore): (yul_expression::N, ignore::N)) -> Self {
        Self {
            yul_expression,
            ignore,
        }
    }
}

impl DefaultTest for yul_function_call::_T3 {}

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
                                            (((function, ignore_0), yul_identifier), ignore_1),
                                            open_paren_char,
                                        ),
                                        ignore_2,
                                    ),
                                    _t2s,
                                ),
                                ignore_3,
                            ),
                            close_paren_char,
                        ),
                        ignore_4,
                    ),
                    _t4,
                ),
                ignore_5,
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
                    Option<Box<yul_function_definition::_T4>>,
                ),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            function,
            ignore_0,
            yul_identifier,
            ignore_1,
            open_paren_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            _t4,
            ignore_5,
            yul_block,
        }
    }
}

impl DefaultTest for yul_function_definition::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.ignore_0.is_default()
            && self.yul_identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_4.is_default()
            && self._t4.is_default()
            && self.ignore_5.is_default()
            && self.yul_block.is_default()
    }
}

impl yul_function_definition::_T4 {
    pub fn new(
        ((minus_greater, ignore), _t6s): (
            (FixedTerminal<2usize>, ignore::N),
            Box<yul_function_definition::_T5>,
        ),
    ) -> Self {
        Self {
            minus_greater,
            ignore,
            _t6s,
        }
    }
}

impl DefaultTest for yul_function_definition::_T4 {
    fn is_default(&self) -> bool {
        self.minus_greater.is_default() && self.ignore.is_default() && self._t6s.is_default()
    }
}

impl yul_function_definition::_T5 {
    pub fn new(
        (_t6s, _t7s): (
            Vec<Box<yul_function_definition::_T6>>,
            Vec<Box<yul_function_definition::_T7>>,
        ),
    ) -> Self {
        Self { _t6s, _t7s }
    }
}

impl DefaultTest for yul_function_definition::_T5 {
    fn is_default(&self) -> bool {
        self._t6s.is_default() && self._t7s.is_default()
    }
}

impl yul_function_definition::_T7 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for yul_function_definition::_T7 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl yul_function_definition::_T6 {
    pub fn new((yul_identifier, ignore): (yul_identifier::N, ignore::N)) -> Self {
        Self {
            yul_identifier,
            ignore,
        }
    }
}

impl DefaultTest for yul_function_definition::_T6 {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default() && self.ignore.is_default()
    }
}

impl yul_function_definition::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<yul_function_definition::_T2>>,
            Vec<Box<yul_function_definition::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for yul_function_definition::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl yul_function_definition::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for yul_function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl yul_function_definition::_T2 {
    pub fn new((yul_identifier, ignore): (yul_identifier::N, ignore::N)) -> Self {
        Self {
            yul_identifier,
            ignore,
        }
    }
}

impl DefaultTest for yul_function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default() && self.ignore.is_default()
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
    pub fn new(
        (((period_char, ignore_0), _t3), ignore_1): (
            ((FixedTerminal<1usize>, ignore::N), Box<yul_path::_T3>),
            ignore::N,
        ),
    ) -> Self {
        Self {
            period_char,
            ignore_0,
            _t3,
            ignore_1,
        }
    }
}

impl DefaultTest for yul_path::_T2 {}

impl DefaultTest for yul_path::_T3 {}

impl enum_definition::_T0 {
    pub fn new(
        (
            (
                ((((((r#enum, ignore_0), identifier), ignore_1), open_brace_char), ignore_2), _t2s),
                ignore_3,
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
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_brace_char,
        }
    }
}

impl DefaultTest for enum_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#enum.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_brace_char.is_default()
    }
}

impl enum_definition::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<enum_definition::_T2>>,
            Vec<Box<enum_definition::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for enum_definition::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl enum_definition::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for enum_definition::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl enum_definition::_T2 {
    pub fn new((identifier, ignore): (identifier::N, ignore::N)) -> Self {
        Self { identifier, ignore }
    }
}

impl DefaultTest for enum_definition::_T2 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.ignore.is_default()
    }
}

impl identifier_path::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<identifier_path::_T1>>,
            Vec<Box<identifier_path::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for identifier_path::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl identifier_path::_T2 {
    pub fn new((period_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self {
            period_char,
            ignore,
        }
    }
}

impl DefaultTest for identifier_path::_T2 {
    fn is_default(&self) -> bool {
        self.period_char.is_default() && self.ignore.is_default()
    }
}

impl identifier_path::_T1 {
    pub fn new((identifier, ignore): (identifier::N, ignore::N)) -> Self {
        Self { identifier, ignore }
    }
}

impl DefaultTest for identifier_path::_T1 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.ignore.is_default()
    }
}

impl DefaultTest for literal::_T0 {}

impl named_argument::_T0 {
    pub fn new(
        ((((identifier, ignore_0), colon_char), ignore_1), expression): (
            (
                ((identifier::N, ignore::N), FixedTerminal<1usize>),
                ignore::N,
            ),
            expression::N,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore_0,
            colon_char,
            ignore_1,
            expression,
        }
    }
}

impl DefaultTest for named_argument::_T0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default()
            && self.ignore_0.is_default()
            && self.colon_char.is_default()
            && self.ignore_1.is_default()
            && self.expression.is_default()
    }
}

impl parameter_declaration::_T0 {
    pub fn new(
        ((((type_name, ignore_0), _1), ignore_1), identifier): (
            (((type_name::N, ignore::N), Option<usize>), ignore::N),
            Option<identifier::N>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            _1,
            ignore_1,
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
    pub fn new(
        ((r#as, ignore), identifier): ((FixedTerminal<2usize>, ignore::N), identifier::N),
    ) -> Self {
        Self {
            r#as,
            ignore,
            identifier,
        }
    }
}

impl DefaultTest for selected_import::_T1 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.ignore.is_default() && self.identifier.is_default()
    }
}

impl user_defined_value_type_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#type, ignore_0), identifier), ignore_1), is), ignore_2),
                    elementary_type_with_payable,
                ),
                ignore_3,
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
            ignore_0,
            identifier,
            ignore_1,
            is,
            ignore_2,
            elementary_type_with_payable,
            ignore_3,
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
                            (((((mapping, ignore_0), open_paren_char), ignore_1), _t1), ignore_2),
                            equal_greater,
                        ),
                        ignore_3,
                    ),
                    type_name,
                ),
                ignore_4,
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
            ignore_0,
            open_paren_char,
            ignore_1,
            _t1,
            ignore_2,
            equal_greater,
            ignore_3,
            type_name,
            ignore_4,
            close_paren_char,
        }
    }
}

impl DefaultTest for mapping_type::_T0 {}

impl DefaultTest for mapping_type::_T1 {}

impl named_argument_list::_T0 {
    pub fn new(
        ((((open_brace_char, ignore_0), _t2s), ignore_1), close_brace_char): (
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
            ignore_0,
            _t2s,
            ignore_1,
            close_brace_char,
        }
    }
}

impl DefaultTest for named_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_0.is_default()
            && self._t2s.is_default()
            && self.ignore_1.is_default()
            && self.close_brace_char.is_default()
    }
}

impl named_argument_list::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<named_argument_list::_T2>>,
            Vec<Box<named_argument_list::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for named_argument_list::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl named_argument_list::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for named_argument_list::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl named_argument_list::_T2 {
    pub fn new((named_argument, ignore): (named_argument::N, ignore::N)) -> Self {
        Self {
            named_argument,
            ignore,
        }
    }
}

impl DefaultTest for named_argument_list::_T2 {
    fn is_default(&self) -> bool {
        self.named_argument.is_default() && self.ignore.is_default()
    }
}

impl non_empty_parameter_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _t2s), ignore_1), close_paren_char): (
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
            ignore_0,
            _t2s,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for non_empty_parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t2s.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl non_empty_parameter_list::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<non_empty_parameter_list::_T2>>,
            Vec<Box<non_empty_parameter_list::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for non_empty_parameter_list::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl non_empty_parameter_list::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for non_empty_parameter_list::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl non_empty_parameter_list::_T2 {
    pub fn new((parameter_declaration, ignore): (parameter_declaration::N, ignore::N)) -> Self {
        Self {
            parameter_declaration,
            ignore,
        }
    }
}

impl DefaultTest for non_empty_parameter_list::_T2 {}

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
        ((((open_paren_char, ignore_0), _t3s), ignore_1), close_paren_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Box<override_specifier::_T2>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _t3s,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for override_specifier::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t3s.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl override_specifier::_T2 {
    pub fn new(
        (_t3s, _t4s): (
            Vec<Box<override_specifier::_T3>>,
            Vec<Box<override_specifier::_T4>>,
        ),
    ) -> Self {
        Self { _t3s, _t4s }
    }
}

impl DefaultTest for override_specifier::_T2 {
    fn is_default(&self) -> bool {
        self._t3s.is_default() && self._t4s.is_default()
    }
}

impl override_specifier::_T4 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for override_specifier::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl override_specifier::_T3 {
    pub fn new((identifier_path, ignore): (identifier_path::N, ignore::N)) -> Self {
        Self {
            identifier_path,
            ignore,
        }
    }
}

impl DefaultTest for override_specifier::_T3 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.ignore.is_default()
    }
}

impl parameter_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _t2s), ignore_1), close_paren_char): (
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
            ignore_0,
            _t2s,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t2s.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl parameter_list::_T1 {
    pub fn new(
        (_t2s, _t3s): (Vec<Box<parameter_list::_T2>>, Vec<Box<parameter_list::_T3>>),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for parameter_list::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl parameter_list::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for parameter_list::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl parameter_list::_T2 {
    pub fn new((parameter_declaration, ignore): (parameter_declaration::N, ignore::N)) -> Self {
        Self {
            parameter_declaration,
            ignore,
        }
    }
}

impl DefaultTest for parameter_list::_T2 {}

impl selecting_import_directive::_T0 {
    pub fn new(
        (
            (
                (
                    (((((open_brace_char, ignore_0), _t2s), ignore_1), close_brace_char), ignore_2),
                    from,
                ),
                ignore_3,
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
            ignore_0,
            _t2s,
            ignore_1,
            close_brace_char,
            ignore_2,
            from,
            ignore_3,
            import_path,
        }
    }
}

impl DefaultTest for selecting_import_directive::_T0 {}

impl selecting_import_directive::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<selecting_import_directive::_T2>>,
            Vec<Box<selecting_import_directive::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for selecting_import_directive::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl selecting_import_directive::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for selecting_import_directive::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl selecting_import_directive::_T2 {
    pub fn new((selected_import, ignore): (selected_import::N, ignore::N)) -> Self {
        Self {
            selected_import,
            ignore,
        }
    }
}

impl DefaultTest for selecting_import_directive::_T2 {
    fn is_default(&self) -> bool {
        self.selected_import.is_default() && self.ignore.is_default()
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
    pub fn new(
        (((r#as, ignore_0), identifier), ignore_1): (
            ((FixedTerminal<2usize>, ignore::N), identifier::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            r#as,
            ignore_0,
            identifier,
            ignore_1,
        }
    }
}

impl DefaultTest for simple_import_directive::_T2 {
    fn is_default(&self) -> bool {
        self.r#as.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
    }
}

impl star_import_directive::_T0 {
    pub fn new(
        (
            (((((((star_char, ignore_0), r#as), ignore_1), identifier), ignore_2), from), ignore_3),
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
            ignore_0,
            r#as,
            ignore_1,
            identifier,
            ignore_2,
            from,
            ignore_3,
            import_path,
        }
    }
}

impl DefaultTest for star_import_directive::_T0 {}

impl DefaultTest for yul_expression::_T0 {}

impl argument_list::_T0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _t1), ignore_1), close_paren_char): (
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
            ignore_0,
            _t1,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t1.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl DefaultTest for argument_list::_T1 {}

impl catch_clause::_T0 {
    pub fn new(
        ((((catch, ignore_0), _t1), ignore_1), block): (
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
            ignore_0,
            _t1,
            ignore_1,
            block,
        }
    }
}

impl DefaultTest for catch_clause::_T0 {
    fn is_default(&self) -> bool {
        self.catch.is_default()
            && self.ignore_0.is_default()
            && self._t1.is_default()
            && self.ignore_1.is_default()
            && self.block.is_default()
    }
}

impl catch_clause::_T1 {
    pub fn new(
        ((identifier, ignore), non_empty_parameter_list): (
            (Option<identifier::N>, ignore::N),
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for catch_clause::_T1 {
    fn is_default(&self) -> bool {
        self.identifier.is_default()
            && self.ignore.is_default()
            && self.non_empty_parameter_list.is_default()
    }
}

impl function_type::_T0 {
    pub fn new(
        ((((((function, ignore_0), parameter_list), ignore_1), _t2s), ignore_2), _t3): (
            (
                (
                    (
                        ((FixedTerminal<8usize>, ignore::N), parameter_list::N),
                        ignore::N,
                    ),
                    Vec<Box<function_type::_T2>>,
                ),
                ignore::N,
            ),
            Option<Box<function_type::_T3>>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_0,
            parameter_list,
            ignore_1,
            _t2s,
            ignore_2,
            _t3,
        }
    }
}

impl DefaultTest for function_type::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.ignore_0.is_default()
            && self.parameter_list.is_default()
            && self.ignore_1.is_default()
            && self._t2s.is_default()
            && self.ignore_2.is_default()
            && self._t3.is_default()
    }
}

impl function_type::_T3 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (FixedTerminal<7usize>, ignore::N),
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            returns,
            ignore,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for function_type::_T3 {
    fn is_default(&self) -> bool {
        self.returns.is_default()
            && self.ignore.is_default()
            && self.non_empty_parameter_list.is_default()
    }
}

impl function_type::_T2 {
    pub fn new((_0, ignore): (usize, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for function_type::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl import_directive::_T0 {
    pub fn new(
        ((((import, ignore_0), _t1), ignore_1), semicolon_char): (
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
            ignore_0,
            _t1,
            ignore_1,
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
        ((((_t5s, ignore_0), colon_equal), ignore_1), yul_function_call): (
            (
                (
                    (Vec<Box<yul_assignment::_T5>>, ignore::N),
                    FixedTerminal<2usize>,
                ),
                ignore::N,
            ),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            _t5s,
            ignore_0,
            colon_equal,
            ignore_1,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_assignment::_T3 {}

impl yul_assignment::_T5 {
    pub fn new(
        (((comma_char, ignore_0), yul_path), ignore_1): (
            ((FixedTerminal<1usize>, ignore::N), yul_path::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            comma_char,
            ignore_0,
            yul_path,
            ignore_1,
        }
    }
}

impl DefaultTest for yul_assignment::_T5 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default()
            && self.ignore_0.is_default()
            && self.yul_path.is_default()
            && self.ignore_1.is_default()
    }
}

impl yul_assignment::_T2 {
    pub fn new(
        ((colon_equal, ignore), yul_expression): (
            (FixedTerminal<2usize>, ignore::N),
            yul_expression::N,
        ),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
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
                    (((((r#for, ignore_0), yul_block_0), ignore_1), yul_expression), ignore_2),
                    yul_block_1,
                ),
                ignore_3,
            ),
            yul_block_2,
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
            ignore_0,
            yul_block_0,
            ignore_1,
            yul_expression,
            ignore_2,
            yul_block_1,
            ignore_3,
            yul_block_2,
        }
    }
}

impl DefaultTest for yul_for_statement::_T0 {}

impl yul_if_statement::_T0 {
    pub fn new(
        ((((r#if, ignore_0), yul_expression), ignore_1), yul_block): (
            (
                ((FixedTerminal<2usize>, ignore::N), yul_expression::N),
                ignore::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#if,
            ignore_0,
            yul_expression,
            ignore_1,
            yul_block,
        }
    }
}

impl DefaultTest for yul_if_statement::_T0 {}

impl yul_switch_statement::_T0 {
    pub fn new(
        ((((switch, ignore_0), yul_expression), ignore_1), _t1): (
            (
                ((FixedTerminal<6usize>, ignore::N), yul_expression::N),
                ignore::N,
            ),
            Box<yul_switch_statement::_T1>,
        ),
    ) -> Self {
        Self {
            switch,
            ignore_0,
            yul_expression,
            ignore_1,
            _t1,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T0 {}

impl DefaultTest for yul_switch_statement::_T1 {}

impl yul_switch_statement::_T6 {
    pub fn new(
        ((default, ignore), yul_block): ((FixedTerminal<7usize>, ignore::N), yul_block::N),
    ) -> Self {
        Self {
            default,
            ignore,
            yul_block,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T6 {
    fn is_default(&self) -> bool {
        self.default.is_default() && self.ignore.is_default() && self.yul_block.is_default()
    }
}

impl yul_switch_statement::_T2 {
    pub fn new(
        ((_t4s, ignore), _t5): (
            (Vec<Box<yul_switch_statement::_T4>>, ignore::N),
            Option<Box<yul_switch_statement::_T5>>,
        ),
    ) -> Self {
        Self { _t4s, ignore, _t5 }
    }
}

impl DefaultTest for yul_switch_statement::_T2 {
    fn is_default(&self) -> bool {
        self._t4s.is_default() && self.ignore.is_default() && self._t5.is_default()
    }
}

impl yul_switch_statement::_T5 {
    pub fn new(
        ((default, ignore), yul_block): ((FixedTerminal<7usize>, ignore::N), yul_block::N),
    ) -> Self {
        Self {
            default,
            ignore,
            yul_block,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T5 {
    fn is_default(&self) -> bool {
        self.default.is_default() && self.ignore.is_default() && self.yul_block.is_default()
    }
}

impl yul_switch_statement::_T4 {
    pub fn new(
        (((((case, ignore_0), yul_literal), ignore_1), yul_block), ignore): (
            (
                (
                    ((FixedTerminal<4usize>, ignore::N), yul_literal::N),
                    ignore::N,
                ),
                yul_block::N,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            case,
            ignore_0,
            yul_literal,
            ignore_1,
            yul_block,
            ignore,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T4 {}

impl yul_variable_declaration::_T0 {
    pub fn new(
        ((((r#let, ignore_0), yul_identifier), ignore_1), _t1): (
            (
                ((FixedTerminal<3usize>, ignore::N), yul_identifier::N),
                ignore::N,
            ),
            Option<Box<yul_variable_declaration::_T1>>,
        ),
    ) -> Self {
        Self {
            r#let,
            ignore_0,
            yul_identifier,
            ignore_1,
            _t1,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T0 {
    fn is_default(&self) -> bool {
        self.r#let.is_default()
            && self.ignore_0.is_default()
            && self.yul_identifier.is_default()
            && self.ignore_1.is_default()
            && self._t1.is_default()
    }
}

impl DefaultTest for yul_variable_declaration::_T1 {}

impl yul_variable_declaration::_T3 {
    pub fn new(
        ((_t4, ignore), _t5): (
            (Option<Box<yul_variable_declaration::_T4>>, ignore::N),
            Option<Box<yul_variable_declaration::_T5>>,
        ),
    ) -> Self {
        Self { _t4, ignore, _t5 }
    }
}

impl DefaultTest for yul_variable_declaration::_T3 {
    fn is_default(&self) -> bool {
        self._t4.is_default() && self.ignore.is_default() && self._t5.is_default()
    }
}

impl yul_variable_declaration::_T5 {
    pub fn new(
        ((colon_equal, ignore), yul_function_call): (
            (FixedTerminal<2usize>, ignore::N),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T5 {}

impl yul_variable_declaration::_T4 {
    pub fn new(
        ((comma_char, ignore), yul_identifier): (
            (FixedTerminal<1usize>, ignore::N),
            yul_identifier::N,
        ),
    ) -> Self {
        Self {
            comma_char,
            ignore,
            yul_identifier,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default() && self.yul_identifier.is_default()
    }
}

impl yul_variable_declaration::_T2 {
    pub fn new(
        ((colon_equal, ignore), yul_expression): (
            (FixedTerminal<2usize>, ignore::N),
            yul_expression::N,
        ),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
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
        (((((open_bracket_char, ignore_0), expression), ignore_1), close_bracket_char), ignore): (
            (
                (
                    ((FixedTerminal<1usize>, ignore::N), Option<expression::N>),
                    ignore::N,
                ),
                FixedTerminal<1usize>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            expression,
            ignore_1,
            close_bracket_char,
            ignore,
        }
    }
}

impl DefaultTest for type_name::_T3 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self.close_bracket_char.is_default()
            && self.ignore.is_default()
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
        ((((type_name, ignore_0), indexed), ignore_1), identifier): (
            (
                ((type_name::N, ignore::N), Option<FixedTerminal<7usize>>),
                ignore::N,
            ),
            Option<identifier::N>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            indexed,
            ignore_1,
            identifier,
        }
    }
}

impl DefaultTest for event_parameter::_T0 {}

impl DefaultTest for fallback_function_attribute::_T0 {}

impl DefaultTest for function_attribute::_T0 {}

impl inheritance_specifier_list::_T0 {
    pub fn new(
        ((is, ignore), _t2s): (
            (FixedTerminal<2usize>, ignore::N),
            Box<inheritance_specifier_list::_T1>,
        ),
    ) -> Self {
        Self { is, ignore, _t2s }
    }
}

impl DefaultTest for inheritance_specifier_list::_T0 {
    fn is_default(&self) -> bool {
        self.is.is_default() && self.ignore.is_default() && self._t2s.is_default()
    }
}

impl inheritance_specifier_list::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<inheritance_specifier_list::_T2>>,
            Vec<Box<inheritance_specifier_list::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for inheritance_specifier_list::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl inheritance_specifier_list::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for inheritance_specifier_list::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl inheritance_specifier_list::_T2 {
    pub fn new((inheritance_specifier, ignore): (inheritance_specifier::N, ignore::N)) -> Self {
        Self {
            inheritance_specifier,
            ignore,
        }
    }
}

impl DefaultTest for inheritance_specifier_list::_T2 {
    fn is_default(&self) -> bool {
        self.inheritance_specifier.is_default() && self.ignore.is_default()
    }
}

impl DefaultTest for primary_expression::_T0 {}

impl primary_expression::_T8 {
    pub fn new(
        ((((open_bracket_char, ignore_0), _t10s), ignore_1), close_bracket_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Box<primary_expression::_T9>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            _t10s,
            ignore_1,
            close_bracket_char,
        }
    }
}

impl DefaultTest for primary_expression::_T8 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.ignore_0.is_default()
            && self._t10s.is_default()
            && self.ignore_1.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl primary_expression::_T9 {
    pub fn new(
        (_t10s, _t11s): (
            Vec<Box<primary_expression::_T10>>,
            Vec<Box<primary_expression::_T11>>,
        ),
    ) -> Self {
        Self { _t10s, _t11s }
    }
}

impl DefaultTest for primary_expression::_T9 {
    fn is_default(&self) -> bool {
        self._t10s.is_default() && self._t11s.is_default()
    }
}

impl primary_expression::_T11 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for primary_expression::_T11 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl primary_expression::_T10 {
    pub fn new((expression, ignore): (expression::N, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}

impl DefaultTest for primary_expression::_T10 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self.ignore.is_default()
    }
}

impl primary_expression::_T4 {
    pub fn new(
        ((((open_paren_char, ignore_0), _t6s), ignore_1), close_paren_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Box<primary_expression::_T5>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _t6s,
            ignore_1,
            close_paren_char,
        }
    }
}

impl DefaultTest for primary_expression::_T4 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.ignore_0.is_default()
            && self._t6s.is_default()
            && self.ignore_1.is_default()
            && self.close_paren_char.is_default()
    }
}

impl primary_expression::_T5 {
    pub fn new(
        (_t6s, _t7s): (
            Vec<Option<Box<primary_expression::_T6>>>,
            Vec<Box<primary_expression::_T7>>,
        ),
    ) -> Self {
        Self { _t6s, _t7s }
    }
}

impl DefaultTest for primary_expression::_T5 {
    fn is_default(&self) -> bool {
        self._t6s.is_default() && self._t7s.is_default()
    }
}

impl primary_expression::_T7 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for primary_expression::_T7 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl primary_expression::_T6 {
    pub fn new((expression, ignore): (expression::N, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}

impl DefaultTest for primary_expression::_T6 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self.ignore.is_default()
    }
}

impl primary_expression::_T3 {
    pub fn new(
        ((new, ignore), type_name): ((FixedTerminal<3usize>, ignore::N), type_name::N),
    ) -> Self {
        Self {
            new,
            ignore,
            type_name,
        }
    }
}

impl DefaultTest for primary_expression::_T3 {}

impl primary_expression::_T2 {
    pub fn new(
        (
            (((((r#type, ignore_0), open_paren_char), ignore_1), type_name), ignore_2),
            close_paren_char,
        ): (
            (
                (
                    (
                        ((FixedTerminal<4usize>, ignore::N), FixedTerminal<1usize>),
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
            r#type,
            ignore_0,
            open_paren_char,
            ignore_1,
            type_name,
            ignore_2,
            close_paren_char,
        }
    }
}

impl DefaultTest for primary_expression::_T2 {}

impl primary_expression::_T1 {
    pub fn new(
        ((payable, ignore), argument_list): ((FixedTerminal<7usize>, ignore::N), argument_list::N),
    ) -> Self {
        Self {
            payable,
            ignore,
            argument_list,
        }
    }
}

impl DefaultTest for primary_expression::_T1 {
    fn is_default(&self) -> bool {
        self.payable.is_default() && self.ignore.is_default() && self.argument_list.is_default()
    }
}

impl DefaultTest for receive_function_attribute::_T0 {}

impl struct_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#struct, ignore_0), identifier), ignore_1), open_brace_char), ignore_2),
                    _t2s,
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
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_brace_char,
        }
    }
}

impl DefaultTest for struct_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#struct.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_brace_char.is_default()
    }
}

impl struct_definition::_T2 {
    pub fn new(
        (((((type_name, ignore_0), identifier), ignore_1), semicolon_char), ignore): (
            (
                (((type_name::N, ignore::N), identifier::N), ignore::N),
                FixedTerminal<1usize>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            identifier,
            ignore_1,
            semicolon_char,
            ignore,
        }
    }
}

impl DefaultTest for struct_definition::_T2 {}

impl using_directive::_T0 {
    pub fn new(
        (
            (
                (
                    (((((((using, ignore_0), _t1), ignore_1), r#for), ignore_2), _t6), ignore_3),
                    global,
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
                            Box<using_directive::_T6>,
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
            ignore_0,
            _t1,
            ignore_1,
            r#for,
            ignore_2,
            _t6,
            ignore_3,
            global,
            ignore_4,
            semicolon_char,
        }
    }
}

impl DefaultTest for using_directive::_T0 {}

impl DefaultTest for using_directive::_T6 {}

impl DefaultTest for using_directive::_T1 {}

impl using_directive::_T2 {
    pub fn new(
        ((((open_brace_char, ignore_0), _t4s), ignore_1), close_brace_char): (
            (
                (
                    (FixedTerminal<1usize>, ignore::N),
                    Box<using_directive::_T3>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _t4s,
            ignore_1,
            close_brace_char,
        }
    }
}

impl DefaultTest for using_directive::_T2 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_0.is_default()
            && self._t4s.is_default()
            && self.ignore_1.is_default()
            && self.close_brace_char.is_default()
    }
}

impl using_directive::_T3 {
    pub fn new(
        (_t4s, _t5s): (
            Vec<Box<using_directive::_T4>>,
            Vec<Box<using_directive::_T5>>,
        ),
    ) -> Self {
        Self { _t4s, _t5s }
    }
}

impl DefaultTest for using_directive::_T3 {
    fn is_default(&self) -> bool {
        self._t4s.is_default() && self._t5s.is_default()
    }
}

impl using_directive::_T5 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for using_directive::_T5 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl using_directive::_T4 {
    pub fn new((identifier_path, ignore): (identifier_path::N, ignore::N)) -> Self {
        Self {
            identifier_path,
            ignore,
        }
    }
}

impl DefaultTest for using_directive::_T4 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.ignore.is_default()
    }
}

impl variable_declaration::_T0 {
    pub fn new(
        ((((type_name, ignore_0), _1), ignore_1), identifier): (
            (((type_name::N, ignore::N), Option<usize>), ignore::N),
            identifier::N,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            _1,
            ignore_1,
            identifier,
        }
    }
}

impl DefaultTest for variable_declaration::_T0 {}

impl yul_block::_T0 {
    pub fn new(
        ((((open_brace_char, ignore_0), _t2s), ignore_1), close_brace_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), Vec<Box<yul_block::_T2>>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _t2s,
            ignore_1,
            close_brace_char,
        }
    }
}

impl DefaultTest for yul_block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_0.is_default()
            && self._t2s.is_default()
            && self.ignore_1.is_default()
            && self.close_brace_char.is_default()
    }
}

impl yul_block::_T2 {
    pub fn new((yul_statement, ignore): (yul_statement::N, ignore::N)) -> Self {
        Self {
            yul_statement,
            ignore,
        }
    }
}

impl DefaultTest for yul_block::_T2 {}

impl assembly_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (((assembly, ignore_0), double_quote_evmasm_double_quote), ignore_1),
                    assembly_flags,
                ),
                ignore_2,
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
            ignore_0,
            double_quote_evmasm_double_quote,
            ignore_1,
            assembly_flags,
            ignore_2,
            yul_block,
        }
    }
}

impl DefaultTest for assembly_statement::_T0 {
    fn is_default(&self) -> bool {
        self.assembly.is_default()
            && self.ignore_0.is_default()
            && self.double_quote_evmasm_double_quote.is_default()
            && self.ignore_1.is_default()
            && self.assembly_flags.is_default()
            && self.ignore_2.is_default()
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
                                ((((error, ignore_0), identifier), ignore_1), open_paren_char),
                                ignore_2,
                            ),
                            _t2s,
                        ),
                        ignore_3,
                    ),
                    close_paren_char,
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
            ignore_0,
            identifier,
            ignore_1,
            open_paren_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            semicolon_char,
        }
    }
}

impl DefaultTest for error_definition::_T0 {
    fn is_default(&self) -> bool {
        self.error.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_4.is_default()
            && self.semicolon_char.is_default()
    }
}

impl error_definition::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<error_definition::_T2>>,
            Vec<Box<error_definition::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for error_definition::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl error_definition::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for error_definition::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl error_definition::_T2 {
    pub fn new((error_parameter, ignore): (error_parameter::N, ignore::N)) -> Self {
        Self {
            error_parameter,
            ignore,
        }
    }
}

impl DefaultTest for error_definition::_T2 {}

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
                                            (((event, ignore_0), identifier), ignore_1),
                                            open_paren_char,
                                        ),
                                        ignore_2,
                                    ),
                                    _t2s,
                                ),
                                ignore_3,
                            ),
                            close_paren_char,
                        ),
                        ignore_4,
                    ),
                    anonymous,
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
            ignore_0,
            identifier,
            ignore_1,
            open_paren_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            anonymous,
            ignore_5,
            semicolon_char,
        }
    }
}

impl DefaultTest for event_definition::_T0 {
    fn is_default(&self) -> bool {
        self.event.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_paren_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_paren_char.is_default()
            && self.ignore_4.is_default()
            && self.anonymous.is_default()
            && self.ignore_5.is_default()
            && self.semicolon_char.is_default()
    }
}

impl event_definition::_T1 {
    pub fn new(
        (_t2s, _t3s): (
            Vec<Box<event_definition::_T2>>,
            Vec<Box<event_definition::_T3>>,
        ),
    ) -> Self {
        Self { _t2s, _t3s }
    }
}

impl DefaultTest for event_definition::_T1 {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self._t3s.is_default()
    }
}

impl event_definition::_T3 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for event_definition::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl event_definition::_T2 {
    pub fn new((event_parameter, ignore): (event_parameter::N, ignore::N)) -> Self {
        Self {
            event_parameter,
            ignore,
        }
    }
}

impl DefaultTest for event_definition::_T2 {}

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
        (
            (
                (((((open_bracket_char, ignore_0), expression), ignore_1), _t3), ignore_2),
                close_bracket_char,
            ),
            ignore,
        ): (
            (
                (
                    (
                        (
                            ((FixedTerminal<1usize>, ignore::N), Option<expression::N>),
                            ignore::N,
                        ),
                        Option<Box<index_access_expression::_T3>>,
                    ),
                    ignore::N,
                ),
                FixedTerminal<1usize>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            expression,
            ignore_1,
            _t3,
            ignore_2,
            close_bracket_char,
            ignore,
        }
    }
}

impl DefaultTest for index_access_expression::_T2 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self._t3.is_default()
            && self.ignore_2.is_default()
            && self.close_bracket_char.is_default()
            && self.ignore.is_default()
    }
}

impl index_access_expression::_T3 {
    pub fn new(
        ((colon_char, ignore), expression): (
            (FixedTerminal<1usize>, ignore::N),
            Option<expression::N>,
        ),
    ) -> Self {
        Self {
            colon_char,
            ignore,
            expression,
        }
    }
}

impl DefaultTest for index_access_expression::_T3 {
    fn is_default(&self) -> bool {
        self.colon_char.is_default() && self.ignore.is_default() && self.expression.is_default()
    }
}

impl variable_declaration_tuple::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((open_paren_char, ignore_0), _t2s), ignore_1), variable_declaration),
                        ignore_2,
                    ),
                    _t4s,
                ),
                ignore_3,
            ),
            close_paren_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (FixedTerminal<1usize>, ignore::N),
                                    Vec<Box<variable_declaration_tuple::_T2>>,
                                ),
                                ignore::N,
                            ),
                            variable_declaration::N,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<variable_declaration_tuple::_T4>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _t2s,
            ignore_1,
            variable_declaration,
            ignore_2,
            _t4s,
            ignore_3,
            close_paren_char,
        }
    }
}

impl DefaultTest for variable_declaration_tuple::_T0 {}

impl variable_declaration_tuple::_T4 {
    pub fn new(
        (((comma_char, ignore_0), variable_declaration), ignore_1): (
            (
                (FixedTerminal<1usize>, ignore::N),
                Option<variable_declaration::N>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            comma_char,
            ignore_0,
            variable_declaration,
            ignore_1,
        }
    }
}

impl DefaultTest for variable_declaration_tuple::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default()
            && self.ignore_0.is_default()
            && self.variable_declaration.is_default()
            && self.ignore_1.is_default()
    }
}

impl variable_declaration_tuple::_T2 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for variable_declaration_tuple::_T2 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
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
        (((period_char, ignore_0), _t3), ignore_1): (
            (
                (FixedTerminal<1usize>, ignore::N),
                Box<member_access_expression::_T3>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            period_char,
            ignore_0,
            _t3,
            ignore_1,
        }
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
        (((((open_brace_char, ignore_0), _t4s), ignore_1), close_brace_char), ignore): (
            (
                (
                    (
                        (FixedTerminal<1usize>, ignore::N),
                        Box<function_call_options_expression::_T3>,
                    ),
                    ignore::N,
                ),
                FixedTerminal<1usize>,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _t4s,
            ignore_1,
            close_brace_char,
            ignore,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T2 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_0.is_default()
            && self._t4s.is_default()
            && self.ignore_1.is_default()
            && self.close_brace_char.is_default()
            && self.ignore.is_default()
    }
}

impl function_call_options_expression::_T3 {
    pub fn new(
        (_t4s, _t5s): (
            Vec<Box<function_call_options_expression::_T4>>,
            Vec<Box<function_call_options_expression::_T5>>,
        ),
    ) -> Self {
        Self { _t4s, _t5s }
    }
}

impl DefaultTest for function_call_options_expression::_T3 {
    fn is_default(&self) -> bool {
        self._t4s.is_default() && self._t5s.is_default()
    }
}

impl function_call_options_expression::_T5 {
    pub fn new((comma_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}

impl DefaultTest for function_call_options_expression::_T5 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.ignore.is_default()
    }
}

impl function_call_options_expression::_T4 {
    pub fn new((named_argument, ignore): (named_argument::N, ignore::N)) -> Self {
        Self {
            named_argument,
            ignore,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T4 {
    fn is_default(&self) -> bool {
        self.named_argument.is_default() && self.ignore.is_default()
    }
}

impl function_call_expression::_T0 {
    pub fn new(
        ((function_call_options_expression, ignore), _t2s): (
            (function_call_options_expression::N, ignore::N),
            Vec<Box<function_call_expression::_T2>>,
        ),
    ) -> Self {
        Self {
            function_call_options_expression,
            ignore,
            _t2s,
        }
    }
}

impl DefaultTest for function_call_expression::_T0 {}

impl function_call_expression::_T2 {
    pub fn new((argument_list, ignore): (argument_list::N, ignore::N)) -> Self {
        Self {
            argument_list,
            ignore,
        }
    }
}

impl DefaultTest for function_call_expression::_T2 {
    fn is_default(&self) -> bool {
        self.argument_list.is_default() && self.ignore.is_default()
    }
}

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
        ((unary_prefix_expression, ignore), _1): (
            (unary_prefix_expression::N, ignore::N),
            Option<FixedTerminal<2usize>>,
        ),
    ) -> Self {
        Self {
            unary_prefix_expression,
            ignore,
            _1,
        }
    }
}

impl DefaultTest for unary_suffix_expression::_T0 {}

impl exponentiation_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<exponentiation_expression::_T1>>,
            Vec<Box<exponentiation_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for exponentiation_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl exponentiation_expression::_T2 {
    pub fn new((star_star, ignore): (FixedTerminal<2usize>, ignore::N)) -> Self {
        Self { star_star, ignore }
    }
}

impl DefaultTest for exponentiation_expression::_T2 {
    fn is_default(&self) -> bool {
        self.star_star.is_default() && self.ignore.is_default()
    }
}

impl exponentiation_expression::_T1 {
    pub fn new((unary_suffix_expression, ignore): (unary_suffix_expression::N, ignore::N)) -> Self {
        Self {
            unary_suffix_expression,
            ignore,
        }
    }
}

impl DefaultTest for exponentiation_expression::_T1 {}

impl mul_div_mod_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<mul_div_mod_expression::_T1>>,
            Vec<Box<mul_div_mod_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for mul_div_mod_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl mul_div_mod_expression::_T2 {
    pub fn new((_0, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for mul_div_mod_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl mul_div_mod_expression::_T1 {
    pub fn new(
        (exponentiation_expression, ignore): (exponentiation_expression::N, ignore::N),
    ) -> Self {
        Self {
            exponentiation_expression,
            ignore,
        }
    }
}

impl DefaultTest for mul_div_mod_expression::_T1 {
    fn is_default(&self) -> bool {
        self.exponentiation_expression.is_default() && self.ignore.is_default()
    }
}

impl add_sub_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<add_sub_expression::_T1>>,
            Vec<Box<add_sub_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for add_sub_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl add_sub_expression::_T2 {
    pub fn new((_0, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for add_sub_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl add_sub_expression::_T1 {
    pub fn new((mul_div_mod_expression, ignore): (mul_div_mod_expression::N, ignore::N)) -> Self {
        Self {
            mul_div_mod_expression,
            ignore,
        }
    }
}

impl DefaultTest for add_sub_expression::_T1 {
    fn is_default(&self) -> bool {
        self.mul_div_mod_expression.is_default() && self.ignore.is_default()
    }
}

impl shift_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<shift_expression::_T1>>,
            Vec<Box<shift_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for shift_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl shift_expression::_T2 {
    pub fn new((_0, ignore): (usize, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for shift_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl shift_expression::_T1 {
    pub fn new((add_sub_expression, ignore): (add_sub_expression::N, ignore::N)) -> Self {
        Self {
            add_sub_expression,
            ignore,
        }
    }
}

impl DefaultTest for shift_expression::_T1 {
    fn is_default(&self) -> bool {
        self.add_sub_expression.is_default() && self.ignore.is_default()
    }
}

impl bit_and_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<bit_and_expression::_T1>>,
            Vec<Box<bit_and_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for bit_and_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl bit_and_expression::_T2 {
    pub fn new((ampersand_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self {
            ampersand_char,
            ignore,
        }
    }
}

impl DefaultTest for bit_and_expression::_T2 {
    fn is_default(&self) -> bool {
        self.ampersand_char.is_default() && self.ignore.is_default()
    }
}

impl bit_and_expression::_T1 {
    pub fn new((shift_expression, ignore): (shift_expression::N, ignore::N)) -> Self {
        Self {
            shift_expression,
            ignore,
        }
    }
}

impl DefaultTest for bit_and_expression::_T1 {
    fn is_default(&self) -> bool {
        self.shift_expression.is_default() && self.ignore.is_default()
    }
}

impl bit_x_or_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<bit_x_or_expression::_T1>>,
            Vec<Box<bit_x_or_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for bit_x_or_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl bit_x_or_expression::_T2 {
    pub fn new((caret_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { caret_char, ignore }
    }
}

impl DefaultTest for bit_x_or_expression::_T2 {
    fn is_default(&self) -> bool {
        self.caret_char.is_default() && self.ignore.is_default()
    }
}

impl bit_x_or_expression::_T1 {
    pub fn new((bit_and_expression, ignore): (bit_and_expression::N, ignore::N)) -> Self {
        Self {
            bit_and_expression,
            ignore,
        }
    }
}

impl DefaultTest for bit_x_or_expression::_T1 {
    fn is_default(&self) -> bool {
        self.bit_and_expression.is_default() && self.ignore.is_default()
    }
}

impl bit_or_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<bit_or_expression::_T1>>,
            Vec<Box<bit_or_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for bit_or_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl bit_or_expression::_T2 {
    pub fn new((bar_char, ignore): (FixedTerminal<1usize>, ignore::N)) -> Self {
        Self { bar_char, ignore }
    }
}

impl DefaultTest for bit_or_expression::_T2 {
    fn is_default(&self) -> bool {
        self.bar_char.is_default() && self.ignore.is_default()
    }
}

impl bit_or_expression::_T1 {
    pub fn new((bit_x_or_expression, ignore): (bit_x_or_expression::N, ignore::N)) -> Self {
        Self {
            bit_x_or_expression,
            ignore,
        }
    }
}

impl DefaultTest for bit_or_expression::_T1 {
    fn is_default(&self) -> bool {
        self.bit_x_or_expression.is_default() && self.ignore.is_default()
    }
}

impl order_comparison_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<order_comparison_expression::_T1>>,
            Vec<Box<order_comparison_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for order_comparison_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl order_comparison_expression::_T2 {
    pub fn new((_0, ignore): (usize, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for order_comparison_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl order_comparison_expression::_T1 {
    pub fn new((bit_or_expression, ignore): (bit_or_expression::N, ignore::N)) -> Self {
        Self {
            bit_or_expression,
            ignore,
        }
    }
}

impl DefaultTest for order_comparison_expression::_T1 {
    fn is_default(&self) -> bool {
        self.bit_or_expression.is_default() && self.ignore.is_default()
    }
}

impl equality_comparison_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<equality_comparison_expression::_T1>>,
            Vec<Box<equality_comparison_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for equality_comparison_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl equality_comparison_expression::_T2 {
    pub fn new((_0, ignore): (FixedTerminal<2usize>, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for equality_comparison_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl equality_comparison_expression::_T1 {
    pub fn new(
        (order_comparison_expression, ignore): (order_comparison_expression::N, ignore::N),
    ) -> Self {
        Self {
            order_comparison_expression,
            ignore,
        }
    }
}

impl DefaultTest for equality_comparison_expression::_T1 {
    fn is_default(&self) -> bool {
        self.order_comparison_expression.is_default() && self.ignore.is_default()
    }
}

impl and_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (Vec<Box<and_expression::_T1>>, Vec<Box<and_expression::_T2>>),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for and_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl and_expression::_T2 {
    pub fn new((ampersand_ampersand, ignore): (FixedTerminal<2usize>, ignore::N)) -> Self {
        Self {
            ampersand_ampersand,
            ignore,
        }
    }
}

impl DefaultTest for and_expression::_T2 {
    fn is_default(&self) -> bool {
        self.ampersand_ampersand.is_default() && self.ignore.is_default()
    }
}

impl and_expression::_T1 {
    pub fn new(
        (equality_comparison_expression, ignore): (equality_comparison_expression::N, ignore::N),
    ) -> Self {
        Self {
            equality_comparison_expression,
            ignore,
        }
    }
}

impl DefaultTest for and_expression::_T1 {
    fn is_default(&self) -> bool {
        self.equality_comparison_expression.is_default() && self.ignore.is_default()
    }
}

impl or_expression::_T0 {
    pub fn new((_t1s, _t2s): (Vec<Box<or_expression::_T1>>, Vec<Box<or_expression::_T2>>)) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for or_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl or_expression::_T2 {
    pub fn new((bar_bar, ignore): (FixedTerminal<2usize>, ignore::N)) -> Self {
        Self { bar_bar, ignore }
    }
}

impl DefaultTest for or_expression::_T2 {
    fn is_default(&self) -> bool {
        self.bar_bar.is_default() && self.ignore.is_default()
    }
}

impl or_expression::_T1 {
    pub fn new((and_expression, ignore): (and_expression::N, ignore::N)) -> Self {
        Self {
            and_expression,
            ignore,
        }
    }
}

impl DefaultTest for or_expression::_T1 {
    fn is_default(&self) -> bool {
        self.and_expression.is_default() && self.ignore.is_default()
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
        (
            (((((question_char, ignore_0), expression_0), ignore_1), colon_char), ignore_2),
            expression_1,
        ): (
            (
                (
                    (
                        ((FixedTerminal<1usize>, ignore::N), expression::N),
                        ignore::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            expression::N,
        ),
    ) -> Self {
        Self {
            question_char,
            ignore_0,
            expression_0,
            ignore_1,
            colon_char,
            ignore_2,
            expression_1,
        }
    }
}

impl DefaultTest for conditional_expression::_T1 {
    fn is_default(&self) -> bool {
        self.question_char.is_default()
            && self.ignore_0.is_default()
            && self.expression_0.is_default()
            && self.ignore_1.is_default()
            && self.colon_char.is_default()
            && self.ignore_2.is_default()
            && self.expression_1.is_default()
    }
}

impl assignment_expression::_T0 {
    pub fn new(
        (_t1s, _t2s): (
            Vec<Box<assignment_expression::_T1>>,
            Vec<Box<assignment_expression::_T2>>,
        ),
    ) -> Self {
        Self { _t1s, _t2s }
    }
}

impl DefaultTest for assignment_expression::_T0 {
    fn is_default(&self) -> bool {
        self._t1s.is_default() && self._t2s.is_default()
    }
}

impl assignment_expression::_T2 {
    pub fn new((_0, ignore): (usize, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}

impl DefaultTest for assignment_expression::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.ignore.is_default()
    }
}

impl assignment_expression::_T1 {
    pub fn new((conditional_expression, ignore): (conditional_expression::N, ignore::N)) -> Self {
        Self {
            conditional_expression,
            ignore,
        }
    }
}

impl DefaultTest for assignment_expression::_T1 {
    fn is_default(&self) -> bool {
        self.conditional_expression.is_default() && self.ignore.is_default()
    }
}

impl constant_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((type_name, ignore_0), constant), ignore_1), identifier), ignore_2),
                            equal_char,
                        ),
                        ignore_3,
                    ),
                    expression,
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
            ignore_0,
            constant,
            ignore_1,
            identifier,
            ignore_2,
            equal_char,
            ignore_3,
            expression,
            ignore_4,
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
                                        ((((r#do, ignore_0), statement), ignore_1), r#while),
                                        ignore_2,
                                    ),
                                    open_paren_char,
                                ),
                                ignore_3,
                            ),
                            expression,
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
            ignore_0,
            statement,
            ignore_1,
            r#while,
            ignore_2,
            open_paren_char,
            ignore_3,
            expression,
            ignore_4,
            close_paren_char,
            ignore_5,
            semicolon_char,
        }
    }
}

impl DefaultTest for do_while_statement::_T0 {}

impl emit_statement::_T0 {
    pub fn new(
        ((((((emit, ignore_0), expression), ignore_1), argument_list), ignore_2), semicolon_char): (
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
            ignore_0,
            expression,
            ignore_1,
            argument_list,
            ignore_2,
            semicolon_char,
        }
    }
}

impl DefaultTest for emit_statement::_T0 {
    fn is_default(&self) -> bool {
        self.emit.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self.argument_list.is_default()
            && self.ignore_2.is_default()
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
                                ((((r#if, ignore_0), open_paren_char), ignore_1), expression),
                                ignore_2,
                            ),
                            close_paren_char,
                        ),
                        ignore_3,
                    ),
                    statement,
                ),
                ignore_4,
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
            ignore_0,
            open_paren_char,
            ignore_1,
            expression,
            ignore_2,
            close_paren_char,
            ignore_3,
            statement,
            ignore_4,
            _t1,
        }
    }
}

impl DefaultTest for if_statement::_T0 {}

impl if_statement::_T1 {
    pub fn new(
        ((r#else, ignore), statement): ((FixedTerminal<4usize>, ignore::N), statement::N),
    ) -> Self {
        Self {
            r#else,
            ignore,
            statement,
        }
    }
}

impl DefaultTest for if_statement::_T1 {}

impl return_statement::_T0 {
    pub fn new(
        ((((r#return, ignore_0), expression), ignore_1), semicolon_char): (
            (
                ((FixedTerminal<6usize>, ignore::N), Option<expression::N>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#return,
            ignore_0,
            expression,
            ignore_1,
            semicolon_char,
        }
    }
}

impl DefaultTest for return_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#return.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self.semicolon_char.is_default()
    }
}

impl revert_statement::_T0 {
    pub fn new(
        ((((((revert , ignore_0) , expression) , ignore_1) , argument_list) , ignore_2) , semicolon_char) : ((((((FixedTerminal < 6usize > , ignore :: N) , expression :: N) , ignore :: N) , argument_list :: N) , ignore :: N) , FixedTerminal < 1usize >),
    ) -> Self {
        Self {
            revert,
            ignore_0,
            expression,
            ignore_1,
            argument_list,
            ignore_2,
            semicolon_char,
        }
    }
}

impl DefaultTest for revert_statement::_T0 {
    fn is_default(&self) -> bool {
        self.revert.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self.argument_list.is_default()
            && self.ignore_2.is_default()
            && self.semicolon_char.is_default()
    }
}

impl state_variable_declaration::_T0 {
    pub fn new(
        (
            (((((((type_name, ignore_0), _t2s), ignore_1), identifier), ignore_2), _t3), ignore_3),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (type_name::N, ignore::N),
                                    Vec<Box<state_variable_declaration::_T2>>,
                                ),
                                ignore::N,
                            ),
                            identifier::N,
                        ),
                        ignore::N,
                    ),
                    Option<Box<state_variable_declaration::_T3>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            _t2s,
            ignore_1,
            identifier,
            ignore_2,
            _t3,
            ignore_3,
            semicolon_char,
        }
    }
}

impl DefaultTest for state_variable_declaration::_T0 {}

impl state_variable_declaration::_T3 {
    pub fn new(
        ((equal_char, ignore), expression): ((FixedTerminal<1usize>, ignore::N), expression::N),
    ) -> Self {
        Self {
            equal_char,
            ignore,
            expression,
        }
    }
}

impl DefaultTest for state_variable_declaration::_T3 {
    fn is_default(&self) -> bool {
        self.equal_char.is_default() && self.ignore.is_default() && self.expression.is_default()
    }
}

impl state_variable_declaration::_T2 {
    pub fn new(
        (state_variable_attribute, ignore): (state_variable_attribute::N, ignore::N),
    ) -> Self {
        Self {
            state_variable_attribute,
            ignore,
        }
    }
}

impl DefaultTest for state_variable_declaration::_T2 {}

impl try_statement::_T0 {
    pub fn new(
        ((((((((r#try , ignore_0) , expression) , ignore_1) , _t1) , ignore_2) , block) , ignore_3) , _t3s) : ((((((((FixedTerminal < 3usize > , ignore :: N) , expression :: N) , ignore :: N) , Option < Box < try_statement :: _T1 > >) , ignore :: N) , block :: N) , ignore :: N) , Vec < Box < try_statement :: _T3 > >),
    ) -> Self {
        Self {
            r#try,
            ignore_0,
            expression,
            ignore_1,
            _t1,
            ignore_2,
            block,
            ignore_3,
            _t3s,
        }
    }
}

impl DefaultTest for try_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#try.is_default()
            && self.ignore_0.is_default()
            && self.expression.is_default()
            && self.ignore_1.is_default()
            && self._t1.is_default()
            && self.ignore_2.is_default()
            && self.block.is_default()
            && self.ignore_3.is_default()
            && self._t3s.is_default()
    }
}

impl try_statement::_T3 {
    pub fn new((catch_clause, ignore): (catch_clause::N, ignore::N)) -> Self {
        Self {
            catch_clause,
            ignore,
        }
    }
}

impl DefaultTest for try_statement::_T3 {
    fn is_default(&self) -> bool {
        self.catch_clause.is_default() && self.ignore.is_default()
    }
}

impl try_statement::_T1 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (FixedTerminal<7usize>, ignore::N),
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            returns,
            ignore,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for try_statement::_T1 {
    fn is_default(&self) -> bool {
        self.returns.is_default()
            && self.ignore.is_default()
            && self.non_empty_parameter_list.is_default()
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
        ((((variable_declaration_tuple, ignore_0), equal_char), ignore_1), expression): (
            (
                (
                    (variable_declaration_tuple::N, ignore::N),
                    FixedTerminal<1usize>,
                ),
                ignore::N,
            ),
            expression::N,
        ),
    ) -> Self {
        Self {
            variable_declaration_tuple,
            ignore_0,
            equal_char,
            ignore_1,
            expression,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T4 {}

impl variable_declaration_statement::_T2 {
    pub fn new(
        ((variable_declaration, ignore), _t3): (
            (variable_declaration::N, ignore::N),
            Option<Box<variable_declaration_statement::_T3>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            ignore,
            _t3,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T2 {}

impl variable_declaration_statement::_T3 {
    pub fn new(
        ((equal_char, ignore), expression): ((FixedTerminal<1usize>, ignore::N), expression::N),
    ) -> Self {
        Self {
            equal_char,
            ignore,
            expression,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T3 {
    fn is_default(&self) -> bool {
        self.equal_char.is_default() && self.ignore.is_default() && self.expression.is_default()
    }
}

impl while_statement::_T0 {
    pub fn new(
        (
            (
                (
                    (((((r#while, ignore_0), open_paren_char), ignore_1), expression), ignore_2),
                    close_paren_char,
                ),
                ignore_3,
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
            ignore_0,
            open_paren_char,
            ignore_1,
            expression,
            ignore_2,
            close_paren_char,
            ignore_3,
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
                                        ((((r#for, ignore_0), open_paren_char), ignore_1), _t1),
                                        ignore_2,
                                    ),
                                    _t2,
                                ),
                                ignore_3,
                            ),
                            expression,
                        ),
                        ignore_4,
                    ),
                    close_paren_char,
                ),
                ignore_5,
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
            ignore_0,
            open_paren_char,
            ignore_1,
            _t1,
            ignore_2,
            _t2,
            ignore_3,
            expression,
            ignore_4,
            close_paren_char,
            ignore_5,
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
        ((((open_brace_char, ignore_0), _t3s), ignore_1), close_brace_char): (
            (
                ((FixedTerminal<1usize>, ignore::N), Vec<Box<block::_T3>>),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _t3s,
            ignore_1,
            close_brace_char,
        }
    }
}

impl DefaultTest for block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.ignore_0.is_default()
            && self._t3s.is_default()
            && self.ignore_1.is_default()
            && self.close_brace_char.is_default()
    }
}

impl block::_T3 {
    pub fn new((_t2, ignore): (Box<block::_T2>, ignore::N)) -> Self {
        Self { _t2, ignore }
    }
}

impl DefaultTest for block::_T3 {}

impl DefaultTest for block::_T2 {}

impl constructor_definition::_T0 {
    pub fn new(
        ((((((constructor, ignore_0), parameter_list), ignore_1), _t2s), ignore_2), block): (
            (
                (
                    (
                        ((FixedTerminal<11usize>, ignore::N), parameter_list::N),
                        ignore::N,
                    ),
                    Vec<Box<constructor_definition::_T2>>,
                ),
                ignore::N,
            ),
            block::N,
        ),
    ) -> Self {
        Self {
            constructor,
            ignore_0,
            parameter_list,
            ignore_1,
            _t2s,
            ignore_2,
            block,
        }
    }
}

impl DefaultTest for constructor_definition::_T0 {
    fn is_default(&self) -> bool {
        self.constructor.is_default()
            && self.ignore_0.is_default()
            && self.parameter_list.is_default()
            && self.ignore_1.is_default()
            && self._t2s.is_default()
            && self.ignore_2.is_default()
            && self.block.is_default()
    }
}

impl constructor_definition::_T2 {
    pub fn new((constructor_attribute, ignore): (constructor_attribute::N, ignore::N)) -> Self {
        Self {
            constructor_attribute,
            ignore,
        }
    }
}

impl DefaultTest for constructor_definition::_T2 {}

impl fallback_function_definition::_T0 {
    pub fn new(
        (
            (
                ((((((fallback, ignore_0), parameter_list), ignore_1), _t2s), ignore_2), _t3),
                ignore_3,
            ),
            _t4,
        ): (
            (
                (
                    (
                        (
                            (
                                ((FixedTerminal<8usize>, ignore::N), parameter_list::N),
                                ignore::N,
                            ),
                            Vec<Box<fallback_function_definition::_T2>>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<fallback_function_definition::_T3>>,
                ),
                ignore::N,
            ),
            Box<fallback_function_definition::_T4>,
        ),
    ) -> Self {
        Self {
            fallback,
            ignore_0,
            parameter_list,
            ignore_1,
            _t2s,
            ignore_2,
            _t3,
            ignore_3,
            _t4,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T0 {}

impl DefaultTest for fallback_function_definition::_T4 {}

impl fallback_function_definition::_T3 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (FixedTerminal<7usize>, ignore::N),
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            returns,
            ignore,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.returns.is_default()
            && self.ignore.is_default()
            && self.non_empty_parameter_list.is_default()
    }
}

impl fallback_function_definition::_T2 {
    pub fn new(
        (fallback_function_attribute, ignore): (fallback_function_attribute::N, ignore::N),
    ) -> Self {
        Self {
            fallback_function_attribute,
            ignore,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T2 {}

impl function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((function, ignore_0), _t1), ignore_1), parameter_list), ignore_2),
                            _t3s,
                        ),
                        ignore_3,
                    ),
                    _t4,
                ),
                ignore_4,
            ),
            _t5,
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
                            Vec<Box<function_definition::_T3>>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<function_definition::_T4>>,
                ),
                ignore::N,
            ),
            Box<function_definition::_T5>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_0,
            _t1,
            ignore_1,
            parameter_list,
            ignore_2,
            _t3s,
            ignore_3,
            _t4,
            ignore_4,
            _t5,
        }
    }
}

impl DefaultTest for function_definition::_T0 {}

impl DefaultTest for function_definition::_T5 {}

impl function_definition::_T4 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (FixedTerminal<7usize>, ignore::N),
            non_empty_parameter_list::N,
        ),
    ) -> Self {
        Self {
            returns,
            ignore,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for function_definition::_T4 {
    fn is_default(&self) -> bool {
        self.returns.is_default()
            && self.ignore.is_default()
            && self.non_empty_parameter_list.is_default()
    }
}

impl function_definition::_T3 {
    pub fn new((function_attribute, ignore): (function_attribute::N, ignore::N)) -> Self {
        Self {
            function_attribute,
            ignore,
        }
    }
}

impl DefaultTest for function_definition::_T3 {}

impl DefaultTest for function_definition::_T1 {}

impl modifier_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((modifier, ignore_0), identifier), ignore_1), parameter_list), ignore_2),
                    _t2s,
                ),
                ignore_3,
            ),
            _t3,
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
                    Vec<Box<modifier_definition::_T2>>,
                ),
                ignore::N,
            ),
            Box<modifier_definition::_T3>,
        ),
    ) -> Self {
        Self {
            modifier,
            ignore_0,
            identifier,
            ignore_1,
            parameter_list,
            ignore_2,
            _t2s,
            ignore_3,
            _t3,
        }
    }
}

impl DefaultTest for modifier_definition::_T0 {}

impl DefaultTest for modifier_definition::_T3 {}

impl modifier_definition::_T2 {
    pub fn new((method_attribute, ignore): (method_attribute::N, ignore::N)) -> Self {
        Self {
            method_attribute,
            ignore,
        }
    }
}

impl DefaultTest for modifier_definition::_T2 {}

impl receive_function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((receive, ignore_0), open_paren_char), ignore_1), close_paren_char),
                        ignore_2,
                    ),
                    _t2s,
                ),
                ignore_3,
            ),
            _t3,
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
                    Vec<Box<receive_function_definition::_T2>>,
                ),
                ignore::N,
            ),
            Box<receive_function_definition::_T3>,
        ),
    ) -> Self {
        Self {
            receive,
            ignore_0,
            open_paren_char,
            ignore_1,
            close_paren_char,
            ignore_2,
            _t2s,
            ignore_3,
            _t3,
        }
    }
}

impl DefaultTest for receive_function_definition::_T0 {}

impl DefaultTest for receive_function_definition::_T3 {}

impl receive_function_definition::_T2 {
    pub fn new(
        (receive_function_attribute, ignore): (receive_function_attribute::N, ignore::N),
    ) -> Self {
        Self {
            receive_function_attribute,
            ignore,
        }
    }
}

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
                                            (((r#abstract, ignore_0), contract), ignore_1),
                                            identifier,
                                        ),
                                        ignore_2,
                                    ),
                                    inheritance_specifier_list,
                                ),
                                ignore_3,
                            ),
                            open_brace_char,
                        ),
                        ignore_4,
                    ),
                    _t2s,
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
                    Vec<Box<contract_definition::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#abstract,
            ignore_0,
            contract,
            ignore_1,
            identifier,
            ignore_2,
            inheritance_specifier_list,
            ignore_3,
            open_brace_char,
            ignore_4,
            _t2s,
            ignore_5,
            close_brace_char,
        }
    }
}

impl DefaultTest for contract_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#abstract.is_default()
            && self.ignore_0.is_default()
            && self.contract.is_default()
            && self.ignore_1.is_default()
            && self.identifier.is_default()
            && self.ignore_2.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.ignore_3.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_4.is_default()
            && self._t2s.is_default()
            && self.ignore_5.is_default()
            && self.close_brace_char.is_default()
    }
}

impl contract_definition::_T2 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}

impl DefaultTest for contract_definition::_T2 {}

impl interface_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (((interface, ignore_0), identifier), ignore_1),
                                    inheritance_specifier_list,
                                ),
                                ignore_2,
                            ),
                            open_brace_char,
                        ),
                        ignore_3,
                    ),
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
                    Vec<Box<interface_definition::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            interface,
            ignore_0,
            identifier,
            ignore_1,
            inheritance_specifier_list,
            ignore_2,
            open_brace_char,
            ignore_3,
            _t2s,
            ignore_4,
            close_brace_char,
        }
    }
}

impl DefaultTest for interface_definition::_T0 {
    fn is_default(&self) -> bool {
        self.interface.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.ignore_2.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_3.is_default()
            && self._t2s.is_default()
            && self.ignore_4.is_default()
            && self.close_brace_char.is_default()
    }
}

impl interface_definition::_T2 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}

impl DefaultTest for interface_definition::_T2 {}

impl library_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((((library, ignore_0), identifier), ignore_1), open_brace_char), ignore_2),
                    _t2s,
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
                                ((FixedTerminal<7usize>, ignore::N), identifier::N),
                                ignore::N,
                            ),
                            FixedTerminal<1usize>,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<library_definition::_T2>>,
                ),
                ignore::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            library,
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _t2s,
            ignore_3,
            close_brace_char,
        }
    }
}

impl DefaultTest for library_definition::_T0 {
    fn is_default(&self) -> bool {
        self.library.is_default()
            && self.ignore_0.is_default()
            && self.identifier.is_default()
            && self.ignore_1.is_default()
            && self.open_brace_char.is_default()
            && self.ignore_2.is_default()
            && self._t2s.is_default()
            && self.ignore_3.is_default()
            && self.close_brace_char.is_default()
    }
}

impl library_definition::_T2 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}

impl DefaultTest for library_definition::_T2 {}

impl DefaultTest for definition::_T0 {}

impl source_unit::_T0 {
    pub fn new(
        ((((ignore_0, ignore_1), _t3s), ignore_2), end_marker): (
            (
                ((ignore::N, ignore::N), Vec<Box<source_unit::_T3>>),
                ignore::N,
            ),
            (),
        ),
    ) -> Self {
        Self {
            ignore_0,
            ignore_1,
            _t3s,
            ignore_2,
            end_marker,
        }
    }
}

impl DefaultTest for source_unit::_T0 {
    fn is_default(&self) -> bool {
        self.ignore_0.is_default()
            && self.ignore_1.is_default()
            && self._t3s.is_default()
            && self.ignore_2.is_default()
            && self.end_marker.is_default()
    }
}

impl source_unit::_T3 {
    pub fn new((_t2, ignore): (Box<source_unit::_T2>, ignore::N)) -> Self {
        Self { _t2, ignore }
    }
}

impl DefaultTest for source_unit::_T3 {}

impl DefaultTest for source_unit::_T2 {}
