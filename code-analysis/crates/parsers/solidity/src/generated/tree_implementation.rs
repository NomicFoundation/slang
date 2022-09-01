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
impl DefaultTest for VariableSizeTerminal {
    fn is_default(&self) -> bool {
        self.0 == 0
    }
}
impl DefaultTest for VariableSizeTerminalWithTrivia {
    fn is_default(&self) -> bool {
        self.content.is_default() && self.leading.is_default() && self.trailing.is_default()
    }
}
impl<const N: usize> DefaultTest for FixedSizeTerminal<N> {
    fn is_default(&self) -> bool {
        true
    }
}
impl<const N: usize> DefaultTest for FixedSizeTerminalWithTrivia<N> {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.trailing.is_default()
    }
}

impl decimal_integer::_T2 {
    pub fn from_parse(
        (underscore_char, _1): (Option<FixedSizeTerminal<1>>, FixedSizeTerminal<1>),
    ) -> Self {
        Self {
            underscore_char,
            _1,
        }
    }
}
impl Default for decimal_integer::_T2 {
    fn default() -> Self {
        Self {
            underscore_char: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for decimal_integer::_T2 {
    fn is_default(&self) -> bool {
        self.underscore_char.is_default() && self._1.is_default()
    }
}
impl decimal_integer::_T0 {
    pub fn from_parse((_0, _t2s): (FixedSizeTerminal<1>, Vec<decimal_integer::_T2>)) -> Self {
        Self { _0, _t2s }
    }
}
impl Default for decimal_integer::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _t2s: Default::default(),
        }
    }
}
impl DefaultTest for decimal_integer::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._t2s.is_default()
    }
}
impl DefaultTest for decimal_integer::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for end_of_line::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl fixed_bytes_type::_T0 {
    pub fn from_parse(
        (bytes, _1): (FixedSizeTerminal<5usize>, Option<VariableSizeTerminal>),
    ) -> Self {
        Self { bytes, _1 }
    }
}
impl Default for fixed_bytes_type::_T0 {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for fixed_bytes_type::_T0 {
    fn is_default(&self) -> bool {
        self.bytes.is_default() && self._1.is_default()
    }
}
impl DefaultTest for fixed_bytes_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl hex_byte_escape::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self { _0, _1 }
    }
}
impl Default for hex_byte_escape::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for hex_byte_escape::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}
impl DefaultTest for hex_byte_escape::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl hex_number::_T3 {
    pub fn from_parse(
        (underscore_char, _1): (Option<FixedSizeTerminal<1>>, FixedSizeTerminal<1>),
    ) -> Self {
        Self {
            underscore_char,
            _1,
        }
    }
}
impl Default for hex_number::_T3 {
    fn default() -> Self {
        Self {
            underscore_char: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::_T3 {
    fn is_default(&self) -> bool {
        self.underscore_char.is_default() && self._1.is_default()
    }
}
impl hex_number::_T1 {
    pub fn from_parse((_0, _t3s): (FixedSizeTerminal<1>, Vec<hex_number::_T3>)) -> Self {
        Self { _0, _t3s }
    }
}
impl Default for hex_number::_T1 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _t3s: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._t3s.is_default()
    }
}
impl hex_number::_T0 {
    pub fn from_parse((zero_x, _t1): (FixedSizeTerminal<2usize>, hex_number::_T1)) -> Self {
        Self { zero_x, _t1 }
    }
}
impl Default for hex_number::_T0 {
    fn default() -> Self {
        Self {
            zero_x: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::_T0 {
    fn is_default(&self) -> bool {
        self.zero_x.is_default() && self._t1.is_default()
    }
}
impl DefaultTest for hex_number::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl multiline_comment::_T3 {
    pub fn from_parse((star_chars, _1): (VariableSizeTerminal, FixedSizeTerminal<1>)) -> Self {
        Self { star_chars, _1 }
    }
}
impl Default for multiline_comment::_T3 {
    fn default() -> Self {
        Self {
            star_chars: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::_T3 {
    fn is_default(&self) -> bool {
        self.star_chars.is_default() && self._1.is_default()
    }
}
impl multiline_comment::Content {
    pub fn from_parse(
        (_t2s, star_chars): (Vec<Box<multiline_comment::_T2>>, VariableSizeTerminal),
    ) -> Self {
        Self { _t2s, star_chars }
    }
}
impl Default for multiline_comment::Content {
    fn default() -> Self {
        Self {
            _t2s: Default::default(),
            star_chars: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::Content {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self.star_chars.is_default()
    }
}
impl Default for multiline_comment::_T0 {
    fn default() -> Self {
        Self {
            slash_star: Default::default(),
            content: Default::default(),
            star_slash: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::_T0 {
    fn is_default(&self) -> bool {
        self.slash_star.is_default() && self.content.is_default() && self.star_slash.is_default()
    }
}
impl multiline_comment::_T0 {
    pub fn from_parse(
        ((slash_star, content), star_slash): (
            (FixedSizeTerminal<2usize>, multiline_comment::Content),
            FixedSizeTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            slash_star,
            content,
            star_slash,
        }
    }
}
impl DefaultTest for multiline_comment::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl possibly_separated_pairs_of_hex_digits::_T3 {
    pub fn from_parse(
        (underscore_char, _1): (Option<FixedSizeTerminal<1>>, VariableSizeTerminal),
    ) -> Self {
        Self {
            underscore_char,
            _1,
        }
    }
}
impl Default for possibly_separated_pairs_of_hex_digits::_T3 {
    fn default() -> Self {
        Self {
            underscore_char: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::_T3 {
    fn is_default(&self) -> bool {
        self.underscore_char.is_default() && self._1.is_default()
    }
}
impl possibly_separated_pairs_of_hex_digits::_T0 {
    pub fn from_parse(
        (_0, _t3s): (
            VariableSizeTerminal,
            Vec<possibly_separated_pairs_of_hex_digits::_T3>,
        ),
    ) -> Self {
        Self { _0, _t3s }
    }
}
impl Default for possibly_separated_pairs_of_hex_digits::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _t3s: Default::default(),
        }
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._t3s.is_default()
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl raw_identifier::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self { _0, _1 }
    }
}
impl Default for raw_identifier::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for raw_identifier::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}
impl DefaultTest for raw_identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl signed_fixed_type::_T1 {
    pub fn from_parse(
        ((_0, _1), _2): (
            (VariableSizeTerminal, FixedSizeTerminal<1>),
            VariableSizeTerminal,
        ),
    ) -> Self {
        Self { _0, _1, _2 }
    }
}
impl Default for signed_fixed_type::_T1 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _1: Default::default(),
            _2: Default::default(),
        }
    }
}
impl DefaultTest for signed_fixed_type::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default() && self._2.is_default()
    }
}
impl signed_fixed_type::_T0 {
    pub fn from_parse(
        (fixed, _t1): (FixedSizeTerminal<5usize>, Option<signed_fixed_type::_T1>),
    ) -> Self {
        Self { fixed, _t1 }
    }
}
impl Default for signed_fixed_type::_T0 {
    fn default() -> Self {
        Self {
            fixed: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for signed_fixed_type::_T0 {
    fn is_default(&self) -> bool {
        self.fixed.is_default() && self._t1.is_default()
    }
}
impl DefaultTest for signed_fixed_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl signed_integer_type::_T0 {
    pub fn from_parse(
        (int, _1): (FixedSizeTerminal<3usize>, Option<VariableSizeTerminal>),
    ) -> Self {
        Self { int, _1 }
    }
}
impl Default for signed_integer_type::_T0 {
    fn default() -> Self {
        Self {
            int: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for signed_integer_type::_T0 {
    fn is_default(&self) -> bool {
        self.int.is_default() && self._1.is_default()
    }
}
impl DefaultTest for signed_integer_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl single_line_comment::_T0 {
    pub fn from_parse(
        (slash_slash, _1): (FixedSizeTerminal<2usize>, VariableSizeTerminal),
    ) -> Self {
        Self { slash_slash, _1 }
    }
}
impl Default for single_line_comment::_T0 {
    fn default() -> Self {
        Self {
            slash_slash: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for single_line_comment::_T0 {
    fn is_default(&self) -> bool {
        self.slash_slash.is_default() && self._1.is_default()
    }
}
impl DefaultTest for single_line_comment::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl unicode_escape::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self { _0, _1 }
    }
}
impl Default for unicode_escape::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for unicode_escape::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}
impl DefaultTest for unicode_escape::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl Default for version_pragma_value::_T0 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for version_pragma_value::_T0 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl DefaultTest for version_pragma_value::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for whitespace::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl yul_decimal_number_literal::_T1 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self { _0, _1 }
    }
}
impl Default for yul_decimal_number_literal::_T1 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for yul_decimal_number_literal::_T1 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self._1.is_default()
    }
}

impl yul_hex_literal::_T0 {
    pub fn from_parse((zero_x, _1): (FixedSizeTerminal<2usize>, VariableSizeTerminal)) -> Self {
        Self { zero_x, _1 }
    }
}
impl Default for yul_hex_literal::_T0 {
    fn default() -> Self {
        Self {
            zero_x: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for yul_hex_literal::_T0 {
    fn is_default(&self) -> bool {
        self.zero_x.is_default() && self._1.is_default()
    }
}
impl DefaultTest for yul_hex_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl decimal_exponent::_T0 {
    pub fn from_parse(
        ((_0, minus_char), decimal_integer): (
            (FixedSizeTerminal<1>, Option<FixedSizeTerminal<1>>),
            DecimalInteger,
        ),
    ) -> Self {
        Self {
            _0,
            minus_char,
            decimal_integer,
        }
    }
}
impl Default for decimal_exponent::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            minus_char: Default::default(),
            decimal_integer: Default::default(),
        }
    }
}
impl DefaultTest for decimal_exponent::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.minus_char.is_default() && self.decimal_integer.is_default()
    }
}
impl DefaultTest for decimal_exponent::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl decimal_float::_T0 {
    pub fn from_parse(
        ((decimal_integer_1, period_char), decimal_integer_2): (
            (Option<DecimalInteger>, FixedSizeTerminal<1>),
            DecimalInteger,
        ),
    ) -> Self {
        Self {
            decimal_integer_1,
            period_char,
            decimal_integer_2,
        }
    }
}
impl Default for decimal_float::_T0 {
    fn default() -> Self {
        Self {
            decimal_integer_1: Default::default(),
            period_char: Default::default(),
            decimal_integer_2: Default::default(),
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
impl DefaultTest for decimal_float::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for end_of_file_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl escape_sequence::_T0 {
    pub fn from_parse(
        (backslash_char, _t1): (FixedSizeTerminal<1>, Box<escape_sequence::_T1>),
    ) -> Self {
        Self {
            backslash_char,
            _t1,
        }
    }
}

impl Default for hex_string_literal::_T2 {
    fn default() -> Self {
        Self {
            double_quote_1: Default::default(),
            possibly_separated_pairs_of_hex_digits: Default::default(),
            double_quote_2: Default::default(),
        }
    }
}
impl DefaultTest for hex_string_literal::_T2 {
    fn is_default(&self) -> bool {
        self.double_quote_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.double_quote_2.is_default()
    }
}
impl hex_string_literal::_T2 {
    pub fn from_parse(
        ((double_quote_1, possibly_separated_pairs_of_hex_digits), double_quote_2): (
            (
                FixedSizeTerminal<1usize>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_1,
            possibly_separated_pairs_of_hex_digits,
            double_quote_2,
        }
    }
}
impl Default for hex_string_literal::_T3 {
    fn default() -> Self {
        Self {
            quote_1: Default::default(),
            possibly_separated_pairs_of_hex_digits: Default::default(),
            quote_2: Default::default(),
        }
    }
}
impl DefaultTest for hex_string_literal::_T3 {
    fn is_default(&self) -> bool {
        self.quote_1.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.quote_2.is_default()
    }
}
impl hex_string_literal::_T3 {
    pub fn from_parse(
        ((quote_1, possibly_separated_pairs_of_hex_digits), quote_2): (
            (
                FixedSizeTerminal<1usize>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_1,
            possibly_separated_pairs_of_hex_digits,
            quote_2,
        }
    }
}
impl hex_string_literal::_T0 {
    pub fn from_parse(
        (hex, _t1): (FixedSizeTerminal<3usize>, Box<hex_string_literal::_T1>),
    ) -> Self {
        Self { hex, _t1 }
    }
}

impl DefaultTest for leading_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl trailing_trivia::_T0 {
    pub fn from_parse(
        (_t2s, _t3): (Vec<Box<trailing_trivia::_T2>>, Box<trailing_trivia::_T3>),
    ) -> Self {
        Self { _t2s, _t3 }
    }
}
impl DefaultTest for trailing_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl unsigned_fixed_type::_T0 {
    pub fn from_parse((_0, signed_fixed_type): (FixedSizeTerminal<1>, SignedFixedType)) -> Self {
        Self {
            _0,
            signed_fixed_type,
        }
    }
}
impl Default for unsigned_fixed_type::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            signed_fixed_type: Default::default(),
        }
    }
}
impl DefaultTest for unsigned_fixed_type::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.signed_fixed_type.is_default()
    }
}
impl DefaultTest for unsigned_fixed_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl unsigned_integer_type::_T0 {
    pub fn from_parse(
        (_0, signed_integer_type): (FixedSizeTerminal<1>, SignedIntegerType),
    ) -> Self {
        Self {
            _0,
            signed_integer_type,
        }
    }
}
impl Default for unsigned_integer_type::_T0 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            signed_integer_type: Default::default(),
        }
    }
}
impl DefaultTest for unsigned_integer_type::_T0 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.signed_integer_type.is_default()
    }
}
impl DefaultTest for unsigned_integer_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for yul_identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl address_type::_T0 {
    pub fn from_parse(
        (address, payable): (
            FixedSizeTerminalWithTrivia<7usize>,
            Option<FixedSizeTerminalWithTrivia<7usize>>,
        ),
    ) -> Self {
        Self { address, payable }
    }
}
impl Default for address_type::_T0 {
    fn default() -> Self {
        Self {
            address: Default::default(),
            payable: Default::default(),
        }
    }
}
impl DefaultTest for address_type::_T0 {
    fn is_default(&self) -> bool {
        self.address.is_default() && self.payable.is_default()
    }
}

impl Default for array_literal::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for array_literal::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for array_literal::_T0 {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            expressions: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for array_literal::_T0 {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default()
            && self.expressions.is_default()
            && self.close_bracket.is_default()
    }
}
impl array_literal::_T0 {
    pub fn from_parse(
        ((open_bracket, expressions), close_bracket): (
            (FixedSizeTerminalWithTrivia<1usize>, array_literal::_T1),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket,
            expressions,
            close_bracket,
        }
    }
}

impl break_statement::_T0 {
    pub fn from_parse(
        (r#break, semicolon_char): (
            FixedSizeTerminalWithTrivia<5usize>,
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#break,
            semicolon_char,
        }
    }
}
impl Default for break_statement::_T0 {
    fn default() -> Self {
        Self {
            r#break: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for break_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#break.is_default() && self.semicolon_char.is_default()
    }
}

impl continue_statement::_T0 {
    pub fn from_parse(
        (r#continue, semicolon_char): (
            FixedSizeTerminalWithTrivia<8usize>,
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#continue,
            semicolon_char,
        }
    }
}
impl Default for continue_statement::_T0 {
    fn default() -> Self {
        Self {
            r#continue: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for continue_statement::_T0 {
    fn is_default(&self) -> bool {
        self.r#continue.is_default() && self.semicolon_char.is_default()
    }
}

impl decimal_number::_T0 {
    pub fn from_parse(
        (_t1, decimal_exponent): (Box<decimal_number::_T1>, Option<DecimalExponent>),
    ) -> Self {
        Self {
            _t1,
            decimal_exponent,
        }
    }
}

impl Default for double_quoted_ascii_string_literal::_T0 {
    fn default() -> Self {
        Self {
            double_quote_1: Default::default(),
            runs: Default::default(),
            double_quote_2: Default::default(),
        }
    }
}
impl DefaultTest for double_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.double_quote_1.is_default()
            && self.runs.is_default()
            && self.double_quote_2.is_default()
    }
}
impl double_quoted_ascii_string_literal::_T0 {
    pub fn from_parse(
        ((double_quote_1, runs), double_quote_2): (
            (
                FixedSizeTerminal<1usize>,
                Vec<Box<double_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            double_quote_1,
            runs,
            double_quote_2,
        }
    }
}
impl DefaultTest for double_quoted_ascii_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl Default for double_quoted_unicode_string_literal::_T0 {
    fn default() -> Self {
        Self {
            unicode_double_quote: Default::default(),
            runs: Default::default(),
            double_quote: Default::default(),
        }
    }
}
impl DefaultTest for double_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_double_quote.is_default()
            && self.runs.is_default()
            && self.double_quote.is_default()
    }
}
impl double_quoted_unicode_string_literal::_T0 {
    pub fn from_parse(
        ((unicode_double_quote, runs), double_quote): (
            (
                FixedSizeTerminal<8usize>,
                Vec<Box<double_quoted_unicode_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            runs,
            double_quote,
        }
    }
}
impl DefaultTest for double_quoted_unicode_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl Default for parenthesis_expression::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for parenthesis_expression::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for parenthesis_expression::_T0 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            expressions: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for parenthesis_expression::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.expressions.is_default()
            && self.close_paren.is_default()
    }
}
impl parenthesis_expression::_T0 {
    pub fn from_parse(
        ((open_paren, expressions), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                parenthesis_expression::_T1,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            expressions,
            close_paren,
        }
    }
}

impl Default for positional_argument_list::_T0 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for positional_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}

impl Default for single_quoted_ascii_string_literal::_T0 {
    fn default() -> Self {
        Self {
            quote_1: Default::default(),
            runs: Default::default(),
            quote_2: Default::default(),
        }
    }
}
impl DefaultTest for single_quoted_ascii_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.quote_1.is_default() && self.runs.is_default() && self.quote_2.is_default()
    }
}
impl single_quoted_ascii_string_literal::_T0 {
    pub fn from_parse(
        ((quote_1, runs), quote_2): (
            (
                FixedSizeTerminal<1usize>,
                Vec<Box<single_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            quote_1,
            runs,
            quote_2,
        }
    }
}
impl DefaultTest for single_quoted_ascii_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl Default for single_quoted_unicode_string_literal::_T0 {
    fn default() -> Self {
        Self {
            unicode_quote: Default::default(),
            runs: Default::default(),
            quote: Default::default(),
        }
    }
}
impl DefaultTest for single_quoted_unicode_string_literal::_T0 {
    fn is_default(&self) -> bool {
        self.unicode_quote.is_default() && self.runs.is_default() && self.quote.is_default()
    }
}
impl single_quoted_unicode_string_literal::_T0 {
    pub fn from_parse(
        ((unicode_quote, runs), quote): (
            (
                FixedSizeTerminal<8usize>,
                Vec<Box<single_quoted_unicode_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            runs,
            quote,
        }
    }
}
impl DefaultTest for single_quoted_unicode_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl unchecked_block::_T0 {
    pub fn from_parse((unchecked, block): (FixedSizeTerminalWithTrivia<9usize>, Block)) -> Self {
        Self { unchecked, block }
    }
}
impl Default for unchecked_block::_T0 {
    fn default() -> Self {
        Self {
            unchecked: Default::default(),
            block: Default::default(),
        }
    }
}
impl DefaultTest for unchecked_block::_T0 {
    fn is_default(&self) -> bool {
        self.unchecked.is_default() && self.block.is_default()
    }
}

impl version_pragma_specifier::_T2 {
    pub fn from_parse(
        (_0, version_pragma_value): (
            VariableSizeTerminalWithTrivia,
            version_pragma_value::WithTrivia,
        ),
    ) -> Self {
        Self {
            _0,
            version_pragma_value,
        }
    }
}
impl Default for version_pragma_specifier::_T2 {
    fn default() -> Self {
        Self {
            _0: Default::default(),
            version_pragma_value: Default::default(),
        }
    }
}
impl DefaultTest for version_pragma_specifier::_T2 {
    fn is_default(&self) -> bool {
        self._0.is_default() && self.version_pragma_value.is_default()
    }
}
impl version_pragma_specifier::_T0 {
    pub fn from_parse(
        (solidity, _t2s): (
            FixedSizeTerminalWithTrivia<8usize>,
            Vec<version_pragma_specifier::_T2>,
        ),
    ) -> Self {
        Self { solidity, _t2s }
    }
}
impl Default for version_pragma_specifier::_T0 {
    fn default() -> Self {
        Self {
            solidity: Default::default(),
            _t2s: Default::default(),
        }
    }
}
impl DefaultTest for version_pragma_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.solidity.is_default() && self._t2s.is_default()
    }
}

impl Default for yul_function_call::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for yul_function_call::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            yul_expressions: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.yul_expressions.is_default()
            && self.close_paren.is_default()
    }
}
impl yul_function_call::_T1 {
    pub fn from_parse(
        ((open_paren, yul_expressions), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<yul_function_call::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            yul_expressions,
            close_paren,
        }
    }
}
impl yul_function_call::_T0 {
    pub fn from_parse(
        (yul_identifier, _t1): (yul_identifier::WithTrivia, yul_function_call::_T1),
    ) -> Self {
        Self {
            yul_identifier,
            _t1,
        }
    }
}
impl Default for yul_function_call::_T0 {
    fn default() -> Self {
        Self {
            yul_identifier: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::_T0 {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default() && self._t1.is_default()
    }
}

impl Default for yul_function_definition::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for yul_function_definition::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            yul_identifiers: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.yul_identifiers.is_default()
            && self.close_paren.is_default()
    }
}
impl yul_function_definition::_T1 {
    pub fn from_parse(
        ((open_paren, yul_identifiers), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<yul_function_definition::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            yul_identifiers,
            close_paren,
        }
    }
}
impl Default for yul_function_definition::_T4 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::_T4 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl yul_function_definition::_T3 {
    pub fn from_parse(
        (minus_greater, yul_identifiers): (
            FixedSizeTerminalWithTrivia<2usize>,
            yul_function_definition::_T4,
        ),
    ) -> Self {
        Self {
            minus_greater,
            yul_identifiers,
        }
    }
}
impl Default for yul_function_definition::_T3 {
    fn default() -> Self {
        Self {
            minus_greater: Default::default(),
            yul_identifiers: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.minus_greater.is_default() && self.yul_identifiers.is_default()
    }
}
impl yul_function_definition::_T0 {
    pub fn from_parse(
        ((((function, yul_identifier), _t1), _t3), yul_block): (
            (
                (
                    (
                        FixedSizeTerminalWithTrivia<8usize>,
                        yul_identifier::WithTrivia,
                    ),
                    yul_function_definition::_T1,
                ),
                Option<yul_function_definition::_T3>,
            ),
            YulBlock,
        ),
    ) -> Self {
        Self {
            function,
            yul_identifier,
            _t1,
            _t3,
            yul_block,
        }
    }
}
impl Default for yul_function_definition::_T0 {
    fn default() -> Self {
        Self {
            function: Default::default(),
            yul_identifier: Default::default(),
            _t1: Default::default(),
            _t3: Default::default(),
            yul_block: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::_T0 {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.yul_identifier.is_default()
            && self._t1.is_default()
            && self._t3.is_default()
            && self.yul_block.is_default()
    }
}

impl Default for yul_identifier_path::_T0 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_identifier_path::_T0 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}

impl Default for assembly_flags::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for assembly_flags::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for assembly_flags::_T0 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            double_quoted_ascii_string_literals: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for assembly_flags::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.double_quoted_ascii_string_literals.is_default()
            && self.close_paren.is_default()
    }
}
impl assembly_flags::_T0 {
    pub fn from_parse(
        ((open_paren, double_quoted_ascii_string_literals), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, assembly_flags::_T1),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            double_quoted_ascii_string_literals,
            close_paren,
        }
    }
}

impl DefaultTest for identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl numeric_literal::_T0 {
    pub fn from_parse(
        (_t1, _1): (Box<numeric_literal::_T1>, Option<VariableSizeTerminal>),
    ) -> Self {
        Self { _t1, _1 }
    }
}

impl abi_coder_pragma_specifier::_T0 {
    pub fn from_parse(
        (abicoder, identifier): (FixedSizeTerminalWithTrivia<8usize>, identifier::WithTrivia),
    ) -> Self {
        Self {
            abicoder,
            identifier,
        }
    }
}
impl Default for abi_coder_pragma_specifier::_T0 {
    fn default() -> Self {
        Self {
            abicoder: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for abi_coder_pragma_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.abicoder.is_default() && self.identifier.is_default()
    }
}

impl delete_statement::_T0 {
    pub fn from_parse(
        ((delete, identifier), semicolon_char): (
            (FixedSizeTerminalWithTrivia<6usize>, identifier::WithTrivia),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            delete,
            identifier,
            semicolon_char,
        }
    }
}
impl Default for delete_statement::_T0 {
    fn default() -> Self {
        Self {
            delete: Default::default(),
            identifier: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for delete_statement::_T0 {
    fn is_default(&self) -> bool {
        self.delete.is_default() && self.identifier.is_default() && self.semicolon_char.is_default()
    }
}

impl Default for enum_definition::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for enum_definition::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            identifiers: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.identifiers.is_default()
            && self.close_brace.is_default()
    }
}
impl enum_definition::_T1 {
    pub fn from_parse(
        ((open_brace, identifiers), close_brace): (
            (FixedSizeTerminalWithTrivia<1usize>, enum_definition::_T2),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            identifiers,
            close_brace,
        }
    }
}
impl enum_definition::_T0 {
    pub fn from_parse(
        ((r#enum, identifier), _t1): (
            (FixedSizeTerminalWithTrivia<4usize>, identifier::WithTrivia),
            enum_definition::_T1,
        ),
    ) -> Self {
        Self {
            r#enum,
            identifier,
            _t1,
        }
    }
}
impl Default for enum_definition::_T0 {
    fn default() -> Self {
        Self {
            r#enum: Default::default(),
            identifier: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#enum.is_default() && self.identifier.is_default() && self._t1.is_default()
    }
}

impl experimental_pragma_specifier::_T0 {
    pub fn from_parse(
        (experimental, identifier): (FixedSizeTerminalWithTrivia<12usize>, identifier::WithTrivia),
    ) -> Self {
        Self {
            experimental,
            identifier,
        }
    }
}
impl Default for experimental_pragma_specifier::_T0 {
    fn default() -> Self {
        Self {
            experimental: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for experimental_pragma_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.experimental.is_default() && self.identifier.is_default()
    }
}

impl Default for identifier_path::_T0 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for identifier_path::_T0 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}

impl named_argument::_T0 {
    pub fn from_parse(
        ((identifier, colon_char), expression): (
            (identifier::WithTrivia, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            identifier,
            colon_char,
            expression,
        }
    }
}

impl parameter_declaration::_T0 {
    pub fn from_parse(
        ((type_name, _1), identifier): (
            (TypeName, Option<VariableSizeTerminalWithTrivia>),
            Option<identifier::WithTrivia>,
        ),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl selected_import::_T1 {
    pub fn from_parse(
        (r#as, identifier): (FixedSizeTerminalWithTrivia<2usize>, identifier::WithTrivia),
    ) -> Self {
        Self { r#as, identifier }
    }
}
impl Default for selected_import::_T1 {
    fn default() -> Self {
        Self {
            r#as: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for selected_import::_T1 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}
impl selected_import::_T0 {
    pub fn from_parse(
        (identifier, _t1): (identifier::WithTrivia, Option<selected_import::_T1>),
    ) -> Self {
        Self { identifier, _t1 }
    }
}
impl Default for selected_import::_T0 {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for selected_import::_T0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self._t1.is_default()
    }
}

impl user_defined_value_type_definition::_T0 {
    pub fn from_parse(
        ((((r#type, identifier), is), elementary_type), semicolon_char): (
            (
                (
                    (FixedSizeTerminalWithTrivia<4usize>, identifier::WithTrivia),
                    FixedSizeTerminalWithTrivia<2usize>,
                ),
                ElementaryType,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#type,
            identifier,
            is,
            elementary_type,
            semicolon_char,
        }
    }
}

impl mapping_type::_T2 {
    pub fn from_parse(
        ((_t3, equal_greater), type_name): (
            (Box<mapping_type::_T3>, FixedSizeTerminalWithTrivia<2usize>),
            TypeName,
        ),
    ) -> Self {
        Self {
            _t3,
            equal_greater,
            type_name,
        }
    }
}
impl mapping_type::_T1 {
    pub fn from_parse(
        ((open_paren, _t2), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, mapping_type::_T2),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            _t2,
            close_paren,
        }
    }
}
impl mapping_type::_T0 {
    pub fn from_parse(
        (mapping, _t1): (FixedSizeTerminalWithTrivia<7usize>, mapping_type::_T1),
    ) -> Self {
        Self { mapping, _t1 }
    }
}

impl Default for named_argument_list::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for named_argument_list::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for named_argument_list::_T0 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            named_arguments: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for named_argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.named_arguments.is_default()
            && self.close_brace.is_default()
    }
}
impl named_argument_list::_T0 {
    pub fn from_parse(
        ((open_brace, named_arguments), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<named_argument_list::_T1>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            named_arguments,
            close_brace,
        }
    }
}

impl Default for override_specifier::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for override_specifier::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            identifier_paths: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.identifier_paths.is_default()
            && self.close_paren.is_default()
    }
}
impl override_specifier::_T1 {
    pub fn from_parse(
        ((open_paren, identifier_paths), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, override_specifier::_T2),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            identifier_paths,
            close_paren,
        }
    }
}
impl override_specifier::_T0 {
    pub fn from_parse(
        (r#override, _t1): (
            FixedSizeTerminalWithTrivia<8usize>,
            Option<override_specifier::_T1>,
        ),
    ) -> Self {
        Self { r#override, _t1 }
    }
}
impl Default for override_specifier::_T0 {
    fn default() -> Self {
        Self {
            r#override: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.r#override.is_default() && self._t1.is_default()
    }
}

impl Default for parameter_list::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for parameter_list::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for parameter_list::_T0 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            parameter_declarations: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for parameter_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.parameter_declarations.is_default()
            && self.close_paren.is_default()
    }
}
impl parameter_list::_T0 {
    pub fn from_parse(
        ((open_paren, parameter_declarations), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<parameter_list::_T1>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            parameter_declarations,
            close_paren,
        }
    }
}

impl pragma_directive::_T0 {
    pub fn from_parse(
        ((pragma, _t1), semicolon_char): (
            (
                FixedSizeTerminalWithTrivia<6usize>,
                Box<pragma_directive::_T1>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            pragma,
            _t1,
            semicolon_char,
        }
    }
}

impl Default for selecting_import_directive::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for selecting_import_directive::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for selecting_import_directive::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            selected_imports: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for selecting_import_directive::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.selected_imports.is_default()
            && self.close_brace.is_default()
    }
}
impl selecting_import_directive::_T1 {
    pub fn from_parse(
        ((open_brace, selected_imports), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                selecting_import_directive::_T2,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            selected_imports,
            close_brace,
        }
    }
}
impl selecting_import_directive::_T0 {
    pub fn from_parse(
        ((_t1, from), import_path): (
            (
                selecting_import_directive::_T1,
                FixedSizeTerminalWithTrivia<4usize>,
            ),
            ImportPath,
        ),
    ) -> Self {
        Self {
            _t1,
            from,
            import_path,
        }
    }
}

impl simple_import_directive::_T2 {
    pub fn from_parse(
        (r#as, identifier): (FixedSizeTerminalWithTrivia<2usize>, identifier::WithTrivia),
    ) -> Self {
        Self { r#as, identifier }
    }
}
impl Default for simple_import_directive::_T2 {
    fn default() -> Self {
        Self {
            r#as: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for simple_import_directive::_T2 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}
impl simple_import_directive::_T0 {
    pub fn from_parse(
        (import_path, _t2s): (ImportPath, Vec<simple_import_directive::_T2>),
    ) -> Self {
        Self { import_path, _t2s }
    }
}

impl star_import_directive::_T0 {
    pub fn from_parse(
        ((((star_char, r#as), identifier), from), import_path): (
            (
                (
                    (
                        FixedSizeTerminalWithTrivia<1>,
                        FixedSizeTerminalWithTrivia<2usize>,
                    ),
                    identifier::WithTrivia,
                ),
                FixedSizeTerminalWithTrivia<4usize>,
            ),
            ImportPath,
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

impl Default for argument_list::_T0 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            _t1: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for argument_list::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default() && self._t1.is_default() && self.close_paren.is_default()
    }
}
impl argument_list::_T0 {
    pub fn from_parse(
        ((open_paren, _t1), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<Box<argument_list::_T1>>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            _t1,
            close_paren,
        }
    }
}

impl catch_clause::_T1 {
    pub fn from_parse(
        (identifier, parameter_list): (Option<identifier::WithTrivia>, ParameterList),
    ) -> Self {
        Self {
            identifier,
            parameter_list,
        }
    }
}
impl Default for catch_clause::_T1 {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for catch_clause::_T1 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.parameter_list.is_default()
    }
}
impl catch_clause::_T0 {
    pub fn from_parse(
        ((catch, _t1), block): (
            (
                FixedSizeTerminalWithTrivia<5usize>,
                Option<catch_clause::_T1>,
            ),
            Block,
        ),
    ) -> Self {
        Self { catch, _t1, block }
    }
}
impl Default for catch_clause::_T0 {
    fn default() -> Self {
        Self {
            catch: Default::default(),
            _t1: Default::default(),
            block: Default::default(),
        }
    }
}
impl DefaultTest for catch_clause::_T0 {
    fn is_default(&self) -> bool {
        self.catch.is_default() && self._t1.is_default() && self.block.is_default()
    }
}

impl function_type::_T2 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for function_type::_T2 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for function_type::_T2 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl function_type::_T0 {
    pub fn from_parse(
        (((function, parameter_list), _2), _t2): (
            (
                (FixedSizeTerminalWithTrivia<8usize>, ParameterList),
                Vec<VariableSizeTerminalWithTrivia>,
            ),
            Option<function_type::_T2>,
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
impl Default for function_type::_T0 {
    fn default() -> Self {
        Self {
            function: Default::default(),
            parameter_list: Default::default(),
            _2: Default::default(),
            _t2: Default::default(),
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

impl import_directive::_T0 {
    pub fn from_parse(
        ((import, _t1), semicolon_char): (
            (
                FixedSizeTerminalWithTrivia<6usize>,
                Box<import_directive::_T1>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            import,
            _t1,
            semicolon_char,
        }
    }
}

impl Default for yul_assignment_statement::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_assignment_statement::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl yul_assignment_statement::_T0 {
    pub fn from_parse(
        ((yul_identifier_paths, colon_equal), yul_expression): (
            (
                yul_assignment_statement::_T1,
                FixedSizeTerminalWithTrivia<2usize>,
            ),
            YulExpression,
        ),
    ) -> Self {
        Self {
            yul_identifier_paths,
            colon_equal,
            yul_expression,
        }
    }
}

impl yul_for_statement::_T0 {
    pub fn from_parse(
        ((((r#for, yul_block_1), yul_expression), yul_block_2), yul_block_3): (
            (
                (
                    (FixedSizeTerminalWithTrivia<3usize>, YulBlock),
                    YulExpression,
                ),
                YulBlock,
            ),
            YulBlock,
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

impl yul_if_statement::_T0 {
    pub fn from_parse(
        ((r#if, yul_expression), yul_block): (
            (FixedSizeTerminalWithTrivia<2usize>, YulExpression),
            YulBlock,
        ),
    ) -> Self {
        Self {
            r#if,
            yul_expression,
            yul_block,
        }
    }
}

impl yul_switch_statement::_T4 {
    pub fn from_parse(
        (case, yul_literal): (FixedSizeTerminalWithTrivia<4usize>, YulLiteral),
    ) -> Self {
        Self { case, yul_literal }
    }
}
impl yul_switch_statement::_T2 {
    pub fn from_parse((_t3, yul_block): (Box<yul_switch_statement::_T3>, YulBlock)) -> Self {
        Self { _t3, yul_block }
    }
}
impl yul_switch_statement::_T0 {
    pub fn from_parse(
        ((switch, yul_expression), _t2s): (
            (FixedSizeTerminalWithTrivia<6usize>, YulExpression),
            Vec<yul_switch_statement::_T2>,
        ),
    ) -> Self {
        Self {
            switch,
            yul_expression,
            _t2s,
        }
    }
}

impl Default for yul_variable_declaration::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for yul_variable_declaration::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl yul_variable_declaration::_T2 {
    pub fn from_parse(
        (colon_equal, yul_expression): (FixedSizeTerminalWithTrivia<2usize>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_variable_declaration::_T0 {
    pub fn from_parse(
        ((r#let, yul_identifier_paths), _t2): (
            (
                FixedSizeTerminalWithTrivia<3usize>,
                yul_variable_declaration::_T1,
            ),
            Option<yul_variable_declaration::_T2>,
        ),
    ) -> Self {
        Self {
            r#let,
            yul_identifier_paths,
            _t2,
        }
    }
}
impl Default for yul_variable_declaration::_T0 {
    fn default() -> Self {
        Self {
            r#let: Default::default(),
            yul_identifier_paths: Default::default(),
            _t2: Default::default(),
        }
    }
}
impl DefaultTest for yul_variable_declaration::_T0 {
    fn is_default(&self) -> bool {
        self.r#let.is_default() && self.yul_identifier_paths.is_default() && self._t2.is_default()
    }
}

impl emit_statement::_T0 {
    pub fn from_parse(
        (((emit, identifier_path), argument_list), semicolon_char): (
            (
                (FixedSizeTerminalWithTrivia<4usize>, IdentifierPath),
                ArgumentList,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            emit,
            identifier_path,
            argument_list,
            semicolon_char,
        }
    }
}
impl Default for emit_statement::_T0 {
    fn default() -> Self {
        Self {
            emit: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for emit_statement::_T0 {
    fn is_default(&self) -> bool {
        self.emit.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
            && self.semicolon_char.is_default()
    }
}

impl inheritance_specifier::_T0 {
    pub fn from_parse(
        (identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl Default for inheritance_specifier::_T0 {
    fn default() -> Self {
        Self {
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl modifier_invocation::_T0 {
    pub fn from_parse(
        (identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl Default for modifier_invocation::_T0 {
    fn default() -> Self {
        Self {
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for modifier_invocation::_T0 {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl new_expression::_T0 {
    pub fn from_parse(
        ((new, identifier_path), argument_list): (
            (FixedSizeTerminalWithTrivia<3usize>, IdentifierPath),
            ArgumentList,
        ),
    ) -> Self {
        Self {
            new,
            identifier_path,
            argument_list,
        }
    }
}
impl Default for new_expression::_T0 {
    fn default() -> Self {
        Self {
            new: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for new_expression::_T0 {
    fn is_default(&self) -> bool {
        self.new.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
    }
}

impl payable_expression::_T0 {
    pub fn from_parse(
        (payable, argument_list): (FixedSizeTerminalWithTrivia<7usize>, ArgumentList),
    ) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}
impl Default for payable_expression::_T0 {
    fn default() -> Self {
        Self {
            payable: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for payable_expression::_T0 {
    fn is_default(&self) -> bool {
        self.payable.is_default() && self.argument_list.is_default()
    }
}

impl revert_statement::_T0 {
    pub fn from_parse(
        (((revert, identifier_path), argument_list), semicolon_char): (
            (
                (FixedSizeTerminalWithTrivia<6usize>, Option<IdentifierPath>),
                ArgumentList,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            revert,
            identifier_path,
            argument_list,
            semicolon_char,
        }
    }
}
impl Default for revert_statement::_T0 {
    fn default() -> Self {
        Self {
            revert: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for revert_statement::_T0 {
    fn is_default(&self) -> bool {
        self.revert.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
            && self.semicolon_char.is_default()
    }
}

impl Default for type_name::_T3 {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            expression: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for type_name::_T3 {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default()
            && self.expression.is_default()
            && self.close_bracket.is_default()
    }
}
impl type_name::_T3 {
    pub fn from_parse(
        ((open_bracket, expression), close_bracket): (
            (FixedSizeTerminalWithTrivia<1usize>, Option<Expression>),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket,
            expression,
            close_bracket,
        }
    }
}
impl type_name::_T0 {
    pub fn from_parse((_t1, _t3s): (Box<type_name::_T1>, Vec<type_name::_T3>)) -> Self {
        Self { _t1, _t3s }
    }
}

impl error_parameter::_T0 {
    pub fn from_parse((type_name, identifier): (TypeName, Option<identifier::WithTrivia>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}

impl event_parameter::_T0 {
    pub fn from_parse(
        ((type_name, indexed), identifier): (
            (TypeName, Option<FixedSizeTerminalWithTrivia<7usize>>),
            Option<identifier::WithTrivia>,
        ),
    ) -> Self {
        Self {
            type_name,
            indexed,
            identifier,
        }
    }
}

impl Default for inheritance_specifier_list::_T1 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier_list::_T1 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl inheritance_specifier_list::_T0 {
    pub fn from_parse(
        (is, inheritance_specifiers): (
            FixedSizeTerminalWithTrivia<2usize>,
            inheritance_specifier_list::_T1,
        ),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers,
        }
    }
}
impl Default for inheritance_specifier_list::_T0 {
    fn default() -> Self {
        Self {
            is: Default::default(),
            inheritance_specifiers: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier_list::_T0 {
    fn is_default(&self) -> bool {
        self.is.is_default() && self.inheritance_specifiers.is_default()
    }
}

impl struct_member::_T0 {
    pub fn from_parse(
        ((type_name, identifier), semicolon_char): (
            (TypeName, identifier::WithTrivia),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            identifier,
            semicolon_char,
        }
    }
}

impl type_expression::_T1 {
    pub fn from_parse(
        ((open_paren, type_name), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, TypeName),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            type_name,
            close_paren,
        }
    }
}
impl type_expression::_T0 {
    pub fn from_parse(
        (r#type, _t1): (FixedSizeTerminalWithTrivia<4usize>, type_expression::_T1),
    ) -> Self {
        Self { r#type, _t1 }
    }
}

impl Default for using_directive::_T3 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for using_directive::_T3 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for using_directive::_T2 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            identifier_paths: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for using_directive::_T2 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.identifier_paths.is_default()
            && self.close_brace.is_default()
    }
}
impl using_directive::_T2 {
    pub fn from_parse(
        ((open_brace, identifier_paths), close_brace): (
            (FixedSizeTerminalWithTrivia<1usize>, using_directive::_T3),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            identifier_paths,
            close_brace,
        }
    }
}
impl using_directive::_T0 {
    pub fn from_parse(
        (((((using, _t1), r#for), _t4), global), semicolon_char): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithTrivia<5usize>,
                            Box<using_directive::_T1>,
                        ),
                        FixedSizeTerminalWithTrivia<3usize>,
                    ),
                    Box<using_directive::_T4>,
                ),
                Option<FixedSizeTerminalWithTrivia<6usize>>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            using,
            _t1,
            r#for,
            _t4,
            global,
            semicolon_char,
        }
    }
}

impl Default for yul_block::_T0 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            yul_statements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for yul_block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.yul_statements.is_default()
            && self.close_brace.is_default()
    }
}
impl yul_block::_T0 {
    pub fn from_parse(
        ((open_brace, yul_statements), close_brace): (
            (FixedSizeTerminalWithTrivia<1usize>, Vec<YulStatement>),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            yul_statements,
            close_brace,
        }
    }
}

impl assembly_statement::_T0 {
    pub fn from_parse(
        (((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block): (
            (
                (
                    FixedSizeTerminalWithTrivia<8usize>,
                    Option<FixedSizeTerminalWithTrivia<8usize>>,
                ),
                Option<AssemblyFlags>,
            ),
            YulBlock,
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
impl Default for assembly_statement::_T0 {
    fn default() -> Self {
        Self {
            assembly: Default::default(),
            double_quote_evmasm_double_quote: Default::default(),
            assembly_flags: Default::default(),
            yul_block: Default::default(),
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

impl Default for error_definition::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for error_definition::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            error_parameters: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.error_parameters.is_default()
            && self.close_paren.is_default()
    }
}
impl error_definition::_T1 {
    pub fn from_parse(
        ((open_paren, error_parameters), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<error_definition::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            error_parameters,
            close_paren,
        }
    }
}
impl error_definition::_T0 {
    pub fn from_parse(
        (((error, identifier), _t1), semicolon_char): (
            (
                (FixedSizeTerminalWithTrivia<5usize>, identifier::WithTrivia),
                error_definition::_T1,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            error,
            identifier,
            _t1,
            semicolon_char,
        }
    }
}
impl Default for error_definition::_T0 {
    fn default() -> Self {
        Self {
            error: Default::default(),
            identifier: Default::default(),
            _t1: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::_T0 {
    fn is_default(&self) -> bool {
        self.error.is_default()
            && self.identifier.is_default()
            && self._t1.is_default()
            && self.semicolon_char.is_default()
    }
}

impl Default for event_definition::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for event_definition::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            event_parameters: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.event_parameters.is_default()
            && self.close_paren.is_default()
    }
}
impl event_definition::_T1 {
    pub fn from_parse(
        ((open_paren, event_parameters), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<event_definition::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            event_parameters,
            close_paren,
        }
    }
}
impl event_definition::_T0 {
    pub fn from_parse(
        ((((event, identifier), _t1), anonymous), semicolon_char): (
            (
                (
                    (FixedSizeTerminalWithTrivia<5usize>, identifier::WithTrivia),
                    event_definition::_T1,
                ),
                Option<FixedSizeTerminalWithTrivia<9usize>>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            event,
            identifier,
            _t1,
            anonymous,
            semicolon_char,
        }
    }
}
impl Default for event_definition::_T0 {
    fn default() -> Self {
        Self {
            event: Default::default(),
            identifier: Default::default(),
            _t1: Default::default(),
            anonymous: Default::default(),
            semicolon_char: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::_T0 {
    fn is_default(&self) -> bool {
        self.event.is_default()
            && self.identifier.is_default()
            && self._t1.is_default()
            && self.anonymous.is_default()
            && self.semicolon_char.is_default()
    }
}

impl Default for struct_definition::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            struct_members: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for struct_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.struct_members.is_default()
            && self.close_brace.is_default()
    }
}
impl struct_definition::_T1 {
    pub fn from_parse(
        ((open_brace, struct_members), close_brace): (
            (FixedSizeTerminalWithTrivia<1usize>, Vec<StructMember>),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            struct_members,
            close_brace,
        }
    }
}
impl struct_definition::_T0 {
    pub fn from_parse(
        ((r#struct, identifier), _t1): (
            (FixedSizeTerminalWithTrivia<6usize>, identifier::WithTrivia),
            struct_definition::_T1,
        ),
    ) -> Self {
        Self {
            r#struct,
            identifier,
            _t1,
        }
    }
}
impl Default for struct_definition::_T0 {
    fn default() -> Self {
        Self {
            r#struct: Default::default(),
            identifier: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for struct_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#struct.is_default() && self.identifier.is_default() && self._t1.is_default()
    }
}

impl index_access_expression::_T3 {
    pub fn from_parse(
        (colon_char, expression): (FixedSizeTerminalWithTrivia<1>, Option<Expression>),
    ) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}
impl Default for index_access_expression::_T3 {
    fn default() -> Self {
        Self {
            colon_char: Default::default(),
            expression: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::_T3 {
    fn is_default(&self) -> bool {
        self.colon_char.is_default() && self.expression.is_default()
    }
}
impl index_access_expression::_T2 {
    pub fn from_parse(
        (expression, _t3): (Option<Expression>, Option<index_access_expression::_T3>),
    ) -> Self {
        Self { expression, _t3 }
    }
}
impl Default for index_access_expression::_T2 {
    fn default() -> Self {
        Self {
            expression: Default::default(),
            _t3: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::_T2 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self._t3.is_default()
    }
}
impl Default for index_access_expression::_T1 {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            _t2: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::_T1 {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default() && self._t2.is_default() && self.close_bracket.is_default()
    }
}
impl index_access_expression::_T1 {
    pub fn from_parse(
        ((open_bracket, _t2), close_bracket): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                index_access_expression::_T2,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket,
            _t2,
            close_bracket,
        }
    }
}

impl member_access_expression::Operator {
    pub fn from_parse(
        (period_char, _t1): (
            FixedSizeTerminalWithTrivia<1>,
            Box<member_access_expression::_T1>,
        ),
    ) -> Self {
        Self { period_char, _t1 }
    }
}

impl Default for function_call_expression::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for function_call_expression::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for function_call_expression::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            named_arguments: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for function_call_expression::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.named_arguments.is_default()
            && self.close_brace.is_default()
    }
}
impl function_call_expression::_T1 {
    pub fn from_parse(
        ((open_brace, named_arguments), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                function_call_expression::_T2,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            named_arguments,
            close_brace,
        }
    }
}
impl function_call_expression::Operator {
    pub fn from_parse(
        (_t1, argument_list): (Option<function_call_expression::_T1>, ArgumentList),
    ) -> Self {
        Self { _t1, argument_list }
    }
}
impl Default for function_call_expression::Operator {
    fn default() -> Self {
        Self {
            _t1: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for function_call_expression::Operator {
    fn is_default(&self) -> bool {
        self._t1.is_default() && self.argument_list.is_default()
    }
}

impl conditional_expression::_T1 {
    pub fn from_parse(
        (((question_char, expression_1), colon_char), expression_2): (
            (
                (FixedSizeTerminalWithTrivia<1>, Expression),
                FixedSizeTerminalWithTrivia<1>,
            ),
            Expression,
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

impl constant_definition::_T0 {
    pub fn from_parse(
        (((((type_name, constant), identifier), equal_char), expression), semicolon_char): (
            (
                (
                    (
                        (TypeName, FixedSizeTerminalWithTrivia<8usize>),
                        identifier::WithTrivia,
                    ),
                    FixedSizeTerminalWithTrivia<1>,
                ),
                Expression,
            ),
            FixedSizeTerminalWithTrivia<1>,
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

impl do_while_statement::_T1 {
    pub fn from_parse(
        ((open_paren, expression), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, Expression),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            expression,
            close_paren,
        }
    }
}
impl do_while_statement::_T0 {
    pub fn from_parse(
        ((((r#do, statement), r#while), _t1), semicolon_char): (
            (
                (
                    (FixedSizeTerminalWithTrivia<2usize>, Statement),
                    FixedSizeTerminalWithTrivia<5usize>,
                ),
                do_while_statement::_T1,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#do,
            statement,
            r#while,
            _t1,
            semicolon_char,
        }
    }
}

impl expression_statement::_T0 {
    pub fn from_parse(
        (expression, semicolon_char): (Expression, FixedSizeTerminalWithTrivia<1>),
    ) -> Self {
        Self {
            expression,
            semicolon_char,
        }
    }
}

impl if_statement::_T1 {
    pub fn from_parse(
        ((open_paren, expression), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, Expression),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            expression,
            close_paren,
        }
    }
}
impl if_statement::_T2 {
    pub fn from_parse(
        (r#else, statement): (FixedSizeTerminalWithTrivia<4usize>, Statement),
    ) -> Self {
        Self { r#else, statement }
    }
}
impl if_statement::_T0 {
    pub fn from_parse(
        (((r#if, _t1), statement), _t2): (
            (
                (FixedSizeTerminalWithTrivia<2usize>, if_statement::_T1),
                Statement,
            ),
            Option<if_statement::_T2>,
        ),
    ) -> Self {
        Self {
            r#if,
            _t1,
            statement,
            _t2,
        }
    }
}

impl return_statement::_T0 {
    pub fn from_parse(
        ((r#return, expression), semicolon_char): (
            (FixedSizeTerminalWithTrivia<6usize>, Option<Expression>),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#return,
            expression,
            semicolon_char,
        }
    }
}
impl Default for return_statement::_T0 {
    fn default() -> Self {
        Self {
            r#return: Default::default(),
            expression: Default::default(),
            semicolon_char: Default::default(),
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

impl state_variable_declaration::_T2 {
    pub fn from_parse(
        (equal_char, expression): (FixedSizeTerminalWithTrivia<1>, Expression),
    ) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl state_variable_declaration::_T0 {
    pub fn from_parse(
        ((((type_name, state_variable_attributes), identifier), _t2), semicolon_char): (
            (
                (
                    (TypeName, Vec<StateVariableAttribute>),
                    identifier::WithTrivia,
                ),
                Option<state_variable_declaration::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1>,
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

impl try_statement::_T1 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for try_statement::_T1 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for try_statement::_T1 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl try_statement::_T0 {
    pub fn from_parse(
        ((((r#try, expression), _t1), block), catch_clauses): (
            (
                (
                    (FixedSizeTerminalWithTrivia<3usize>, Expression),
                    Option<try_statement::_T1>,
                ),
                Block,
            ),
            Vec<CatchClause>,
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

impl tuple_deconstruction_statement::_T3 {
    pub fn from_parse((type_name, identifier): (Option<TypeName>, identifier::WithTrivia)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}
impl Default for tuple_deconstruction_statement::_T3 {
    fn default() -> Self {
        Self {
            type_name: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::_T3 {
    fn is_default(&self) -> bool {
        self.type_name.is_default() && self.identifier.is_default()
    }
}
impl Default for tuple_deconstruction_statement::_T2 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::_T2 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}
impl Default for tuple_deconstruction_statement::_T1 {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            _t3s: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::_T1 {
    fn is_default(&self) -> bool {
        self.open_paren.is_default() && self._t3s.is_default() && self.close_paren.is_default()
    }
}
impl tuple_deconstruction_statement::_T1 {
    pub fn from_parse(
        ((open_paren, _t3s), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<tuple_deconstruction_statement::_T2>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            _t3s,
            close_paren,
        }
    }
}
impl tuple_deconstruction_statement::_T0 {
    pub fn from_parse(
        (((_t1, equal_char), expression), semicolon_char): (
            (
                (
                    tuple_deconstruction_statement::_T1,
                    FixedSizeTerminalWithTrivia<1>,
                ),
                Expression,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            _t1,
            equal_char,
            expression,
            semicolon_char,
        }
    }
}

impl variable_declaration_statement::_T1 {
    pub fn from_parse(
        (equal_char, expression): (FixedSizeTerminalWithTrivia<1>, Expression),
    ) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl variable_declaration_statement::_T0 {
    pub fn from_parse(
        ((((type_name, _1), identifier), _t1), semicolon_char): (
            (
                (
                    (TypeName, Option<VariableSizeTerminalWithTrivia>),
                    identifier::WithTrivia,
                ),
                Option<variable_declaration_statement::_T1>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
            _t1,
            semicolon_char,
        }
    }
}

impl while_statement::_T1 {
    pub fn from_parse(
        ((open_paren, expression), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, Expression),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            expression,
            close_paren,
        }
    }
}
impl while_statement::_T0 {
    pub fn from_parse(
        ((r#while, _t1), statement): (
            (FixedSizeTerminalWithTrivia<5usize>, while_statement::_T1),
            Statement,
        ),
    ) -> Self {
        Self {
            r#while,
            _t1,
            statement,
        }
    }
}

impl for_statement::_T2 {
    pub fn from_parse(
        ((_t3, _t4), expression): (
            (Box<for_statement::_T3>, Box<for_statement::_T4>),
            Option<Expression>,
        ),
    ) -> Self {
        Self {
            _t3,
            _t4,
            expression,
        }
    }
}
impl for_statement::_T1 {
    pub fn from_parse(
        ((open_paren, _t2), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, for_statement::_T2),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            _t2,
            close_paren,
        }
    }
}
impl for_statement::_T0 {
    pub fn from_parse(
        ((r#for, _t1), statement): (
            (FixedSizeTerminalWithTrivia<3usize>, for_statement::_T1),
            Statement,
        ),
    ) -> Self {
        Self {
            r#for,
            _t1,
            statement,
        }
    }
}

impl Default for block::_T0 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            _t2s: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for block::_T0 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default() && self._t2s.is_default() && self.close_brace.is_default()
    }
}
impl block::_T0 {
    pub fn from_parse(
        ((open_brace, _t2s), close_brace): (
            (FixedSizeTerminalWithTrivia<1usize>, Vec<Box<block::_T2>>),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            _t2s,
            close_brace,
        }
    }
}

impl constructor_definition::_T0 {
    pub fn from_parse(
        (((constructor, parameter_list), constructor_attributes), block): (
            (
                (FixedSizeTerminalWithTrivia<11usize>, ParameterList),
                Vec<ConstructorAttribute>,
            ),
            Block,
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
impl Default for constructor_definition::_T0 {
    fn default() -> Self {
        Self {
            constructor: Default::default(),
            parameter_list: Default::default(),
            constructor_attributes: Default::default(),
            block: Default::default(),
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

impl fallback_function_definition::_T2 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for fallback_function_definition::_T2 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for fallback_function_definition::_T2 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl fallback_function_definition::_T0 {
    pub fn from_parse(
        ((((fallback, parameter_list), fallback_function_attributes), _t2), _t3): (
            (
                (
                    (FixedSizeTerminalWithTrivia<8usize>, ParameterList),
                    Vec<FallbackFunctionAttribute>,
                ),
                Option<fallback_function_definition::_T2>,
            ),
            Box<fallback_function_definition::_T3>,
        ),
    ) -> Self {
        Self {
            fallback,
            parameter_list,
            fallback_function_attributes,
            _t2,
            _t3,
        }
    }
}

impl function_definition::_T3 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for function_definition::_T3 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for function_definition::_T3 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl function_definition::_T0 {
    pub fn from_parse(
        (((((function, _t1), parameter_list), function_attributes), _t3), _t4): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithTrivia<8usize>,
                            Box<function_definition::_T1>,
                        ),
                        ParameterList,
                    ),
                    Vec<FunctionAttribute>,
                ),
                Option<function_definition::_T3>,
            ),
            Box<function_definition::_T4>,
        ),
    ) -> Self {
        Self {
            function,
            _t1,
            parameter_list,
            function_attributes,
            _t3,
            _t4,
        }
    }
}

impl modifier_definition::_T0 {
    pub fn from_parse(
        ((((modifier, identifier), parameter_list), modifier_attributes), _t2): (
            (
                (
                    (FixedSizeTerminalWithTrivia<8usize>, identifier::WithTrivia),
                    Option<ParameterList>,
                ),
                Vec<ModifierAttribute>,
            ),
            Box<modifier_definition::_T2>,
        ),
    ) -> Self {
        Self {
            modifier,
            identifier,
            parameter_list,
            modifier_attributes,
            _t2,
        }
    }
}

impl receive_function_definition::_T0 {
    pub fn from_parse(
        (((receive, parameter_list), receive_function_attributes), _t2): (
            (
                (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
                Vec<ReceiveFunctionAttribute>,
            ),
            Box<receive_function_definition::_T2>,
        ),
    ) -> Self {
        Self {
            receive,
            parameter_list,
            receive_function_attributes,
            _t2,
        }
    }
}

impl Default for contract_definition::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for contract_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl contract_definition::_T1 {
    pub fn from_parse(
        ((open_brace, contract_body_elements), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            contract_body_elements,
            close_brace,
        }
    }
}
impl contract_definition::_T0 {
    pub fn from_parse(
        ((((r#abstract, contract), identifier), inheritance_specifier_list), _t1): (
            (
                (
                    (
                        Option<FixedSizeTerminalWithTrivia<8usize>>,
                        FixedSizeTerminalWithTrivia<8usize>,
                    ),
                    identifier::WithTrivia,
                ),
                Option<InheritanceSpecifierList>,
            ),
            contract_definition::_T1,
        ),
    ) -> Self {
        Self {
            r#abstract,
            contract,
            identifier,
            inheritance_specifier_list,
            _t1,
        }
    }
}
impl Default for contract_definition::_T0 {
    fn default() -> Self {
        Self {
            r#abstract: Default::default(),
            contract: Default::default(),
            identifier: Default::default(),
            inheritance_specifier_list: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for contract_definition::_T0 {
    fn is_default(&self) -> bool {
        self.r#abstract.is_default()
            && self.contract.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self._t1.is_default()
    }
}

impl Default for interface_definition::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for interface_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl interface_definition::_T1 {
    pub fn from_parse(
        ((open_brace, contract_body_elements), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            contract_body_elements,
            close_brace,
        }
    }
}
impl interface_definition::_T0 {
    pub fn from_parse(
        (((interface, identifier), inheritance_specifier_list), _t1): (
            (
                (FixedSizeTerminalWithTrivia<9usize>, identifier::WithTrivia),
                Option<InheritanceSpecifierList>,
            ),
            interface_definition::_T1,
        ),
    ) -> Self {
        Self {
            interface,
            identifier,
            inheritance_specifier_list,
            _t1,
        }
    }
}
impl Default for interface_definition::_T0 {
    fn default() -> Self {
        Self {
            interface: Default::default(),
            identifier: Default::default(),
            inheritance_specifier_list: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for interface_definition::_T0 {
    fn is_default(&self) -> bool {
        self.interface.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self._t1.is_default()
    }
}

impl Default for library_definition::_T1 {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for library_definition::_T1 {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl library_definition::_T1 {
    pub fn from_parse(
        ((open_brace, contract_body_elements), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            contract_body_elements,
            close_brace,
        }
    }
}
impl library_definition::_T0 {
    pub fn from_parse(
        ((library, identifier), _t1): (
            (FixedSizeTerminalWithTrivia<7usize>, identifier::WithTrivia),
            library_definition::_T1,
        ),
    ) -> Self {
        Self {
            library,
            identifier,
            _t1,
        }
    }
}
impl Default for library_definition::_T0 {
    fn default() -> Self {
        Self {
            library: Default::default(),
            identifier: Default::default(),
            _t1: Default::default(),
        }
    }
}
impl DefaultTest for library_definition::_T0 {
    fn is_default(&self) -> bool {
        self.library.is_default() && self.identifier.is_default() && self._t1.is_default()
    }
}

impl source_unit::_T0 {
    pub fn from_parse(
        (((leading_trivia, _t2s), end_of_file_trivia), end_marker): (
            (
                (leading_trivia::WithTrivia, Vec<Box<source_unit::_T2>>),
                end_of_file_trivia::WithTrivia,
            ),
            (),
        ),
    ) -> Self {
        Self {
            leading_trivia,
            _t2s,
            end_of_file_trivia,
            end_marker,
        }
    }
}
impl Default for source_unit::_T0 {
    fn default() -> Self {
        Self {
            leading_trivia: Default::default(),
            _t2s: Default::default(),
            end_of_file_trivia: Default::default(),
            end_marker: Default::default(),
        }
    }
}
impl DefaultTest for source_unit::_T0 {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self._t2s.is_default()
            && self.end_of_file_trivia.is_default()
            && self.end_marker.is_default()
    }
}
