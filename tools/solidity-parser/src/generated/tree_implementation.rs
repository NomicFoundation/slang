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

impl comment::_T2 {
    pub fn new((star_chars, _1): (usize, FixedTerminal<1>)) -> Self {
        Self { star_chars, _1 }
    }
}
impl comment::Content {
    pub fn new((comments, star_chars): (Vec<Box<comment::Comment>>, usize)) -> Self {
        Self {
            comments,
            star_chars,
        }
    }
}
impl comment::_T0 {
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

impl fixed_bytes_type::_T0 {
    pub fn new((bytes, _1): (FixedTerminal<5usize>, usize)) -> Self {
        Self { bytes, _1 }
    }
}

impl fixed_type::_T1 {
    pub fn new(
        ((((_0, _1), _2), _3), _4): (
            (
                ((FixedTerminal<1>, usize), FixedTerminal<1>),
                FixedTerminal<1>,
            ),
            usize,
        ),
    ) -> Self {
        Self { _0, _1, _2, _3, _4 }
    }
}
impl fixed_type::_T0 {
    pub fn new((fixed, _t1): (FixedTerminal<5usize>, Option<Box<fixed_type::_T1>>)) -> Self {
        Self { fixed, _t1 }
    }
}

impl line_comment::_T0 {
    pub fn new((slash_slash, _1): (FixedTerminal<2usize>, usize)) -> Self {
        Self { slash_slash, _1 }
    }
}

impl pragma_directive::_T0 {
    pub fn new(
        ((pragma, not_semicolon_chars), semicolon_char): (
            (FixedTerminal<6usize>, usize),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            pragma,
            not_semicolon_chars,
            semicolon_char,
        }
    }
}

impl signed_integer_type::_T0 {
    pub fn new((int, _1): (FixedTerminal<3usize>, usize)) -> Self {
        Self { int, _1 }
    }
}

impl yul_decimal_number_literal::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl yul_hex_literal::_T0 {
    pub fn new((zero_x, _1): (FixedTerminal<2usize>, usize)) -> Self {
        Self { zero_x, _1 }
    }
}

impl decimal_exponent::_T0 {
    pub fn new(
        ((_0, minus_char), decimal_integer): (
            (FixedTerminal<1>, Option<FixedTerminal<1>>),
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
    pub fn new(
        ((decimal_integer_1, period_char), decimal_integer_2): (
            (Option<DecimalInteger>, FixedTerminal<1>),
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

impl hex_byte_escape::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl hex_number::_T0 {
    pub fn new(
        ((zero_char, _1), _2): ((FixedTerminal<1>, FixedTerminal<1>), hex_number::_T1),
    ) -> Self {
        Self { zero_char, _1, _2 }
    }
}

impl ufixed_type::_T0 {
    pub fn new((_0, fixed_type): (FixedTerminal<1>, FixedType)) -> Self {
        Self { _0, fixed_type }
    }
}

impl unicode_escape::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl unsigned_integer_type::_T0 {
    pub fn new((_0, signed_integer_type): (FixedTerminal<1>, SignedIntegerType)) -> Self {
        Self {
            _0,
            signed_integer_type,
        }
    }
}

impl break_statement::_T0 {
    pub fn new(
        (r#break, semicolon_char): (
            WithNoise<FixedTerminal<5usize>>,
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            r#break,
            semicolon_char,
        }
    }
}

impl conditional_expression::_T1 {
    pub fn new(
        (((question_char, expression_1), colon_char), expression_2): (
            (
                (WithNoise<FixedTerminal<1>>, Expression),
                WithNoise<FixedTerminal<1>>,
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

impl continue_statement::_T0 {
    pub fn new(
        (r#continue, semicolon_char): (
            WithNoise<FixedTerminal<8usize>>,
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            r#continue,
            semicolon_char,
        }
    }
}

impl decimal_number::_T0 {
    pub fn new(
        (decimal_number, decimal_exponent): (
            Box<decimal_number::DecimalNumber>,
            Option<DecimalExponent>,
        ),
    ) -> Self {
        Self {
            decimal_number,
            decimal_exponent,
        }
    }
}

impl escape_sequence::_T0 {
    pub fn new(
        (backslash_char, escape_sequence): (FixedTerminal<1>, Box<escape_sequence::EscapeSequence>),
    ) -> Self {
        Self {
            backslash_char,
            escape_sequence,
        }
    }
}

impl hex_string_literal::_T1 {
    pub fn new(
        ((double_quote_char_1, possibly_separated_pairs_of_hex_digits), double_quote_char_2): (
            (FixedTerminal<1>, Option<PossiblySeparatedPairsOfHexDigits>),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            possibly_separated_pairs_of_hex_digits,
            double_quote_char_2,
        }
    }
}
impl hex_string_literal::_T2 {
    pub fn new(
        ((quote_char_1, possibly_separated_pairs_of_hex_digits), quote_char_2): (
            (FixedTerminal<1>, Option<PossiblySeparatedPairsOfHexDigits>),
            FixedTerminal<1>,
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

impl index_access_expression::_T1 {
    pub fn new(
        (colon_char, expression): (WithNoise<FixedTerminal<1>>, Option<Expression>),
    ) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}
impl index_access_expression::Operator {
    pub fn new(
        (((open_bracket_char, expression_2), _t1), close_bracket_char): (
            (
                (WithNoise<FixedTerminal<1>>, Option<Expression>),
                Option<Box<index_access_expression::_T1>>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl raw_identifier::_T0 {
    pub fn new((_0, _1): (FixedTerminal<1>, usize)) -> Self {
        Self { _0, _1 }
    }
}

impl unchecked_block::_T0 {
    pub fn new((unchecked, block): (WithNoise<FixedTerminal<9usize>>, Block)) -> Self {
        Self { unchecked, block }
    }
}

impl double_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((double_quote_char_1, double_quoted_ascii_string_literals), double_quote_char_2): (
            (
                FixedTerminal<1>,
                Vec<Box<double_quoted_ascii_string_literal::DoubleQuotedAsciiStringLiteral>>,
            ),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            double_quote_char_1,
            double_quoted_ascii_string_literals,
            double_quote_char_2,
        }
    }
}

impl double_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_double_quote, double_quoted_unicode_string_literals), double_quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<double_quoted_unicode_string_literal::DoubleQuotedUnicodeStringLiteral>>,
            ),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            double_quoted_unicode_string_literals,
            double_quote_char,
        }
    }
}

impl elementary_type_with_payable::_T0 {
    pub fn new(
        (address, payable): (
            WithNoise<FixedTerminal<7usize>>,
            Option<WithNoise<FixedTerminal<7usize>>>,
        ),
    ) -> Self {
        Self { address, payable }
    }
}

impl numeric_literal::_T0 {
    pub fn new(
        (numeric_literal, _1): (
            Box<numeric_literal::NumericLiteral>,
            Option<WithNoise<usize>>,
        ),
    ) -> Self {
        Self {
            numeric_literal,
            _1,
        }
    }
}

impl single_quoted_ascii_string_literal::_T0 {
    pub fn new(
        ((quote_char_1, single_quoted_ascii_string_literals), quote_char_2): (
            (
                FixedTerminal<1>,
                Vec<Box<single_quoted_ascii_string_literal::SingleQuotedAsciiStringLiteral>>,
            ),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            quote_char_1,
            single_quoted_ascii_string_literals,
            quote_char_2,
        }
    }
}

impl single_quoted_unicode_string_literal::_T0 {
    pub fn new(
        ((unicode_quote, single_quoted_unicode_string_literals), quote_char): (
            (
                FixedTerminal<8usize>,
                Vec<Box<single_quoted_unicode_string_literal::SingleQuotedUnicodeStringLiteral>>,
            ),
            FixedTerminal<1>,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            single_quoted_unicode_string_literals,
            quote_char,
        }
    }
}

impl assembly_flags::_T0 {
    pub fn new(
        ((open_paren_char, double_quoted_ascii_string_literals), close_paren_char): (
            (WithNoise<FixedTerminal<1>>, assembly_flags::_T1),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            double_quoted_ascii_string_literals,
            close_paren_char,
        }
    }
}

impl yul_function_call::_T0 {
    pub fn new(
        (((yul_function_call, open_paren_char), yul_expressions), close_paren_char): (
            (
                (
                    Box<yul_function_call::YulFunctionCall>,
                    WithNoise<FixedTerminal<1>>,
                ),
                Option<yul_function_call::_T1>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl yul_function_definition::_T2 {
    pub fn new(
        (minus_greater, yul_identifiers): (
            WithNoise<FixedTerminal<2usize>>,
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
                            (WithNoise<FixedTerminal<8usize>>, YulIdentifier),
                            WithNoise<FixedTerminal<1>>,
                        ),
                        Option<yul_function_definition::_T1>,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Option<Box<yul_function_definition::_T2>>,
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
    pub fn new(
        (period_char, yul_path): (WithNoise<FixedTerminal<1>>, Box<yul_path::YulPath>),
    ) -> Self {
        Self {
            period_char,
            yul_path,
        }
    }
}
impl yul_path::_T0 {
    pub fn new((yul_identifier, _t2s): (YulIdentifier, Vec<Box<yul_path::_T2>>)) -> Self {
        Self {
            yul_identifier,
            _t2s,
        }
    }
}

impl enum_definition::_T0 {
    pub fn new(
        ((((r#enum, identifier), open_brace_char), identifiers), close_brace_char): (
            (
                (
                    (WithNoise<FixedTerminal<4usize>>, Identifier),
                    WithNoise<FixedTerminal<1>>,
                ),
                enum_definition::_T1,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl member_access_expression::Operator {
    pub fn new(
        (period_char, member_access_expression): (
            WithNoise<FixedTerminal<1>>,
            Box<member_access_expression::MemberAccessExpression>,
        ),
    ) -> Self {
        Self {
            period_char,
            member_access_expression,
        }
    }
}

impl named_argument::_T0 {
    pub fn new(
        ((identifier, colon_char), expression): (
            (Identifier, WithNoise<FixedTerminal<1>>),
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
    pub fn new(
        ((type_name, _1), identifier): ((TypeName, Option<WithNoise<usize>>), Option<Identifier>),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl selected_import::_T1 {
    pub fn new((r#as, identifier): (WithNoise<FixedTerminal<2usize>>, Identifier)) -> Self {
        Self { r#as, identifier }
    }
}
impl selected_import::_T0 {
    pub fn new((identifier, _t1): (Identifier, Option<Box<selected_import::_T1>>)) -> Self {
        Self { identifier, _t1 }
    }
}

impl user_defined_value_type_definition::_T0 {
    pub fn new(
        ((((r#type, identifier), is), elementary_type_with_payable), semicolon_char): (
            (
                (
                    (WithNoise<FixedTerminal<4usize>>, Identifier),
                    WithNoise<FixedTerminal<2usize>>,
                ),
                ElementaryTypeWithPayable,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl function_call_options_expression::Operator {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (
                WithNoise<FixedTerminal<1>>,
                function_call_options_expression::_T1,
            ),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
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
                        (
                            WithNoise<FixedTerminal<7usize>>,
                            WithNoise<FixedTerminal<1>>,
                        ),
                        Box<mapping_type::MappingType>,
                    ),
                    WithNoise<FixedTerminal<2usize>>,
                ),
                TypeName,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl named_argument_list::_T0 {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (
                WithNoise<FixedTerminal<1>>,
                Option<named_argument_list::_T1>,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (WithNoise<FixedTerminal<1>>, non_empty_parameter_list::_T1),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((open_paren_char, identifier_paths), close_paren_char): (
            (WithNoise<FixedTerminal<1>>, override_specifier::_T2),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (r#override, _t1): (
            WithNoise<FixedTerminal<8usize>>,
            Option<Box<override_specifier::_T1>>,
        ),
    ) -> Self {
        Self { r#override, _t1 }
    }
}

impl parameter_list::_T0 {
    pub fn new(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (WithNoise<FixedTerminal<1>>, Option<parameter_list::_T1>),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((((open_brace_char, selected_imports), close_brace_char), from), import_path): (
            (
                (
                    (WithNoise<FixedTerminal<1>>, selecting_import_directive::_T1),
                    WithNoise<FixedTerminal<1>>,
                ),
                WithNoise<FixedTerminal<4usize>>,
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

impl simple_import_directive::_T2 {
    pub fn new((r#as, identifier): (WithNoise<FixedTerminal<2usize>>, Identifier)) -> Self {
        Self { r#as, identifier }
    }
}
impl simple_import_directive::_T0 {
    pub fn new((import_path, _t2s): (ImportPath, Vec<Box<simple_import_directive::_T2>>)) -> Self {
        Self { import_path, _t2s }
    }
}

impl star_import_directive::_T0 {
    pub fn new(
        ((((star_char, r#as), identifier), from), import_path): (
            (
                (
                    (
                        WithNoise<FixedTerminal<1>>,
                        WithNoise<FixedTerminal<2usize>>,
                    ),
                    Identifier,
                ),
                WithNoise<FixedTerminal<4usize>>,
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

impl argument_list::_T0 {
    pub fn new(
        ((open_paren_char, argument_list), close_paren_char): (
            (
                WithNoise<FixedTerminal<1>>,
                Option<Box<argument_list::ArgumentList>>,
            ),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            argument_list,
            close_paren_char,
        }
    }
}

impl catch_clause::_T1 {
    pub fn new(
        (identifier, non_empty_parameter_list): (Option<Identifier>, NonEmptyParameterList),
    ) -> Self {
        Self {
            identifier,
            non_empty_parameter_list,
        }
    }
}
impl catch_clause::_T0 {
    pub fn new(
        ((catch, _t1), block): (
            (
                WithNoise<FixedTerminal<5usize>>,
                Option<Box<catch_clause::_T1>>,
            ),
            Block,
        ),
    ) -> Self {
        Self { catch, _t1, block }
    }
}

impl function_type::_T2 {
    pub fn new(
        (returns, non_empty_parameter_list): (
            WithNoise<FixedTerminal<7usize>>,
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
    pub fn new(
        (((function, parameter_list), _2), _t2): (
            (
                (WithNoise<FixedTerminal<8usize>>, ParameterList),
                Vec<WithNoise<usize>>,
            ),
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

impl import_directive::_T0 {
    pub fn new(
        ((import, import_directive), semicolon_char): (
            (
                WithNoise<FixedTerminal<6usize>>,
                Box<import_directive::ImportDirective>,
            ),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            import,
            import_directive,
            semicolon_char,
        }
    }
}

impl yul_assignment::_T1 {
    pub fn new(
        (colon_equal, yul_expression): (WithNoise<FixedTerminal<2usize>>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_assignment::_T4 {
    pub fn new((comma_char, yul_path): (WithNoise<FixedTerminal<1>>, YulPath)) -> Self {
        Self {
            comma_char,
            yul_path,
        }
    }
}
impl yul_assignment::_T2 {
    pub fn new(
        ((_t4s, colon_equal), yul_function_call): (
            (
                Vec<Box<yul_assignment::_T4>>,
                WithNoise<FixedTerminal<2usize>>,
            ),
            YulFunctionCall,
        ),
    ) -> Self {
        Self {
            _t4s,
            colon_equal,
            yul_function_call,
        }
    }
}
impl yul_assignment::_T0 {
    pub fn new((yul_path, yul_assignment): (YulPath, Box<yul_assignment::YulAssignment>)) -> Self {
        Self {
            yul_path,
            yul_assignment,
        }
    }
}

impl yul_for_statement::_T0 {
    pub fn new(
        ((((r#for, yul_block_1), yul_expression), yul_block_2), yul_block_3): (
            (
                ((WithNoise<FixedTerminal<3usize>>, YulBlock), YulExpression),
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
    pub fn new(
        ((r#if, yul_expression), yul_block): (
            (WithNoise<FixedTerminal<2usize>>, YulExpression),
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

impl yul_switch_statement::_T3 {
    pub fn new(
        ((case, yul_literal), yul_block): (
            (WithNoise<FixedTerminal<4usize>>, YulLiteral),
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
impl yul_switch_statement::_T4 {
    pub fn new((default, yul_block): (WithNoise<FixedTerminal<7usize>>, YulBlock)) -> Self {
        Self { default, yul_block }
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
impl yul_switch_statement::_T5 {
    pub fn new((default, yul_block): (WithNoise<FixedTerminal<7usize>>, YulBlock)) -> Self {
        Self { default, yul_block }
    }
}
impl yul_switch_statement::_T0 {
    pub fn new(
        ((switch, yul_expression), yul_switch_statement): (
            (WithNoise<FixedTerminal<6usize>>, YulExpression),
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

impl yul_variable_declaration::_T1 {
    pub fn new(
        (colon_equal, yul_expression): (WithNoise<FixedTerminal<2usize>>, YulExpression),
    ) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_variable_declaration::_T3 {
    pub fn new((comma_char, yul_identifier): (WithNoise<FixedTerminal<1>>, YulIdentifier)) -> Self {
        Self {
            comma_char,
            yul_identifier,
        }
    }
}
impl yul_variable_declaration::_T4 {
    pub fn new(
        (colon_equal, yul_function_call): (WithNoise<FixedTerminal<2usize>>, YulFunctionCall),
    ) -> Self {
        Self {
            colon_equal,
            yul_function_call,
        }
    }
}
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
impl yul_variable_declaration::_T0 {
    pub fn new(
        ((r#let, yul_identifier), yul_variable_declaration): (
            (WithNoise<FixedTerminal<3usize>>, YulIdentifier),
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

impl inheritance_specifier::_T0 {
    pub fn new((identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>)) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}

impl modifier_invocation::_T0 {
    pub fn new((identifier_path, argument_list): (IdentifierPath, Option<ArgumentList>)) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}

impl type_name::_T2 {
    pub fn new(
        ((open_bracket_char, expression), close_bracket_char): (
            (WithNoise<FixedTerminal<1>>, Option<Expression>),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new((type_name, _t2s): (Box<type_name::TypeName>, Vec<Box<type_name::_T2>>)) -> Self {
        Self { type_name, _t2s }
    }
}

impl error_parameter::_T0 {
    pub fn new((type_name, identifier): (TypeName, Option<Identifier>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}

impl event_parameter::_T0 {
    pub fn new(
        ((type_name, indexed), identifier): (
            (TypeName, Option<WithNoise<FixedTerminal<7usize>>>),
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
    pub fn new(
        (is, inheritance_specifiers): (
            WithNoise<FixedTerminal<2usize>>,
            inheritance_specifier_list::_T1,
        ),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers,
        }
    }
}

impl primary_expression::_T0 {
    pub fn new((payable, argument_list): (WithNoise<FixedTerminal<7usize>>, ArgumentList)) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}
impl primary_expression::_T1 {
    pub fn new(
        (((r#type, open_paren_char), type_name), close_paren_char): (
            (
                (
                    WithNoise<FixedTerminal<4usize>>,
                    WithNoise<FixedTerminal<1>>,
                ),
                TypeName,
            ),
            WithNoise<FixedTerminal<1>>,
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
impl primary_expression::_T2 {
    pub fn new((new, type_name): (WithNoise<FixedTerminal<3usize>>, TypeName)) -> Self {
        Self { new, type_name }
    }
}
impl primary_expression::_T3 {
    pub fn new(
        ((open_paren_char, expressions), close_paren_char): (
            (WithNoise<FixedTerminal<1>>, primary_expression::_T4),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            expressions,
            close_paren_char,
        }
    }
}
impl primary_expression::_T5 {
    pub fn new(
        ((open_bracket_char, expressions), close_bracket_char): (
            (WithNoise<FixedTerminal<1>>, primary_expression::_T6),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((type_name, identifier), semicolon_char): (
            (TypeName, Identifier),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((((r#struct, identifier), open_brace_char), _t2s), close_brace_char): (
            (
                (
                    (WithNoise<FixedTerminal<6usize>>, Identifier),
                    WithNoise<FixedTerminal<1>>,
                ),
                Vec<Box<struct_definition::_T2>>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl using_directive::_T1 {
    pub fn new(
        ((open_brace_char, identifier_paths), close_brace_char): (
            (WithNoise<FixedTerminal<1>>, using_directive::_T2),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (((((using, using_directive_1), r#for), using_directive_2), global), semicolon_char): (
            (
                (
                    (
                        (
                            WithNoise<FixedTerminal<5usize>>,
                            Box<using_directive::UsingDirective>,
                        ),
                        WithNoise<FixedTerminal<3usize>>,
                    ),
                    Box<using_directive::UsingDirective>,
                ),
                Option<WithNoise<FixedTerminal<6usize>>>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl variable_declaration::_T0 {
    pub fn new(
        ((type_name, _1), identifier): ((TypeName, Option<WithNoise<usize>>), Identifier),
    ) -> Self {
        Self {
            type_name,
            _1,
            identifier,
        }
    }
}

impl yul_block::_T0 {
    pub fn new(
        ((open_brace_char, yul_statements), close_brace_char): (
            (WithNoise<FixedTerminal<1>>, Vec<YulStatement>),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block): (
            (
                (
                    WithNoise<FixedTerminal<8usize>>,
                    Option<WithNoise<FixedTerminal<8usize>>>,
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

impl error_definition::_T0 {
    pub fn new(
        (
            ((((error, identifier), open_paren_char), error_parameters), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        (WithNoise<FixedTerminal<5usize>>, Identifier),
                        WithNoise<FixedTerminal<1>>,
                    ),
                    Option<error_definition::_T1>,
                ),
                WithNoise<FixedTerminal<1>>,
            ),
            WithNoise<FixedTerminal<1>>,
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
                            (WithNoise<FixedTerminal<5usize>>, Identifier),
                            WithNoise<FixedTerminal<1>>,
                        ),
                        Option<event_definition::_T1>,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Option<WithNoise<FixedTerminal<9usize>>>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl variable_declaration_tuple::_T3 {
    pub fn new(
        (comma_char, variable_declaration): (
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((((open_paren_char, comma_chars), variable_declaration), _t3s), close_paren_char): (
            (
                ((WithNoise<FixedTerminal<1>>, usize), VariableDeclaration),
                Vec<Box<variable_declaration_tuple::_T3>>,
            ),
            WithNoise<FixedTerminal<1>>,
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

impl constant_definition::_T0 {
    pub fn new(
        (((((type_name, constant), identifier), equal_char), expression), semicolon_char): (
            (
                (
                    ((TypeName, WithNoise<FixedTerminal<8usize>>), Identifier),
                    WithNoise<FixedTerminal<1>>,
                ),
                Expression,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (
            (((((r#do, statement), r#while), open_paren_char), expression), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (WithNoise<FixedTerminal<2usize>>, Statement),
                            WithNoise<FixedTerminal<5usize>>,
                        ),
                        WithNoise<FixedTerminal<1>>,
                    ),
                    Expression,
                ),
                WithNoise<FixedTerminal<1>>,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (((emit, expression), argument_list), semicolon_char): (
            ((WithNoise<FixedTerminal<4usize>>, Expression), ArgumentList),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new((expression, semicolon_char): (Expression, WithNoise<FixedTerminal<1>>)) -> Self {
        Self {
            expression,
            semicolon_char,
        }
    }
}

impl if_statement::_T1 {
    pub fn new((r#else, statement): (WithNoise<FixedTerminal<4usize>>, Statement)) -> Self {
        Self { r#else, statement }
    }
}
impl if_statement::_T0 {
    pub fn new(
        (((((r#if, open_paren_char), expression), close_paren_char), statement), _t1): (
            (
                (
                    (
                        (
                            WithNoise<FixedTerminal<2usize>>,
                            WithNoise<FixedTerminal<1>>,
                        ),
                        Expression,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Statement,
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

impl return_statement::_T0 {
    pub fn new(
        ((r#return, expression), semicolon_char): (
            (WithNoise<FixedTerminal<6usize>>, Option<Expression>),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (((revert, expression), argument_list), semicolon_char): (
            ((WithNoise<FixedTerminal<6usize>>, Expression), ArgumentList),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new((equal_char, expression): (WithNoise<FixedTerminal<1>>, Expression)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl state_variable_declaration::_T0 {
    pub fn new(
        ((((type_name, state_variable_attributes), identifier), _t2), semicolon_char): (
            (
                ((TypeName, Vec<StateVariableAttribute>), Identifier),
                Option<Box<state_variable_declaration::_T2>>,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        (returns, non_empty_parameter_list): (
            WithNoise<FixedTerminal<7usize>>,
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
    pub fn new(
        ((((r#try, expression), _t1), block), catch_clauses): (
            (
                (
                    (WithNoise<FixedTerminal<3usize>>, Expression),
                    Option<Box<try_statement::_T1>>,
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

impl variable_declaration_statement::_T2 {
    pub fn new((equal_char, expression): (WithNoise<FixedTerminal<1>>, Expression)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl variable_declaration_statement::_T1 {
    pub fn new(
        (variable_declaration, _t2): (
            VariableDeclaration,
            Option<Box<variable_declaration_statement::_T2>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            _t2,
        }
    }
}
impl variable_declaration_statement::_T3 {
    pub fn new(
        ((variable_declaration_tuple, equal_char), expression): (
            (VariableDeclarationTuple, WithNoise<FixedTerminal<1>>),
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
    pub fn new(
        (variable_declaration_statement, semicolon_char): (
            Box<variable_declaration_statement::VariableDeclarationStatement>,
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            variable_declaration_statement,
            semicolon_char,
        }
    }
}

impl while_statement::_T0 {
    pub fn new(
        ((((r#while, open_paren_char), expression), close_paren_char), statement): (
            (
                (
                    (
                        WithNoise<FixedTerminal<5usize>>,
                        WithNoise<FixedTerminal<1>>,
                    ),
                    Expression,
                ),
                WithNoise<FixedTerminal<1>>,
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
                            (
                                WithNoise<FixedTerminal<3usize>>,
                                WithNoise<FixedTerminal<1>>,
                            ),
                            Box<for_statement::ForStatement>,
                        ),
                        Box<for_statement::ForStatement>,
                    ),
                    Option<Expression>,
                ),
                WithNoise<FixedTerminal<1>>,
            ),
            Statement,
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

impl block::_T0 {
    pub fn new(
        ((open_brace_char, blocks), close_brace_char): (
            (WithNoise<FixedTerminal<1>>, Vec<Box<block::Block>>),
            WithNoise<FixedTerminal<1>>,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            blocks,
            close_brace_char,
        }
    }
}

impl constructor_definition::_T0 {
    pub fn new(
        (((constructor, parameter_list), constructor_attributes), block): (
            (
                (WithNoise<FixedTerminal<11usize>>, ParameterList),
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
    pub fn new(
        (returns, non_empty_parameter_list): (
            WithNoise<FixedTerminal<7usize>>,
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
    pub fn new(
        (
            (((fallback, parameter_list), fallback_function_attributes), _t2),
            fallback_function_definition,
        ): (
            (
                (
                    (WithNoise<FixedTerminal<8usize>>, ParameterList),
                    Vec<FallbackFunctionAttribute>,
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

impl function_definition::_T2 {
    pub fn new(
        (returns, non_empty_parameter_list): (
            WithNoise<FixedTerminal<7usize>>,
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
    pub fn new(
        (
            ((((function, function_definition_1), parameter_list), function_attributes), _t2),
            function_definition_2,
        ): (
            (
                (
                    (
                        (
                            WithNoise<FixedTerminal<8usize>>,
                            Box<function_definition::FunctionDefinition>,
                        ),
                        ParameterList,
                    ),
                    Vec<FunctionAttribute>,
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

impl modifier_definition::_T0 {
    pub fn new(
        ((((modifier, identifier), parameter_list), method_attributes), modifier_definition): (
            (
                (
                    (WithNoise<FixedTerminal<8usize>>, Identifier),
                    Option<ParameterList>,
                ),
                Vec<MethodAttribute>,
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

impl receive_function_definition::_T0 {
    pub fn new(
        (
            (((receive, open_paren_char), close_paren_char), receive_function_attributes),
            receive_function_definition,
        ): (
            (
                (
                    (
                        WithNoise<FixedTerminal<7usize>>,
                        WithNoise<FixedTerminal<1>>,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Vec<ReceiveFunctionAttribute>,
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
                            (
                                Option<WithNoise<FixedTerminal<8usize>>>,
                                WithNoise<FixedTerminal<8usize>>,
                            ),
                            Identifier,
                        ),
                        Option<InheritanceSpecifierList>,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Vec<ContractBodyElement>,
            ),
            WithNoise<FixedTerminal<1>>,
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
                        (WithNoise<FixedTerminal<9usize>>, Identifier),
                        Option<InheritanceSpecifierList>,
                    ),
                    WithNoise<FixedTerminal<1>>,
                ),
                Vec<ContractBodyElement>,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((((library, identifier), open_brace_char), contract_body_elements), close_brace_char): (
            (
                (
                    (WithNoise<FixedTerminal<7usize>>, Identifier),
                    WithNoise<FixedTerminal<1>>,
                ),
                Vec<ContractBodyElement>,
            ),
            WithNoise<FixedTerminal<1>>,
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
    pub fn new(
        ((ignore, source_units), end_marker): ((Ignore, Vec<Box<source_unit::SourceUnit>>), ()),
    ) -> Self {
        Self {
            ignore,
            source_units,
            end_marker,
        }
    }
}
