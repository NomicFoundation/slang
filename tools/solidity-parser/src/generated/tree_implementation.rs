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
        *self == 0
    }
}
impl<const N: usize> DefaultTest for FixedSizeTerminal<N> {
    fn is_default(&self) -> bool {
        true
    }
}

impl comment::_T3 {
    pub fn from_parse((star_chars, _1): (usize, FixedSizeTerminal<1>)) -> Self {
        Self { star_chars, _1 }
    }
}
impl comment::Content {
    pub fn from_parse((_t2s, star_chars): (Vec<Box<comment::_T2>>, usize)) -> Self {
        Self { _t2s, star_chars }
    }
}
impl comment::_T0 {
    pub fn from_parse(
        ((slash_star, content), star_slash): (
            (FixedSizeTerminal<2usize>, comment::Content),
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

impl fixed_bytes_type::_T0 {
    pub fn from_parse((bytes, _1): (FixedSizeTerminal<5usize>, VariableSizeTerminal)) -> Self {
        Self { bytes, _1 }
    }
}

impl fixed_type::_T1 {
    pub fn from_parse(
        ((((_0, _1), _2), _3), _4): (
            (
                ((FixedSizeTerminal<1>, usize), FixedSizeTerminal<1>),
                FixedSizeTerminal<1>,
            ),
            usize,
        ),
    ) -> Self {
        Self { _0, _1, _2, _3, _4 }
    }
}
impl fixed_type::_T0 {
    pub fn from_parse((fixed, _t1): (FixedSizeTerminal<5usize>, Option<fixed_type::_T1>)) -> Self {
        Self { fixed, _t1 }
    }
}

impl hex_byte_escape::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl hex_number::_T0 {
    pub fn from_parse(
        ((zero_char, _1), _2): (
            (FixedSizeTerminal<1>, FixedSizeTerminal<1>),
            hex_number::_T1,
        ),
    ) -> Self {
        Self { zero_char, _1, _2 }
    }
}

impl line_comment::_T0 {
    pub fn from_parse((slash_slash, _1): (FixedSizeTerminal<2usize>, usize)) -> Self {
        Self { slash_slash, _1 }
    }
}

impl pragma_directive::_T0 {
    pub fn from_parse(
        ((pragma, not_semicolon_chars), semicolon_char): (
            (FixedSizeTerminal<6usize>, usize),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            pragma,
            not_semicolon_chars,
            semicolon_char,
        }
    }
}

impl raw_identifier::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl signed_integer_type::_T0 {
    pub fn from_parse((int, _1): (FixedSizeTerminal<3usize>, VariableSizeTerminal)) -> Self {
        Self { int, _1 }
    }
}

impl unicode_escape::_T0 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl yul_decimal_number_literal::_T1 {
    pub fn from_parse((_0, _1): (FixedSizeTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl yul_hex_literal::_T0 {
    pub fn from_parse((zero_x, _1): (FixedSizeTerminal<2usize>, usize)) -> Self {
        Self { zero_x, _1 }
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

impl hex_string_literal::_T2 {
    pub fn from_parse(
        ((double_quote_char_1, possibly_separated_pairs_of_hex_digits), double_quote_char_2): (
            (
                FixedSizeTerminal<1>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            possibly_separated_pairs_of_hex_digits,
            double_quote_char_2,
        }
    }
}
impl hex_string_literal::_T3 {
    pub fn from_parse(
        ((quote_char_1, possibly_separated_pairs_of_hex_digits), quote_char_2): (
            (
                FixedSizeTerminal<1>,
                Option<PossiblySeparatedPairsOfHexDigits>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            possibly_separated_pairs_of_hex_digits,
            quote_char_2,
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

impl ufixed_type::_T0 {
    pub fn from_parse((_0, fixed_type): (FixedSizeTerminal<1>, FixedType)) -> Self {
        Self { _0, fixed_type }
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

impl break_statement::_T0 {
    pub fn from_parse(
        (r#break, semicolon_char): (
            FixedSizeTerminalWithNoise<5usize>,
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            r#break,
            semicolon_char,
        }
    }
}

impl continue_statement::_T0 {
    pub fn from_parse(
        (r#continue, semicolon_char): (
            FixedSizeTerminalWithNoise<8usize>,
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            r#continue,
            semicolon_char,
        }
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

impl double_quoted_ascii_string_literal::_T0 {
    pub fn from_parse(
        ((double_quote_char_1, runs), double_quote_char_2): (
            (
                FixedSizeTerminal<1>,
                Vec<Box<double_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            runs,
            double_quote_char_2,
        }
    }
}

impl double_quoted_unicode_string_literal::_T0 {
    pub fn from_parse(
        ((unicode_double_quote, runs), double_quote_char): (
            (
                FixedSizeTerminal<8usize>,
                Vec<Box<double_quoted_unicode_string_literal::Run>>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            runs,
            double_quote_char,
        }
    }
}

impl single_quoted_ascii_string_literal::_T0 {
    pub fn from_parse(
        ((quote_char_1, runs), quote_char_2): (
            (
                FixedSizeTerminal<1>,
                Vec<Box<single_quoted_ascii_string_literal::Run>>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            runs,
            quote_char_2,
        }
    }
}

impl single_quoted_unicode_string_literal::_T0 {
    pub fn from_parse(
        ((unicode_quote, runs), quote_char): (
            (
                FixedSizeTerminal<8usize>,
                Vec<Box<single_quoted_unicode_string_literal::Run>>,
            ),
            FixedSizeTerminal<1>,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            runs,
            quote_char,
        }
    }
}

impl unchecked_block::_T0 {
    pub fn from_parse((unchecked, block): (FixedSizeTerminalWithNoise<9usize>, Block)) -> Self {
        Self { unchecked, block }
    }
}

impl yul_function_call::_T0 {
    pub fn from_parse(
        (((_t1, open_paren_char), yul_expressions), close_paren_char): (
            (
                (Box<yul_function_call::_T1>, FixedSizeTerminalWithNoise<1>),
                Option<yul_function_call::_T2>,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            _t1,
            open_paren_char,
            yul_expressions,
            close_paren_char,
        }
    }
}

impl yul_function_definition::_T2 {
    pub fn from_parse(
        (minus_greater, yul_identifiers): (
            FixedSizeTerminalWithNoise<2usize>,
            yul_function_definition::_T3,
        ),
    ) -> Self {
        Self {
            minus_greater,
            yul_identifiers,
        }
    }
}
impl yul_function_definition::_T0 {
    pub fn from_parse(
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
                            (FixedSizeTerminalWithNoise<8usize>, YulIdentifier),
                            FixedSizeTerminalWithNoise<1>,
                        ),
                        Option<yul_function_definition::_T1>,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Option<yul_function_definition::_T2>,
            ),
            YulBlock,
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

impl yul_path::_T2 {
    pub fn from_parse(
        (period_char, _t3): (FixedSizeTerminalWithNoise<1>, Box<yul_path::_T3>),
    ) -> Self {
        Self { period_char, _t3 }
    }
}
impl yul_path::_T0 {
    pub fn from_parse((yul_identifier, _t2s): (YulIdentifier, Vec<yul_path::_T2>)) -> Self {
        Self {
            yul_identifier,
            _t2s,
        }
    }
}

impl assembly_flags::_T0 {
    pub fn from_parse(
        ((open_paren_char, double_quoted_ascii_string_literals), close_paren_char): (
            (FixedSizeTerminalWithNoise<1>, assembly_flags::_T1),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            double_quoted_ascii_string_literals,
            close_paren_char,
        }
    }
}

impl elementary_type_with_payable::_T1 {
    pub fn from_parse(
        (address, payable): (
            FixedSizeTerminalWithNoise<7usize>,
            Option<FixedSizeTerminalWithNoise<7usize>>,
        ),
    ) -> Self {
        Self { address, payable }
    }
}

impl numeric_literal::_T0 {
    pub fn from_parse(
        (_t1, _1): (
            Box<numeric_literal::_T1>,
            Option<VariableSizeTerminalWithNoise>,
        ),
    ) -> Self {
        Self { _t1, _1 }
    }
}

impl enum_definition::_T0 {
    pub fn from_parse(
        ((((r#enum, identifier), open_brace_char), identifiers), close_brace_char): (
            (
                (
                    (FixedSizeTerminalWithNoise<4usize>, Identifier),
                    FixedSizeTerminalWithNoise<1>,
                ),
                enum_definition::_T1,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl named_argument::_T0 {
    pub fn from_parse(
        ((identifier, colon_char), expression): (
            (Identifier, FixedSizeTerminalWithNoise<1>),
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
            (TypeName, Option<VariableSizeTerminalWithNoise>),
            Option<Identifier>,
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
        (r#as, identifier): (FixedSizeTerminalWithNoise<2usize>, Identifier),
    ) -> Self {
        Self { r#as, identifier }
    }
}
impl selected_import::_T0 {
    pub fn from_parse((identifier, _t1): (Identifier, Option<selected_import::_T1>)) -> Self {
        Self { identifier, _t1 }
    }
}

impl simple_import_directive::_T2 {
    pub fn from_parse(
        (r#as, identifier): (FixedSizeTerminalWithNoise<2usize>, Identifier),
    ) -> Self {
        Self { r#as, identifier }
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
                        FixedSizeTerminalWithNoise<1>,
                        FixedSizeTerminalWithNoise<2usize>,
                    ),
                    Identifier,
                ),
                FixedSizeTerminalWithNoise<4usize>,
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

impl user_defined_value_type_definition::_T0 {
    pub fn from_parse(
        ((((r#type, identifier), is), elementary_type_with_payable), semicolon_char): (
            (
                (
                    (FixedSizeTerminalWithNoise<4usize>, Identifier),
                    FixedSizeTerminalWithNoise<2usize>,
                ),
                ElementaryTypeWithPayable,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl mapping_type::_T0 {
    pub fn from_parse(
        (((((mapping, open_paren_char), _t1), equal_greater), type_name), close_paren_char): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithNoise<7usize>,
                            FixedSizeTerminalWithNoise<1>,
                        ),
                        Box<mapping_type::_T1>,
                    ),
                    FixedSizeTerminalWithNoise<2usize>,
                ),
                TypeName,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            mapping,
            open_paren_char,
            _t1,
            equal_greater,
            type_name,
            close_paren_char,
        }
    }
}

impl named_argument_list::_T0 {
    pub fn from_parse(
        ((open_brace_char, named_arguments), close_brace_char): (
            (
                FixedSizeTerminalWithNoise<1>,
                Option<named_argument_list::_T1>,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}

impl non_empty_parameter_list::_T0 {
    pub fn from_parse(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (FixedSizeTerminalWithNoise<1>, non_empty_parameter_list::_T1),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}

impl override_specifier::_T1 {
    pub fn from_parse(
        ((open_paren_char, identifier_paths), close_paren_char): (
            (FixedSizeTerminalWithNoise<1>, override_specifier::_T2),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            identifier_paths,
            close_paren_char,
        }
    }
}
impl override_specifier::_T0 {
    pub fn from_parse(
        (r#override, _t1): (
            FixedSizeTerminalWithNoise<8usize>,
            Option<override_specifier::_T1>,
        ),
    ) -> Self {
        Self { r#override, _t1 }
    }
}

impl parameter_list::_T0 {
    pub fn from_parse(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (FixedSizeTerminalWithNoise<1>, Option<parameter_list::_T1>),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}

impl selecting_import_directive::_T0 {
    pub fn from_parse(
        ((((open_brace_char, selected_imports), close_brace_char), from), import_path): (
            (
                (
                    (
                        FixedSizeTerminalWithNoise<1>,
                        selecting_import_directive::_T1,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                FixedSizeTerminalWithNoise<4usize>,
            ),
            ImportPath,
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

impl yul_assignment::_T2 {
    pub fn from_parse(
        (colon_equal, yul_expression): (FixedSizeTerminalWithNoise<2usize>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_assignment::_T5 {
    pub fn from_parse((comma_char, yul_path): (FixedSizeTerminalWithNoise<1>, YulPath)) -> Self {
        Self {
            comma_char,
            yul_path,
        }
    }
}
impl yul_assignment::_T3 {
    pub fn from_parse(
        ((_t5s, colon_equal), yul_function_call): (
            (Vec<yul_assignment::_T5>, FixedSizeTerminalWithNoise<2usize>),
            YulFunctionCall,
        ),
    ) -> Self {
        Self {
            _t5s,
            colon_equal,
            yul_function_call,
        }
    }
}
impl yul_assignment::_T0 {
    pub fn from_parse((yul_path, _t1): (YulPath, Box<yul_assignment::_T1>)) -> Self {
        Self { yul_path, _t1 }
    }
}

impl yul_for_statement::_T0 {
    pub fn from_parse(
        ((((r#for, yul_block_1), yul_expression), yul_block_2), yul_block_3): (
            (
                (
                    (FixedSizeTerminalWithNoise<3usize>, YulBlock),
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
            (FixedSizeTerminalWithNoise<2usize>, YulExpression),
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
        ((case, yul_literal), yul_block): (
            (FixedSizeTerminalWithNoise<4usize>, YulLiteral),
            YulBlock,
        ),
    ) -> Self {
        Self {
            case,
            yul_literal,
            yul_block,
        }
    }
}
impl yul_switch_statement::_T5 {
    pub fn from_parse(
        (default, yul_block): (FixedSizeTerminalWithNoise<7usize>, YulBlock),
    ) -> Self {
        Self { default, yul_block }
    }
}
impl yul_switch_statement::_T2 {
    pub fn from_parse(
        (_t4s, _t5): (
            Vec<yul_switch_statement::_T4>,
            Option<yul_switch_statement::_T5>,
        ),
    ) -> Self {
        Self { _t4s, _t5 }
    }
}
impl yul_switch_statement::_T6 {
    pub fn from_parse(
        (default, yul_block): (FixedSizeTerminalWithNoise<7usize>, YulBlock),
    ) -> Self {
        Self { default, yul_block }
    }
}
impl yul_switch_statement::_T0 {
    pub fn from_parse(
        ((switch, yul_expression), _t1): (
            (FixedSizeTerminalWithNoise<6usize>, YulExpression),
            Box<yul_switch_statement::_T1>,
        ),
    ) -> Self {
        Self {
            switch,
            yul_expression,
            _t1,
        }
    }
}

impl yul_variable_declaration::_T2 {
    pub fn from_parse(
        (colon_equal, yul_expression): (FixedSizeTerminalWithNoise<2usize>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_variable_declaration::_T4 {
    pub fn from_parse(
        (comma_char, yul_identifier): (FixedSizeTerminalWithNoise<1>, YulIdentifier),
    ) -> Self {
        Self {
            comma_char,
            yul_identifier,
        }
    }
}
impl yul_variable_declaration::_T5 {
    pub fn from_parse(
        (colon_equal, yul_function_call): (FixedSizeTerminalWithNoise<2usize>, YulFunctionCall),
    ) -> Self {
        Self {
            colon_equal,
            yul_function_call,
        }
    }
}
impl yul_variable_declaration::_T3 {
    pub fn from_parse(
        (_t4, _t5): (
            Option<yul_variable_declaration::_T4>,
            Option<yul_variable_declaration::_T5>,
        ),
    ) -> Self {
        Self { _t4, _t5 }
    }
}
impl yul_variable_declaration::_T0 {
    pub fn from_parse(
        ((r#let, yul_identifier), _t1): (
            (FixedSizeTerminalWithNoise<3usize>, YulIdentifier),
            Option<Box<yul_variable_declaration::_T1>>,
        ),
    ) -> Self {
        Self {
            r#let,
            yul_identifier,
            _t1,
        }
    }
}

impl argument_list::_T0 {
    pub fn from_parse(
        ((open_paren_char, _t1), close_paren_char): (
            (
                FixedSizeTerminalWithNoise<1>,
                Option<Box<argument_list::_T1>>,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            _t1,
            close_paren_char,
        }
    }
}

impl catch_clause::_T1 {
    pub fn from_parse(
        (identifier, non_empty_parameter_list): (Option<Identifier>, NonEmptyParameterList),
    ) -> Self {
        Self {
            identifier,
            non_empty_parameter_list,
        }
    }
}
impl catch_clause::_T0 {
    pub fn from_parse(
        ((catch, _t1), block): (
            (
                FixedSizeTerminalWithNoise<5usize>,
                Option<catch_clause::_T1>,
            ),
            Block,
        ),
    ) -> Self {
        Self { catch, _t1, block }
    }
}

impl function_type::_T2 {
    pub fn from_parse(
        (returns, non_empty_parameter_list): (
            FixedSizeTerminalWithNoise<7usize>,
            NonEmptyParameterList,
        ),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl function_type::_T0 {
    pub fn from_parse(
        (((function, parameter_list), _2), _t2): (
            (
                (FixedSizeTerminalWithNoise<8usize>, ParameterList),
                Vec<VariableSizeTerminalWithNoise>,
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

impl import_directive::_T0 {
    pub fn from_parse(
        ((import, _t1), semicolon_char): (
            (
                FixedSizeTerminalWithNoise<6usize>,
                Box<import_directive::_T1>,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            import,
            _t1,
            semicolon_char,
        }
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

impl type_name::_T3 {
    pub fn from_parse(
        ((open_bracket_char, expression), close_bracket_char): (
            (FixedSizeTerminalWithNoise<1>, Option<Expression>),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            close_bracket_char,
        }
    }
}
impl type_name::_T0 {
    pub fn from_parse((_t1, _t3s): (Box<type_name::_T1>, Vec<type_name::_T3>)) -> Self {
        Self { _t1, _t3s }
    }
}

impl yul_block::_T0 {
    pub fn from_parse(
        ((open_brace_char, yul_statements), close_brace_char): (
            (FixedSizeTerminalWithNoise<1>, Vec<YulStatement>),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            yul_statements,
            close_brace_char,
        }
    }
}

impl assembly_statement::_T0 {
    pub fn from_parse(
        (((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block): (
            (
                (
                    FixedSizeTerminalWithNoise<8usize>,
                    Option<FixedSizeTerminalWithNoise<8usize>>,
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

impl error_parameter::_T0 {
    pub fn from_parse((type_name, identifier): (TypeName, Option<Identifier>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}

impl event_parameter::_T0 {
    pub fn from_parse(
        ((type_name, indexed), identifier): (
            (TypeName, Option<FixedSizeTerminalWithNoise<7usize>>),
            Option<Identifier>,
        ),
    ) -> Self {
        Self {
            type_name,
            indexed,
            identifier,
        }
    }
}

impl inheritance_specifier_list::_T0 {
    pub fn from_parse(
        (is, inheritance_specifiers): (
            FixedSizeTerminalWithNoise<2usize>,
            inheritance_specifier_list::_T1,
        ),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers,
        }
    }
}

impl primary_expression::_T1 {
    pub fn from_parse(
        (payable, argument_list): (FixedSizeTerminalWithNoise<7usize>, ArgumentList),
    ) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}
impl primary_expression::_T2 {
    pub fn from_parse(
        (((r#type, open_paren_char), type_name), close_paren_char): (
            (
                (
                    FixedSizeTerminalWithNoise<4usize>,
                    FixedSizeTerminalWithNoise<1>,
                ),
                TypeName,
            ),
            FixedSizeTerminalWithNoise<1>,
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
impl primary_expression::_T3 {
    pub fn from_parse((new, type_name): (FixedSizeTerminalWithNoise<3usize>, TypeName)) -> Self {
        Self { new, type_name }
    }
}
impl primary_expression::_T4 {
    pub fn from_parse(
        ((open_paren_char, expressions), close_paren_char): (
            (FixedSizeTerminalWithNoise<1>, primary_expression::_T5),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            expressions,
            close_paren_char,
        }
    }
}
impl primary_expression::_T6 {
    pub fn from_parse(
        ((open_bracket_char, expressions), close_bracket_char): (
            (FixedSizeTerminalWithNoise<1>, primary_expression::_T7),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expressions,
            close_bracket_char,
        }
    }
}

impl struct_definition::_T2 {
    pub fn from_parse(
        ((type_name, identifier), semicolon_char): (
            (TypeName, Identifier),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            type_name,
            identifier,
            semicolon_char,
        }
    }
}
impl struct_definition::_T0 {
    pub fn from_parse(
        ((((r#struct, identifier), open_brace_char), _t2s), close_brace_char): (
            (
                (
                    (FixedSizeTerminalWithNoise<6usize>, Identifier),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Vec<struct_definition::_T2>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl using_directive::_T2 {
    pub fn from_parse(
        ((open_brace_char, identifier_paths), close_brace_char): (
            (FixedSizeTerminalWithNoise<1>, using_directive::_T3),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            identifier_paths,
            close_brace_char,
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
                            FixedSizeTerminalWithNoise<5usize>,
                            Box<using_directive::_T1>,
                        ),
                        FixedSizeTerminalWithNoise<3usize>,
                    ),
                    Box<using_directive::_T4>,
                ),
                Option<FixedSizeTerminalWithNoise<6usize>>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl variable_declaration::_T0 {
    pub fn from_parse(
        ((type_name, _1), identifier): (
            (TypeName, Option<VariableSizeTerminalWithNoise>),
            Identifier,
        ),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl error_definition::_T0 {
    pub fn from_parse(
        (
            ((((error, identifier), open_paren_char), error_parameters), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        (FixedSizeTerminalWithNoise<5usize>, Identifier),
                        FixedSizeTerminalWithNoise<1>,
                    ),
                    Option<error_definition::_T1>,
                ),
                FixedSizeTerminalWithNoise<1>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl event_definition::_T0 {
    pub fn from_parse(
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
                            (FixedSizeTerminalWithNoise<5usize>, Identifier),
                            FixedSizeTerminalWithNoise<1>,
                        ),
                        Option<event_definition::_T1>,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Option<FixedSizeTerminalWithNoise<9usize>>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl index_access_expression::_T1 {
    pub fn from_parse(
        (colon_char, expression): (FixedSizeTerminalWithNoise<1>, Option<Expression>),
    ) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}
impl index_access_expression::Operator {
    pub fn from_parse(
        (((open_bracket_char, expression_2), _t1), close_bracket_char): (
            (
                (FixedSizeTerminalWithNoise<1>, Option<Expression>),
                Option<index_access_expression::_T1>,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression_2,
            _t1,
            close_bracket_char,
        }
    }
}

impl variable_declaration_tuple::_T3 {
    pub fn from_parse(
        (comma_char, variable_declaration): (
            FixedSizeTerminalWithNoise<1>,
            Option<VariableDeclaration>,
        ),
    ) -> Self {
        Self {
            comma_char,
            variable_declaration,
        }
    }
}
impl variable_declaration_tuple::_T0 {
    pub fn from_parse(
        ((((open_paren_char, comma_chars), variable_declaration), _t3s), close_paren_char): (
            (
                ((FixedSizeTerminalWithNoise<1>, usize), VariableDeclaration),
                Vec<variable_declaration_tuple::_T3>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl member_access_expression::Operator {
    pub fn from_parse(
        (period_char, _t1): (
            FixedSizeTerminalWithNoise<1>,
            Box<member_access_expression::_T1>,
        ),
    ) -> Self {
        Self { period_char, _t1 }
    }
}

impl function_call_options_expression::Operator {
    pub fn from_parse(
        ((open_brace_char, named_arguments), close_brace_char): (
            (
                FixedSizeTerminalWithNoise<1>,
                function_call_options_expression::_T1,
            ),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}

impl conditional_expression::_T1 {
    pub fn from_parse(
        (((question_char, expression_1), colon_char), expression_2): (
            (
                (FixedSizeTerminalWithNoise<1>, Expression),
                FixedSizeTerminalWithNoise<1>,
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
                    ((TypeName, FixedSizeTerminalWithNoise<8usize>), Identifier),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Expression,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl do_while_statement::_T0 {
    pub fn from_parse(
        (
            (((((r#do, statement), r#while), open_paren_char), expression), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (FixedSizeTerminalWithNoise<2usize>, Statement),
                            FixedSizeTerminalWithNoise<5usize>,
                        ),
                        FixedSizeTerminalWithNoise<1>,
                    ),
                    Expression,
                ),
                FixedSizeTerminalWithNoise<1>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl emit_statement::_T0 {
    pub fn from_parse(
        (((emit, expression), argument_list), semicolon_char): (
            (
                (FixedSizeTerminalWithNoise<4usize>, Expression),
                ArgumentList,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl expression_statement::_T0 {
    pub fn from_parse(
        (expression, semicolon_char): (Expression, FixedSizeTerminalWithNoise<1>),
    ) -> Self {
        Self {
            expression,
            semicolon_char,
        }
    }
}

impl if_statement::_T1 {
    pub fn from_parse(
        (r#else, statement): (FixedSizeTerminalWithNoise<4usize>, Statement),
    ) -> Self {
        Self { r#else, statement }
    }
}
impl if_statement::_T0 {
    pub fn from_parse(
        (((((r#if, open_paren_char), expression), close_paren_char), statement), _t1): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithNoise<2usize>,
                            FixedSizeTerminalWithNoise<1>,
                        ),
                        Expression,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Statement,
            ),
            Option<if_statement::_T1>,
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

impl return_statement::_T0 {
    pub fn from_parse(
        ((r#return, expression), semicolon_char): (
            (FixedSizeTerminalWithNoise<6usize>, Option<Expression>),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            r#return,
            expression,
            semicolon_char,
        }
    }
}

impl revert_statement::_T0 {
    pub fn from_parse(
        (((revert, expression), argument_list), semicolon_char): (
            (
                (FixedSizeTerminalWithNoise<6usize>, Expression),
                ArgumentList,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl state_variable_declaration::_T2 {
    pub fn from_parse(
        (equal_char, expression): (FixedSizeTerminalWithNoise<1>, Expression),
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
                ((TypeName, Vec<StateVariableAttribute>), Identifier),
                Option<state_variable_declaration::_T2>,
            ),
            FixedSizeTerminalWithNoise<1>,
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
        (returns, non_empty_parameter_list): (
            FixedSizeTerminalWithNoise<7usize>,
            NonEmptyParameterList,
        ),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl try_statement::_T0 {
    pub fn from_parse(
        ((((r#try, expression), _t1), block), catch_clauses): (
            (
                (
                    (FixedSizeTerminalWithNoise<3usize>, Expression),
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

impl variable_declaration_statement::_T3 {
    pub fn from_parse(
        (equal_char, expression): (FixedSizeTerminalWithNoise<1>, Expression),
    ) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl variable_declaration_statement::_T2 {
    pub fn from_parse(
        (variable_declaration, _t3): (
            VariableDeclaration,
            Option<variable_declaration_statement::_T3>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            _t3,
        }
    }
}
impl variable_declaration_statement::_T4 {
    pub fn from_parse(
        ((variable_declaration_tuple, equal_char), expression): (
            (VariableDeclarationTuple, FixedSizeTerminalWithNoise<1>),
            Expression,
        ),
    ) -> Self {
        Self {
            variable_declaration_tuple,
            equal_char,
            expression,
        }
    }
}
impl variable_declaration_statement::_T0 {
    pub fn from_parse(
        (_t1, semicolon_char): (
            Box<variable_declaration_statement::_T1>,
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            _t1,
            semicolon_char,
        }
    }
}

impl while_statement::_T0 {
    pub fn from_parse(
        ((((r#while, open_paren_char), expression), close_paren_char), statement): (
            (
                (
                    (
                        FixedSizeTerminalWithNoise<5usize>,
                        FixedSizeTerminalWithNoise<1>,
                    ),
                    Expression,
                ),
                FixedSizeTerminalWithNoise<1>,
            ),
            Statement,
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

impl for_statement::_T0 {
    pub fn from_parse(
        ((((((r#for, open_paren_char), _t1), _t2), expression), close_paren_char), statement): (
            (
                (
                    (
                        (
                            (
                                FixedSizeTerminalWithNoise<3usize>,
                                FixedSizeTerminalWithNoise<1>,
                            ),
                            Box<for_statement::_T1>,
                        ),
                        Box<for_statement::_T2>,
                    ),
                    Option<Expression>,
                ),
                FixedSizeTerminalWithNoise<1>,
            ),
            Statement,
        ),
    ) -> Self {
        Self {
            r#for,
            open_paren_char,
            _t1,
            _t2,
            expression,
            close_paren_char,
            statement,
        }
    }
}

impl block::_T0 {
    pub fn from_parse(
        ((open_brace_char, _t2s), close_brace_char): (
            (FixedSizeTerminalWithNoise<1>, Vec<Box<block::_T2>>),
            FixedSizeTerminalWithNoise<1>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            _t2s,
            close_brace_char,
        }
    }
}

impl constructor_definition::_T0 {
    pub fn from_parse(
        (((constructor, parameter_list), constructor_attributes), block): (
            (
                (FixedSizeTerminalWithNoise<11usize>, ParameterList),
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

impl fallback_function_definition::_T2 {
    pub fn from_parse(
        (returns, non_empty_parameter_list): (
            FixedSizeTerminalWithNoise<7usize>,
            NonEmptyParameterList,
        ),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl fallback_function_definition::_T0 {
    pub fn from_parse(
        ((((fallback, parameter_list), fallback_function_attributes), _t2), _t3): (
            (
                (
                    (FixedSizeTerminalWithNoise<8usize>, ParameterList),
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
        (returns, non_empty_parameter_list): (
            FixedSizeTerminalWithNoise<7usize>,
            NonEmptyParameterList,
        ),
    ) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl function_definition::_T0 {
    pub fn from_parse(
        (((((function, _t1), parameter_list), function_attributes), _t3), _t4): (
            (
                (
                    (
                        (
                            FixedSizeTerminalWithNoise<8usize>,
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
        ((((modifier, identifier), parameter_list), method_attributes), _t2): (
            (
                (
                    (FixedSizeTerminalWithNoise<8usize>, Identifier),
                    Option<ParameterList>,
                ),
                Vec<MethodAttribute>,
            ),
            Box<modifier_definition::_T2>,
        ),
    ) -> Self {
        Self {
            modifier,
            identifier,
            parameter_list,
            method_attributes,
            _t2,
        }
    }
}

impl receive_function_definition::_T0 {
    pub fn from_parse(
        ((((receive, open_paren_char), close_paren_char), receive_function_attributes), _t2): (
            (
                (
                    (
                        FixedSizeTerminalWithNoise<7usize>,
                        FixedSizeTerminalWithNoise<1>,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Vec<ReceiveFunctionAttribute>,
            ),
            Box<receive_function_definition::_T2>,
        ),
    ) -> Self {
        Self {
            receive,
            open_paren_char,
            close_paren_char,
            receive_function_attributes,
            _t2,
        }
    }
}

impl contract_definition::_T0 {
    pub fn from_parse(
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
                            (
                                Option<FixedSizeTerminalWithNoise<8usize>>,
                                FixedSizeTerminalWithNoise<8usize>,
                            ),
                            Identifier,
                        ),
                        Option<InheritanceSpecifierList>,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl interface_definition::_T0 {
    pub fn from_parse(
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
                        (FixedSizeTerminalWithNoise<9usize>, Identifier),
                        Option<InheritanceSpecifierList>,
                    ),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl library_definition::_T0 {
    pub fn from_parse(
        ((((library, identifier), open_brace_char), contract_body_elements), close_brace_char): (
            (
                (
                    (FixedSizeTerminalWithNoise<7usize>, Identifier),
                    FixedSizeTerminalWithNoise<1>,
                ),
                Vec<ContractBodyElement>,
            ),
            FixedSizeTerminalWithNoise<1>,
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

impl source_unit::_T0 {
    pub fn from_parse(
        ((ignore, _t2s), end_marker): ((Ignore, Vec<Box<source_unit::_T2>>), ()),
    ) -> Self {
        Self {
            ignore,
            _t2s,
            end_marker,
        }
    }
}
