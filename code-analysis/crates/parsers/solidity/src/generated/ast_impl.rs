// This file is generated via `cargo build`. Please don't edit by hand.

use super::ast::*;
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
        self.terminal.is_default()
            && self.leading_trivia.is_default()
            && self.trailing_trivia.is_default()
    }
}
impl<const N: usize> DefaultTest for FixedSizeTerminal<N> {
    fn is_default(&self) -> bool {
        true
    }
}
impl<const N: usize> DefaultTest for FixedSizeTerminalWithTrivia<N> {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default() && self.trailing_trivia.is_default()
    }
}

impl decimal_integer::Sequence1 {
    pub fn from_parse(
        (underscore, filter_2): (Option<FixedSizeTerminal<1>>, FixedSizeTerminal<1>),
    ) -> Self {
        Self {
            underscore,
            filter_2,
        }
    }
}
impl Default for decimal_integer::Sequence1 {
    fn default() -> Self {
        Self {
            underscore: Default::default(),
            filter_2: Default::default(),
        }
    }
}
impl DefaultTest for decimal_integer::Sequence1 {
    fn is_default(&self) -> bool {
        self.underscore.is_default() && self.filter_2.is_default()
    }
}
impl decimal_integer::DecimalInteger {
    pub fn from_parse(
        (filter_0, sequence_1s): (FixedSizeTerminal<1>, Vec<decimal_integer::Sequence1>),
    ) -> Self {
        Self {
            filter_0,
            sequence_1s,
        }
    }
}
impl Default for decimal_integer::DecimalInteger {
    fn default() -> Self {
        Self {
            filter_0: Default::default(),
            sequence_1s: Default::default(),
        }
    }
}
impl DefaultTest for decimal_integer::DecimalInteger {
    fn is_default(&self) -> bool {
        self.filter_0.is_default() && self.sequence_1s.is_default()
    }
}
impl DefaultTest for decimal_integer::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl DefaultTest for end_of_line::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl fixed_bytes_type::FixedBytesType {
    pub fn from_parse((bytes, count): (FixedSizeTerminal<5usize>, VariableSizeTerminal)) -> Self {
        Self { bytes, count }
    }
}
impl Default for fixed_bytes_type::FixedBytesType {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            count: Default::default(),
        }
    }
}
impl DefaultTest for fixed_bytes_type::FixedBytesType {
    fn is_default(&self) -> bool {
        self.bytes.is_default() && self.count.is_default()
    }
}
impl DefaultTest for fixed_bytes_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl hex_byte_escape::HexByteEscape {
    pub fn from_parse(
        (latin_small_letter_x, filter_0s): (FixedSizeTerminal<1>, VariableSizeTerminal),
    ) -> Self {
        Self {
            latin_small_letter_x,
            filter_0s,
        }
    }
}
impl Default for hex_byte_escape::HexByteEscape {
    fn default() -> Self {
        Self {
            latin_small_letter_x: Default::default(),
            filter_0s: Default::default(),
        }
    }
}
impl DefaultTest for hex_byte_escape::HexByteEscape {
    fn is_default(&self) -> bool {
        self.latin_small_letter_x.is_default() && self.filter_0s.is_default()
    }
}
impl DefaultTest for hex_byte_escape::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl hex_number::Sequence2 {
    pub fn from_parse(
        (underscore, filter_3): (Option<FixedSizeTerminal<1>>, FixedSizeTerminal<1>),
    ) -> Self {
        Self {
            underscore,
            filter_3,
        }
    }
}
impl Default for hex_number::Sequence2 {
    fn default() -> Self {
        Self {
            underscore: Default::default(),
            filter_3: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::Sequence2 {
    fn is_default(&self) -> bool {
        self.underscore.is_default() && self.filter_3.is_default()
    }
}
impl hex_number::Sequence0 {
    pub fn from_parse(
        (filter_1, sequence_2s): (FixedSizeTerminal<1>, Vec<hex_number::Sequence2>),
    ) -> Self {
        Self {
            filter_1,
            sequence_2s,
        }
    }
}
impl Default for hex_number::Sequence0 {
    fn default() -> Self {
        Self {
            filter_1: Default::default(),
            sequence_2s: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::Sequence0 {
    fn is_default(&self) -> bool {
        self.filter_1.is_default() && self.sequence_2s.is_default()
    }
}
impl hex_number::HexNumber {
    pub fn from_parse(
        (zero_x, sequence_0): (FixedSizeTerminal<2usize>, hex_number::Sequence0),
    ) -> Self {
        Self { zero_x, sequence_0 }
    }
}
impl Default for hex_number::HexNumber {
    fn default() -> Self {
        Self {
            zero_x: Default::default(),
            sequence_0: Default::default(),
        }
    }
}
impl DefaultTest for hex_number::HexNumber {
    fn is_default(&self) -> bool {
        self.zero_x.is_default() && self.sequence_0.is_default()
    }
}
impl DefaultTest for hex_number::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl multiline_comment::Sequence1 {
    pub fn from_parse((stars, filter_2): (VariableSizeTerminal, FixedSizeTerminal<1>)) -> Self {
        Self { stars, filter_2 }
    }
}
impl Default for multiline_comment::Sequence1 {
    fn default() -> Self {
        Self {
            stars: Default::default(),
            filter_2: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::Sequence1 {
    fn is_default(&self) -> bool {
        self.stars.is_default() && self.filter_2.is_default()
    }
}
impl multiline_comment::Content {
    pub fn from_parse(
        (choices_0s, stars): (Vec<Box<multiline_comment::Choices0>>, VariableSizeTerminal),
    ) -> Self {
        Self { choices_0s, stars }
    }
}
impl Default for multiline_comment::Content {
    fn default() -> Self {
        Self {
            choices_0s: Default::default(),
            stars: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::Content {
    fn is_default(&self) -> bool {
        self.choices_0s.is_default() && self.stars.is_default()
    }
}
impl Default for multiline_comment::MultilineComment {
    fn default() -> Self {
        Self {
            slash_star: Default::default(),
            content: Default::default(),
            star_slash: Default::default(),
        }
    }
}
impl DefaultTest for multiline_comment::MultilineComment {
    fn is_default(&self) -> bool {
        self.slash_star.is_default() && self.content.is_default() && self.star_slash.is_default()
    }
}
impl multiline_comment::MultilineComment {
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
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl possibly_separated_pairs_of_hex_digits::Sequence1 {
    pub fn from_parse(
        (underscore, filter_2s): (Option<FixedSizeTerminal<1>>, VariableSizeTerminal),
    ) -> Self {
        Self {
            underscore,
            filter_2s,
        }
    }
}
impl Default for possibly_separated_pairs_of_hex_digits::Sequence1 {
    fn default() -> Self {
        Self {
            underscore: Default::default(),
            filter_2s: Default::default(),
        }
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::Sequence1 {
    fn is_default(&self) -> bool {
        self.underscore.is_default() && self.filter_2s.is_default()
    }
}
impl possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits {
    pub fn from_parse(
        (filter_0s, sequence_1s): (
            VariableSizeTerminal,
            Vec<possibly_separated_pairs_of_hex_digits::Sequence1>,
        ),
    ) -> Self {
        Self {
            filter_0s,
            sequence_1s,
        }
    }
}
impl Default for possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits {
    fn default() -> Self {
        Self {
            filter_0s: Default::default(),
            sequence_1s: Default::default(),
        }
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::PossiblySeparatedPairsOfHexDigits {
    fn is_default(&self) -> bool {
        self.filter_0s.is_default() && self.sequence_1s.is_default()
    }
}
impl DefaultTest for possibly_separated_pairs_of_hex_digits::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl raw_identifier::RawIdentifier {
    pub fn from_parse((filter_0, filter_1s): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self {
            filter_0,
            filter_1s,
        }
    }
}
impl Default for raw_identifier::RawIdentifier {
    fn default() -> Self {
        Self {
            filter_0: Default::default(),
            filter_1s: Default::default(),
        }
    }
}
impl DefaultTest for raw_identifier::RawIdentifier {
    fn is_default(&self) -> bool {
        self.filter_0.is_default() && self.filter_1s.is_default()
    }
}
impl DefaultTest for raw_identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl DefaultTest for reserved_keyword::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl signed_fixed_type::Sequence0 {
    pub fn from_parse(
        ((filter_1s, latin_small_letter_x), filter_2s): (
            (VariableSizeTerminal, FixedSizeTerminal<1>),
            VariableSizeTerminal,
        ),
    ) -> Self {
        Self {
            filter_1s,
            latin_small_letter_x,
            filter_2s,
        }
    }
}
impl Default for signed_fixed_type::Sequence0 {
    fn default() -> Self {
        Self {
            filter_1s: Default::default(),
            latin_small_letter_x: Default::default(),
            filter_2s: Default::default(),
        }
    }
}
impl DefaultTest for signed_fixed_type::Sequence0 {
    fn is_default(&self) -> bool {
        self.filter_1s.is_default()
            && self.latin_small_letter_x.is_default()
            && self.filter_2s.is_default()
    }
}
impl signed_fixed_type::SignedFixedType {
    pub fn from_parse(
        (fixed, sequence_0): (
            FixedSizeTerminal<5usize>,
            Option<signed_fixed_type::Sequence0>,
        ),
    ) -> Self {
        Self { fixed, sequence_0 }
    }
}
impl Default for signed_fixed_type::SignedFixedType {
    fn default() -> Self {
        Self {
            fixed: Default::default(),
            sequence_0: Default::default(),
        }
    }
}
impl DefaultTest for signed_fixed_type::SignedFixedType {
    fn is_default(&self) -> bool {
        self.fixed.is_default() && self.sequence_0.is_default()
    }
}
impl DefaultTest for signed_fixed_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl signed_integer_type::SignedIntegerType {
    pub fn from_parse(
        (int, byte_count): (FixedSizeTerminal<3usize>, Option<VariableSizeTerminal>),
    ) -> Self {
        Self { int, byte_count }
    }
}
impl Default for signed_integer_type::SignedIntegerType {
    fn default() -> Self {
        Self {
            int: Default::default(),
            byte_count: Default::default(),
        }
    }
}
impl DefaultTest for signed_integer_type::SignedIntegerType {
    fn is_default(&self) -> bool {
        self.int.is_default() && self.byte_count.is_default()
    }
}
impl DefaultTest for signed_integer_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl single_line_comment::SingleLineComment {
    pub fn from_parse(
        (slash_slash, filter_0s): (FixedSizeTerminal<2usize>, VariableSizeTerminal),
    ) -> Self {
        Self {
            slash_slash,
            filter_0s,
        }
    }
}
impl Default for single_line_comment::SingleLineComment {
    fn default() -> Self {
        Self {
            slash_slash: Default::default(),
            filter_0s: Default::default(),
        }
    }
}
impl DefaultTest for single_line_comment::SingleLineComment {
    fn is_default(&self) -> bool {
        self.slash_slash.is_default() && self.filter_0s.is_default()
    }
}
impl DefaultTest for single_line_comment::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl unicode_escape::UnicodeEscape {
    pub fn from_parse(
        (latin_small_letter_u, filter_0s): (FixedSizeTerminal<1>, VariableSizeTerminal),
    ) -> Self {
        Self {
            latin_small_letter_u,
            filter_0s,
        }
    }
}
impl Default for unicode_escape::UnicodeEscape {
    fn default() -> Self {
        Self {
            latin_small_letter_u: Default::default(),
            filter_0s: Default::default(),
        }
    }
}
impl DefaultTest for unicode_escape::UnicodeEscape {
    fn is_default(&self) -> bool {
        self.latin_small_letter_u.is_default() && self.filter_0s.is_default()
    }
}
impl DefaultTest for unicode_escape::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl Default for version_pragma_value::VersionPragmaValue {
    fn default() -> Self {
        Self {
            filter_0s: Default::default(),
            periods: Default::default(),
        }
    }
}
impl DefaultTest for version_pragma_value::VersionPragmaValue {
    fn is_default(&self) -> bool {
        self.filter_0s.is_default() && self.periods.is_default()
    }
}
impl DefaultTest for version_pragma_value::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl DefaultTest for whitespace::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl yul_decimal_number_literal::Sequence0 {
    pub fn from_parse((filter_1, filter_2s): (FixedSizeTerminal<1>, VariableSizeTerminal)) -> Self {
        Self {
            filter_1,
            filter_2s,
        }
    }
}
impl Default for yul_decimal_number_literal::Sequence0 {
    fn default() -> Self {
        Self {
            filter_1: Default::default(),
            filter_2s: Default::default(),
        }
    }
}
impl DefaultTest for yul_decimal_number_literal::Sequence0 {
    fn is_default(&self) -> bool {
        self.filter_1.is_default() && self.filter_2s.is_default()
    }
}

impl yul_hex_literal::YulHexLiteral {
    pub fn from_parse(
        (zero_x, filter_0s): (FixedSizeTerminal<2usize>, VariableSizeTerminal),
    ) -> Self {
        Self { zero_x, filter_0s }
    }
}
impl Default for yul_hex_literal::YulHexLiteral {
    fn default() -> Self {
        Self {
            zero_x: Default::default(),
            filter_0s: Default::default(),
        }
    }
}
impl DefaultTest for yul_hex_literal::YulHexLiteral {
    fn is_default(&self) -> bool {
        self.zero_x.is_default() && self.filter_0s.is_default()
    }
}
impl DefaultTest for yul_hex_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl decimal_exponent::DecimalExponent {
    pub fn from_parse(
        ((filter_0, minus), decimal_integer): (
            (FixedSizeTerminal<1>, Option<FixedSizeTerminal<1>>),
            DecimalInteger,
        ),
    ) -> Self {
        Self {
            filter_0,
            minus,
            decimal_integer,
        }
    }
}
impl Default for decimal_exponent::DecimalExponent {
    fn default() -> Self {
        Self {
            filter_0: Default::default(),
            minus: Default::default(),
            decimal_integer: Default::default(),
        }
    }
}
impl DefaultTest for decimal_exponent::DecimalExponent {
    fn is_default(&self) -> bool {
        self.filter_0.is_default() && self.minus.is_default() && self.decimal_integer.is_default()
    }
}
impl DefaultTest for decimal_exponent::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl decimal_float::DecimalFloat {
    pub fn from_parse(
        ((decimal_integer_0_, period), decimal_integer_1_): (
            (Option<DecimalInteger>, FixedSizeTerminal<1>),
            DecimalInteger,
        ),
    ) -> Self {
        Self {
            decimal_integer_0_,
            period,
            decimal_integer_1_,
        }
    }
}
impl Default for decimal_float::DecimalFloat {
    fn default() -> Self {
        Self {
            decimal_integer_0_: Default::default(),
            period: Default::default(),
            decimal_integer_1_: Default::default(),
        }
    }
}
impl DefaultTest for decimal_float::DecimalFloat {
    fn is_default(&self) -> bool {
        self.decimal_integer_0_.is_default()
            && self.period.is_default()
            && self.decimal_integer_1_.is_default()
    }
}
impl DefaultTest for decimal_float::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl DefaultTest for end_of_file_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl escape_sequence::EscapeSequence {
    pub fn from_parse(
        (backslash, choices_0): (FixedSizeTerminal<1>, Box<escape_sequence::Choices0>),
    ) -> Self {
        Self {
            backslash,
            choices_0,
        }
    }
}

impl Default for hex_string_literal::DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote {
    fn default() -> Self {
        Self {
            opening_double_quote: Default::default(),
            possibly_separated_pairs_of_hex_digits: Default::default(),
            closing_double_quote: Default::default(),
        }
    }
}
impl DefaultTest
    for hex_string_literal::DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote
{
    fn is_default(&self) -> bool {
        self.opening_double_quote.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.closing_double_quote.is_default()
    }
}
impl hex_string_literal::DoubleQuoteAndPossiblySeparatedPairsOfHexDigitsAndDoubleQuote {
    pub fn from_parse(
        ((opening_double_quote, possibly_separated_pairs_of_hex_digits), closing_double_quote): (
            (
                FixedSizeTerminal<1usize>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            opening_double_quote,
            possibly_separated_pairs_of_hex_digits,
            closing_double_quote,
        }
    }
}
impl Default for hex_string_literal::QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote {
    fn default() -> Self {
        Self {
            opening_quote: Default::default(),
            possibly_separated_pairs_of_hex_digits: Default::default(),
            closing_quote: Default::default(),
        }
    }
}
impl DefaultTest for hex_string_literal::QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote {
    fn is_default(&self) -> bool {
        self.opening_quote.is_default()
            && self.possibly_separated_pairs_of_hex_digits.is_default()
            && self.closing_quote.is_default()
    }
}
impl hex_string_literal::QuoteAndPossiblySeparatedPairsOfHexDigitsAndQuote {
    pub fn from_parse(
        ((opening_quote, possibly_separated_pairs_of_hex_digits), closing_quote): (
            (
                FixedSizeTerminal<1usize>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            opening_quote,
            possibly_separated_pairs_of_hex_digits,
            closing_quote,
        }
    }
}
impl hex_string_literal::HexStringLiteral {
    pub fn from_parse(
        (hex, choices_0): (FixedSizeTerminal<3usize>, Box<hex_string_literal::Choices0>),
    ) -> Self {
        Self { hex, choices_0 }
    }
}

impl trailing_trivia::TrailingTrivia {
    pub fn from_parse(
        (choices_0s, choices_1): (
            Vec<Box<trailing_trivia::Choices0>>,
            Box<trailing_trivia::Choices1>,
        ),
    ) -> Self {
        Self {
            choices_0s,
            choices_1,
        }
    }
}

impl unsigned_fixed_type::UnsignedFixedType {
    pub fn from_parse(
        (latin_small_letter_u, signed_fixed_type): (FixedSizeTerminal<1>, SignedFixedType),
    ) -> Self {
        Self {
            latin_small_letter_u,
            signed_fixed_type,
        }
    }
}
impl Default for unsigned_fixed_type::UnsignedFixedType {
    fn default() -> Self {
        Self {
            latin_small_letter_u: Default::default(),
            signed_fixed_type: Default::default(),
        }
    }
}
impl DefaultTest for unsigned_fixed_type::UnsignedFixedType {
    fn is_default(&self) -> bool {
        self.latin_small_letter_u.is_default() && self.signed_fixed_type.is_default()
    }
}
impl DefaultTest for unsigned_fixed_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl unsigned_integer_type::UnsignedIntegerType {
    pub fn from_parse(
        (latin_small_letter_u, signed_integer_type): (FixedSizeTerminal<1>, SignedIntegerType),
    ) -> Self {
        Self {
            latin_small_letter_u,
            signed_integer_type,
        }
    }
}
impl Default for unsigned_integer_type::UnsignedIntegerType {
    fn default() -> Self {
        Self {
            latin_small_letter_u: Default::default(),
            signed_integer_type: Default::default(),
        }
    }
}
impl DefaultTest for unsigned_integer_type::UnsignedIntegerType {
    fn is_default(&self) -> bool {
        self.latin_small_letter_u.is_default() && self.signed_integer_type.is_default()
    }
}
impl DefaultTest for unsigned_integer_type::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl address_type::AddressType {
    pub fn from_parse(
        (address, payable): (
            FixedSizeTerminalWithTrivia<7usize>,
            Option<FixedSizeTerminalWithTrivia<7usize>>,
        ),
    ) -> Self {
        Self { address, payable }
    }
}
impl Default for address_type::AddressType {
    fn default() -> Self {
        Self {
            address: Default::default(),
            payable: Default::default(),
        }
    }
}
impl DefaultTest for address_type::AddressType {
    fn is_default(&self) -> bool {
        self.address.is_default() && self.payable.is_default()
    }
}

impl Default for array_literal::ExpressionsAndCommas {
    fn default() -> Self {
        Self {
            expressions: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for array_literal::ExpressionsAndCommas {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.commas.is_default()
    }
}
impl Default for array_literal::ArrayLiteral {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            expressions_and_commas: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for array_literal::ArrayLiteral {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default()
            && self.expressions_and_commas.is_default()
            && self.close_bracket.is_default()
    }
}
impl array_literal::ArrayLiteral {
    pub fn from_parse(
        ((open_bracket, expressions_and_commas), close_bracket): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                array_literal::ExpressionsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket,
            expressions_and_commas,
            close_bracket,
        }
    }
}

impl break_statement::BreakStatement {
    pub fn from_parse(
        (r#break, semicolon): (
            FixedSizeTerminalWithTrivia<5usize>,
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self { r#break, semicolon }
    }
}
impl Default for break_statement::BreakStatement {
    fn default() -> Self {
        Self {
            r#break: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for break_statement::BreakStatement {
    fn is_default(&self) -> bool {
        self.r#break.is_default() && self.semicolon.is_default()
    }
}

impl continue_statement::ContinueStatement {
    pub fn from_parse(
        (r#continue, semicolon): (
            FixedSizeTerminalWithTrivia<8usize>,
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#continue,
            semicolon,
        }
    }
}
impl Default for continue_statement::ContinueStatement {
    fn default() -> Self {
        Self {
            r#continue: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for continue_statement::ContinueStatement {
    fn is_default(&self) -> bool {
        self.r#continue.is_default() && self.semicolon.is_default()
    }
}

impl decimal_number::DecimalNumber {
    pub fn from_parse(
        (choices_0, decimal_exponent): (Box<decimal_number::Choices0>, Option<DecimalExponent>),
    ) -> Self {
        Self {
            choices_0,
            decimal_exponent,
        }
    }
}

impl Default for double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral {
    fn default() -> Self {
        Self {
            opening_double_quote: Default::default(),
            runs: Default::default(),
            closing_double_quote: Default::default(),
        }
    }
}
impl DefaultTest for double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral {
    fn is_default(&self) -> bool {
        self.opening_double_quote.is_default()
            && self.runs.is_default()
            && self.closing_double_quote.is_default()
    }
}
impl double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral {
    pub fn from_parse(
        ((opening_double_quote, runs), closing_double_quote): (
            (
                FixedSizeTerminal<1usize>,
                Vec<Box<double_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            opening_double_quote,
            runs,
            closing_double_quote,
        }
    }
}
impl DefaultTest for double_quoted_ascii_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl Default for double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral {
    fn default() -> Self {
        Self {
            unicode_double_quote: Default::default(),
            runs: Default::default(),
            double_quote: Default::default(),
        }
    }
}
impl DefaultTest for double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral {
    fn is_default(&self) -> bool {
        self.unicode_double_quote.is_default()
            && self.runs.is_default()
            && self.double_quote.is_default()
    }
}
impl double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral {
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
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl Default for parenthesis_expression::ExpressionsAndCommas {
    fn default() -> Self {
        Self {
            expressions: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for parenthesis_expression::ExpressionsAndCommas {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.commas.is_default()
    }
}
impl Default for parenthesis_expression::ParenthesisExpression {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            expressions_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for parenthesis_expression::ParenthesisExpression {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.expressions_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl parenthesis_expression::ParenthesisExpression {
    pub fn from_parse(
        ((open_paren, expressions_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                parenthesis_expression::ExpressionsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            expressions_and_commas,
            close_paren,
        }
    }
}

impl Default for positional_argument_list::PositionalArgumentList {
    fn default() -> Self {
        Self {
            expressions: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for positional_argument_list::PositionalArgumentList {
    fn is_default(&self) -> bool {
        self.expressions.is_default() && self.commas.is_default()
    }
}

impl Default for single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral {
    fn default() -> Self {
        Self {
            opening_quote: Default::default(),
            runs: Default::default(),
            closing_quote: Default::default(),
        }
    }
}
impl DefaultTest for single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral {
    fn is_default(&self) -> bool {
        self.opening_quote.is_default() && self.runs.is_default() && self.closing_quote.is_default()
    }
}
impl single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral {
    pub fn from_parse(
        ((opening_quote, runs), closing_quote): (
            (
                FixedSizeTerminal<1usize>,
                Vec<Box<single_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1usize>,
        ),
    ) -> Self {
        Self {
            opening_quote,
            runs,
            closing_quote,
        }
    }
}
impl DefaultTest for single_quoted_ascii_string_literal::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl Default for single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral {
    fn default() -> Self {
        Self {
            unicode_quote: Default::default(),
            runs: Default::default(),
            quote: Default::default(),
        }
    }
}
impl DefaultTest for single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral {
    fn is_default(&self) -> bool {
        self.unicode_quote.is_default() && self.runs.is_default() && self.quote.is_default()
    }
}
impl single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral {
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
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl unchecked_block::UncheckedBlock {
    pub fn from_parse((unchecked, block): (FixedSizeTerminalWithTrivia<9usize>, Block)) -> Self {
        Self { unchecked, block }
    }
}
impl Default for unchecked_block::UncheckedBlock {
    fn default() -> Self {
        Self {
            unchecked: Default::default(),
            block: Default::default(),
        }
    }
}
impl DefaultTest for unchecked_block::UncheckedBlock {
    fn is_default(&self) -> bool {
        self.unchecked.is_default() && self.block.is_default()
    }
}

impl version_pragma_specifier::Sequence0 {
    pub fn from_parse(
        (version_pragma_operator, version_pragma_value): (
            version_pragma_operator::WithTrivia,
            version_pragma_value::WithTrivia,
        ),
    ) -> Self {
        Self {
            version_pragma_operator,
            version_pragma_value,
        }
    }
}
impl version_pragma_specifier::VersionPragmaSpecifier {
    pub fn from_parse(
        (solidity, sequence_0s): (
            FixedSizeTerminalWithTrivia<8usize>,
            Vec<version_pragma_specifier::Sequence0>,
        ),
    ) -> Self {
        Self {
            solidity,
            sequence_0s,
        }
    }
}
impl Default for version_pragma_specifier::VersionPragmaSpecifier {
    fn default() -> Self {
        Self {
            solidity: Default::default(),
            sequence_0s: Default::default(),
        }
    }
}
impl DefaultTest for version_pragma_specifier::VersionPragmaSpecifier {
    fn is_default(&self) -> bool {
        self.solidity.is_default() && self.sequence_0s.is_default()
    }
}

impl DefaultTest for yul_identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl Default for assembly_flags::DoubleQuotedAsciiStringLiteralsAndCommas {
    fn default() -> Self {
        Self {
            double_quoted_ascii_string_literals: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for assembly_flags::DoubleQuotedAsciiStringLiteralsAndCommas {
    fn is_default(&self) -> bool {
        self.double_quoted_ascii_string_literals.is_default() && self.commas.is_default()
    }
}
impl Default for assembly_flags::AssemblyFlags {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            double_quoted_ascii_string_literals_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for assembly_flags::AssemblyFlags {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self
                .double_quoted_ascii_string_literals_and_commas
                .is_default()
            && self.close_paren.is_default()
    }
}
impl assembly_flags::AssemblyFlags {
    pub fn from_parse(
        ((open_paren, double_quoted_ascii_string_literals_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                assembly_flags::DoubleQuotedAsciiStringLiteralsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            double_quoted_ascii_string_literals_and_commas,
            close_paren,
        }
    }
}

impl DefaultTest for identifier::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.terminal.is_default()
            && self.trailing_trivia.is_default()
    }
}

impl numeric_literal::NumericLiteral {
    pub fn from_parse(
        (choices_0, number_unit): (Box<numeric_literal::Choices0>, Option<NumberUnit>),
    ) -> Self {
        Self {
            choices_0,
            number_unit,
        }
    }
}

impl Default for yul_function_call::YulExpressionsAndCommas {
    fn default() -> Self {
        Self {
            yul_expressions: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::YulExpressionsAndCommas {
    fn is_default(&self) -> bool {
        self.yul_expressions.is_default() && self.commas.is_default()
    }
}
impl Default for yul_function_call::OpenParenAndYulExpressionsAndCommasAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            yul_expressions_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::OpenParenAndYulExpressionsAndCommasAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.yul_expressions_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl yul_function_call::OpenParenAndYulExpressionsAndCommasAndCloseParen {
    pub fn from_parse(
        ((open_paren, yul_expressions_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<yul_function_call::YulExpressionsAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            yul_expressions_and_commas,
            close_paren,
        }
    }
}
impl yul_function_call::YulFunctionCall {
    pub fn from_parse(
        (yul_identifier, open_paren_and_yul_expressions_and_commas_and_close_paren): (
            yul_identifier::WithTrivia,
            yul_function_call::OpenParenAndYulExpressionsAndCommasAndCloseParen,
        ),
    ) -> Self {
        Self {
            yul_identifier,
            open_paren_and_yul_expressions_and_commas_and_close_paren,
        }
    }
}
impl Default for yul_function_call::YulFunctionCall {
    fn default() -> Self {
        Self {
            yul_identifier: Default::default(),
            open_paren_and_yul_expressions_and_commas_and_close_paren: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_call::YulFunctionCall {
    fn is_default(&self) -> bool {
        self.yul_identifier.is_default()
            && self
                .open_paren_and_yul_expressions_and_commas_and_close_paren
                .is_default()
    }
}

impl Default for yul_function_definition::Arguments {
    fn default() -> Self {
        Self {
            yul_identifiers: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::Arguments {
    fn is_default(&self) -> bool {
        self.yul_identifiers.is_default() && self.commas.is_default()
    }
}
impl Default for yul_function_definition::OpenParenAndArgumentsAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            arguments: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::OpenParenAndArgumentsAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default() && self.arguments.is_default() && self.close_paren.is_default()
    }
}
impl yul_function_definition::OpenParenAndArgumentsAndCloseParen {
    pub fn from_parse(
        ((open_paren, arguments), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<yul_function_definition::Arguments>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            arguments,
            close_paren,
        }
    }
}
impl Default for yul_function_definition::Results {
    fn default() -> Self {
        Self {
            yul_identifiers: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::Results {
    fn is_default(&self) -> bool {
        self.yul_identifiers.is_default() && self.commas.is_default()
    }
}
impl yul_function_definition::Sequence0 {
    pub fn from_parse(
        (minus_greater, results): (
            FixedSizeTerminalWithTrivia<2usize>,
            yul_function_definition::Results,
        ),
    ) -> Self {
        Self {
            minus_greater,
            results,
        }
    }
}
impl Default for yul_function_definition::Sequence0 {
    fn default() -> Self {
        Self {
            minus_greater: Default::default(),
            results: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::Sequence0 {
    fn is_default(&self) -> bool {
        self.minus_greater.is_default() && self.results.is_default()
    }
}
impl yul_function_definition::YulFunctionDefinition {
    pub fn from_parse(
        (
            (((function, yul_identifier), open_paren_and_arguments_and_close_paren), sequence_0),
            yul_block,
        ): (
            (
                (
                    (
                        FixedSizeTerminalWithTrivia<8usize>,
                        yul_identifier::WithTrivia,
                    ),
                    yul_function_definition::OpenParenAndArgumentsAndCloseParen,
                ),
                Option<yul_function_definition::Sequence0>,
            ),
            YulBlock,
        ),
    ) -> Self {
        Self {
            function,
            yul_identifier,
            open_paren_and_arguments_and_close_paren,
            sequence_0,
            yul_block,
        }
    }
}
impl Default for yul_function_definition::YulFunctionDefinition {
    fn default() -> Self {
        Self {
            function: Default::default(),
            yul_identifier: Default::default(),
            open_paren_and_arguments_and_close_paren: Default::default(),
            sequence_0: Default::default(),
            yul_block: Default::default(),
        }
    }
}
impl DefaultTest for yul_function_definition::YulFunctionDefinition {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.yul_identifier.is_default()
            && self.open_paren_and_arguments_and_close_paren.is_default()
            && self.sequence_0.is_default()
            && self.yul_block.is_default()
    }
}

impl Default for yul_identifier_path::YulIdentifierPath {
    fn default() -> Self {
        Self {
            yul_identifiers: Default::default(),
            periods: Default::default(),
        }
    }
}
impl DefaultTest for yul_identifier_path::YulIdentifierPath {
    fn is_default(&self) -> bool {
        self.yul_identifiers.is_default() && self.periods.is_default()
    }
}

impl abi_coder_pragma_specifier::AbiCoderPragmaSpecifier {
    pub fn from_parse(
        (abicoder, identifier): (FixedSizeTerminalWithTrivia<8usize>, identifier::WithTrivia),
    ) -> Self {
        Self {
            abicoder,
            identifier,
        }
    }
}
impl Default for abi_coder_pragma_specifier::AbiCoderPragmaSpecifier {
    fn default() -> Self {
        Self {
            abicoder: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for abi_coder_pragma_specifier::AbiCoderPragmaSpecifier {
    fn is_default(&self) -> bool {
        self.abicoder.is_default() && self.identifier.is_default()
    }
}

impl delete_statement::DeleteStatement {
    pub fn from_parse(
        ((delete, identifier), semicolon): (
            (FixedSizeTerminalWithTrivia<6usize>, identifier::WithTrivia),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            delete,
            identifier,
            semicolon,
        }
    }
}
impl Default for delete_statement::DeleteStatement {
    fn default() -> Self {
        Self {
            delete: Default::default(),
            identifier: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for delete_statement::DeleteStatement {
    fn is_default(&self) -> bool {
        self.delete.is_default() && self.identifier.is_default() && self.semicolon.is_default()
    }
}

impl Default for enum_definition::IdentifiersAndCommas {
    fn default() -> Self {
        Self {
            identifiers: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::IdentifiersAndCommas {
    fn is_default(&self) -> bool {
        self.identifiers.is_default() && self.commas.is_default()
    }
}
impl Default for enum_definition::OpenBraceAndIdentifiersAndCommasAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            identifiers_and_commas: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::OpenBraceAndIdentifiersAndCommasAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.identifiers_and_commas.is_default()
            && self.close_brace.is_default()
    }
}
impl enum_definition::OpenBraceAndIdentifiersAndCommasAndCloseBrace {
    pub fn from_parse(
        ((open_brace, identifiers_and_commas), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                enum_definition::IdentifiersAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            identifiers_and_commas,
            close_brace,
        }
    }
}
impl enum_definition::EnumDefinition {
    pub fn from_parse(
        ((r#enum, identifier), open_brace_and_identifiers_and_commas_and_close_brace): (
            (FixedSizeTerminalWithTrivia<4usize>, identifier::WithTrivia),
            enum_definition::OpenBraceAndIdentifiersAndCommasAndCloseBrace,
        ),
    ) -> Self {
        Self {
            r#enum,
            identifier,
            open_brace_and_identifiers_and_commas_and_close_brace,
        }
    }
}
impl Default for enum_definition::EnumDefinition {
    fn default() -> Self {
        Self {
            r#enum: Default::default(),
            identifier: Default::default(),
            open_brace_and_identifiers_and_commas_and_close_brace: Default::default(),
        }
    }
}
impl DefaultTest for enum_definition::EnumDefinition {
    fn is_default(&self) -> bool {
        self.r#enum.is_default()
            && self.identifier.is_default()
            && self
                .open_brace_and_identifiers_and_commas_and_close_brace
                .is_default()
    }
}

impl experimental_pragma_specifier::ExperimentalPragmaSpecifier {
    pub fn from_parse(
        (experimental, identifier): (FixedSizeTerminalWithTrivia<12usize>, identifier::WithTrivia),
    ) -> Self {
        Self {
            experimental,
            identifier,
        }
    }
}
impl Default for experimental_pragma_specifier::ExperimentalPragmaSpecifier {
    fn default() -> Self {
        Self {
            experimental: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for experimental_pragma_specifier::ExperimentalPragmaSpecifier {
    fn is_default(&self) -> bool {
        self.experimental.is_default() && self.identifier.is_default()
    }
}

impl Default for identifier_path::IdentifierPath {
    fn default() -> Self {
        Self {
            identifiers: Default::default(),
            periods: Default::default(),
        }
    }
}
impl DefaultTest for identifier_path::IdentifierPath {
    fn is_default(&self) -> bool {
        self.identifiers.is_default() && self.periods.is_default()
    }
}

impl named_argument::NamedArgument {
    pub fn from_parse(
        ((identifier, colon), expression): (
            (identifier::WithTrivia, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            identifier,
            colon,
            expression,
        }
    }
}

impl parameter_declaration::ParameterDeclaration {
    pub fn from_parse(
        ((type_name, data_location), identifier): (
            (TypeName, Option<DataLocation>),
            Option<identifier::WithTrivia>,
        ),
    ) -> Self {
        Self {
            type_name,
            data_location,
            identifier,
        }
    }
}

impl selected_import::Sequence0 {
    pub fn from_parse(
        (r#as, identifier): (FixedSizeTerminalWithTrivia<2usize>, identifier::WithTrivia),
    ) -> Self {
        Self { r#as, identifier }
    }
}
impl Default for selected_import::Sequence0 {
    fn default() -> Self {
        Self {
            r#as: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for selected_import::Sequence0 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}
impl selected_import::SelectedImport {
    pub fn from_parse(
        (identifier, sequence_0): (identifier::WithTrivia, Option<selected_import::Sequence0>),
    ) -> Self {
        Self {
            identifier,
            sequence_0,
        }
    }
}
impl Default for selected_import::SelectedImport {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            sequence_0: Default::default(),
        }
    }
}
impl DefaultTest for selected_import::SelectedImport {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.sequence_0.is_default()
    }
}

impl user_defined_value_type_definition::UserDefinedValueTypeDefinition {
    pub fn from_parse(
        ((((r#type, identifier), is), elementary_type), semicolon): (
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
            semicolon,
        }
    }
}

impl mapping_type::Sequence0 {
    pub fn from_parse(
        ((choices_1, equal_greater), type_name): (
            (
                Box<mapping_type::Choices1>,
                FixedSizeTerminalWithTrivia<2usize>,
            ),
            TypeName,
        ),
    ) -> Self {
        Self {
            choices_1,
            equal_greater,
            type_name,
        }
    }
}
impl mapping_type::OpenParenAndSequence0AndCloseParen {
    pub fn from_parse(
        ((open_paren, sequence_0), close_paren): (
            (FixedSizeTerminalWithTrivia<1usize>, mapping_type::Sequence0),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            sequence_0,
            close_paren,
        }
    }
}
impl mapping_type::MappingType {
    pub fn from_parse(
        (mapping, open_paren_and_sequence_0_and_close_paren): (
            FixedSizeTerminalWithTrivia<7usize>,
            mapping_type::OpenParenAndSequence0AndCloseParen,
        ),
    ) -> Self {
        Self {
            mapping,
            open_paren_and_sequence_0_and_close_paren,
        }
    }
}

impl Default for named_argument_list::NamedArgumentsAndCommas {
    fn default() -> Self {
        Self {
            named_arguments: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for named_argument_list::NamedArgumentsAndCommas {
    fn is_default(&self) -> bool {
        self.named_arguments.is_default() && self.commas.is_default()
    }
}
impl Default for named_argument_list::NamedArgumentList {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            named_arguments_and_commas: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for named_argument_list::NamedArgumentList {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.named_arguments_and_commas.is_default()
            && self.close_brace.is_default()
    }
}
impl named_argument_list::NamedArgumentList {
    pub fn from_parse(
        ((open_brace, named_arguments_and_commas), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<named_argument_list::NamedArgumentsAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            named_arguments_and_commas,
            close_brace,
        }
    }
}

impl Default for override_specifier::IdentifierPathsAndCommas {
    fn default() -> Self {
        Self {
            identifier_paths: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::IdentifierPathsAndCommas {
    fn is_default(&self) -> bool {
        self.identifier_paths.is_default() && self.commas.is_default()
    }
}
impl Default for override_specifier::OpenParenAndIdentifierPathsAndCommasAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            identifier_paths_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::OpenParenAndIdentifierPathsAndCommasAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.identifier_paths_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl override_specifier::OpenParenAndIdentifierPathsAndCommasAndCloseParen {
    pub fn from_parse(
        ((open_paren, identifier_paths_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                override_specifier::IdentifierPathsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            identifier_paths_and_commas,
            close_paren,
        }
    }
}
impl override_specifier::OverrideSpecifier {
    pub fn from_parse(
        (r#override, open_paren_and_identifier_paths_and_commas_and_close_paren): (
            FixedSizeTerminalWithTrivia<8usize>,
            Option<override_specifier::OpenParenAndIdentifierPathsAndCommasAndCloseParen>,
        ),
    ) -> Self {
        Self {
            r#override,
            open_paren_and_identifier_paths_and_commas_and_close_paren,
        }
    }
}
impl Default for override_specifier::OverrideSpecifier {
    fn default() -> Self {
        Self {
            r#override: Default::default(),
            open_paren_and_identifier_paths_and_commas_and_close_paren: Default::default(),
        }
    }
}
impl DefaultTest for override_specifier::OverrideSpecifier {
    fn is_default(&self) -> bool {
        self.r#override.is_default()
            && self
                .open_paren_and_identifier_paths_and_commas_and_close_paren
                .is_default()
    }
}

impl Default for parameter_list::ParameterDeclarationsAndCommas {
    fn default() -> Self {
        Self {
            parameter_declarations: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for parameter_list::ParameterDeclarationsAndCommas {
    fn is_default(&self) -> bool {
        self.parameter_declarations.is_default() && self.commas.is_default()
    }
}
impl Default for parameter_list::ParameterList {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            parameter_declarations_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for parameter_list::ParameterList {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.parameter_declarations_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl parameter_list::ParameterList {
    pub fn from_parse(
        ((open_paren, parameter_declarations_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<parameter_list::ParameterDeclarationsAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            parameter_declarations_and_commas,
            close_paren,
        }
    }
}

impl pragma_directive::PragmaDirective {
    pub fn from_parse(
        ((pragma, choices_0), semicolon): (
            (
                FixedSizeTerminalWithTrivia<6usize>,
                Box<pragma_directive::Choices0>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            pragma,
            choices_0,
            semicolon,
        }
    }
}

impl Default for selecting_import_directive::SelectedImportsAndCommas {
    fn default() -> Self {
        Self {
            selected_imports: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for selecting_import_directive::SelectedImportsAndCommas {
    fn is_default(&self) -> bool {
        self.selected_imports.is_default() && self.commas.is_default()
    }
}
impl Default for selecting_import_directive::OpenBraceAndSelectedImportsAndCommasAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            selected_imports_and_commas: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for selecting_import_directive::OpenBraceAndSelectedImportsAndCommasAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.selected_imports_and_commas.is_default()
            && self.close_brace.is_default()
    }
}
impl selecting_import_directive::OpenBraceAndSelectedImportsAndCommasAndCloseBrace {
    pub fn from_parse(
        ((open_brace, selected_imports_and_commas), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                selecting_import_directive::SelectedImportsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            selected_imports_and_commas,
            close_brace,
        }
    }
}
impl selecting_import_directive::SelectingImportDirective {
    pub fn from_parse(
        ((open_brace_and_selected_imports_and_commas_and_close_brace, from), import_path): (
            (
                selecting_import_directive::OpenBraceAndSelectedImportsAndCommasAndCloseBrace,
                FixedSizeTerminalWithTrivia<4usize>,
            ),
            ImportPath,
        ),
    ) -> Self {
        Self {
            open_brace_and_selected_imports_and_commas_and_close_brace,
            from,
            import_path,
        }
    }
}

impl simple_import_directive::Sequence0 {
    pub fn from_parse(
        (r#as, identifier): (FixedSizeTerminalWithTrivia<2usize>, identifier::WithTrivia),
    ) -> Self {
        Self { r#as, identifier }
    }
}
impl Default for simple_import_directive::Sequence0 {
    fn default() -> Self {
        Self {
            r#as: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for simple_import_directive::Sequence0 {
    fn is_default(&self) -> bool {
        self.r#as.is_default() && self.identifier.is_default()
    }
}
impl simple_import_directive::SimpleImportDirective {
    pub fn from_parse(
        (import_path, sequence_0s): (ImportPath, Vec<simple_import_directive::Sequence0>),
    ) -> Self {
        Self {
            import_path,
            sequence_0s,
        }
    }
}

impl star_import_directive::StarImportDirective {
    pub fn from_parse(
        ((((star, r#as), identifier), from), import_path): (
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
            star,
            r#as,
            identifier,
            from,
            import_path,
        }
    }
}

impl Default for argument_list::ArgumentList {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            choices_0: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for argument_list::ArgumentList {
    fn is_default(&self) -> bool {
        self.open_paren.is_default() && self.choices_0.is_default() && self.close_paren.is_default()
    }
}
impl argument_list::ArgumentList {
    pub fn from_parse(
        ((open_paren, choices_0), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<Box<argument_list::Choices0>>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            choices_0,
            close_paren,
        }
    }
}

impl catch_clause::Sequence0 {
    pub fn from_parse(
        (identifier, parameter_list): (Option<identifier::WithTrivia>, ParameterList),
    ) -> Self {
        Self {
            identifier,
            parameter_list,
        }
    }
}
impl Default for catch_clause::Sequence0 {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for catch_clause::Sequence0 {
    fn is_default(&self) -> bool {
        self.identifier.is_default() && self.parameter_list.is_default()
    }
}
impl catch_clause::CatchClause {
    pub fn from_parse(
        ((catch, sequence_0), block): (
            (
                FixedSizeTerminalWithTrivia<5usize>,
                Option<catch_clause::Sequence0>,
            ),
            Block,
        ),
    ) -> Self {
        Self {
            catch,
            sequence_0,
            block,
        }
    }
}
impl Default for catch_clause::CatchClause {
    fn default() -> Self {
        Self {
            catch: Default::default(),
            sequence_0: Default::default(),
            block: Default::default(),
        }
    }
}
impl DefaultTest for catch_clause::CatchClause {
    fn is_default(&self) -> bool {
        self.catch.is_default() && self.sequence_0.is_default() && self.block.is_default()
    }
}

impl function_type::Sequence1 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for function_type::Sequence1 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for function_type::Sequence1 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl function_type::FunctionType {
    pub fn from_parse(
        (((function, parameter_list), choices_0s), sequence_1): (
            (
                (FixedSizeTerminalWithTrivia<8usize>, ParameterList),
                Vec<Box<function_type::Choices0>>,
            ),
            Option<function_type::Sequence1>,
        ),
    ) -> Self {
        Self {
            function,
            parameter_list,
            choices_0s,
            sequence_1,
        }
    }
}
impl Default for function_type::FunctionType {
    fn default() -> Self {
        Self {
            function: Default::default(),
            parameter_list: Default::default(),
            choices_0s: Default::default(),
            sequence_1: Default::default(),
        }
    }
}
impl DefaultTest for function_type::FunctionType {
    fn is_default(&self) -> bool {
        self.function.is_default()
            && self.parameter_list.is_default()
            && self.choices_0s.is_default()
            && self.sequence_1.is_default()
    }
}

impl import_directive::ImportDirective {
    pub fn from_parse(
        ((import, choices_0), semicolon): (
            (
                FixedSizeTerminalWithTrivia<6usize>,
                Box<import_directive::Choices0>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            import,
            choices_0,
            semicolon,
        }
    }
}

impl Default for yul_assignment_statement::YulIdentifierPathsAndCommas {
    fn default() -> Self {
        Self {
            yul_identifier_paths: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for yul_assignment_statement::YulIdentifierPathsAndCommas {
    fn is_default(&self) -> bool {
        self.yul_identifier_paths.is_default() && self.commas.is_default()
    }
}
impl yul_assignment_statement::YulAssignmentStatement {
    pub fn from_parse(
        ((yul_identifier_paths_and_commas, colon_equal), yul_expression): (
            (
                yul_assignment_statement::YulIdentifierPathsAndCommas,
                FixedSizeTerminalWithTrivia<2usize>,
            ),
            YulExpression,
        ),
    ) -> Self {
        Self {
            yul_identifier_paths_and_commas,
            colon_equal,
            yul_expression,
        }
    }
}

impl yul_for_statement::YulForStatement {
    pub fn from_parse(
        ((((r#for, yul_block_0_), yul_expression), yul_block_1_), yul_block_2_): (
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
            yul_block_0_,
            yul_expression,
            yul_block_1_,
            yul_block_2_,
        }
    }
}

impl yul_if_statement::YulIfStatement {
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

impl yul_switch_statement::Sequence2 {
    pub fn from_parse(
        (case, yul_literal): (FixedSizeTerminalWithTrivia<4usize>, YulLiteral),
    ) -> Self {
        Self { case, yul_literal }
    }
}
impl yul_switch_statement::Sequence0 {
    pub fn from_parse(
        (choices_1, yul_block): (Box<yul_switch_statement::Choices1>, YulBlock),
    ) -> Self {
        Self {
            choices_1,
            yul_block,
        }
    }
}
impl yul_switch_statement::YulSwitchStatement {
    pub fn from_parse(
        ((switch, yul_expression), sequence_0s): (
            (FixedSizeTerminalWithTrivia<6usize>, YulExpression),
            Vec<yul_switch_statement::Sequence0>,
        ),
    ) -> Self {
        Self {
            switch,
            yul_expression,
            sequence_0s,
        }
    }
}

impl Default for yul_variable_declaration::YulIdentifierPathsAndCommas {
    fn default() -> Self {
        Self {
            yul_identifier_paths: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for yul_variable_declaration::YulIdentifierPathsAndCommas {
    fn is_default(&self) -> bool {
        self.yul_identifier_paths.is_default() && self.commas.is_default()
    }
}
impl yul_variable_declaration::Sequence0 {
    pub fn from_parse(
        (colon_equal, yul_expression): (FixedSizeTerminalWithTrivia<2usize>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_variable_declaration::YulVariableDeclaration {
    pub fn from_parse(
        ((r#let, yul_identifier_paths_and_commas), sequence_0): (
            (
                FixedSizeTerminalWithTrivia<3usize>,
                yul_variable_declaration::YulIdentifierPathsAndCommas,
            ),
            Option<yul_variable_declaration::Sequence0>,
        ),
    ) -> Self {
        Self {
            r#let,
            yul_identifier_paths_and_commas,
            sequence_0,
        }
    }
}
impl Default for yul_variable_declaration::YulVariableDeclaration {
    fn default() -> Self {
        Self {
            r#let: Default::default(),
            yul_identifier_paths_and_commas: Default::default(),
            sequence_0: Default::default(),
        }
    }
}
impl DefaultTest for yul_variable_declaration::YulVariableDeclaration {
    fn is_default(&self) -> bool {
        self.r#let.is_default()
            && self.yul_identifier_paths_and_commas.is_default()
            && self.sequence_0.is_default()
    }
}

impl emit_statement::EmitStatement {
    pub fn from_parse(
        (((emit, identifier_path), argument_list), semicolon): (
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
            semicolon,
        }
    }
}
impl Default for emit_statement::EmitStatement {
    fn default() -> Self {
        Self {
            emit: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for emit_statement::EmitStatement {
    fn is_default(&self) -> bool {
        self.emit.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
            && self.semicolon.is_default()
    }
}

impl inheritance_specifier::InheritanceSpecifier {
    pub fn from_parse(
        (identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl Default for inheritance_specifier::InheritanceSpecifier {
    fn default() -> Self {
        Self {
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier::InheritanceSpecifier {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl modifier_invocation::ModifierInvocation {
    pub fn from_parse(
        (identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl Default for modifier_invocation::ModifierInvocation {
    fn default() -> Self {
        Self {
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for modifier_invocation::ModifierInvocation {
    fn is_default(&self) -> bool {
        self.identifier_path.is_default() && self.argument_list.is_default()
    }
}

impl new_expression::NewExpression {
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
impl Default for new_expression::NewExpression {
    fn default() -> Self {
        Self {
            new: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for new_expression::NewExpression {
    fn is_default(&self) -> bool {
        self.new.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
    }
}

impl payable_expression::PayableExpression {
    pub fn from_parse(
        (payable, argument_list): (FixedSizeTerminalWithTrivia<7usize>, ArgumentList),
    ) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}
impl Default for payable_expression::PayableExpression {
    fn default() -> Self {
        Self {
            payable: Default::default(),
            argument_list: Default::default(),
        }
    }
}
impl DefaultTest for payable_expression::PayableExpression {
    fn is_default(&self) -> bool {
        self.payable.is_default() && self.argument_list.is_default()
    }
}

impl revert_statement::RevertStatement {
    pub fn from_parse(
        (((revert, identifier_path), argument_list), semicolon): (
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
            semicolon,
        }
    }
}
impl Default for revert_statement::RevertStatement {
    fn default() -> Self {
        Self {
            revert: Default::default(),
            identifier_path: Default::default(),
            argument_list: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for revert_statement::RevertStatement {
    fn is_default(&self) -> bool {
        self.revert.is_default()
            && self.identifier_path.is_default()
            && self.argument_list.is_default()
            && self.semicolon.is_default()
    }
}

impl Default for type_name::OpenBracketAndExpressionAndCloseBracket {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            expression: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for type_name::OpenBracketAndExpressionAndCloseBracket {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default()
            && self.expression.is_default()
            && self.close_bracket.is_default()
    }
}
impl type_name::OpenBracketAndExpressionAndCloseBracket {
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
impl type_name::TypeName {
    pub fn from_parse(
        (choices_0, open_bracket_and_expression_and_close_brackets): (
            Box<type_name::Choices0>,
            Vec<type_name::OpenBracketAndExpressionAndCloseBracket>,
        ),
    ) -> Self {
        Self {
            choices_0,
            open_bracket_and_expression_and_close_brackets,
        }
    }
}

impl error_parameter::ErrorParameter {
    pub fn from_parse((type_name, identifier): (TypeName, Option<identifier::WithTrivia>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}

impl event_parameter::EventParameter {
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

impl Default for inheritance_specifier_list::InheritanceSpecifiersAndCommas {
    fn default() -> Self {
        Self {
            inheritance_specifiers: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier_list::InheritanceSpecifiersAndCommas {
    fn is_default(&self) -> bool {
        self.inheritance_specifiers.is_default() && self.commas.is_default()
    }
}
impl inheritance_specifier_list::InheritanceSpecifierList {
    pub fn from_parse(
        (is, inheritance_specifiers_and_commas): (
            FixedSizeTerminalWithTrivia<2usize>,
            inheritance_specifier_list::InheritanceSpecifiersAndCommas,
        ),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers_and_commas,
        }
    }
}
impl Default for inheritance_specifier_list::InheritanceSpecifierList {
    fn default() -> Self {
        Self {
            is: Default::default(),
            inheritance_specifiers_and_commas: Default::default(),
        }
    }
}
impl DefaultTest for inheritance_specifier_list::InheritanceSpecifierList {
    fn is_default(&self) -> bool {
        self.is.is_default() && self.inheritance_specifiers_and_commas.is_default()
    }
}

impl struct_member::StructMember {
    pub fn from_parse(
        ((type_name, identifier), semicolon): (
            (TypeName, identifier::WithTrivia),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            identifier,
            semicolon,
        }
    }
}

impl type_expression::OpenParenAndTypeNameAndCloseParen {
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
impl type_expression::TypeExpression {
    pub fn from_parse(
        (r#type, open_paren_and_type_name_and_close_paren): (
            FixedSizeTerminalWithTrivia<4usize>,
            type_expression::OpenParenAndTypeNameAndCloseParen,
        ),
    ) -> Self {
        Self {
            r#type,
            open_paren_and_type_name_and_close_paren,
        }
    }
}

impl Default for using_directive::IdentifierPathsAndCommas {
    fn default() -> Self {
        Self {
            identifier_paths: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for using_directive::IdentifierPathsAndCommas {
    fn is_default(&self) -> bool {
        self.identifier_paths.is_default() && self.commas.is_default()
    }
}
impl Default for using_directive::OpenBraceAndIdentifierPathsAndCommasAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            identifier_paths_and_commas: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for using_directive::OpenBraceAndIdentifierPathsAndCommasAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.identifier_paths_and_commas.is_default()
            && self.close_brace.is_default()
    }
}
impl using_directive::OpenBraceAndIdentifierPathsAndCommasAndCloseBrace {
    pub fn from_parse(
        ((open_brace, identifier_paths_and_commas), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                using_directive::IdentifierPathsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            identifier_paths_and_commas,
            close_brace,
        }
    }
}
impl using_directive::UsingDirective {
    pub fn from_parse(
        (((((using, choices_0), r#for), choices_1), global), semicolon): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithTrivia<5usize>,
                            Box<using_directive::Choices0>,
                        ),
                        FixedSizeTerminalWithTrivia<3usize>,
                    ),
                    Box<using_directive::Choices1>,
                ),
                Option<FixedSizeTerminalWithTrivia<6usize>>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            using,
            choices_0,
            r#for,
            choices_1,
            global,
            semicolon,
        }
    }
}

impl Default for yul_block::YulBlock {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            yul_statements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for yul_block::YulBlock {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.yul_statements.is_default()
            && self.close_brace.is_default()
    }
}
impl yul_block::YulBlock {
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

impl assembly_statement::AssemblyStatement {
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
impl Default for assembly_statement::AssemblyStatement {
    fn default() -> Self {
        Self {
            assembly: Default::default(),
            double_quote_evmasm_double_quote: Default::default(),
            assembly_flags: Default::default(),
            yul_block: Default::default(),
        }
    }
}
impl DefaultTest for assembly_statement::AssemblyStatement {
    fn is_default(&self) -> bool {
        self.assembly.is_default()
            && self.double_quote_evmasm_double_quote.is_default()
            && self.assembly_flags.is_default()
            && self.yul_block.is_default()
    }
}

impl Default for error_definition::ErrorParametersAndCommas {
    fn default() -> Self {
        Self {
            error_parameters: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::ErrorParametersAndCommas {
    fn is_default(&self) -> bool {
        self.error_parameters.is_default() && self.commas.is_default()
    }
}
impl Default for error_definition::OpenParenAndErrorParametersAndCommasAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            error_parameters_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::OpenParenAndErrorParametersAndCommasAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.error_parameters_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl error_definition::OpenParenAndErrorParametersAndCommasAndCloseParen {
    pub fn from_parse(
        ((open_paren, error_parameters_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<error_definition::ErrorParametersAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            error_parameters_and_commas,
            close_paren,
        }
    }
}
impl error_definition::ErrorDefinition {
    pub fn from_parse(
        (
            ((error, identifier), open_paren_and_error_parameters_and_commas_and_close_paren),
            semicolon,
        ): (
            (
                (FixedSizeTerminalWithTrivia<5usize>, identifier::WithTrivia),
                error_definition::OpenParenAndErrorParametersAndCommasAndCloseParen,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            error,
            identifier,
            open_paren_and_error_parameters_and_commas_and_close_paren,
            semicolon,
        }
    }
}
impl Default for error_definition::ErrorDefinition {
    fn default() -> Self {
        Self {
            error: Default::default(),
            identifier: Default::default(),
            open_paren_and_error_parameters_and_commas_and_close_paren: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for error_definition::ErrorDefinition {
    fn is_default(&self) -> bool {
        self.error.is_default()
            && self.identifier.is_default()
            && self
                .open_paren_and_error_parameters_and_commas_and_close_paren
                .is_default()
            && self.semicolon.is_default()
    }
}

impl Default for event_definition::EventParametersAndCommas {
    fn default() -> Self {
        Self {
            event_parameters: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::EventParametersAndCommas {
    fn is_default(&self) -> bool {
        self.event_parameters.is_default() && self.commas.is_default()
    }
}
impl Default for event_definition::OpenParenAndEventParametersAndCommasAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            event_parameters_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::OpenParenAndEventParametersAndCommasAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.event_parameters_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl event_definition::OpenParenAndEventParametersAndCommasAndCloseParen {
    pub fn from_parse(
        ((open_paren, event_parameters_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<event_definition::EventParametersAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            event_parameters_and_commas,
            close_paren,
        }
    }
}
impl event_definition::EventDefinition {
    pub fn from_parse(
        (
            (
                ((event, identifier), open_paren_and_event_parameters_and_commas_and_close_paren),
                anonymous,
            ),
            semicolon,
        ): (
            (
                (
                    (FixedSizeTerminalWithTrivia<5usize>, identifier::WithTrivia),
                    event_definition::OpenParenAndEventParametersAndCommasAndCloseParen,
                ),
                Option<FixedSizeTerminalWithTrivia<9usize>>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            event,
            identifier,
            open_paren_and_event_parameters_and_commas_and_close_paren,
            anonymous,
            semicolon,
        }
    }
}
impl Default for event_definition::EventDefinition {
    fn default() -> Self {
        Self {
            event: Default::default(),
            identifier: Default::default(),
            open_paren_and_event_parameters_and_commas_and_close_paren: Default::default(),
            anonymous: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for event_definition::EventDefinition {
    fn is_default(&self) -> bool {
        self.event.is_default()
            && self.identifier.is_default()
            && self
                .open_paren_and_event_parameters_and_commas_and_close_paren
                .is_default()
            && self.anonymous.is_default()
            && self.semicolon.is_default()
    }
}

impl Default for struct_definition::OpenBraceAndStructMembersAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            struct_members: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for struct_definition::OpenBraceAndStructMembersAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.struct_members.is_default()
            && self.close_brace.is_default()
    }
}
impl struct_definition::OpenBraceAndStructMembersAndCloseBrace {
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
impl struct_definition::StructDefinition {
    pub fn from_parse(
        ((r#struct, identifier), open_brace_and_struct_members_and_close_brace): (
            (FixedSizeTerminalWithTrivia<6usize>, identifier::WithTrivia),
            struct_definition::OpenBraceAndStructMembersAndCloseBrace,
        ),
    ) -> Self {
        Self {
            r#struct,
            identifier,
            open_brace_and_struct_members_and_close_brace,
        }
    }
}
impl Default for struct_definition::StructDefinition {
    fn default() -> Self {
        Self {
            r#struct: Default::default(),
            identifier: Default::default(),
            open_brace_and_struct_members_and_close_brace: Default::default(),
        }
    }
}
impl DefaultTest for struct_definition::StructDefinition {
    fn is_default(&self) -> bool {
        self.r#struct.is_default()
            && self.identifier.is_default()
            && self
                .open_brace_and_struct_members_and_close_brace
                .is_default()
    }
}

impl index_access_expression::Sequence1 {
    pub fn from_parse(
        (colon, expression): (FixedSizeTerminalWithTrivia<1>, Option<Expression>),
    ) -> Self {
        Self { colon, expression }
    }
}
impl Default for index_access_expression::Sequence1 {
    fn default() -> Self {
        Self {
            colon: Default::default(),
            expression: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::Sequence1 {
    fn is_default(&self) -> bool {
        self.colon.is_default() && self.expression.is_default()
    }
}
impl index_access_expression::Sequence0 {
    pub fn from_parse(
        (expression, sequence_1): (
            Option<Expression>,
            Option<index_access_expression::Sequence1>,
        ),
    ) -> Self {
        Self {
            expression,
            sequence_1,
        }
    }
}
impl Default for index_access_expression::Sequence0 {
    fn default() -> Self {
        Self {
            expression: Default::default(),
            sequence_1: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::Sequence0 {
    fn is_default(&self) -> bool {
        self.expression.is_default() && self.sequence_1.is_default()
    }
}
impl Default for index_access_expression::OpenBracketAndSequence0AndCloseBracket {
    fn default() -> Self {
        Self {
            open_bracket: Default::default(),
            sequence_0: Default::default(),
            close_bracket: Default::default(),
        }
    }
}
impl DefaultTest for index_access_expression::OpenBracketAndSequence0AndCloseBracket {
    fn is_default(&self) -> bool {
        self.open_bracket.is_default()
            && self.sequence_0.is_default()
            && self.close_bracket.is_default()
    }
}
impl index_access_expression::OpenBracketAndSequence0AndCloseBracket {
    pub fn from_parse(
        ((open_bracket, sequence_0), close_bracket): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                index_access_expression::Sequence0,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_bracket,
            sequence_0,
            close_bracket,
        }
    }
}
impl index_access_expression::Anonexpfrag3 {
    pub fn from_parse(
        (expression, open_bracket_and_sequence_0_and_close_bracket): (
            Expression,
            index_access_expression::OpenBracketAndSequence0AndCloseBracket,
        ),
    ) -> Self {
        Self {
            expression,
            open_bracket_and_sequence_0_and_close_bracket,
        }
    }
}

impl member_access_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression, period), choices_0): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Box<member_access_expression::Choices0>,
        ),
    ) -> Self {
        Self {
            expression,
            period,
            choices_0,
        }
    }
}

impl Default for function_call_expression::NamedArgumentsAndCommas {
    fn default() -> Self {
        Self {
            named_arguments: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for function_call_expression::NamedArgumentsAndCommas {
    fn is_default(&self) -> bool {
        self.named_arguments.is_default() && self.commas.is_default()
    }
}
impl Default for function_call_expression::OpenBraceAndNamedArgumentsAndCommasAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            named_arguments_and_commas: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for function_call_expression::OpenBraceAndNamedArgumentsAndCommasAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.named_arguments_and_commas.is_default()
            && self.close_brace.is_default()
    }
}
impl function_call_expression::OpenBraceAndNamedArgumentsAndCommasAndCloseBrace {
    pub fn from_parse(
        ((open_brace, named_arguments_and_commas), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                function_call_expression::NamedArgumentsAndCommas,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            named_arguments_and_commas,
            close_brace,
        }
    }
}
impl function_call_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression, open_brace_and_named_arguments_and_commas_and_close_brace), argument_list): (
            (
                Expression,
                Option<function_call_expression::OpenBraceAndNamedArgumentsAndCommasAndCloseBrace>,
            ),
            ArgumentList,
        ),
    ) -> Self {
        Self {
            expression,
            open_brace_and_named_arguments_and_commas_and_close_brace,
            argument_list,
        }
    }
}

impl unary_prefix_expression::Anonexpfrag3 {
    pub fn from_parse(
        (choices_0, expression): (Box<unary_prefix_expression::Choices0>, Expression),
    ) -> Self {
        Self {
            choices_0,
            expression,
        }
    }
}

impl unary_suffix_expression::Anonexpfrag3 {
    pub fn from_parse(
        (expression, choices_0): (Expression, Box<unary_suffix_expression::Choices0>),
    ) -> Self {
        Self {
            expression,
            choices_0,
        }
    }
}

impl exponentiation_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, star_star), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<2usize>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            star_star,
            expression_1_,
        }
    }
}

impl mul_div_mod_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, filter_0), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            filter_0,
            expression_1_,
        }
    }
}

impl add_sub_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, filter_0), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            filter_0,
            expression_1_,
        }
    }
}

impl shift_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, choices_0), expression_1_): (
            (Expression, Box<shift_expression::Choices0>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            choices_0,
            expression_1_,
        }
    }
}

impl bit_and_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, ampersand), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            ampersand,
            expression_1_,
        }
    }
}

impl bit_x_or_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, caret), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            caret,
            expression_1_,
        }
    }
}

impl bit_or_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, pipe), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            pipe,
            expression_1_,
        }
    }
}

impl order_comparison_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, choices_0), expression_1_): (
            (Expression, Box<order_comparison_expression::Choices0>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            choices_0,
            expression_1_,
        }
    }
}

impl equality_comparison_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, choices_0), expression_1_): (
            (Expression, Box<equality_comparison_expression::Choices0>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            choices_0,
            expression_1_,
        }
    }
}

impl and_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, ampersand_ampersand), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<2usize>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            ampersand_ampersand,
            expression_1_,
        }
    }
}

impl or_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, pipe_pipe), expression_1_): (
            (Expression, FixedSizeTerminalWithTrivia<2usize>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            pipe_pipe,
            expression_1_,
        }
    }
}

impl conditional_expression::Sequence0 {
    pub fn from_parse(
        (((question, expression_0_), colon), expression_1_): (
            (
                (FixedSizeTerminalWithTrivia<1>, Expression),
                FixedSizeTerminalWithTrivia<1>,
            ),
            Expression,
        ),
    ) -> Self {
        Self {
            question,
            expression_0_,
            colon,
            expression_1_,
        }
    }
}
impl conditional_expression::Anonexpfrag3 {
    pub fn from_parse(
        (expression, sequence_0): (Expression, conditional_expression::Sequence0),
    ) -> Self {
        Self {
            expression,
            sequence_0,
        }
    }
}

impl assignment_expression::Anonexpfrag4 {
    pub fn from_parse(
        ((expression_0_, choices_0), expression_1_): (
            (Expression, Box<assignment_expression::Choices0>),
            Expression,
        ),
    ) -> Self {
        Self {
            expression_0_,
            choices_0,
            expression_1_,
        }
    }
}

impl constant_definition::ConstantDefinition {
    pub fn from_parse(
        (((((type_name, constant), identifier), equal), expression), semicolon): (
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
            equal,
            expression,
            semicolon,
        }
    }
}

impl do_while_statement::OpenParenAndExpressionAndCloseParen {
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
impl do_while_statement::DoWhileStatement {
    pub fn from_parse(
        ((((r#do, statement), r#while), open_paren_and_expression_and_close_paren), semicolon): (
            (
                (
                    (FixedSizeTerminalWithTrivia<2usize>, Statement),
                    FixedSizeTerminalWithTrivia<5usize>,
                ),
                do_while_statement::OpenParenAndExpressionAndCloseParen,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#do,
            statement,
            r#while,
            open_paren_and_expression_and_close_paren,
            semicolon,
        }
    }
}

impl expression_statement::ExpressionStatement {
    pub fn from_parse(
        (expression, semicolon): (Expression, FixedSizeTerminalWithTrivia<1>),
    ) -> Self {
        Self {
            expression,
            semicolon,
        }
    }
}

impl if_statement::OpenParenAndExpressionAndCloseParen {
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
impl if_statement::Sequence0 {
    pub fn from_parse(
        (r#else, statement): (FixedSizeTerminalWithTrivia<4usize>, Statement),
    ) -> Self {
        Self { r#else, statement }
    }
}
impl if_statement::IfStatement {
    pub fn from_parse(
        (((r#if, open_paren_and_expression_and_close_paren), statement), sequence_0): (
            (
                (
                    FixedSizeTerminalWithTrivia<2usize>,
                    if_statement::OpenParenAndExpressionAndCloseParen,
                ),
                Statement,
            ),
            Option<if_statement::Sequence0>,
        ),
    ) -> Self {
        Self {
            r#if,
            open_paren_and_expression_and_close_paren,
            statement,
            sequence_0,
        }
    }
}

impl return_statement::ReturnStatement {
    pub fn from_parse(
        ((r#return, expression), semicolon): (
            (FixedSizeTerminalWithTrivia<6usize>, Option<Expression>),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            r#return,
            expression,
            semicolon,
        }
    }
}
impl Default for return_statement::ReturnStatement {
    fn default() -> Self {
        Self {
            r#return: Default::default(),
            expression: Default::default(),
            semicolon: Default::default(),
        }
    }
}
impl DefaultTest for return_statement::ReturnStatement {
    fn is_default(&self) -> bool {
        self.r#return.is_default() && self.expression.is_default() && self.semicolon.is_default()
    }
}

impl state_variable_declaration::Sequence0 {
    pub fn from_parse((equal, expression): (FixedSizeTerminalWithTrivia<1>, Expression)) -> Self {
        Self { equal, expression }
    }
}
impl state_variable_declaration::StateVariableDeclaration {
    pub fn from_parse(
        ((((type_name, state_variable_attributes), identifier), sequence_0), semicolon): (
            (
                (
                    (TypeName, Vec<StateVariableAttribute>),
                    identifier::WithTrivia,
                ),
                Option<state_variable_declaration::Sequence0>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            state_variable_attributes,
            identifier,
            sequence_0,
            semicolon,
        }
    }
}

impl try_statement::Sequence0 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for try_statement::Sequence0 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for try_statement::Sequence0 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl try_statement::TryStatement {
    pub fn from_parse(
        ((((r#try, expression), sequence_0), block), catch_clauses): (
            (
                (
                    (FixedSizeTerminalWithTrivia<3usize>, Expression),
                    Option<try_statement::Sequence0>,
                ),
                Block,
            ),
            Vec<CatchClause>,
        ),
    ) -> Self {
        Self {
            r#try,
            expression,
            sequence_0,
            block,
            catch_clauses,
        }
    }
}

impl tuple_deconstruction_statement::Sequence0 {
    pub fn from_parse((type_name, identifier): (Option<TypeName>, identifier::WithTrivia)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}
impl Default for tuple_deconstruction_statement::Sequence0 {
    fn default() -> Self {
        Self {
            type_name: Default::default(),
            identifier: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::Sequence0 {
    fn is_default(&self) -> bool {
        self.type_name.is_default() && self.identifier.is_default()
    }
}
impl Default for tuple_deconstruction_statement::Sequence0SAndCommas {
    fn default() -> Self {
        Self {
            sequence_0s: Default::default(),
            commas: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::Sequence0SAndCommas {
    fn is_default(&self) -> bool {
        self.sequence_0s.is_default() && self.commas.is_default()
    }
}
impl Default for tuple_deconstruction_statement::OpenParenAndSequence0SAndCommasAndCloseParen {
    fn default() -> Self {
        Self {
            open_paren: Default::default(),
            sequence_0s_and_commas: Default::default(),
            close_paren: Default::default(),
        }
    }
}
impl DefaultTest for tuple_deconstruction_statement::OpenParenAndSequence0SAndCommasAndCloseParen {
    fn is_default(&self) -> bool {
        self.open_paren.is_default()
            && self.sequence_0s_and_commas.is_default()
            && self.close_paren.is_default()
    }
}
impl tuple_deconstruction_statement::OpenParenAndSequence0SAndCommasAndCloseParen {
    pub fn from_parse(
        ((open_paren, sequence_0s_and_commas), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Option<tuple_deconstruction_statement::Sequence0SAndCommas>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            sequence_0s_and_commas,
            close_paren,
        }
    }
}
impl tuple_deconstruction_statement::TupleDeconstructionStatement {
    pub fn from_parse(
        (((open_paren_and_sequence_0s_and_commas_and_close_paren, equal), expression), semicolon): (
            (
                (
                    tuple_deconstruction_statement::OpenParenAndSequence0SAndCommasAndCloseParen,
                    FixedSizeTerminalWithTrivia<1>,
                ),
                Expression,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            open_paren_and_sequence_0s_and_commas_and_close_paren,
            equal,
            expression,
            semicolon,
        }
    }
}

impl variable_declaration_statement::Sequence0 {
    pub fn from_parse((equal, expression): (FixedSizeTerminalWithTrivia<1>, Expression)) -> Self {
        Self { equal, expression }
    }
}
impl variable_declaration_statement::VariableDeclarationStatement {
    pub fn from_parse(
        ((((type_name, data_location), identifier), sequence_0), semicolon): (
            (
                ((TypeName, Option<DataLocation>), identifier::WithTrivia),
                Option<variable_declaration_statement::Sequence0>,
            ),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            data_location,
            identifier,
            sequence_0,
            semicolon,
        }
    }
}

impl while_statement::OpenParenAndExpressionAndCloseParen {
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
impl while_statement::WhileStatement {
    pub fn from_parse(
        ((r#while, open_paren_and_expression_and_close_paren), statement): (
            (
                FixedSizeTerminalWithTrivia<5usize>,
                while_statement::OpenParenAndExpressionAndCloseParen,
            ),
            Statement,
        ),
    ) -> Self {
        Self {
            r#while,
            open_paren_and_expression_and_close_paren,
            statement,
        }
    }
}

impl for_statement::Sequence0 {
    pub fn from_parse(
        ((choices_1, choices_2), expression): (
            (Box<for_statement::Choices1>, Box<for_statement::Choices2>),
            Option<Expression>,
        ),
    ) -> Self {
        Self {
            choices_1,
            choices_2,
            expression,
        }
    }
}
impl for_statement::OpenParenAndSequence0AndCloseParen {
    pub fn from_parse(
        ((open_paren, sequence_0), close_paren): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                for_statement::Sequence0,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_paren,
            sequence_0,
            close_paren,
        }
    }
}
impl for_statement::ForStatement {
    pub fn from_parse(
        ((r#for, open_paren_and_sequence_0_and_close_paren), statement): (
            (
                FixedSizeTerminalWithTrivia<3usize>,
                for_statement::OpenParenAndSequence0AndCloseParen,
            ),
            Statement,
        ),
    ) -> Self {
        Self {
            r#for,
            open_paren_and_sequence_0_and_close_paren,
            statement,
        }
    }
}

impl Default for block::Block {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            choices_0s: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for block::Block {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.choices_0s.is_default()
            && self.close_brace.is_default()
    }
}
impl block::Block {
    pub fn from_parse(
        ((open_brace, choices_0s), close_brace): (
            (
                FixedSizeTerminalWithTrivia<1usize>,
                Vec<Box<block::Choices0>>,
            ),
            FixedSizeTerminalWithTrivia<1usize>,
        ),
    ) -> Self {
        Self {
            open_brace,
            choices_0s,
            close_brace,
        }
    }
}

impl constructor_definition::ConstructorDefinition {
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
impl Default for constructor_definition::ConstructorDefinition {
    fn default() -> Self {
        Self {
            constructor: Default::default(),
            parameter_list: Default::default(),
            constructor_attributes: Default::default(),
            block: Default::default(),
        }
    }
}
impl DefaultTest for constructor_definition::ConstructorDefinition {
    fn is_default(&self) -> bool {
        self.constructor.is_default()
            && self.parameter_list.is_default()
            && self.constructor_attributes.is_default()
            && self.block.is_default()
    }
}

impl fallback_function_definition::Sequence0 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for fallback_function_definition::Sequence0 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for fallback_function_definition::Sequence0 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl fallback_function_definition::FallbackFunctionDefinition {
    pub fn from_parse(
        ((((fallback, parameter_list), fallback_function_attributes), sequence_0), choices_1): (
            (
                (
                    (FixedSizeTerminalWithTrivia<8usize>, ParameterList),
                    Vec<FallbackFunctionAttribute>,
                ),
                Option<fallback_function_definition::Sequence0>,
            ),
            Box<fallback_function_definition::Choices1>,
        ),
    ) -> Self {
        Self {
            fallback,
            parameter_list,
            fallback_function_attributes,
            sequence_0,
            choices_1,
        }
    }
}

impl function_definition::Sequence1 {
    pub fn from_parse(
        (returns, parameter_list): (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
    ) -> Self {
        Self {
            returns,
            parameter_list,
        }
    }
}
impl Default for function_definition::Sequence1 {
    fn default() -> Self {
        Self {
            returns: Default::default(),
            parameter_list: Default::default(),
        }
    }
}
impl DefaultTest for function_definition::Sequence1 {
    fn is_default(&self) -> bool {
        self.returns.is_default() && self.parameter_list.is_default()
    }
}
impl function_definition::FunctionDefinition {
    pub fn from_parse(
        (((((function, choices_0), parameter_list), function_attributes), sequence_1), choices_2): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithTrivia<8usize>,
                            Box<function_definition::Choices0>,
                        ),
                        ParameterList,
                    ),
                    Vec<FunctionAttribute>,
                ),
                Option<function_definition::Sequence1>,
            ),
            Box<function_definition::Choices2>,
        ),
    ) -> Self {
        Self {
            function,
            choices_0,
            parameter_list,
            function_attributes,
            sequence_1,
            choices_2,
        }
    }
}

impl modifier_definition::ModifierDefinition {
    pub fn from_parse(
        ((((modifier, identifier), parameter_list), modifier_attributes), choices_0): (
            (
                (
                    (FixedSizeTerminalWithTrivia<8usize>, identifier::WithTrivia),
                    Option<ParameterList>,
                ),
                Vec<ModifierAttribute>,
            ),
            Box<modifier_definition::Choices0>,
        ),
    ) -> Self {
        Self {
            modifier,
            identifier,
            parameter_list,
            modifier_attributes,
            choices_0,
        }
    }
}

impl receive_function_definition::ReceiveFunctionDefinition {
    pub fn from_parse(
        (((receive, parameter_list), receive_function_attributes), choices_0): (
            (
                (FixedSizeTerminalWithTrivia<7usize>, ParameterList),
                Vec<ReceiveFunctionAttribute>,
            ),
            Box<receive_function_definition::Choices0>,
        ),
    ) -> Self {
        Self {
            receive,
            parameter_list,
            receive_function_attributes,
            choices_0,
        }
    }
}

impl Default for contract_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for contract_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl contract_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
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
impl contract_definition::ContractDefinition {
    pub fn from_parse(
        (
            (((r#abstract, contract), identifier), inheritance_specifier_list),
            open_brace_and_contract_body_elements_and_close_brace,
        ): (
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
            contract_definition::OpenBraceAndContractBodyElementsAndCloseBrace,
        ),
    ) -> Self {
        Self {
            r#abstract,
            contract,
            identifier,
            inheritance_specifier_list,
            open_brace_and_contract_body_elements_and_close_brace,
        }
    }
}
impl Default for contract_definition::ContractDefinition {
    fn default() -> Self {
        Self {
            r#abstract: Default::default(),
            contract: Default::default(),
            identifier: Default::default(),
            inheritance_specifier_list: Default::default(),
            open_brace_and_contract_body_elements_and_close_brace: Default::default(),
        }
    }
}
impl DefaultTest for contract_definition::ContractDefinition {
    fn is_default(&self) -> bool {
        self.r#abstract.is_default()
            && self.contract.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self
                .open_brace_and_contract_body_elements_and_close_brace
                .is_default()
    }
}

impl Default for interface_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for interface_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl interface_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
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
impl interface_definition::InterfaceDefinition {
    pub fn from_parse(
        (
            ((interface, identifier), inheritance_specifier_list),
            open_brace_and_contract_body_elements_and_close_brace,
        ): (
            (
                (FixedSizeTerminalWithTrivia<9usize>, identifier::WithTrivia),
                Option<InheritanceSpecifierList>,
            ),
            interface_definition::OpenBraceAndContractBodyElementsAndCloseBrace,
        ),
    ) -> Self {
        Self {
            interface,
            identifier,
            inheritance_specifier_list,
            open_brace_and_contract_body_elements_and_close_brace,
        }
    }
}
impl Default for interface_definition::InterfaceDefinition {
    fn default() -> Self {
        Self {
            interface: Default::default(),
            identifier: Default::default(),
            inheritance_specifier_list: Default::default(),
            open_brace_and_contract_body_elements_and_close_brace: Default::default(),
        }
    }
}
impl DefaultTest for interface_definition::InterfaceDefinition {
    fn is_default(&self) -> bool {
        self.interface.is_default()
            && self.identifier.is_default()
            && self.inheritance_specifier_list.is_default()
            && self
                .open_brace_and_contract_body_elements_and_close_brace
                .is_default()
    }
}

impl Default for library_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn default() -> Self {
        Self {
            open_brace: Default::default(),
            contract_body_elements: Default::default(),
            close_brace: Default::default(),
        }
    }
}
impl DefaultTest for library_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
    fn is_default(&self) -> bool {
        self.open_brace.is_default()
            && self.contract_body_elements.is_default()
            && self.close_brace.is_default()
    }
}
impl library_definition::OpenBraceAndContractBodyElementsAndCloseBrace {
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
impl library_definition::LibraryDefinition {
    pub fn from_parse(
        ((library, identifier), open_brace_and_contract_body_elements_and_close_brace): (
            (FixedSizeTerminalWithTrivia<7usize>, identifier::WithTrivia),
            library_definition::OpenBraceAndContractBodyElementsAndCloseBrace,
        ),
    ) -> Self {
        Self {
            library,
            identifier,
            open_brace_and_contract_body_elements_and_close_brace,
        }
    }
}
impl Default for library_definition::LibraryDefinition {
    fn default() -> Self {
        Self {
            library: Default::default(),
            identifier: Default::default(),
            open_brace_and_contract_body_elements_and_close_brace: Default::default(),
        }
    }
}
impl DefaultTest for library_definition::LibraryDefinition {
    fn is_default(&self) -> bool {
        self.library.is_default()
            && self.identifier.is_default()
            && self
                .open_brace_and_contract_body_elements_and_close_brace
                .is_default()
    }
}

impl source_unit::SourceUnit {
    pub fn from_parse(
        (((leading_trivia, choices_0s), end_of_file_trivia), end_of_input): (
            (
                (LeadingTrivia, Vec<Box<source_unit::Choices0>>),
                EndOfFileTrivia,
            ),
            (),
        ),
    ) -> Self {
        Self {
            leading_trivia,
            choices_0s,
            end_of_file_trivia,
            end_of_input,
        }
    }
}
impl Default for source_unit::SourceUnit {
    fn default() -> Self {
        Self {
            leading_trivia: Default::default(),
            choices_0s: Default::default(),
            end_of_file_trivia: Default::default(),
            end_of_input: Default::default(),
        }
    }
}
impl DefaultTest for source_unit::SourceUnit {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.choices_0s.is_default()
            && self.end_of_file_trivia.is_default()
            && self.end_of_input.is_default()
    }
}
