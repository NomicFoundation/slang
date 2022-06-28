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
        (((slash_star, comments), star_chars), star_slash): (
            ((FixedTerminal<2usize>, Vec<Box<comment::Comment>>), usize),
            FixedTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            slash_star,
            comments,
            star_chars,
            star_slash,
        }
    }
}

impl DefaultTest for comment::_T0 {
    fn is_default(&self) -> bool {
        self.slash_star.is_default()
            && self.comments.is_default()
            && self.star_chars.is_default()
            && self.star_slash.is_default()
    }
}

impl DefaultTest for comment::Comment {}

impl comment::_T2 {
    pub fn new((star_chars, _1): (usize, FixedTerminal<1usize>)) -> Self {
        Self { star_chars, _1 }
    }
}

impl DefaultTest for comment::_T2 {
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

impl DefaultTest for yul_decimal_number_literal::YulDecimalNumberLiteral {}

impl yul_decimal_number_literal::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1usize>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl DefaultTest for yul_decimal_number_literal::_T0 {
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

impl DefaultTest for ignore::Ignore {}

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

impl add_sub_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, FixedTerminal<1usize>), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for add_sub_expression::_T0 {}

impl and_expression::_T0 {
    pub fn new(
        ((expression_1, ampersand_ampersand), expression_2): (
            (expression::N, FixedTerminal<2usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            ampersand_ampersand,
            expression_2,
        }
    }
}

impl DefaultTest for and_expression::_T0 {}

impl assignment_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, usize), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for assignment_expression::_T0 {}

impl bit_and_expression::_T0 {
    pub fn new(
        ((expression_1, ampersand_char), expression_2): (
            (expression::N, FixedTerminal<1usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            ampersand_char,
            expression_2,
        }
    }
}

impl DefaultTest for bit_and_expression::_T0 {}

impl bit_or_expression::_T0 {
    pub fn new(
        ((expression_1, bar_char), expression_2): (
            (expression::N, FixedTerminal<1usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            bar_char,
            expression_2,
        }
    }
}

impl DefaultTest for bit_or_expression::_T0 {}

impl bit_x_or_expression::_T0 {
    pub fn new(
        ((expression_1, caret_char), expression_2): (
            (expression::N, FixedTerminal<1usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            caret_char,
            expression_2,
        }
    }
}

impl DefaultTest for bit_x_or_expression::_T0 {}

impl break_statement::_T0 {
    pub fn new((r#break, semicolon_char): (FixedTerminal<5usize>, FixedTerminal<1usize>)) -> Self {
        Self {
            r#break,
            semicolon_char,
        }
    }
}

impl DefaultTest for break_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#break.is_default() && self.semicolon_char.is_default()
    }
}

impl conditional_expression::_T0 {
    pub fn new((expression, _t1): (expression::N, Box<conditional_expression::_T1>)) -> Self {
        Self { expression, _t1 }
    }
}

impl DefaultTest for conditional_expression::_T0 {}

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

impl DefaultTest for conditional_expression::_T1 {}

impl continue_statement::_T0 {
    pub fn new(
        (r#continue, semicolon_char): (FixedTerminal<8usize>, FixedTerminal<1usize>),
    ) -> Self {
        Self {
            r#continue,
            semicolon_char,
        }
    }
}

impl DefaultTest for continue_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#continue.is_default() && self.semicolon_char.is_default()
    }
}

impl decimal_number::_T0 {
    pub fn new(
        (decimal_number, decimal_exponent): (
            Box<decimal_number::DecimalNumber>,
            Option<decimal_exponent::N>,
        ),
    ) -> Self {
        Self {
            decimal_number,
            decimal_exponent,
        }
    }
}

impl DefaultTest for decimal_number::_T0 {}

impl DefaultTest for decimal_number::DecimalNumber {}

impl DefaultTest for elementary_type::ElementaryType {}

impl equality_comparison_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, FixedTerminal<2usize>), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for equality_comparison_expression::_T0 {}

impl escape_sequence::_T0 {
    pub fn new(
        (backslash_char, escape_sequence): (
            FixedTerminal<1usize>,
            Box<escape_sequence::EscapeSequence>,
        ),
    ) -> Self {
        Self {
            backslash_char,
            escape_sequence,
        }
    }
}

impl DefaultTest for escape_sequence::_T0 {}

impl DefaultTest for escape_sequence::EscapeSequence {}

impl exponentiation_expression::_T0 {
    pub fn new(
        ((expression_1, star_star), expression_2): (
            (expression::N, FixedTerminal<2usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            star_star,
            expression_2,
        }
    }
}

impl DefaultTest for exponentiation_expression::_T0 {}

impl hex_string_literal::_T0 {
    pub fn new(
        (hex, hex_string_literal): (
            FixedTerminal<3usize>,
            Box<hex_string_literal::HexStringLiteral>,
        ),
    ) -> Self {
        Self {
            hex,
            hex_string_literal,
        }
    }
}

impl DefaultTest for hex_string_literal::_T0 {}

impl DefaultTest for hex_string_literal::HexStringLiteral {}

impl hex_string_literal::_T2 {
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

impl DefaultTest for hex_string_literal::_T2 {
    fn is_default(&self) -> bool {
        self.quote_char_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.quote_char_2.is_default()
    }
}

impl hex_string_literal::_T1 {
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

impl DefaultTest for hex_string_literal::_T1 {
    fn is_default(&self) -> bool {
        self.double_quote_char_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.double_quote_char_2.is_default()
    }
}

impl index_access_expression::_T0 {
    pub fn new(
        ((((expression_1, open_bracket_char), expression_2), _t1), close_bracket_char): (
            (
                (
                    (expression::N, FixedTerminal<1usize>),
                    Option<expression::N>,
                ),
                Option<Box<index_access_expression::_T1>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            expression_1,
            open_bracket_char,
            expression_2,
            _t1,
            close_bracket_char,
        }
    }
}

impl DefaultTest for index_access_expression::_T0 {}

impl index_access_expression::_T1 {
    pub fn new((colon_char, expression): (FixedTerminal<1usize>, Option<expression::N>)) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}

impl DefaultTest for index_access_expression::_T1 {
    fn is_default(&self) -> bool {
        self.colon_char.is_default() && self.expression.is_default()
    }
}

impl DefaultTest for keyword::Keyword {}

impl mul_div_mod_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, FixedTerminal<1usize>), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for mul_div_mod_expression::_T0 {}

impl or_expression::_T0 {
    pub fn new(
        ((expression_1, bar_bar), expression_2): (
            (expression::N, FixedTerminal<2usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            expression_1,
            bar_bar,
            expression_2,
        }
    }
}

impl DefaultTest for or_expression::_T0 {}

impl order_comparison_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, usize), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for order_comparison_expression::_T0 {}

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

impl shift_expression::_T0 {
    pub fn new(
        ((expression_1, _1), expression_2): ((expression::N, usize), expression::N),
    ) -> Self {
        Self {
            expression_1,
            _1,
            expression_2,
        }
    }
}

impl DefaultTest for shift_expression::_T0 {}

impl unary_prefix_expression::_T0 {
    pub fn new((_0, expression): (usize, expression::N)) -> Self {
        Self { _0, expression }
    }
}

impl DefaultTest for unary_prefix_expression::_T0 {}

impl unary_suffix_expression::_T0 {
    pub fn new((expression, _1): (expression::N, FixedTerminal<2usize>)) -> Self {
        Self { expression, _1 }
    }
}

impl DefaultTest for unary_suffix_expression::_T0 {}

impl unchecked_block::_T0 {
    pub fn new((unchecked, block): (FixedTerminal<9usize>, block::N)) -> Self {
        Self { unchecked, block }
    }
}

impl DefaultTest for unchecked_block::_T0 {
    fn is_default(&self) -> bool {
        self.unchecked.is_default() && self.block.is_default()
    }
}

impl double_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((double_quote_char_1, double_quoted_ascii_string_literals), double_quote_char_2): (
            (
                FixedTerminal<1usize>,
                Vec<Box<double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            double_quoted_ascii_string_literals,
            double_quote_char_2,
        }
    }
}

impl DefaultTest for double_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.double_quote_char_1.is_default()
            && self.double_quoted_ascii_string_literals.is_default()
            && self.double_quote_char_2.is_default()
    }
}

impl DefaultTest for double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral {}

impl double_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_double_quote, double_quoted_unicode_string_literals), double_quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            double_quoted_unicode_string_literals,
            double_quote_char,
        }
    }
}

impl DefaultTest for double_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_double_quote.is_default()
            && self.double_quoted_unicode_string_literals.is_default()
            && self.double_quote_char.is_default()
    }
}

impl DefaultTest for double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral {}

impl DefaultTest for elementary_type_with_payable::ElementaryTypeWithPayable {}

impl elementary_type_with_payable::_T0 {
    pub fn new((address, payable): (FixedTerminal<7usize>, Option<FixedTerminal<7usize>>)) -> Self {
        Self { address, payable }
    }
}

impl DefaultTest for elementary_type_with_payable::_T0 {
    fn is_default(&self) -> bool {
        self.address.is_default() && self.payable.is_default()
    }
}

impl DefaultTest for elementary_type_without_payable::ElementaryTypeWithoutPayable {}

impl numeric_literal::_T0 {
    pub fn new(
        (numeric_literal, _1): (Box<numeric_literal::NumericLiteral>, Option<usize>),
    ) -> Self {
        Self {
            numeric_literal,
            _1,
        }
    }
}

impl DefaultTest for numeric_literal::_T0 {}

impl DefaultTest for numeric_literal::NumericLiteral {}

impl DefaultTest for reserved_word::ReservedWord {}

impl single_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((quote_char_1, single_quoted_ascii_string_literals), quote_char_2): (
            (
                FixedTerminal<1usize>,
                Vec<Box<single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            single_quoted_ascii_string_literals,
            quote_char_2,
        }
    }
}

impl DefaultTest for single_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.quote_char_1.is_default()
            && self.single_quoted_ascii_string_literals.is_default()
            && self.quote_char_2.is_default()
    }
}

impl DefaultTest for single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral {}

impl single_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_quote, single_quoted_unicode_string_literals), quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            single_quoted_unicode_string_literals,
            quote_char,
        }
    }
}

impl DefaultTest for single_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_quote.is_default()
            && self.single_quoted_unicode_string_literals.is_default()
            && self.quote_char.is_default()
    }
}

impl DefaultTest for single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral {}

impl DefaultTest for ascii_string_literal::AsciiStringLiteral {}

impl assembly_flags::_T0 {
    pub fn new(
        ((open_paren_char, double_quoted_ascii_string_literals), close_paren_char): (
            (FixedTerminal<1usize>, Box<assembly_flags::_T1>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            double_quoted_ascii_string_literals,
            close_paren_char,
        }
    }
}

impl DefaultTest for assembly_flags::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.double_quoted_ascii_string_literals.is_default()
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

impl DefaultTest for unicode_string_literal::UnicodeStringLiteral {}

impl yul_function_call::_T0 {
    pub fn new(
        (((yul_function_call, open_paren_char), yul_expressions), close_paren_char): (
            (
                (
                    Box<yul_function_call::YulFunctionCall>,
                    FixedTerminal<1usize>,
                ),
                Option<Box<yul_function_call::_T1>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            yul_function_call,
            open_paren_char,
            yul_expressions,
            close_paren_char,
        }
    }
}

impl DefaultTest for yul_function_call::_T0 {}

impl yul_function_call::_T1 {
    pub fn new(
        (yul_expressions, comma_chars): (Vec<yul_expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            yul_expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for yul_function_call::_T1 {
    fn is_default(&self) -> bool {
        self.yul_expressions.is_default() && self.comma_chars.is_default()
    }
}

impl DefaultTest for yul_function_call::YulFunctionCall {}

impl yul_function_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((function, yul_identifier), open_paren_char), yul_identifiers),
                    close_paren_char,
                ),
                _t2,
            ),
            yul_block,
        ): (
            (
                (
                    (
                        (
                            (FixedTerminal<8usize>, yul_identifier::N),
                            FixedTerminal<1usize>,
                        ),
                        Option<Box<yul_function_definition::_T1>>,
                    ),
                    FixedTerminal<1usize>,
                ),
                Option<Box<yul_function_definition::_T2>>,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            function,
            yul_identifier,
            open_paren_char,
            yul_identifiers,
            close_paren_char,
            _t2,
            yul_block,
        }
    }
}

impl DefaultTest for yul_function_definition::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.yul_identifier.is_default()
            && self.open_paren_char.is_default()
            && self.yul_identifiers.is_default()
            && self.close_paren_char.is_default()
            && self._t2.is_default()
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
    pub fn new((yul_identifier, _t2s): (yul_identifier::N, Vec<Box<yul_path::_T2>>)) -> Self {
        Self {
            yul_identifier,
            _t2s,
        }
    }
}

impl DefaultTest for yul_path::_T0 {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default() && self._t2s.is_default()
    }
}

impl yul_path::_T2 {
    pub fn new((period_char, yul_path): (FixedTerminal<1usize>, Box<yul_path::YulPath>)) -> Self {
        Self {
            period_char,
            yul_path,
        }
    }
}

impl DefaultTest for yul_path::_T2 {}

impl DefaultTest for yul_path::YulPath {}

impl enum_definition::_T0 {
    pub fn new(
        ((((r#enum, identifier), open_brace_char), identifiers), close_brace_char): (
            (
                (
                    (FixedTerminal<4usize>, identifier::N),
                    FixedTerminal<1usize>,
                ),
                Box<enum_definition::_T1>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#enum,
            identifier,
            open_brace_char,
            identifiers,
            close_brace_char,
        }
    }
}

impl DefaultTest for enum_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#enum.is_default()
            && self.identifier.is_default()
            && self.open_brace_char.is_default()
            && self.identifiers.is_default()
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

impl DefaultTest for literal::Literal {}

impl member_access_expression::_T0 {
    pub fn new(
        ((expression, period_char), member_access_expression): (
            (expression::N, FixedTerminal<1usize>),
            Box<member_access_expression::MemberAccessExpression>,
        ),
    ) -> Self {
        Self {
            expression,
            period_char,
            member_access_expression,
        }
    }
}

impl DefaultTest for member_access_expression::_T0 {}

impl DefaultTest for member_access_expression::MemberAccessExpression {}

impl named_argument::_T0 {
    pub fn new(
        ((identifier, colon_char), expression): (
            (identifier::N, FixedTerminal<1usize>),
            expression::N,
        ),
    ) -> Self {
        Self {
            identifier,
            colon_char,
            expression,
        }
    }
}

impl DefaultTest for named_argument::_T0 {}

impl parameter_declaration::_T0 {
    pub fn new(
        ((type_name, _1), identifier): ((type_name::N, Option<usize>), Option<identifier::N>),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl DefaultTest for parameter_declaration::_T0 {}

impl selected_import::_T0 {
    pub fn new((identifier, _t1): (identifier::N, Option<Box<selected_import::_T1>>)) -> Self {
        Self { identifier, _t1 }
    }
}

impl DefaultTest for selected_import::_T0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self._t1.is_default()
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
        ((((r#type, identifier), is), elementary_type_with_payable), semicolon_char): (
            (
                (
                    (FixedTerminal<4usize>, identifier::N),
                    FixedTerminal<2usize>,
                ),
                elementary_type_with_payable::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#type,
            identifier,
            is,
            elementary_type_with_payable,
            semicolon_char,
        }
    }
}

impl DefaultTest for user_defined_value_type_definition::_T0 {}

impl DefaultTest for yul_literal::YulLiteral {}

impl function_call_options_expression::_T0 {
    pub fn new(
        (((expression, open_brace_char), named_arguments), close_brace_char): (
            (
                (expression::N, FixedTerminal<1usize>),
                Box<function_call_options_expression::_T1>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            expression,
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T0 {}

impl function_call_options_expression::_T1 {
    pub fn new(
        (named_arguments, comma_chars): (Vec<named_argument::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            named_arguments,
            comma_chars,
        }
    }
}

impl DefaultTest for function_call_options_expression::_T1 {
    fn is_default(&self) -> bool {
        self.named_arguments.is_default() && self.comma_chars.is_default()
    }
}

impl mapping_type::_T0 {
    pub fn new(
        (
            ((((mapping, open_paren_char), mapping_type), equal_greater), type_name),
            close_paren_char,
        ): (
            (
                (
                    (
                        (FixedTerminal<7usize>, FixedTerminal<1usize>),
                        Box<mapping_type::MappingType>,
                    ),
                    FixedTerminal<2usize>,
                ),
                type_name::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            mapping,
            open_paren_char,
            mapping_type,
            equal_greater,
            type_name,
            close_paren_char,
        }
    }
}

impl DefaultTest for mapping_type::_T0 {}

impl DefaultTest for mapping_type::MappingType {}

impl named_argument_list::_T0 {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (FixedTerminal<1usize>, Option<Box<named_argument_list::_T1>>),
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

impl DefaultTest for named_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.named_arguments.is_default()
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
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (FixedTerminal<1usize>, Box<non_empty_parameter_list::_T1>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}

impl DefaultTest for non_empty_parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.parameter_declarations.is_default()
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
        (r#override, _t1): (FixedTerminal<8usize>, Option<Box<override_specifier::_T1>>),
    ) -> Self {
        Self { r#override, _t1 }
    }
}

impl DefaultTest for override_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.r#override.is_default() && self._t1.is_default()
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
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (FixedTerminal<1usize>, Option<Box<parameter_list::_T1>>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}

impl DefaultTest for parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.parameter_declarations.is_default()
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
        ((((open_brace_char, selected_imports), close_brace_char), from), import_path): (
            (
                (
                    (FixedTerminal<1usize>, Box<selecting_import_directive::_T1>),
                    FixedTerminal<1usize>,
                ),
                FixedTerminal<4usize>,
            ),
            import_path::N,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            selected_imports,
            close_brace_char,
            from,
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
        (import_path, _t2s): (import_path::N, Vec<Box<simple_import_directive::_T2>>),
    ) -> Self {
        Self { import_path, _t2s }
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
        ((((star_char, r#as), identifier), from), import_path): (
            (
                (
                    (FixedTerminal<1usize>, FixedTerminal<2usize>),
                    identifier::N,
                ),
                FixedTerminal<4usize>,
            ),
            import_path::N,
        ),
    ) -> Self {
        Self {
            star_char,
            r#as,
            identifier,
            from,
            import_path,
        }
    }
}

impl DefaultTest for star_import_directive::_T0 {}

impl DefaultTest for yul_expression::YulExpression {}

impl argument_list::_T0 {
    pub fn new(
        ((open_paren_char, argument_list), close_paren_char): (
            (
                FixedTerminal<1usize>,
                Option<Box<argument_list::ArgumentList>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            argument_list,
            close_paren_char,
        }
    }
}

impl DefaultTest for argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.argument_list.is_default()
            && self.close_paren_char.is_default()
    }
}

impl DefaultTest for argument_list::ArgumentList {}

impl catch_clause::_T0 {
    pub fn new(
        ((catch, _t1), block): (
            (FixedTerminal<5usize>, Option<Box<catch_clause::_T1>>),
            block::N,
        ),
    ) -> Self {
        Self { catch, _t1, block }
    }
}

impl DefaultTest for catch_clause::_T0 {
    fn is_default(&self) -> bool {
        self.catch.is_default() && self._t1.is_default() && self.block.is_default()
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
        (((function, parameter_list), _2), _t2): (
            ((FixedTerminal<8usize>, parameter_list::N), Vec<usize>),
            Option<Box<function_type::_T2>>,
        ),
    ) -> Self {
        Self {
            function,
            parameter_list,
            _2,
            _t2,
        }
    }
}

impl DefaultTest for function_type::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.parameter_list.is_default()
            && self._2.is_default()
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
        ((import, import_directive), semicolon_char): (
            (
                FixedTerminal<6usize>,
                Box<import_directive::ImportDirective>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            import,
            import_directive,
            semicolon_char,
        }
    }
}

impl DefaultTest for import_directive::_T0 {}

impl DefaultTest for import_directive::ImportDirective {}

impl DefaultTest for method_attribute::MethodAttribute {}

impl DefaultTest for state_variable_attribute::StateVariableAttribute {}

impl yul_assignment::_T0 {
    pub fn new(
        (yul_path, yul_assignment): (yul_path::N, Box<yul_assignment::YulAssignment>),
    ) -> Self {
        Self {
            yul_path,
            yul_assignment,
        }
    }
}

impl DefaultTest for yul_assignment::_T0 {}

impl DefaultTest for yul_assignment::YulAssignment {}

impl yul_assignment::_T2 {
    pub fn new(
        ((_t4s, colon_equal), yul_function_call): (
            (Vec<Box<yul_assignment::_T4>>, FixedTerminal<2usize>),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            _t4s,
            colon_equal,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_assignment::_T2 {}

impl yul_assignment::_T4 {
    pub fn new((comma_char, yul_path): (FixedTerminal<1usize>, yul_path::N)) -> Self {
        Self {
            comma_char,
            yul_path,
        }
    }
}

impl DefaultTest for yul_assignment::_T4 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.yul_path.is_default()
    }
}

impl yul_assignment::_T1 {
    pub fn new((colon_equal, yul_expression): (FixedTerminal<2usize>, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}

impl DefaultTest for yul_assignment::_T1 {}

impl yul_for_statement::_T0 {
    pub fn new(
        ((((r#for, yul_block_1), yul_expression), yul_block_2), yul_block_3): (
            (
                ((FixedTerminal<3usize>, yul_block::N), yul_expression::N),
                yul_block::N,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#for,
            yul_block_1,
            yul_expression,
            yul_block_2,
            yul_block_3,
        }
    }
}

impl DefaultTest for yul_for_statement::_T0 {}

impl yul_if_statement::_T0 {
    pub fn new(
        ((r#if, yul_expression), yul_block): (
            (FixedTerminal<2usize>, yul_expression::N),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#if,
            yul_expression,
            yul_block,
        }
    }
}

impl DefaultTest for yul_if_statement::_T0 {}

impl yul_switch_statement::_T0 {
    pub fn new(
        ((switch, yul_expression), yul_switch_statement): (
            (FixedTerminal<6usize>, yul_expression::N),
            Box<yul_switch_statement::YulSwitchStatement>,
        ),
    ) -> Self {
        Self {
            switch,
            yul_expression,
            yul_switch_statement,
        }
    }
}

impl DefaultTest for yul_switch_statement::_T0 {}

impl DefaultTest for yul_switch_statement::YulSwitchStatement {}

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

impl yul_switch_statement::_T1 {
    pub fn new(
        (_t3s, _t4): (
            Vec<Box<yul_switch_statement::_T3>>,
            Option<Box<yul_switch_statement::_T4>>,
        ),
    ) -> Self {
        Self { _t3s, _t4 }
    }
}

impl DefaultTest for yul_switch_statement::_T1 {
    fn is_default(&self) -> bool {
        self._t3s.is_default() && self._t4.is_default()
    }
}

impl yul_switch_statement::_T4 {
    pub fn new((default, yul_block): (FixedTerminal<7usize>, yul_block::N)) -> Self {
        Self { default, yul_block }
    }
}

impl DefaultTest for yul_switch_statement::_T4 {
    fn is_default(&self) -> bool {
        self.default.is_default() && self.yul_block.is_default()
    }
}

impl yul_switch_statement::_T3 {
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

impl DefaultTest for yul_switch_statement::_T3 {}

impl yul_variable_declaration::_T0 {
    pub fn new(
        ((r#let, yul_identifier), yul_variable_declaration): (
            (FixedTerminal<3usize>, yul_identifier::N),
            Option<Box<yul_variable_declaration::YulVariableDeclaration>>,
        ),
    ) -> Self {
        Self {
            r#let,
            yul_identifier,
            yul_variable_declaration,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T0 {
    fn is_default(&self) -> bool {
        self.r#let.is_default()
            && self.yul_identifier.is_default()
            && self.yul_variable_declaration.is_default()
    }
}

impl DefaultTest for yul_variable_declaration::YulVariableDeclaration {}

impl yul_variable_declaration::_T2 {
    pub fn new(
        (_t3, _t4): (
            Option<Box<yul_variable_declaration::_T3>>,
            Option<Box<yul_variable_declaration::_T4>>,
        ),
    ) -> Self {
        Self { _t3, _t4 }
    }
}

impl DefaultTest for yul_variable_declaration::_T2 {
    fn is_default(&self) -> bool {
        self._t3.is_default() && self._t4.is_default()
    }
}

impl yul_variable_declaration::_T4 {
    pub fn new(
        (colon_equal, yul_function_call): (FixedTerminal<2usize>, yul_function_call::N),
    ) -> Self {
        Self {
            colon_equal,
            yul_function_call,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T4 {}

impl yul_variable_declaration::_T3 {
    pub fn new((comma_char, yul_identifier): (FixedTerminal<1usize>, yul_identifier::N)) -> Self {
        Self {
            comma_char,
            yul_identifier,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T3 {
    fn is_default(&self) -> bool {
        self.comma_char.is_default() && self.yul_identifier.is_default()
    }
}

impl yul_variable_declaration::_T1 {
    pub fn new((colon_equal, yul_expression): (FixedTerminal<2usize>, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}

impl DefaultTest for yul_variable_declaration::_T1 {}

impl function_call_expression::_T0 {
    pub fn new((expression, argument_list): (expression::N, argument_list::N)) -> Self {
        Self {
            expression,
            argument_list,
        }
    }
}

impl DefaultTest for function_call_expression::_T0 {}

impl inheritance_specifier::_T0 {
    pub fn new(
        (identifier_path, argument_list): (identifier_path::N, Option<argument_list::N>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}

impl DefaultTest for inheritance_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl modifier_invocation::_T0 {
    pub fn new(
        (identifier_path, argument_list): (identifier_path::N, Option<argument_list::N>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}

impl DefaultTest for modifier_invocation::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl type_name::_T0 {
    pub fn new((type_name, _t2s): (Box<type_name::TypeName>, Vec<Box<type_name::_T2>>)) -> Self {
        Self { type_name, _t2s }
    }
}

impl DefaultTest for type_name::_T0 {}

impl type_name::_T2 {
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

impl DefaultTest for type_name::_T2 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expression.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl DefaultTest for type_name::TypeName {}

impl DefaultTest for yul_statement::YulStatement {}

impl DefaultTest for constructor_attribute::ConstructorAttribute {}

impl error_parameter::_T0 {
    pub fn new((type_name, identifier): (type_name::N, Option<identifier::N>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}

impl DefaultTest for error_parameter::_T0 {}

impl event_parameter::_T0 {
    pub fn new(
        ((type_name, indexed), identifier): (
            (type_name::N, Option<FixedTerminal<7usize>>),
            Option<identifier::N>,
        ),
    ) -> Self {
        Self {
            type_name,
            indexed,
            identifier,
        }
    }
}

impl DefaultTest for event_parameter::_T0 {}

impl DefaultTest for fallback_function_attribute::FallbackFunctionAttribute {}

impl DefaultTest for function_attribute::FunctionAttribute {}

impl inheritance_specifier_list::_T0 {
    pub fn new(
        (is, inheritance_specifiers): (FixedTerminal<2usize>, Box<inheritance_specifier_list::_T1>),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers,
        }
    }
}

impl DefaultTest for inheritance_specifier_list::_T0 {
    fn is_default(&self) -> bool {
        self.is.is_default() && self.inheritance_specifiers.is_default()
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

impl DefaultTest for primary_expression::PrimaryExpression {}

impl primary_expression::_T5 {
    pub fn new(
        ((open_bracket_char, expressions), close_bracket_char): (
            (FixedTerminal<1usize>, Box<primary_expression::_T6>),
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

impl DefaultTest for primary_expression::_T5 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expressions.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl primary_expression::_T6 {
    pub fn new(
        (expressions, comma_chars): (Vec<expression::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for primary_expression::_T6 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.comma_chars.is_default()
    }
}

impl primary_expression::_T3 {
    pub fn new(
        ((open_paren_char, expressions), close_paren_char): (
            (FixedTerminal<1usize>, Box<primary_expression::_T4>),
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

impl DefaultTest for primary_expression::_T3 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.expressions.is_default()
            && self.close_paren_char.is_default()
    }
}

impl primary_expression::_T4 {
    pub fn new(
        (expressions, comma_chars): (Vec<Option<expression::N>>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}

impl DefaultTest for primary_expression::_T4 {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.comma_chars.is_default()
    }
}

impl primary_expression::_T2 {
    pub fn new((new, type_name): (FixedTerminal<3usize>, type_name::N)) -> Self {
        Self { new, type_name }
    }
}

impl DefaultTest for primary_expression::_T2 {}

impl primary_expression::_T1 {
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

impl DefaultTest for primary_expression::_T1 {}

impl primary_expression::_T0 {
    pub fn new((payable, argument_list): (FixedTerminal<7usize>, argument_list::N)) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}

impl DefaultTest for primary_expression::_T0 {
    fn is_default(&self) -> bool {
        self.payable.is_default() && self.argument_list.is_default()
    }
}

impl DefaultTest for receive_function_attribute::ReceiveFunctionAttribute {}

impl struct_definition::_T0 {
    pub fn new(
        ((((r#struct, identifier), open_brace_char), _t2s), close_brace_char): (
            (
                (
                    (FixedTerminal<6usize>, identifier::N),
                    FixedTerminal<1usize>,
                ),
                Vec<Box<struct_definition::_T2>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#struct,
            identifier,
            open_brace_char,
            _t2s,
            close_brace_char,
        }
    }
}

impl DefaultTest for struct_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#struct.is_default()
            && self.identifier.is_default()
            && self.open_brace_char.is_default()
            && self._t2s.is_default()
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
        (((((using, using_directive_1), r#for), using_directive_2), global), semicolon_char): (
            (
                (
                    (
                        (FixedTerminal<5usize>, Box<using_directive::UsingDirective>),
                        FixedTerminal<3usize>,
                    ),
                    Box<using_directive::UsingDirective>,
                ),
                Option<FixedTerminal<6usize>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            using,
            using_directive_1,
            r#for,
            using_directive_2,
            global,
            semicolon_char,
        }
    }
}

impl DefaultTest for using_directive::_T0 {}

impl DefaultTest for using_directive::UsingDirective {}

impl DefaultTest for using_directive::UsingDirective {}

impl using_directive::_T1 {
    pub fn new(
        ((open_brace_char, identifier_paths), close_brace_char): (
            (FixedTerminal<1usize>, Box<using_directive::_T2>),
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

impl DefaultTest for using_directive::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.identifier_paths.is_default()
            && self.close_brace_char.is_default()
    }
}

impl using_directive::_T2 {
    pub fn new(
        (identifier_paths, comma_chars): (Vec<identifier_path::N>, Vec<FixedTerminal<1usize>>),
    ) -> Self {
        Self {
            identifier_paths,
            comma_chars,
        }
    }
}

impl DefaultTest for using_directive::_T2 {
    fn is_default(&self) -> bool {
        self.identifier_paths.is_default() && self.comma_chars.is_default()
    }
}

impl variable_declaration::_T0 {
    pub fn new(
        ((type_name, _1), identifier): ((type_name::N, Option<usize>), identifier::N),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl DefaultTest for variable_declaration::_T0 {}

impl yul_block::_T0 {
    pub fn new(
        ((open_brace_char, yul_statements), close_brace_char): (
            (FixedTerminal<1usize>, Vec<yul_statement::N>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            yul_statements,
            close_brace_char,
        }
    }
}

impl DefaultTest for yul_block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.yul_statements.is_default()
            && self.close_brace_char.is_default()
    }
}

impl assembly_statement::_T0 {
    pub fn new(
        (((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block): (
            (
                (FixedTerminal<8usize>, Option<FixedTerminal<8usize>>),
                Option<assembly_flags::N>,
            ),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            assembly,
            double_quote_evmasm_double_quote,
            assembly_flags,
            yul_block,
        }
    }
}

impl DefaultTest for assembly_statement::_T0 {
    fn is_default(&self) -> bool {
        self.assembly.is_default()
            && self.double_quote_evmasm_double_quote.is_default()
            && self.assembly_flags.is_default()
            && self.yul_block.is_default()
    }
}

impl DefaultTest for directive::Directive {}

impl error_definition::_T0 {
    pub fn new(
        (
            ((((error, identifier), open_paren_char), error_parameters), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        (FixedTerminal<5usize>, identifier::N),
                        FixedTerminal<1usize>,
                    ),
                    Option<Box<error_definition::_T1>>,
                ),
                FixedTerminal<1usize>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            error,
            identifier,
            open_paren_char,
            error_parameters,
            close_paren_char,
            semicolon_char,
        }
    }
}

impl DefaultTest for error_definition::_T0 {
    fn is_default(&self) -> bool {
        self.error.is_default()
            && self.identifier.is_default()
            && self.open_paren_char.is_default()
            && self.error_parameters.is_default()
            && self.close_paren_char.is_default()
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
                ((((event, identifier), open_paren_char), event_parameters), close_paren_char),
                anonymous,
            ),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (FixedTerminal<5usize>, identifier::N),
                            FixedTerminal<1usize>,
                        ),
                        Option<Box<event_definition::_T1>>,
                    ),
                    FixedTerminal<1usize>,
                ),
                Option<FixedTerminal<9usize>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            event,
            identifier,
            open_paren_char,
            event_parameters,
            close_paren_char,
            anonymous,
            semicolon_char,
        }
    }
}

impl DefaultTest for event_definition::_T0 {
    fn is_default(&self) -> bool {
        self.event.is_default()
            && self.identifier.is_default()
            && self.open_paren_char.is_default()
            && self.event_parameters.is_default()
            && self.close_paren_char.is_default()
            && self.anonymous.is_default()
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

impl DefaultTest for expression::Expression {}

impl variable_declaration_tuple::_T0 {
    pub fn new(
        ((((open_paren_char, comma_chars), variable_declaration), _t3s), close_paren_char): (
            (
                ((FixedTerminal<1usize>, usize), variable_declaration::N),
                Vec<Box<variable_declaration_tuple::_T3>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            comma_chars,
            variable_declaration,
            _t3s,
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

impl constant_definition::_T0 {
    pub fn new(
        (((((type_name, constant), identifier), equal_char), expression), semicolon_char): (
            (
                (
                    ((type_name::N, FixedTerminal<8usize>), identifier::N),
                    FixedTerminal<1usize>,
                ),
                expression::N,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            type_name,
            constant,
            identifier,
            equal_char,
            expression,
            semicolon_char,
        }
    }
}

impl DefaultTest for constant_definition::_T0 {}

impl do_while_statement::_T0 {
    pub fn new(
        (
            (((((r#do, statement), r#while), open_paren_char), expression), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        ((FixedTerminal<2usize>, statement::N), FixedTerminal<5usize>),
                        FixedTerminal<1usize>,
                    ),
                    expression::N,
                ),
                FixedTerminal<1usize>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#do,
            statement,
            r#while,
            open_paren_char,
            expression,
            close_paren_char,
            semicolon_char,
        }
    }
}

impl DefaultTest for do_while_statement::_T0 {}

impl emit_statement::_T0 {
    pub fn new(
        (((emit, expression), argument_list), semicolon_char): (
            ((FixedTerminal<4usize>, expression::N), argument_list::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            emit,
            expression,
            argument_list,
            semicolon_char,
        }
    }
}

impl DefaultTest for emit_statement::_T0 {}

impl expression_statement::_T0 {
    pub fn new((expression, semicolon_char): (expression::N, FixedTerminal<1usize>)) -> Self {
        Self {
            expression,
            semicolon_char,
        }
    }
}

impl DefaultTest for expression_statement::_T0 {}

impl if_statement::_T0 {
    pub fn new(
        (((((r#if, open_paren_char), expression), close_paren_char), statement), _t1): (
            (
                (
                    (
                        (FixedTerminal<2usize>, FixedTerminal<1usize>),
                        expression::N,
                    ),
                    FixedTerminal<1usize>,
                ),
                statement::N,
            ),
            Option<Box<if_statement::_T1>>,
        ),
    ) -> Self {
        Self {
            r#if,
            open_paren_char,
            expression,
            close_paren_char,
            statement,
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
        ((r#return, expression), semicolon_char): (
            (FixedTerminal<6usize>, Option<expression::N>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#return,
            expression,
            semicolon_char,
        }
    }
}

impl DefaultTest for return_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#return.is_default()
            && self.expression.is_default()
            && self.semicolon_char.is_default()
    }
}

impl revert_statement::_T0 {
    pub fn new(
        (((revert, expression), argument_list), semicolon_char): (
            ((FixedTerminal<6usize>, expression::N), argument_list::N),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            revert,
            expression,
            argument_list,
            semicolon_char,
        }
    }
}

impl DefaultTest for revert_statement::_T0 {}

impl state_variable_declaration::_T0 {
    pub fn new(
        ((((type_name, state_variable_attributes), identifier), _t2), semicolon_char): (
            (
                (
                    (type_name::N, Vec<state_variable_attribute::N>),
                    identifier::N,
                ),
                Option<Box<state_variable_declaration::_T2>>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            type_name,
            state_variable_attributes,
            identifier,
            _t2,
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

impl DefaultTest for state_variable_declaration::_T2 {}

impl try_statement::_T0 {
    pub fn new(
        ((((r#try, expression), _t1), block), catch_clauses): (
            (
                (
                    (FixedTerminal<3usize>, expression::N),
                    Option<Box<try_statement::_T1>>,
                ),
                block::N,
            ),
            Vec<catch_clause::N>,
        ),
    ) -> Self {
        Self {
            r#try,
            expression,
            _t1,
            block,
            catch_clauses,
        }
    }
}

impl DefaultTest for try_statement::_T0 {}

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
        (variable_declaration_statement, semicolon_char): (
            Box<variable_declaration_statement::VariableDeclarationStatement>,
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            variable_declaration_statement,
            semicolon_char,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T0 {}

impl DefaultTest for variable_declaration_statement::VariableDeclarationStatement {}

impl variable_declaration_statement::_T3 {
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

impl DefaultTest for variable_declaration_statement::_T3 {}

impl variable_declaration_statement::_T1 {
    pub fn new(
        (variable_declaration, _t2): (
            variable_declaration::N,
            Option<Box<variable_declaration_statement::_T2>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            _t2,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T1 {}

impl variable_declaration_statement::_T2 {
    pub fn new((equal_char, expression): (FixedTerminal<1usize>, expression::N)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}

impl DefaultTest for variable_declaration_statement::_T2 {}

impl while_statement::_T0 {
    pub fn new(
        ((((r#while, open_paren_char), expression), close_paren_char), statement): (
            (
                (
                    (FixedTerminal<5usize>, FixedTerminal<1usize>),
                    expression::N,
                ),
                FixedTerminal<1usize>,
            ),
            statement::N,
        ),
    ) -> Self {
        Self {
            r#while,
            open_paren_char,
            expression,
            close_paren_char,
            statement,
        }
    }
}

impl DefaultTest for while_statement::_T0 {}

impl DefaultTest for simple_statement::SimpleStatement {}

impl for_statement::_T0 {
    pub fn new(
        (
            (
                ((((r#for, open_paren_char), for_statement_1), for_statement_2), expression),
                close_paren_char,
            ),
            statement,
        ): (
            (
                (
                    (
                        (
                            (FixedTerminal<3usize>, FixedTerminal<1usize>),
                            Box<for_statement::ForStatement>,
                        ),
                        Box<for_statement::ForStatement>,
                    ),
                    Option<expression::N>,
                ),
                FixedTerminal<1usize>,
            ),
            statement::N,
        ),
    ) -> Self {
        Self {
            r#for,
            open_paren_char,
            for_statement_1,
            for_statement_2,
            expression,
            close_paren_char,
            statement,
        }
    }
}

impl DefaultTest for for_statement::_T0 {}

impl DefaultTest for for_statement::ForStatement {}

impl DefaultTest for for_statement::ForStatement {}

impl DefaultTest for statement::Statement {}

impl block::_T0 {
    pub fn new(
        ((open_brace_char, blocks), close_brace_char): (
            (FixedTerminal<1usize>, Vec<Box<block::Block>>),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            blocks,
            close_brace_char,
        }
    }
}

impl DefaultTest for block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace_char.is_default()
            && self.blocks.is_default()
            && self.close_brace_char.is_default()
    }
}

impl DefaultTest for block::Block {}

impl constructor_definition::_T0 {
    pub fn new(
        (((constructor, parameter_list), constructor_attributes), block): (
            (
                (FixedTerminal<11usize>, parameter_list::N),
                Vec<constructor_attribute::N>,
            ),
            block::N,
        ),
    ) -> Self {
        Self {
            constructor,
            parameter_list,
            constructor_attributes,
            block,
        }
    }
}

impl DefaultTest for constructor_definition::_T0 {
    fn is_default(&self) -> bool {
        self.constructor.is_default()
            && self.parameter_list.is_default()
            && self.constructor_attributes.is_default()
            && self.block.is_default()
    }
}

impl fallback_function_definition::_T0 {
    pub fn new(
        (
            (((fallback, parameter_list), fallback_function_attributes), _t2),
            fallback_function_definition,
        ): (
            (
                (
                    (FixedTerminal<8usize>, parameter_list::N),
                    Vec<fallback_function_attribute::N>,
                ),
                Option<Box<fallback_function_definition::_T2>>,
            ),
            Box<fallback_function_definition::FallbackFunctionDefinition>,
        ),
    ) -> Self {
        Self {
            fallback,
            parameter_list,
            fallback_function_attributes,
            _t2,
            fallback_function_definition,
        }
    }
}

impl DefaultTest for fallback_function_definition::_T0 {}

impl DefaultTest for fallback_function_definition::FallbackFunctionDefinition {}

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
            ((((function, function_definition_1), parameter_list), function_attributes), _t2),
            function_definition_2,
        ): (
            (
                (
                    (
                        (
                            FixedTerminal<8usize>,
                            Box<function_definition::FunctionDefinition>,
                        ),
                        parameter_list::N,
                    ),
                    Vec<function_attribute::N>,
                ),
                Option<Box<function_definition::_T2>>,
            ),
            Box<function_definition::FunctionDefinition>,
        ),
    ) -> Self {
        Self {
            function,
            function_definition_1,
            parameter_list,
            function_attributes,
            _t2,
            function_definition_2,
        }
    }
}

impl DefaultTest for function_definition::_T0 {}

impl DefaultTest for function_definition::FunctionDefinition {}

impl function_definition::_T2 {
    pub fn new(
        (returns, non_empty_parameter_list): (FixedTerminal<7usize>, non_empty_parameter_list::N),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}

impl DefaultTest for function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.non_empty_parameter_list.is_default()
    }
}

impl DefaultTest for function_definition::FunctionDefinition {}

impl modifier_definition::_T0 {
    pub fn new(
        ((((modifier, identifier), parameter_list), method_attributes), modifier_definition): (
            (
                (
                    (FixedTerminal<8usize>, identifier::N),
                    Option<parameter_list::N>,
                ),
                Vec<method_attribute::N>,
            ),
            Box<modifier_definition::ModifierDefinition>,
        ),
    ) -> Self {
        Self {
            modifier,
            identifier,
            parameter_list,
            method_attributes,
            modifier_definition,
        }
    }
}

impl DefaultTest for modifier_definition::_T0 {}

impl DefaultTest for modifier_definition::ModifierDefinition {}

impl receive_function_definition::_T0 {
    pub fn new(
        (
            (((receive, open_paren_char), close_paren_char), receive_function_attributes),
            receive_function_definition,
        ): (
            (
                (
                    (FixedTerminal<7usize>, FixedTerminal<1usize>),
                    FixedTerminal<1usize>,
                ),
                Vec<receive_function_attribute::N>,
            ),
            Box<receive_function_definition::ReceiveFunctionDefinition>,
        ),
    ) -> Self {
        Self {
            receive,
            open_paren_char,
            close_paren_char,
            receive_function_attributes,
            receive_function_definition,
        }
    }
}

impl DefaultTest for receive_function_definition::_T0 {}

impl DefaultTest for receive_function_definition::ReceiveFunctionDefinition {}

impl DefaultTest for contract_body_element::ContractBodyElement {}

impl contract_definition::_T0 {
    pub fn new(
        (
            (
                (
                    (((r#abstract, contract), identifier), inheritance_specifier_list),
                    open_brace_char,
                ),
                contract_body_elements,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        (
                            (Option<FixedTerminal<8usize>>, FixedTerminal<8usize>),
                            identifier::N,
                        ),
                        Option<inheritance_specifier_list::N>,
                    ),
                    FixedTerminal<1usize>,
                ),
                Vec<contract_body_element::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            r#abstract,
            contract,
            identifier,
            inheritance_specifier_list,
            open_brace_char,
            contract_body_elements,
            close_brace_char,
        }
    }
}

impl DefaultTest for contract_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#abstract.is_default()
            && self.contract.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.open_brace_char.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace_char.is_default()
    }
}

impl interface_definition::_T0 {
    pub fn new(
        (
            (
                (((interface, identifier), inheritance_specifier_list), open_brace_char),
                contract_body_elements,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        (FixedTerminal<9usize>, identifier::N),
                        Option<inheritance_specifier_list::N>,
                    ),
                    FixedTerminal<1usize>,
                ),
                Vec<contract_body_element::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            interface,
            identifier,
            inheritance_specifier_list,
            open_brace_char,
            contract_body_elements,
            close_brace_char,
        }
    }
}

impl DefaultTest for interface_definition::_T0 {
    fn is_default(&self) -> bool {
        self.interface.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self.open_brace_char.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace_char.is_default()
    }
}

impl library_definition::_T0 {
    pub fn new(
        ((((library, identifier), open_brace_char), contract_body_elements), close_brace_char): (
            (
                (
                    (FixedTerminal<7usize>, identifier::N),
                    FixedTerminal<1usize>,
                ),
                Vec<contract_body_element::N>,
            ),
            FixedTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            library,
            identifier,
            open_brace_char,
            contract_body_elements,
            close_brace_char,
        }
    }
}

impl DefaultTest for library_definition::_T0 {
    fn is_default(&self) -> bool {
        self.library.is_default()
            && self.identifier.is_default()
            && self.open_brace_char.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace_char.is_default()
    }
}

impl DefaultTest for definition::Definition {}

impl source_unit::_T0 {
    pub fn new(
        ((ignore, source_units), end_marker): ((ignore::N, Vec<Box<source_unit::SourceUnit>>), ()),
    ) -> Self {
        Self {
            ignore,
            source_units,
            end_marker,
        }
    }
}

impl DefaultTest for source_unit::_T0 {
    fn is_default(&self) -> bool {
        self.ignore.is_default() && self.source_units.is_default() && self.end_marker.is_default()
    }
}

impl DefaultTest for source_unit::SourceUnit {}
