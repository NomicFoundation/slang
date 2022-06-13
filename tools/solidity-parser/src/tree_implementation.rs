use super::tree_interface::*;
impl break_statement::_S0 {
    pub fn new(((r#break, ignore), semicolon_char): ((usize, ignore::N), char)) -> Self {
        Self {
            r#break,
            ignore,
            semicolon_char,
        }
    }
}
impl comment::_S0 {
    pub fn new(
        (((slash_star, _c2s), star_chars), star_slash): (
            ((usize, Vec<Box<comment::_C2>>), Vec<char>),
            usize,
        ),
    ) -> Self {
        Self {
            slash_star,
            _c2s,
            star_chars,
            star_slash,
        }
    }
}
impl comment::_S3 {
    pub fn new((star_chars, _1): (Vec<char>, char)) -> Self {
        Self { star_chars, _1 }
    }
}
impl continue_statement::_S0 {
    pub fn new(((r#continue, ignore), semicolon_char): ((usize, ignore::N), char)) -> Self {
        Self {
            r#continue,
            ignore,
            semicolon_char,
        }
    }
}
impl line_comment::_S0 {
    pub fn new((slash_slash, _1): (usize, Vec<char>)) -> Self {
        Self { slash_slash, _1 }
    }
}
impl positional_argument_list::_S0 {
    pub fn new(
        (_s1s, _s2s): (
            Vec<Box<positional_argument_list::_S1>>,
            Vec<Box<positional_argument_list::_S2>>,
        ),
    ) -> Self {
        Self { _s1s, _s2s }
    }
}
impl positional_argument_list::_S2 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl positional_argument_list::_S1 {
    pub fn new((expression, ignore): (expression::N, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}
impl unchecked_block::_S0 {
    pub fn new(((unchecked, ignore), block): ((usize, ignore::N), block::N)) -> Self {
        Self {
            unchecked,
            ignore,
            block,
        }
    }
}
impl decimal_integer::_S0 {
    pub fn new((expressions, underscore_chars): (Vec<char>, Vec<Option<char>>)) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}
impl fixed_bytes_type::_S0 {
    pub fn new((bytes, _1): (usize, usize)) -> Self {
        Self { bytes, _1 }
    }
}
impl fixed_type::_S0 {
    pub fn new((fixed, _s2): (usize, Option<Box<fixed_type::_S2>>)) -> Self {
        Self { fixed, _s2 }
    }
}
impl fixed_type::_S2 {
    pub fn new(((((_0, _1), _2), _3), _4): ((((char, Vec<char>), char), char), Vec<char>)) -> Self {
        Self { _0, _1, _2, _3, _4 }
    }
}
impl pragma_directive::_S0 {
    pub fn new(
        (((pragma, semicolon_char_0), semicolon_chars), semicolon_char_1): (
            ((usize, char), Vec<char>),
            char,
        ),
    ) -> Self {
        Self {
            pragma,
            semicolon_char_0,
            semicolon_chars,
            semicolon_char_1,
        }
    }
}
impl signed_integer_type::_S0 {
    pub fn new((int, _1): (usize, usize)) -> Self {
        Self { int, _1 }
    }
}
impl yul_decimal_number_literal::_S1 {
    pub fn new((_0, _1): (char, Vec<char>)) -> Self {
        Self { _0, _1 }
    }
}
impl yul_hex_literal::_S0 {
    pub fn new(((zero_x, _1), _2): ((usize, char), Vec<char>)) -> Self {
        Self { zero_x, _1, _2 }
    }
}
impl decimal_exponent::_S0 {
    pub fn new(
        ((_0, minus_char), decimal_integer): ((char, Option<char>), decimal_integer::N),
    ) -> Self {
        Self {
            _0,
            minus_char,
            decimal_integer,
        }
    }
}
impl decimal_float::_S0 {
    pub fn new(
        ((decimal_integer_0, period_char), decimal_integer_1): (
            (Option<decimal_integer::N>, char),
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
impl hex_byte_escape::_S0 {
    pub fn new((_0, _1): (char, Vec<char>)) -> Self {
        Self { _0, _1 }
    }
}
impl hex_number::_S0 {
    pub fn new(((zero_char, _1), _2): ((char, char), Box<hex_number::_S1>)) -> Self {
        Self { zero_char, _1, _2 }
    }
}
impl hex_number::_S1 {
    pub fn new((expressions, underscore_chars): (Vec<char>, Vec<Option<char>>)) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}
impl possibly_separated_pairs_of_hex_digits::_S0 {
    pub fn new((expressions, underscore_chars): (Vec<Vec<char>>, Vec<Option<char>>)) -> Self {
        Self {
            expressions,
            underscore_chars,
        }
    }
}
impl ufixed_type::_S0 {
    pub fn new((_0, fixed_type): (char, fixed_type::N)) -> Self {
        Self { _0, fixed_type }
    }
}
impl unicode_escape::_S0 {
    pub fn new((_0, _1): (char, Vec<char>)) -> Self {
        Self { _0, _1 }
    }
}
impl unsigned_integer_type::_S0 {
    pub fn new((_0, signed_integer_type): (char, signed_integer_type::N)) -> Self {
        Self {
            _0,
            signed_integer_type,
        }
    }
}
impl decimal_number::_S0 {
    pub fn new(
        (_c1, decimal_exponent): (Box<decimal_number::_C1>, Option<decimal_exponent::N>),
    ) -> Self {
        Self {
            _c1,
            decimal_exponent,
        }
    }
}
impl escape_sequence::_S0 {
    pub fn new((backslash_char, _c1): (char, Box<escape_sequence::_C1>)) -> Self {
        Self {
            backslash_char,
            _c1,
        }
    }
}
impl hex_string_literal::_S0 {
    pub fn new((hex, _c1): (usize, Box<hex_string_literal::_C1>)) -> Self {
        Self { hex, _c1 }
    }
}
impl hex_string_literal::_S4 {
    pub fn new(
        ((quote_char_0, possibly_separated_pairs_of_hex_digits), quote_char_1): (
            (char, Option<possibly_separated_pairs_of_hex_digits::N>),
            char,
        ),
    ) -> Self {
        Self {
            quote_char_0,
            possibly_separated_pairs_of_hex_digits,
            quote_char_1,
        }
    }
}
impl hex_string_literal::_S2 {
    pub fn new(
        ((double_quote_char_0, possibly_separated_pairs_of_hex_digits), double_quote_char_1): (
            (char, Option<possibly_separated_pairs_of_hex_digits::N>),
            char,
        ),
    ) -> Self {
        Self {
            double_quote_char_0,
            possibly_separated_pairs_of_hex_digits,
            double_quote_char_1,
        }
    }
}
impl raw_identifier::_S0 {
    pub fn new((_0, _1): (char, Vec<char>)) -> Self {
        Self { _0, _1 }
    }
}
impl double_quoted_ascii_string_literal::_S0 {
    pub fn new(
        ((double_quote_char_0, _c2s), double_quote_char_1): (
            (char, Vec<Box<double_quoted_ascii_string_literal::_C2>>),
            char,
        ),
    ) -> Self {
        Self {
            double_quote_char_0,
            _c2s,
            double_quote_char_1,
        }
    }
}
impl double_quoted_unicode_string_literal::_S0 {
    pub fn new(
        ((unicode_double_quote, _c2s), double_quote_char): (
            (usize, Vec<Box<double_quoted_unicode_string_literal::_C2>>),
            char,
        ),
    ) -> Self {
        Self {
            unicode_double_quote,
            _c2s,
            double_quote_char,
        }
    }
}
impl elementary_type_with_payable::_S1 {
    pub fn new(((address, ignore), payable): ((usize, ignore::N), Option<usize>)) -> Self {
        Self {
            address,
            ignore,
            payable,
        }
    }
}
impl numeric_literal::_S0 {
    pub fn new(
        ((_c1, ignore), _1): ((Box<numeric_literal::_C1>, ignore::N), Option<usize>),
    ) -> Self {
        Self { _c1, ignore, _1 }
    }
}
impl single_quoted_ascii_string_literal::_S0 {
    pub fn new(
        ((quote_char_0, _c2s), quote_char_1): (
            (char, Vec<Box<single_quoted_ascii_string_literal::_C2>>),
            char,
        ),
    ) -> Self {
        Self {
            quote_char_0,
            _c2s,
            quote_char_1,
        }
    }
}
impl single_quoted_unicode_string_literal::_S0 {
    pub fn new(
        ((unicode_quote, _c2s), quote_char): (
            (usize, Vec<Box<single_quoted_unicode_string_literal::_C2>>),
            char,
        ),
    ) -> Self {
        Self {
            unicode_quote,
            _c2s,
            quote_char,
        }
    }
}
impl assembly_flags::_S0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _s2s), ignore_1), close_paren_char): (
            (((char, ignore::N), Box<assembly_flags::_S1>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_paren_char,
        }
    }
}
impl assembly_flags::_S1 {
    pub fn new(
        (_s2s, _s3s): (Vec<Box<assembly_flags::_S2>>, Vec<Box<assembly_flags::_S3>>),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl assembly_flags::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl assembly_flags::_S2 {
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
impl yul_function_call::_S0 {
    pub fn new(
        ((((((_c1, ignore_0), open_paren_char), ignore_1), _s3s), ignore_2), close_paren_char): (
            (
                (
                    (((Box<yul_function_call::_C1>, ignore::N), char), ignore::N),
                    Option<Box<yul_function_call::_S2>>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            _c1,
            ignore_0,
            open_paren_char,
            ignore_1,
            _s3s,
            ignore_2,
            close_paren_char,
        }
    }
}
impl yul_function_call::_S2 {
    pub fn new(
        (_s3s, _s4s): (
            Vec<Box<yul_function_call::_S3>>,
            Vec<Box<yul_function_call::_S4>>,
        ),
    ) -> Self {
        Self { _s3s, _s4s }
    }
}
impl yul_function_call::_S4 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl yul_function_call::_S3 {
    pub fn new((yul_expression, ignore): (yul_expression::N, ignore::N)) -> Self {
        Self {
            yul_expression,
            ignore,
        }
    }
}
impl yul_function_definition::_S0 {
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
                                    _s2s,
                                ),
                                ignore_3,
                            ),
                            close_paren_char,
                        ),
                        ignore_4,
                    ),
                    _s5,
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
                                            (((usize, ignore::N), yul_identifier::N), ignore::N),
                                            char,
                                        ),
                                        ignore::N,
                                    ),
                                    Option<Box<yul_function_definition::_S1>>,
                                ),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    Option<Box<yul_function_definition::_S5>>,
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
            _s2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            _s5,
            ignore_5,
            yul_block,
        }
    }
}
impl yul_function_definition::_S5 {
    pub fn new(
        ((minus_greater, ignore), _s7s): ((usize, ignore::N), Box<yul_function_definition::_S6>),
    ) -> Self {
        Self {
            minus_greater,
            ignore,
            _s7s,
        }
    }
}
impl yul_function_definition::_S6 {
    pub fn new(
        (_s7s, _s8s): (
            Vec<Box<yul_function_definition::_S7>>,
            Vec<Box<yul_function_definition::_S8>>,
        ),
    ) -> Self {
        Self { _s7s, _s8s }
    }
}
impl yul_function_definition::_S8 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl yul_function_definition::_S7 {
    pub fn new((yul_identifier, ignore): (yul_identifier::N, ignore::N)) -> Self {
        Self {
            yul_identifier,
            ignore,
        }
    }
}
impl yul_function_definition::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<yul_function_definition::_S2>>,
            Vec<Box<yul_function_definition::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl yul_function_definition::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl yul_function_definition::_S2 {
    pub fn new((yul_identifier, ignore): (yul_identifier::N, ignore::N)) -> Self {
        Self {
            yul_identifier,
            ignore,
        }
    }
}
impl yul_path::_S0 {
    pub fn new(
        ((yul_identifier, ignore), _s2s): ((yul_identifier::N, ignore::N), Vec<Box<yul_path::_S2>>),
    ) -> Self {
        Self {
            yul_identifier,
            ignore,
            _s2s,
        }
    }
}
impl yul_path::_S2 {
    pub fn new(
        (((period_char, ignore_0), _c3), ignore_1): (
            ((char, ignore::N), Box<yul_path::_C3>),
            ignore::N,
        ),
    ) -> Self {
        Self {
            period_char,
            ignore_0,
            _c3,
            ignore_1,
        }
    }
}
impl enum_definition::_S0 {
    pub fn new(
        (
            (
                ((((((r#enum, ignore_0), identifier), ignore_1), open_brace_char), ignore_2), _s2s),
                ignore_3,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        ((((usize, ignore::N), identifier::N), ignore::N), char),
                        ignore::N,
                    ),
                    Box<enum_definition::_S1>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            r#enum,
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _s2s,
            ignore_3,
            close_brace_char,
        }
    }
}
impl enum_definition::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<enum_definition::_S2>>,
            Vec<Box<enum_definition::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl enum_definition::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl enum_definition::_S2 {
    pub fn new((identifier, ignore): (identifier::N, ignore::N)) -> Self {
        Self { identifier, ignore }
    }
}
impl identifier_path::_S0 {
    pub fn new(
        (_s1s, _s2s): (
            Vec<Box<identifier_path::_S1>>,
            Vec<Box<identifier_path::_S2>>,
        ),
    ) -> Self {
        Self { _s1s, _s2s }
    }
}
impl identifier_path::_S2 {
    pub fn new((period_char, ignore): (char, ignore::N)) -> Self {
        Self {
            period_char,
            ignore,
        }
    }
}
impl identifier_path::_S1 {
    pub fn new((identifier, ignore): (identifier::N, ignore::N)) -> Self {
        Self { identifier, ignore }
    }
}
impl named_argument::_S0 {
    pub fn new(
        ((((identifier, ignore_0), colon_char), ignore_1), expression): (
            (((identifier::N, ignore::N), char), ignore::N),
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
impl parameter_declaration::_S0 {
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
impl selected_import::_S0 {
    pub fn new(
        ((identifier, ignore), _s2): (
            (identifier::N, ignore::N),
            Option<Box<selected_import::_S2>>,
        ),
    ) -> Self {
        Self {
            identifier,
            ignore,
            _s2,
        }
    }
}
impl selected_import::_S2 {
    pub fn new(((r#as, ignore), identifier): ((usize, ignore::N), identifier::N)) -> Self {
        Self {
            r#as,
            ignore,
            identifier,
        }
    }
}
impl user_defined_value_type_definition::_S0 {
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
                        ((((usize, ignore::N), identifier::N), ignore::N), usize),
                        ignore::N,
                    ),
                    elementary_type_with_payable::N,
                ),
                ignore::N,
            ),
            char,
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
impl mapping_type::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((mapping, ignore_0), open_paren_char), ignore_1), _c1), ignore_2),
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
                                    (((usize, ignore::N), char), ignore::N),
                                    Box<mapping_type::_C1>,
                                ),
                                ignore::N,
                            ),
                            usize,
                        ),
                        ignore::N,
                    ),
                    type_name::N,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            mapping,
            ignore_0,
            open_paren_char,
            ignore_1,
            _c1,
            ignore_2,
            equal_greater,
            ignore_3,
            type_name,
            ignore_4,
            close_paren_char,
        }
    }
}
impl named_argument_list::_S0 {
    pub fn new(
        ((((open_brace_char, ignore_0), _s2s), ignore_1), close_brace_char): (
            (
                ((char, ignore::N), Option<Box<named_argument_list::_S1>>),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_brace_char,
        }
    }
}
impl named_argument_list::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<named_argument_list::_S2>>,
            Vec<Box<named_argument_list::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl named_argument_list::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl named_argument_list::_S2 {
    pub fn new((named_argument, ignore): (named_argument::N, ignore::N)) -> Self {
        Self {
            named_argument,
            ignore,
        }
    }
}
impl non_empty_parameter_list::_S0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _s2s), ignore_1), close_paren_char): (
            (
                ((char, ignore::N), Box<non_empty_parameter_list::_S1>),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_paren_char,
        }
    }
}
impl non_empty_parameter_list::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<non_empty_parameter_list::_S2>>,
            Vec<Box<non_empty_parameter_list::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl non_empty_parameter_list::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl non_empty_parameter_list::_S2 {
    pub fn new((parameter_declaration, ignore): (parameter_declaration::N, ignore::N)) -> Self {
        Self {
            parameter_declaration,
            ignore,
        }
    }
}
impl override_specifier::_S0 {
    pub fn new(
        ((r#override, ignore), _s2): ((usize, ignore::N), Option<Box<override_specifier::_S2>>),
    ) -> Self {
        Self {
            r#override,
            ignore,
            _s2,
        }
    }
}
impl override_specifier::_S2 {
    pub fn new(
        ((((open_paren_char, ignore_0), identifier_paths), ignore_1), close_paren_char): (
            (((char, ignore::N), Vec<identifier_path::N>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            identifier_paths,
            ignore_1,
            close_paren_char,
        }
    }
}
impl parameter_list::_S0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _s2s), ignore_1), close_paren_char): (
            (
                ((char, ignore::N), Option<Box<parameter_list::_S1>>),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_paren_char,
        }
    }
}
impl parameter_list::_S1 {
    pub fn new(
        (_s2s, _s3s): (Vec<Box<parameter_list::_S2>>, Vec<Box<parameter_list::_S3>>),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl parameter_list::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl parameter_list::_S2 {
    pub fn new((parameter_declaration, ignore): (parameter_declaration::N, ignore::N)) -> Self {
        Self {
            parameter_declaration,
            ignore,
        }
    }
}
impl selecting_import_directive::_S0 {
    pub fn new(
        (
            (
                (
                    (((((open_brace_char, ignore_0), _s2s), ignore_1), close_brace_char), ignore_2),
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
                                ((char, ignore::N), Box<selecting_import_directive::_S1>),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    usize,
                ),
                ignore::N,
            ),
            import_path::N,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_brace_char,
            ignore_2,
            from,
            ignore_3,
            import_path,
        }
    }
}
impl selecting_import_directive::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<selecting_import_directive::_S2>>,
            Vec<Box<selecting_import_directive::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl selecting_import_directive::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl selecting_import_directive::_S2 {
    pub fn new((selected_import, ignore): (selected_import::N, ignore::N)) -> Self {
        Self {
            selected_import,
            ignore,
        }
    }
}
impl simple_import_directive::_S0 {
    pub fn new(
        ((import_path, ignore), _s2s): (
            (import_path::N, ignore::N),
            Vec<Box<simple_import_directive::_S2>>,
        ),
    ) -> Self {
        Self {
            import_path,
            ignore,
            _s2s,
        }
    }
}
impl simple_import_directive::_S2 {
    pub fn new(
        (((r#as, ignore_0), identifier), ignore_1): (
            ((usize, ignore::N), identifier::N),
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
impl star_import_directive::_S0 {
    pub fn new(
        (
            (((((((star_char, ignore_0), r#as), ignore_1), identifier), ignore_2), from), ignore_3),
            import_path,
        ): (
            (
                (
                    (
                        ((((char, ignore::N), usize), ignore::N), identifier::N),
                        ignore::N,
                    ),
                    usize,
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
impl argument_list::_S0 {
    pub fn new(
        ((((open_paren_char, ignore_0), _c2), ignore_1), close_paren_char): (
            (
                ((char, ignore::N), Option<Box<argument_list::_C2>>),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _c2,
            ignore_1,
            close_paren_char,
        }
    }
}
impl catch_clause::_S0 {
    pub fn new(
        ((((catch, ignore_0), _s2), ignore_1), block): (
            (
                ((usize, ignore::N), Option<Box<catch_clause::_S2>>),
                ignore::N,
            ),
            block::N,
        ),
    ) -> Self {
        Self {
            catch,
            ignore_0,
            _s2,
            ignore_1,
            block,
        }
    }
}
impl catch_clause::_S2 {
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
impl function_type::_S0 {
    pub fn new(
        ((((((function, ignore_0), parameter_list), ignore_1), _s2s), ignore_2), _s4): (
            (
                (
                    (((usize, ignore::N), parameter_list::N), ignore::N),
                    Vec<Box<function_type::_S2>>,
                ),
                ignore::N,
            ),
            Option<Box<function_type::_S4>>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_0,
            parameter_list,
            ignore_1,
            _s2s,
            ignore_2,
            _s4,
        }
    }
}
impl function_type::_S4 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (usize, ignore::N),
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
impl function_type::_S2 {
    pub fn new((_0, ignore): (usize, ignore::N)) -> Self {
        Self { _0, ignore }
    }
}
impl import_directive::_S0 {
    pub fn new(
        ((((import, ignore_0), _c1), ignore_1), semicolon_char): (
            (((usize, ignore::N), Box<import_directive::_C1>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            import,
            ignore_0,
            _c1,
            ignore_1,
            semicolon_char,
        }
    }
}
impl yul_assignment::_S0 {
    pub fn new(
        ((yul_path, ignore), _c1): ((yul_path::N, ignore::N), Box<yul_assignment::_C1>),
    ) -> Self {
        Self {
            yul_path,
            ignore,
            _c1,
        }
    }
}
impl yul_assignment::_S3 {
    pub fn new(
        ((((_s5s, ignore_0), colon_equal), ignore_1), yul_function_call): (
            (
                ((Vec<Box<yul_assignment::_S5>>, ignore::N), usize),
                ignore::N,
            ),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            _s5s,
            ignore_0,
            colon_equal,
            ignore_1,
            yul_function_call,
        }
    }
}
impl yul_assignment::_S5 {
    pub fn new(
        (((comma_char, ignore_0), yul_path), ignore_1): (
            ((char, ignore::N), yul_path::N),
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
impl yul_assignment::_S2 {
    pub fn new(
        ((colon_equal, ignore), yul_expression): ((usize, ignore::N), yul_expression::N),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
            yul_expression,
        }
    }
}
impl yul_for_statement::_S0 {
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
                            (((usize, ignore::N), yul_block::N), ignore::N),
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
impl yul_if_statement::_S0 {
    pub fn new(
        ((((r#if, ignore_0), yul_expression), ignore_1), yul_block): (
            (((usize, ignore::N), yul_expression::N), ignore::N),
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
impl yul_switch_statement::_S0 {
    pub fn new(
        ((((switch, ignore_0), yul_expression), ignore_1), _c1): (
            (((usize, ignore::N), yul_expression::N), ignore::N),
            Box<yul_switch_statement::_C1>,
        ),
    ) -> Self {
        Self {
            switch,
            ignore_0,
            yul_expression,
            ignore_1,
            _c1,
        }
    }
}
impl yul_switch_statement::_S7 {
    pub fn new(((default, ignore), yul_block): ((usize, ignore::N), yul_block::N)) -> Self {
        Self {
            default,
            ignore,
            yul_block,
        }
    }
}
impl yul_switch_statement::_S2 {
    pub fn new(
        ((_s4s, ignore), _s6): (
            (Vec<Box<yul_switch_statement::_S4>>, ignore::N),
            Option<Box<yul_switch_statement::_S6>>,
        ),
    ) -> Self {
        Self { _s4s, ignore, _s6 }
    }
}
impl yul_switch_statement::_S6 {
    pub fn new(((default, ignore), yul_block): ((usize, ignore::N), yul_block::N)) -> Self {
        Self {
            default,
            ignore,
            yul_block,
        }
    }
}
impl yul_switch_statement::_S4 {
    pub fn new(
        (((((case, ignore_0), yul_literal), ignore_1), yul_block), ignore): (
            (
                (((usize, ignore::N), yul_literal::N), ignore::N),
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
impl yul_variable_declaration::_S0 {
    pub fn new(
        ((((r#let, ignore_0), yul_identifier), ignore_1), _c2): (
            (((usize, ignore::N), yul_identifier::N), ignore::N),
            Option<Box<yul_variable_declaration::_C2>>,
        ),
    ) -> Self {
        Self {
            r#let,
            ignore_0,
            yul_identifier,
            ignore_1,
            _c2,
        }
    }
}
impl yul_variable_declaration::_S4 {
    pub fn new(
        ((_s6, ignore), _s8): (
            (Option<Box<yul_variable_declaration::_S6>>, ignore::N),
            Option<Box<yul_variable_declaration::_S8>>,
        ),
    ) -> Self {
        Self { _s6, ignore, _s8 }
    }
}
impl yul_variable_declaration::_S8 {
    pub fn new(
        ((colon_equal, ignore), yul_function_call): ((usize, ignore::N), yul_function_call::N),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
            yul_function_call,
        }
    }
}
impl yul_variable_declaration::_S6 {
    pub fn new(
        ((comma_char, ignore), yul_identifier): ((char, ignore::N), yul_identifier::N),
    ) -> Self {
        Self {
            comma_char,
            ignore,
            yul_identifier,
        }
    }
}
impl yul_variable_declaration::_S3 {
    pub fn new(
        ((colon_equal, ignore), yul_expression): ((usize, ignore::N), yul_expression::N),
    ) -> Self {
        Self {
            colon_equal,
            ignore,
            yul_expression,
        }
    }
}
impl inheritance_specifier::_S0 {
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
impl modifier_invocation::_S0 {
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
impl type_name::_S0 {
    pub fn new(
        ((_c1, ignore), _s3s): ((Box<type_name::_C1>, ignore::N), Vec<Box<type_name::_S3>>),
    ) -> Self {
        Self { _c1, ignore, _s3s }
    }
}
impl type_name::_S3 {
    pub fn new(
        (((((open_bracket_char, ignore_0), expression), ignore_1), close_bracket_char), ignore): (
            (
                (((char, ignore::N), Option<expression::N>), ignore::N),
                char,
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
impl error_parameter::_S0 {
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
impl event_parameter::_S0 {
    pub fn new(
        ((((type_name, ignore_0), indexed), ignore_1), identifier): (
            (((type_name::N, ignore::N), Option<usize>), ignore::N),
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
impl inheritance_specifier_list::_S0 {
    pub fn new(
        ((is, ignore), _s2s): ((usize, ignore::N), Box<inheritance_specifier_list::_S1>),
    ) -> Self {
        Self { is, ignore, _s2s }
    }
}
impl inheritance_specifier_list::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<inheritance_specifier_list::_S2>>,
            Vec<Box<inheritance_specifier_list::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl inheritance_specifier_list::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl inheritance_specifier_list::_S2 {
    pub fn new((inheritance_specifier, ignore): (inheritance_specifier::N, ignore::N)) -> Self {
        Self {
            inheritance_specifier,
            ignore,
        }
    }
}
impl primary_expression::_S9 {
    pub fn new(
        ((((open_bracket_char, ignore_0), _s11s), ignore_1), close_bracket_char): (
            (
                ((char, ignore::N), Box<primary_expression::_S10>),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            _s11s,
            ignore_1,
            close_bracket_char,
        }
    }
}
impl primary_expression::_S10 {
    pub fn new(
        (_s11s, _s12s): (
            Vec<Box<primary_expression::_S11>>,
            Vec<Box<primary_expression::_S12>>,
        ),
    ) -> Self {
        Self { _s11s, _s12s }
    }
}
impl primary_expression::_S12 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl primary_expression::_S11 {
    pub fn new((expression, ignore): (expression::N, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}
impl primary_expression::_S4 {
    pub fn new(
        ((((open_paren_char, ignore_0), _s7s), ignore_1), close_paren_char): (
            (((char, ignore::N), Box<primary_expression::_S5>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _s7s,
            ignore_1,
            close_paren_char,
        }
    }
}
impl primary_expression::_S5 {
    pub fn new(
        (_s7s, _s8s): (
            Vec<Box<primary_expression::_S7>>,
            Vec<Box<primary_expression::_S8>>,
        ),
    ) -> Self {
        Self { _s7s, _s8s }
    }
}
impl primary_expression::_S8 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl primary_expression::_S7 {
    pub fn new((expression, ignore): (Option<expression::N>, ignore::N)) -> Self {
        Self { expression, ignore }
    }
}
impl primary_expression::_S3 {
    pub fn new(((new, ignore), type_name): ((usize, ignore::N), type_name::N)) -> Self {
        Self {
            new,
            ignore,
            type_name,
        }
    }
}
impl primary_expression::_S2 {
    pub fn new(
        (
            (((((r#type, ignore_0), open_paren_char), ignore_1), type_name), ignore_2),
            close_paren_char,
        ): (
            (
                ((((usize, ignore::N), char), ignore::N), type_name::N),
                ignore::N,
            ),
            char,
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
impl primary_expression::_S1 {
    pub fn new(((payable, ignore), argument_list): ((usize, ignore::N), argument_list::N)) -> Self {
        Self {
            payable,
            ignore,
            argument_list,
        }
    }
}
impl struct_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (((((r#struct, ignore_0), identifier), ignore_1), open_brace_char), ignore_2),
                    _s2s,
                ),
                ignore_3,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        ((((usize, ignore::N), identifier::N), ignore::N), char),
                        ignore::N,
                    ),
                    Vec<Box<struct_definition::_S2>>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            r#struct,
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _s2s,
            ignore_3,
            close_brace_char,
        }
    }
}
impl struct_definition::_S2 {
    pub fn new(
        (((((type_name, ignore_0), identifier), ignore_1), semicolon_char), ignore): (
            (
                (((type_name::N, ignore::N), identifier::N), ignore::N),
                char,
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
impl using_directive::_S0 {
    pub fn new(
        (
            (
                (
                    (((((((using, ignore_0), _c1), ignore_1), r#for), ignore_2), _c6), ignore_3),
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
                                    (((usize, ignore::N), Box<using_directive::_C1>), ignore::N),
                                    usize,
                                ),
                                ignore::N,
                            ),
                            Box<using_directive::_C6>,
                        ),
                        ignore::N,
                    ),
                    Option<usize>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            using,
            ignore_0,
            _c1,
            ignore_1,
            r#for,
            ignore_2,
            _c6,
            ignore_3,
            global,
            ignore_4,
            semicolon_char,
        }
    }
}
impl using_directive::_S2 {
    pub fn new(
        ((((open_brace_char, ignore_0), _s4s), ignore_1), close_brace_char): (
            (((char, ignore::N), Box<using_directive::_S3>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s4s,
            ignore_1,
            close_brace_char,
        }
    }
}
impl using_directive::_S3 {
    pub fn new(
        (_s4s, _s5s): (
            Vec<Box<using_directive::_S4>>,
            Vec<Box<using_directive::_S5>>,
        ),
    ) -> Self {
        Self { _s4s, _s5s }
    }
}
impl using_directive::_S5 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl using_directive::_S4 {
    pub fn new((identifier_path, ignore): (identifier_path::N, ignore::N)) -> Self {
        Self {
            identifier_path,
            ignore,
        }
    }
}
impl variable_declaration::_S0 {
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
impl yul_block::_S0 {
    pub fn new(
        ((((open_brace_char, ignore_0), _s2s), ignore_1), close_brace_char): (
            (((char, ignore::N), Vec<Box<yul_block::_S2>>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s2s,
            ignore_1,
            close_brace_char,
        }
    }
}
impl yul_block::_S2 {
    pub fn new((yul_statement, ignore): (yul_statement::N, ignore::N)) -> Self {
        Self {
            yul_statement,
            ignore,
        }
    }
}
impl assembly_statement::_S0 {
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
                    (((usize, ignore::N), Option<usize>), ignore::N),
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
impl error_definition::_S0 {
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
                            _s2s,
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
                                ((((usize, ignore::N), identifier::N), ignore::N), char),
                                ignore::N,
                            ),
                            Option<Box<error_definition::_S1>>,
                        ),
                        ignore::N,
                    ),
                    char,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            error,
            ignore_0,
            identifier,
            ignore_1,
            open_paren_char,
            ignore_2,
            _s2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            semicolon_char,
        }
    }
}
impl error_definition::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<error_definition::_S2>>,
            Vec<Box<error_definition::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl error_definition::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl error_definition::_S2 {
    pub fn new((error_parameter, ignore): (error_parameter::N, ignore::N)) -> Self {
        Self {
            error_parameter,
            ignore,
        }
    }
}
impl event_definition::_S0 {
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
                                    _s2s,
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
                                        ((((usize, ignore::N), identifier::N), ignore::N), char),
                                        ignore::N,
                                    ),
                                    Option<Box<event_definition::_S1>>,
                                ),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    Option<usize>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            event,
            ignore_0,
            identifier,
            ignore_1,
            open_paren_char,
            ignore_2,
            _s2s,
            ignore_3,
            close_paren_char,
            ignore_4,
            anonymous,
            ignore_5,
            semicolon_char,
        }
    }
}
impl event_definition::_S1 {
    pub fn new(
        (_s2s, _s3s): (
            Vec<Box<event_definition::_S2>>,
            Vec<Box<event_definition::_S3>>,
        ),
    ) -> Self {
        Self { _s2s, _s3s }
    }
}
impl event_definition::_S3 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl event_definition::_S2 {
    pub fn new((event_parameter, ignore): (event_parameter::N, ignore::N)) -> Self {
        Self {
            event_parameter,
            ignore,
        }
    }
}
impl index_access_expression::_S0 {
    pub fn new(
        ((primary_expression, ignore), _s2s): (
            (primary_expression::N, ignore::N),
            Vec<Box<index_access_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            primary_expression,
            ignore,
            _s2s,
        }
    }
}
impl index_access_expression::_S2 {
    pub fn new(
        (
            (
                (((((open_bracket_char, ignore_0), expression), ignore_1), _s5), ignore_2),
                close_bracket_char,
            ),
            ignore,
        ): (
            (
                (
                    (
                        (((char, ignore::N), Option<expression::N>), ignore::N),
                        Option<Box<index_access_expression::_S5>>,
                    ),
                    ignore::N,
                ),
                char,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            ignore_0,
            expression,
            ignore_1,
            _s5,
            ignore_2,
            close_bracket_char,
            ignore,
        }
    }
}
impl index_access_expression::_S5 {
    pub fn new(
        ((colon_char, ignore), expression): ((char, ignore::N), Option<expression::N>),
    ) -> Self {
        Self {
            colon_char,
            ignore,
            expression,
        }
    }
}
impl variable_declaration_tuple::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((open_paren_char, ignore_0), _s2s), ignore_1), variable_declaration),
                        ignore_2,
                    ),
                    _s4s,
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
                                ((char, ignore::N), Vec<Box<variable_declaration_tuple::_S2>>),
                                ignore::N,
                            ),
                            variable_declaration::N,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<variable_declaration_tuple::_S4>>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            ignore_0,
            _s2s,
            ignore_1,
            variable_declaration,
            ignore_2,
            _s4s,
            ignore_3,
            close_paren_char,
        }
    }
}
impl variable_declaration_tuple::_S4 {
    pub fn new(
        (((comma_char, ignore_0), variable_declaration), ignore_1): (
            ((char, ignore::N), Option<variable_declaration::N>),
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
impl variable_declaration_tuple::_S2 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl member_access_expression::_S0 {
    pub fn new(
        ((index_access_expression, ignore), _s2s): (
            (index_access_expression::N, ignore::N),
            Vec<Box<member_access_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            index_access_expression,
            ignore,
            _s2s,
        }
    }
}
impl member_access_expression::_S2 {
    pub fn new(
        (((period_char, ignore_0), _c3), ignore_1): (
            ((char, ignore::N), Box<member_access_expression::_C3>),
            ignore::N,
        ),
    ) -> Self {
        Self {
            period_char,
            ignore_0,
            _c3,
            ignore_1,
        }
    }
}
impl function_call_options_expression::_S0 {
    pub fn new(
        ((member_access_expression, ignore), _s2s): (
            (member_access_expression::N, ignore::N),
            Vec<Box<function_call_options_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            member_access_expression,
            ignore,
            _s2s,
        }
    }
}
impl function_call_options_expression::_S2 {
    pub fn new(
        (((((open_brace_char, ignore_0), _s4s), ignore_1), close_brace_char), ignore): (
            (
                (
                    (
                        (char, ignore::N),
                        Box<function_call_options_expression::_S3>,
                    ),
                    ignore::N,
                ),
                char,
            ),
            ignore::N,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s4s,
            ignore_1,
            close_brace_char,
            ignore,
        }
    }
}
impl function_call_options_expression::_S3 {
    pub fn new(
        (_s4s, _s5s): (
            Vec<Box<function_call_options_expression::_S4>>,
            Vec<Box<function_call_options_expression::_S5>>,
        ),
    ) -> Self {
        Self { _s4s, _s5s }
    }
}
impl function_call_options_expression::_S5 {
    pub fn new((comma_char, ignore): (char, ignore::N)) -> Self {
        Self { comma_char, ignore }
    }
}
impl function_call_options_expression::_S4 {
    pub fn new((named_argument, ignore): (named_argument::N, ignore::N)) -> Self {
        Self {
            named_argument,
            ignore,
        }
    }
}
impl function_call_expression::_S0 {
    pub fn new(
        ((function_call_options_expression, ignore), _s2s): (
            (function_call_options_expression::N, ignore::N),
            Vec<Box<function_call_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            function_call_options_expression,
            ignore,
            _s2s,
        }
    }
}
impl function_call_expression::_S2 {
    pub fn new((argument_list, ignore): (argument_list::N, ignore::N)) -> Self {
        Self {
            argument_list,
            ignore,
        }
    }
}
impl unary_prefix_expression::_S0 {
    pub fn new(
        ((_0, ignore), function_call_expression): ((usize, ignore::N), function_call_expression::N),
    ) -> Self {
        Self {
            _0,
            ignore,
            function_call_expression,
        }
    }
}
impl unary_suffix_expression::_S0 {
    pub fn new(
        ((unary_prefix_expression, ignore), _1): ((unary_prefix_expression::N, ignore::N), usize),
    ) -> Self {
        Self {
            unary_prefix_expression,
            ignore,
            _1,
        }
    }
}
impl exp_expression::_S0 {
    pub fn new(
        ((((unary_suffix_expression, ignore_0), star_star), ignore_1), expression): (
            (((unary_suffix_expression::N, ignore::N), usize), ignore::N),
            expression::N,
        ),
    ) -> Self {
        Self {
            unary_suffix_expression,
            ignore_0,
            star_star,
            ignore_1,
            expression,
        }
    }
}
impl mul_div_mod_expression::_S0 {
    pub fn new(
        ((exp_expression, ignore), _s2s): (
            (exp_expression::N, ignore::N),
            Vec<Box<mul_div_mod_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            exp_expression,
            ignore,
            _s2s,
        }
    }
}
impl mul_div_mod_expression::_S2 {
    pub fn new(
        (((_0, ignore_0), exp_expression), ignore_1): (
            ((char, ignore::N), exp_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore_0,
            exp_expression,
            ignore_1,
        }
    }
}
impl add_sub_expression::_S0 {
    pub fn new(
        ((mul_div_mod_expression, ignore), _s2s): (
            (mul_div_mod_expression::N, ignore::N),
            Vec<Box<add_sub_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            mul_div_mod_expression,
            ignore,
            _s2s,
        }
    }
}
impl add_sub_expression::_S2 {
    pub fn new(
        (((_0, ignore_0), mul_div_mod_expression), ignore_1): (
            ((char, ignore::N), mul_div_mod_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore_0,
            mul_div_mod_expression,
            ignore_1,
        }
    }
}
impl shift_expression::_S0 {
    pub fn new(
        ((add_sub_expression, ignore), _s2s): (
            (add_sub_expression::N, ignore::N),
            Vec<Box<shift_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            add_sub_expression,
            ignore,
            _s2s,
        }
    }
}
impl shift_expression::_S2 {
    pub fn new(
        (((_0, ignore_0), add_sub_expression), ignore_1): (
            ((usize, ignore::N), add_sub_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore_0,
            add_sub_expression,
            ignore_1,
        }
    }
}
impl bit_and_expression::_S0 {
    pub fn new(
        ((shift_expression, ignore), _s2s): (
            (shift_expression::N, ignore::N),
            Vec<Box<bit_and_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            shift_expression,
            ignore,
            _s2s,
        }
    }
}
impl bit_and_expression::_S2 {
    pub fn new(
        (((ampersand_char, ignore_0), shift_expression), ignore_1): (
            ((char, ignore::N), shift_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            ampersand_char,
            ignore_0,
            shift_expression,
            ignore_1,
        }
    }
}
impl bit_x_or_expression::_S0 {
    pub fn new(
        ((bit_and_expression, ignore), _s2s): (
            (bit_and_expression::N, ignore::N),
            Vec<Box<bit_x_or_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            bit_and_expression,
            ignore,
            _s2s,
        }
    }
}
impl bit_x_or_expression::_S2 {
    pub fn new(
        (((caret_char, ignore_0), bit_and_expression), ignore_1): (
            ((char, ignore::N), bit_and_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            caret_char,
            ignore_0,
            bit_and_expression,
            ignore_1,
        }
    }
}
impl bit_or_expression::_S0 {
    pub fn new(
        ((bit_x_or_expression, ignore), _s2s): (
            (bit_x_or_expression::N, ignore::N),
            Vec<Box<bit_or_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            bit_x_or_expression,
            ignore,
            _s2s,
        }
    }
}
impl bit_or_expression::_S2 {
    pub fn new(
        (((bar_char, ignore_0), bit_x_or_expression), ignore_1): (
            ((char, ignore::N), bit_x_or_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            bar_char,
            ignore_0,
            bit_x_or_expression,
            ignore_1,
        }
    }
}
impl order_comparison_expression::_S0 {
    pub fn new(
        ((bit_or_expression, ignore), _s2s): (
            (bit_or_expression::N, ignore::N),
            Vec<Box<order_comparison_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            bit_or_expression,
            ignore,
            _s2s,
        }
    }
}
impl order_comparison_expression::_S2 {
    pub fn new(
        (((_0, ignore_0), bit_or_expression), ignore_1): (
            ((usize, ignore::N), bit_or_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore_0,
            bit_or_expression,
            ignore_1,
        }
    }
}
impl equality_comparison_expression::_S0 {
    pub fn new(
        ((order_comparison_expression, ignore), _s2s): (
            (order_comparison_expression::N, ignore::N),
            Vec<Box<equality_comparison_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            order_comparison_expression,
            ignore,
            _s2s,
        }
    }
}
impl equality_comparison_expression::_S2 {
    pub fn new(
        (((_0, ignore_0), order_comparison_expression), ignore_1): (
            ((usize, ignore::N), order_comparison_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            _0,
            ignore_0,
            order_comparison_expression,
            ignore_1,
        }
    }
}
impl and_expression::_S0 {
    pub fn new(
        ((equality_comparison_expression, ignore), _s2s): (
            (equality_comparison_expression::N, ignore::N),
            Vec<Box<and_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            equality_comparison_expression,
            ignore,
            _s2s,
        }
    }
}
impl and_expression::_S2 {
    pub fn new(
        (((ampersand_ampersand, ignore_0), equality_comparison_expression), ignore_1): (
            ((usize, ignore::N), equality_comparison_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            ampersand_ampersand,
            ignore_0,
            equality_comparison_expression,
            ignore_1,
        }
    }
}
impl or_expression::_S0 {
    pub fn new(
        ((and_expression, ignore), _s2s): (
            (and_expression::N, ignore::N),
            Vec<Box<or_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            and_expression,
            ignore,
            _s2s,
        }
    }
}
impl or_expression::_S2 {
    pub fn new(
        (((bar_bar, ignore_0), and_expression), ignore_1): (
            ((usize, ignore::N), and_expression::N),
            ignore::N,
        ),
    ) -> Self {
        Self {
            bar_bar,
            ignore_0,
            and_expression,
            ignore_1,
        }
    }
}
impl conditional_expression::_S0 {
    pub fn new(
        ((or_expression, ignore), _s2): (
            (or_expression::N, ignore::N),
            Option<Box<conditional_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            or_expression,
            ignore,
            _s2,
        }
    }
}
impl conditional_expression::_S2 {
    pub fn new(
        (
            (((((question_char, ignore_0), expression_0), ignore_1), colon_char), ignore_2),
            expression_1,
        ): (
            (
                ((((char, ignore::N), expression::N), ignore::N), char),
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
impl assignment_expression::_S0 {
    pub fn new(
        ((((conditional_expression, ignore_0), _1), ignore_1), expression): (
            (((conditional_expression::N, ignore::N), usize), ignore::N),
            expression::N,
        ),
    ) -> Self {
        Self {
            conditional_expression,
            ignore_0,
            _1,
            ignore_1,
            expression,
        }
    }
}
impl constant_definition::_S0 {
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
                                    (((type_name::N, ignore::N), usize), ignore::N),
                                    identifier::N,
                                ),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    expression::N,
                ),
                ignore::N,
            ),
            char,
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
impl do_while_statement::_S0 {
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
                                        ((((usize, ignore::N), statement::N), ignore::N), usize),
                                        ignore::N,
                                    ),
                                    char,
                                ),
                                ignore::N,
                            ),
                            expression::N,
                        ),
                        ignore::N,
                    ),
                    char,
                ),
                ignore::N,
            ),
            char,
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
impl emit_statement::_S0 {
    pub fn new(
        ((((((emit, ignore_0), expression), ignore_1), argument_list), ignore_2), semicolon_char): (
            (
                (
                    (((usize, ignore::N), expression::N), ignore::N),
                    argument_list::N,
                ),
                ignore::N,
            ),
            char,
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
impl expression_statement::_S0 {
    pub fn new(((expression, ignore), semicolon_char): ((expression::N, ignore::N), char)) -> Self {
        Self {
            expression,
            ignore,
            semicolon_char,
        }
    }
}
impl if_statement::_S0 {
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
            _s2,
        ): (
            (
                (
                    (
                        (
                            (
                                ((((usize, ignore::N), char), ignore::N), expression::N),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    statement::N,
                ),
                ignore::N,
            ),
            Option<Box<if_statement::_S2>>,
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
            _s2,
        }
    }
}
impl if_statement::_S2 {
    pub fn new(((r#else, ignore), statement): ((usize, ignore::N), statement::N)) -> Self {
        Self {
            r#else,
            ignore,
            statement,
        }
    }
}
impl return_statement::_S0 {
    pub fn new(
        ((((r#return, ignore_0), expression), ignore_1), semicolon_char): (
            (((usize, ignore::N), Option<expression::N>), ignore::N),
            char,
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
impl revert_statement::_S0 {
    pub fn new(
        ((((((revert , ignore_0) , expression) , ignore_1) , argument_list) , ignore_2) , semicolon_char) : ((((((usize , ignore :: N) , expression :: N) , ignore :: N) , argument_list :: N) , ignore :: N) , char),
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
impl state_variable_declaration::_S0 {
    pub fn new(
        (
            (((((((type_name, ignore_0), _s2s), ignore_1), identifier), ignore_2), _s4), ignore_3),
            semicolon_char,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (type_name::N, ignore::N),
                                    Vec<Box<state_variable_declaration::_S2>>,
                                ),
                                ignore::N,
                            ),
                            identifier::N,
                        ),
                        ignore::N,
                    ),
                    Option<Box<state_variable_declaration::_S4>>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            type_name,
            ignore_0,
            _s2s,
            ignore_1,
            identifier,
            ignore_2,
            _s4,
            ignore_3,
            semicolon_char,
        }
    }
}
impl state_variable_declaration::_S4 {
    pub fn new(((equal_char, ignore), expression): ((char, ignore::N), expression::N)) -> Self {
        Self {
            equal_char,
            ignore,
            expression,
        }
    }
}
impl state_variable_declaration::_S2 {
    pub fn new(
        (state_variable_attribute, ignore): (state_variable_attribute::N, ignore::N),
    ) -> Self {
        Self {
            state_variable_attribute,
            ignore,
        }
    }
}
impl try_statement::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((((r#try, ignore_0), expression), ignore_1), _s2), ignore_2), block),
                        ignore_3,
                    ),
                    catch_clause,
                ),
                ignore_4,
            ),
            _s4s,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (((usize, ignore::N), expression::N), ignore::N),
                                    Option<Box<try_statement::_S2>>,
                                ),
                                ignore::N,
                            ),
                            block::N,
                        ),
                        ignore::N,
                    ),
                    catch_clause::N,
                ),
                ignore::N,
            ),
            Vec<Box<try_statement::_S4>>,
        ),
    ) -> Self {
        Self {
            r#try,
            ignore_0,
            expression,
            ignore_1,
            _s2,
            ignore_2,
            block,
            ignore_3,
            catch_clause,
            ignore_4,
            _s4s,
        }
    }
}
impl try_statement::_S4 {
    pub fn new((catch_clause, ignore): (catch_clause::N, ignore::N)) -> Self {
        Self {
            catch_clause,
            ignore,
        }
    }
}
impl try_statement::_S2 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (usize, ignore::N),
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
impl variable_declaration_statement::_S0 {
    pub fn new(
        ((_c1, ignore), semicolon_char): (
            (Box<variable_declaration_statement::_C1>, ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            _c1,
            ignore,
            semicolon_char,
        }
    }
}
impl variable_declaration_statement::_S5 {
    pub fn new(
        ((((variable_declaration_tuple, ignore_0), equal_char), ignore_1), expression): (
            (
                ((variable_declaration_tuple::N, ignore::N), char),
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
impl variable_declaration_statement::_S2 {
    pub fn new(
        ((variable_declaration, ignore), _s4): (
            (variable_declaration::N, ignore::N),
            Option<Box<variable_declaration_statement::_S4>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            ignore,
            _s4,
        }
    }
}
impl variable_declaration_statement::_S4 {
    pub fn new(((equal_char, ignore), expression): ((char, ignore::N), expression::N)) -> Self {
        Self {
            equal_char,
            ignore,
            expression,
        }
    }
}
impl while_statement::_S0 {
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
                        ((((usize, ignore::N), char), ignore::N), expression::N),
                        ignore::N,
                    ),
                    char,
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
impl for_statement::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((((r#for, ignore_0), open_paren_char), ignore_1), _c1),
                                        ignore_2,
                                    ),
                                    _c2,
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
                                            (((usize, ignore::N), char), ignore::N),
                                            Box<for_statement::_C1>,
                                        ),
                                        ignore::N,
                                    ),
                                    Box<for_statement::_C2>,
                                ),
                                ignore::N,
                            ),
                            Option<expression::N>,
                        ),
                        ignore::N,
                    ),
                    char,
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
            _c1,
            ignore_2,
            _c2,
            ignore_3,
            expression,
            ignore_4,
            close_paren_char,
            ignore_5,
            statement,
        }
    }
}
impl block::_S0 {
    pub fn new(
        ((((open_brace_char, ignore_0), _s3s), ignore_1), close_brace_char): (
            (((char, ignore::N), Vec<Box<block::_S3>>), ignore::N),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            ignore_0,
            _s3s,
            ignore_1,
            close_brace_char,
        }
    }
}
impl block::_S3 {
    pub fn new((_c2, ignore): (Box<block::_C2>, ignore::N)) -> Self {
        Self { _c2, ignore }
    }
}
impl constructor_definition::_S0 {
    pub fn new(
        ((((((constructor, ignore_0), parameter_list), ignore_1), _s2s), ignore_2), block): (
            (
                (
                    (((usize, ignore::N), parameter_list::N), ignore::N),
                    Vec<Box<constructor_definition::_S2>>,
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
            _s2s,
            ignore_2,
            block,
        }
    }
}
impl constructor_definition::_S2 {
    pub fn new((constructor_attribute, ignore): (constructor_attribute::N, ignore::N)) -> Self {
        Self {
            constructor_attribute,
            ignore,
        }
    }
}
impl fallback_function_definition::_S0 {
    pub fn new(
        (
            (
                ((((((fallback, ignore_0), parameter_list), ignore_1), _s2s), ignore_2), _s4),
                ignore_3,
            ),
            _c5,
        ): (
            (
                (
                    (
                        (
                            (((usize, ignore::N), parameter_list::N), ignore::N),
                            Vec<Box<fallback_function_definition::_S2>>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<fallback_function_definition::_S4>>,
                ),
                ignore::N,
            ),
            Box<fallback_function_definition::_C5>,
        ),
    ) -> Self {
        Self {
            fallback,
            ignore_0,
            parameter_list,
            ignore_1,
            _s2s,
            ignore_2,
            _s4,
            ignore_3,
            _c5,
        }
    }
}
impl fallback_function_definition::_S4 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (usize, ignore::N),
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
impl fallback_function_definition::_S2 {
    pub fn new(
        (fallback_function_attribute, ignore): (fallback_function_attribute::N, ignore::N),
    ) -> Self {
        Self {
            fallback_function_attribute,
            ignore,
        }
    }
}
impl function_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        (
                            (((((function, ignore_0), _c1), ignore_1), parameter_list), ignore_2),
                            _s3s,
                        ),
                        ignore_3,
                    ),
                    _s5,
                ),
                ignore_4,
            ),
            _c6,
        ): (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        ((usize, ignore::N), Box<function_definition::_C1>),
                                        ignore::N,
                                    ),
                                    parameter_list::N,
                                ),
                                ignore::N,
                            ),
                            Vec<Box<function_definition::_S3>>,
                        ),
                        ignore::N,
                    ),
                    Option<Box<function_definition::_S5>>,
                ),
                ignore::N,
            ),
            Box<function_definition::_C6>,
        ),
    ) -> Self {
        Self {
            function,
            ignore_0,
            _c1,
            ignore_1,
            parameter_list,
            ignore_2,
            _s3s,
            ignore_3,
            _s5,
            ignore_4,
            _c6,
        }
    }
}
impl function_definition::_S5 {
    pub fn new(
        ((returns, ignore), non_empty_parameter_list): (
            (usize, ignore::N),
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
impl function_definition::_S3 {
    pub fn new((function_attribute, ignore): (function_attribute::N, ignore::N)) -> Self {
        Self {
            function_attribute,
            ignore,
        }
    }
}
impl modifier_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (((((modifier, ignore_0), identifier), ignore_1), parameter_list), ignore_2),
                    _s3s,
                ),
                ignore_3,
            ),
            _c4,
        ): (
            (
                (
                    (
                        (
                            (((usize, ignore::N), identifier::N), ignore::N),
                            Option<parameter_list::N>,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<modifier_definition::_S3>>,
                ),
                ignore::N,
            ),
            Box<modifier_definition::_C4>,
        ),
    ) -> Self {
        Self {
            modifier,
            ignore_0,
            identifier,
            ignore_1,
            parameter_list,
            ignore_2,
            _s3s,
            ignore_3,
            _c4,
        }
    }
}
impl modifier_definition::_S3 {
    pub fn new((method_attribute, ignore): (method_attribute::N, ignore::N)) -> Self {
        Self {
            method_attribute,
            ignore,
        }
    }
}
impl receive_function_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (
                        ((((receive, ignore_0), open_paren_char), ignore_1), close_paren_char),
                        ignore_2,
                    ),
                    _s2s,
                ),
                ignore_3,
            ),
            _c3,
        ): (
            (
                (
                    (((((usize, ignore::N), char), ignore::N), char), ignore::N),
                    Vec<Box<receive_function_definition::_S2>>,
                ),
                ignore::N,
            ),
            Box<receive_function_definition::_C3>,
        ),
    ) -> Self {
        Self {
            receive,
            ignore_0,
            open_paren_char,
            ignore_1,
            close_paren_char,
            ignore_2,
            _s2s,
            ignore_3,
            _c3,
        }
    }
}
impl receive_function_definition::_S2 {
    pub fn new(
        (receive_function_attribute, ignore): (receive_function_attribute::N, ignore::N),
    ) -> Self {
        Self {
            receive_function_attribute,
            ignore,
        }
    }
}
impl contract_definition::_S0 {
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
                    _s4s,
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
                                            (((Option<usize>, ignore::N), usize), ignore::N),
                                            identifier::N,
                                        ),
                                        ignore::N,
                                    ),
                                    Option<inheritance_specifier_list::N>,
                                ),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<contract_definition::_S4>>,
                ),
                ignore::N,
            ),
            char,
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
            _s4s,
            ignore_5,
            close_brace_char,
        }
    }
}
impl contract_definition::_S4 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}
impl interface_definition::_S0 {
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
                    _s3s,
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
                                    (((usize, ignore::N), identifier::N), ignore::N),
                                    Option<inheritance_specifier_list::N>,
                                ),
                                ignore::N,
                            ),
                            char,
                        ),
                        ignore::N,
                    ),
                    Vec<Box<interface_definition::_S3>>,
                ),
                ignore::N,
            ),
            char,
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
            _s3s,
            ignore_4,
            close_brace_char,
        }
    }
}
impl interface_definition::_S3 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}
impl library_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (((((library, ignore_0), identifier), ignore_1), open_brace_char), ignore_2),
                    _s2s,
                ),
                ignore_3,
            ),
            close_brace_char,
        ): (
            (
                (
                    (
                        ((((usize, ignore::N), identifier::N), ignore::N), char),
                        ignore::N,
                    ),
                    Vec<Box<library_definition::_S2>>,
                ),
                ignore::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            library,
            ignore_0,
            identifier,
            ignore_1,
            open_brace_char,
            ignore_2,
            _s2s,
            ignore_3,
            close_brace_char,
        }
    }
}
impl library_definition::_S2 {
    pub fn new((contract_body_element, ignore): (contract_body_element::N, ignore::N)) -> Self {
        Self {
            contract_body_element,
            ignore,
        }
    }
}
impl source_unit::_S0 {
    pub fn new(
        ((((ignore_0, ignore_1), _s3s), ignore_2), end_marker): (
            (
                ((ignore::N, ignore::N), Vec<Box<source_unit::_S3>>),
                ignore::N,
            ),
            (),
        ),
    ) -> Self {
        Self {
            ignore_0,
            ignore_1,
            _s3s,
            ignore_2,
            end_marker,
        }
    }
}
impl source_unit::_S3 {
    pub fn new((_c2, ignore): (Box<source_unit::_C2>, ignore::N)) -> Self {
        Self { _c2, ignore }
    }
}
