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

impl comment::_T3 {
    pub fn from_parse((star_chars, _1): (VariableSizeTerminal, FixedSizeTerminal<1>)) -> Self {
        Self { star_chars, _1 }
    }
}
impl Default for comment::_T3 {
    fn default() -> Self {
        Self {
            star_chars: Default::default(),
            _1: Default::default(),
        }
    }
}
impl DefaultTest for comment::_T3 {
    fn is_default(&self) -> bool {
        self.star_chars.is_default() && self._1.is_default()
    }
}
impl comment::Content {
    pub fn from_parse((_t2s, star_chars): (Vec<Box<comment::_T2>>, VariableSizeTerminal)) -> Self {
        Self { _t2s, star_chars }
    }
}
impl Default for comment::Content {
    fn default() -> Self {
        Self {
            _t2s: Default::default(),
            star_chars: Default::default(),
        }
    }
}
impl DefaultTest for comment::Content {
    fn is_default(&self) -> bool {
        self._t2s.is_default() && self.star_chars.is_default()
    }
}
impl comment::_T0 {
    pub fn from_parse(
        ((open_paren_star, content), star_close_paren): (
            (FixedSizeTerminal<2usize>, comment::Content),
            FixedSizeTerminal<2usize>,
        ),
    ) -> Self {
        Self {
            open_paren_star,
            content,
            star_close_paren,
        }
    }
}
impl Default for comment::_T0 {
    fn default() -> Self {
        Self {
            open_paren_star: Default::default(),
            content: Default::default(),
            star_close_paren: Default::default(),
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
impl DefaultTest for comment::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for number::WithTrivia {
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

impl string_char::_T1 {
    pub fn from_parse(
        ((u_open_brace, _1), close_brace_char): (
            (FixedSizeTerminal<2usize>, VariableSizeTerminal),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            u_open_brace,
            _1,
            close_brace_char,
        }
    }
}
impl Default for string_char::_T1 {
    fn default() -> Self {
        Self {
            u_open_brace: Default::default(),
            _1: Default::default(),
            close_brace_char: Default::default(),
        }
    }
}
impl DefaultTest for string_char::_T1 {
    fn is_default(&self) -> bool {
        self.u_open_brace.is_default() && self._1.is_default() && self.close_brace_char.is_default()
    }
}
impl string_char::Escape {
    pub fn from_parse(
        (backslash_char, quote_or_backslash_or_hex_escape): (
            FixedSizeTerminal<1>,
            Box<string_char::QuoteOrBackslashOrHexEscape>,
        ),
    ) -> Self {
        Self {
            backslash_char,
            quote_or_backslash_or_hex_escape,
        }
    }
}

impl DefaultTest for whitespace::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl identifier::_T1 {
    pub fn from_parse(
        ((open_double_angle_char, raw_identifier), close_double_angle_char): (
            (FixedSizeTerminal<1>, RawIdentifier),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            open_double_angle_char,
            raw_identifier,
            close_double_angle_char,
        }
    }
}
impl Default for identifier::_T1 {
    fn default() -> Self {
        Self {
            open_double_angle_char: Default::default(),
            raw_identifier: Default::default(),
            close_double_angle_char: Default::default(),
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

impl DefaultTest for leading_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl single_char_string::_T0 {
    pub fn from_parse(
        ((quote_char_1, string_char), quote_char_2): (
            (FixedSizeTerminal<1>, StringChar),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            string_char,
            quote_char_2,
        }
    }
}

impl string::_T0 {
    pub fn from_parse(
        ((quote_char_1, string_chars), quote_char_2): (
            (FixedSizeTerminal<1>, Vec<StringChar>),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            string_chars,
            quote_char_2,
        }
    }
}
impl Default for string::_T0 {
    fn default() -> Self {
        Self {
            quote_char_1: Default::default(),
            string_chars: Default::default(),
            quote_char_2: Default::default(),
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
impl DefaultTest for string::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl DefaultTest for trailing_trivia::WithTrivia {
    fn is_default(&self) -> bool {
        self.leading.is_default() && self.content.is_default() && self.trailing.is_default()
    }
}

impl char_range::_T0 {
    pub fn from_parse(
        ((single_char_string_1, ellipsis_char), single_char_string_2): (
            (
                single_char_string::WithTrivia,
                FixedSizeTerminalWithTrivia<1>,
            ),
            single_char_string::WithTrivia,
        ),
    ) -> Self {
        Self {
            single_char_string_1,
            ellipsis_char,
            single_char_string_2,
        }
    }
}

impl grouped::_T0 {
    pub fn from_parse(
        ((open_paren_char, expression), close_paren_char): (
            (FixedSizeTerminalWithTrivia<1>, Expression),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            expression,
            close_paren_char,
        }
    }
}
impl Default for grouped::_T0 {
    fn default() -> Self {
        Self {
            open_paren_char: Default::default(),
            expression: Default::default(),
            close_paren_char: Default::default(),
        }
    }
}
impl DefaultTest for grouped::_T0 {
    fn is_default(&self) -> bool {
        self.open_paren_char.is_default()
            && self.expression.is_default()
            && self.close_paren_char.is_default()
    }
}

impl optional::_T0 {
    pub fn from_parse(
        ((open_bracket_char, expression), close_bracket_char): (
            (FixedSizeTerminalWithTrivia<1>, Expression),
            FixedSizeTerminalWithTrivia<1>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            close_bracket_char,
        }
    }
}
impl Default for optional::_T0 {
    fn default() -> Self {
        Self {
            open_bracket_char: Default::default(),
            expression: Default::default(),
            close_bracket_char: Default::default(),
        }
    }
}
impl DefaultTest for optional::_T0 {
    fn is_default(&self) -> bool {
        self.open_bracket_char.is_default()
            && self.expression.is_default()
            && self.close_bracket_char.is_default()
    }
}

impl repetition_prefix::_T3 {
    pub fn from_parse(
        (ellipsis_char, number): (FixedSizeTerminalWithTrivia<1>, Option<number::WithTrivia>),
    ) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}
impl Default for repetition_prefix::_T3 {
    fn default() -> Self {
        Self {
            ellipsis_char: Default::default(),
            number: Default::default(),
        }
    }
}
impl DefaultTest for repetition_prefix::_T3 {
    fn is_default(&self) -> bool {
        self.ellipsis_char.is_default() && self.number.is_default()
    }
}
impl repetition_prefix::_T2 {
    pub fn from_parse((number, _t3): (number::WithTrivia, Option<repetition_prefix::_T3>)) -> Self {
        Self { number, _t3 }
    }
}
impl Default for repetition_prefix::_T2 {
    fn default() -> Self {
        Self {
            number: Default::default(),
            _t3: Default::default(),
        }
    }
}
impl DefaultTest for repetition_prefix::_T2 {
    fn is_default(&self) -> bool {
        self.number.is_default() && self._t3.is_default()
    }
}
impl repetition_prefix::_T4 {
    pub fn from_parse(
        (ellipsis_char, number): (FixedSizeTerminalWithTrivia<1>, number::WithTrivia),
    ) -> Self {
        Self {
            ellipsis_char,
            number,
        }
    }
}
impl Default for repetition_prefix::_T4 {
    fn default() -> Self {
        Self {
            ellipsis_char: Default::default(),
            number: Default::default(),
        }
    }
}
impl DefaultTest for repetition_prefix::_T4 {
    fn is_default(&self) -> bool {
        self.ellipsis_char.is_default() && self.number.is_default()
    }
}
impl repetition_prefix::_T0 {
    pub fn from_parse(
        (_t1, star_char): (Box<repetition_prefix::_T1>, FixedSizeTerminalWithTrivia<1>),
    ) -> Self {
        Self { _t1, star_char }
    }
}

impl repetition_separator::_T0 {
    pub fn from_parse(
        (slash_char, expression): (FixedSizeTerminalWithTrivia<1>, Expression),
    ) -> Self {
        Self {
            slash_char,
            expression,
        }
    }
}
impl Default for repetition_separator::_T0 {
    fn default() -> Self {
        Self {
            slash_char: Default::default(),
            expression: Default::default(),
        }
    }
}
impl DefaultTest for repetition_separator::_T0 {
    fn is_default(&self) -> bool {
        self.slash_char.is_default() && self.expression.is_default()
    }
}

impl repeated::_T0 {
    pub fn from_parse(
        (
            (((repetition_prefix, open_brace_char), expression), repetition_separator),
            close_brace_char,
        ): (
            (
                (
                    (Option<RepetitionPrefix>, FixedSizeTerminalWithTrivia<1>),
                    Expression,
                ),
                Option<RepetitionSeparator>,
            ),
            FixedSizeTerminalWithTrivia<1>,
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
impl Default for repeated::_T0 {
    fn default() -> Self {
        Self {
            repetition_prefix: Default::default(),
            open_brace_char: Default::default(),
            expression: Default::default(),
            repetition_separator: Default::default(),
            close_brace_char: Default::default(),
        }
    }
}
impl DefaultTest for repeated::_T0 {
    fn is_default(&self) -> bool {
        self.repetition_prefix.is_default()
            && self.open_brace_char.is_default()
            && self.expression.is_default()
            && self.repetition_separator.is_default()
            && self.close_brace_char.is_default()
    }
}

impl negation::_T0 {
    pub fn from_parse(
        (not_char, primary): (Option<FixedSizeTerminalWithTrivia<1>>, Primary),
    ) -> Self {
        Self { not_char, primary }
    }
}

impl difference::_T1 {
    pub fn from_parse((minus_char, negation): (FixedSizeTerminalWithTrivia<1>, Negation)) -> Self {
        Self {
            minus_char,
            negation,
        }
    }
}
impl difference::_T0 {
    pub fn from_parse((negation, _t1): (Negation, Option<difference::_T1>)) -> Self {
        Self { negation, _t1 }
    }
}

impl Default for expression::_T0 {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            separators: Default::default(),
        }
    }
}
impl DefaultTest for expression::_T0 {
    fn is_default(&self) -> bool {
        self.elements.is_default() && self.separators.is_default()
    }
}

impl production::_T0 {
    pub fn from_parse(
        (((identifier, equal_char), expression), semicolon_char): (
            (
                (identifier::WithTrivia, FixedSizeTerminalWithTrivia<1>),
                Expression,
            ),
            FixedSizeTerminalWithTrivia<1>,
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

impl grammar::_T0 {
    pub fn from_parse(
        (((leading_trivia, productions), trailing_trivia), end_marker): (
            (
                (leading_trivia::WithTrivia, Vec<Production>),
                trailing_trivia::WithTrivia,
            ),
            (),
        ),
    ) -> Self {
        Self {
            leading_trivia,
            productions,
            trailing_trivia,
            end_marker,
        }
    }
}
impl Default for grammar::_T0 {
    fn default() -> Self {
        Self {
            leading_trivia: Default::default(),
            productions: Default::default(),
            trailing_trivia: Default::default(),
            end_marker: Default::default(),
        }
    }
}
impl DefaultTest for grammar::_T0 {
    fn is_default(&self) -> bool {
        self.leading_trivia.is_default()
            && self.productions.is_default()
            && self.trailing_trivia.is_default()
            && self.end_marker.is_default()
    }
}
