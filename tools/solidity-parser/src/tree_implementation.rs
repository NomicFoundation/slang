use super::tree_interface::*;
impl break_statement::_S0 {
    pub fn new((r#break, semicolon_char): (usize, char)) -> Self {
        Self {
            r#break,
            semicolon_char,
        }
    }
}
impl comment::_S0 {
    pub fn new(
        (((slash_star, _c2s), star_chars), slash_char): (
            ((usize, Vec<Box<comment::_C2>>), Vec<char>),
            char,
        ),
    ) -> Self {
        Self {
            slash_star,
            _c2s,
            star_chars,
            slash_char,
        }
    }
}
impl comment::_S3 {
    pub fn new((star_chars, _1): (Vec<char>, char)) -> Self {
        Self { star_chars, _1 }
    }
}
impl continue_statement::_S0 {
    pub fn new((r#continue, semicolon_char): (usize, char)) -> Self {
        Self {
            r#continue,
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
    pub fn new((expressions, comma_chars): (Vec<expression::N>, Vec<char>)) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}
impl unchecked_block::_S0 {
    pub fn new((unchecked, block): (usize, block::N)) -> Self {
        Self { unchecked, block }
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
    pub fn new((address, payable): (usize, Option<usize>)) -> Self {
        Self { address, payable }
    }
}
impl numeric_literal::_S0 {
    pub fn new((_c1, _1): (Box<numeric_literal::_C1>, Option<usize>)) -> Self {
        Self { _c1, _1 }
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
        ((open_paren_char, double_quoted_ascii_string_literals), close_paren_char): (
            (char, Box<assembly_flags::_S1>),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            double_quoted_ascii_string_literals,
            close_paren_char,
        }
    }
}
impl assembly_flags::_S1 {
    pub fn new(
        (double_quoted_ascii_string_literals, comma_chars): (
            Vec<double_quoted_ascii_string_literal::N>,
            Vec<char>,
        ),
    ) -> Self {
        Self {
            double_quoted_ascii_string_literals,
            comma_chars,
        }
    }
}
impl yul_function_call::_S0 {
    pub fn new(
        (((_c1, open_paren_char), yul_expressions), close_paren_char): (
            (
                (Box<yul_function_call::_C1>, char),
                Option<Box<yul_function_call::_S2>>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            _c1,
            open_paren_char,
            yul_expressions,
            close_paren_char,
        }
    }
}
impl yul_function_call::_S2 {
    pub fn new((yul_expressions, comma_chars): (Vec<yul_expression::N>, Vec<char>)) -> Self {
        Self {
            yul_expressions,
            comma_chars,
        }
    }
}
impl yul_function_definition::_S0 {
    pub fn new(
        (
            (
                (
                    (((function, yul_identifier), open_paren_char), yul_identifiers),
                    close_paren_char,
                ),
                _s3,
            ),
            yul_block,
        ): (
            (
                (
                    (
                        ((usize, yul_identifier::N), char),
                        Option<Box<yul_function_definition::_S1>>,
                    ),
                    char,
                ),
                Option<Box<yul_function_definition::_S3>>,
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
            _s3,
            yul_block,
        }
    }
}
impl yul_function_definition::_S3 {
    pub fn new(
        (minus_greater, yul_identifiers): (usize, Box<yul_function_definition::_S4>),
    ) -> Self {
        Self {
            minus_greater,
            yul_identifiers,
        }
    }
}
impl yul_function_definition::_S4 {
    pub fn new((yul_identifiers, comma_chars): (Vec<yul_identifier::N>, Vec<char>)) -> Self {
        Self {
            yul_identifiers,
            comma_chars,
        }
    }
}
impl yul_function_definition::_S1 {
    pub fn new((yul_identifiers, comma_chars): (Vec<yul_identifier::N>, Vec<char>)) -> Self {
        Self {
            yul_identifiers,
            comma_chars,
        }
    }
}
impl yul_path::_S0 {
    pub fn new((yul_identifier, _s2s): (yul_identifier::N, Vec<Box<yul_path::_S2>>)) -> Self {
        Self {
            yul_identifier,
            _s2s,
        }
    }
}
impl yul_path::_S2 {
    pub fn new((period_char, _c3): (char, Box<yul_path::_C3>)) -> Self {
        Self { period_char, _c3 }
    }
}
impl enum_definition::_S0 {
    pub fn new(
        ((((r#enum, identifier), open_brace_char), identifiers), close_brace_char): (
            (((usize, identifier::N), char), Box<enum_definition::_S1>),
            char,
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
impl enum_definition::_S1 {
    pub fn new((identifiers, comma_chars): (Vec<identifier::N>, Vec<char>)) -> Self {
        Self {
            identifiers,
            comma_chars,
        }
    }
}
impl identifier_path::_S0 {
    pub fn new((identifiers, period_chars): (Vec<identifier::N>, Vec<char>)) -> Self {
        Self {
            identifiers,
            period_chars,
        }
    }
}
impl named_argument::_S0 {
    pub fn new(
        ((identifier, colon_char), expression): ((identifier::N, char), expression::N),
    ) -> Self {
        Self {
            identifier,
            colon_char,
            expression,
        }
    }
}
impl parameter_declaration::_S0 {
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
impl selected_import::_S0 {
    pub fn new((identifier, _s2): (identifier::N, Option<Box<selected_import::_S2>>)) -> Self {
        Self { identifier, _s2 }
    }
}
impl selected_import::_S2 {
    pub fn new((r#as, identifier): (usize, identifier::N)) -> Self {
        Self { r#as, identifier }
    }
}
impl user_defined_value_type_definition::_S0 {
    pub fn new(
        ((((r#type, identifier), is), elementary_type_with_payable), semicolon_char): (
            (
                ((usize, identifier::N), usize),
                elementary_type_with_payable::N,
            ),
            char,
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
impl mapping_type::_S0 {
    pub fn new(
        (((((mapping, open_paren_char), _c1), equal_greater), type_name), close_paren_char): (
            (
                (((usize, char), Box<mapping_type::_C1>), usize),
                type_name::N,
            ),
            char,
        ),
    ) -> Self {
        Self {
            mapping,
            open_paren_char,
            _c1,
            equal_greater,
            type_name,
            close_paren_char,
        }
    }
}
impl named_argument_list::_S0 {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (char, Option<Box<named_argument_list::_S1>>),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}
impl named_argument_list::_S1 {
    pub fn new((named_arguments, comma_chars): (Vec<named_argument::N>, Vec<char>)) -> Self {
        Self {
            named_arguments,
            comma_chars,
        }
    }
}
impl non_empty_parameter_list::_S0 {
    pub fn new(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (char, Box<non_empty_parameter_list::_S1>),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}
impl non_empty_parameter_list::_S1 {
    pub fn new(
        (parameter_declarations, comma_chars): (Vec<parameter_declaration::N>, Vec<char>),
    ) -> Self {
        Self {
            parameter_declarations,
            comma_chars,
        }
    }
}
impl override_specifier::_S0 {
    pub fn new((r#override, _s2): (usize, Option<Box<override_specifier::_S2>>)) -> Self {
        Self { r#override, _s2 }
    }
}
impl override_specifier::_S2 {
    pub fn new(
        ((open_paren_char, identifier_paths), close_paren_char): (
            (char, Box<override_specifier::_S3>),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            identifier_paths,
            close_paren_char,
        }
    }
}
impl override_specifier::_S3 {
    pub fn new((identifier_paths, comma_chars): (Vec<identifier_path::N>, Vec<char>)) -> Self {
        Self {
            identifier_paths,
            comma_chars,
        }
    }
}
impl parameter_list::_S0 {
    pub fn new(
        ((open_paren_char, parameter_declarations), close_paren_char): (
            (char, Option<Box<parameter_list::_S1>>),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            parameter_declarations,
            close_paren_char,
        }
    }
}
impl parameter_list::_S1 {
    pub fn new(
        (parameter_declarations, comma_chars): (Vec<parameter_declaration::N>, Vec<char>),
    ) -> Self {
        Self {
            parameter_declarations,
            comma_chars,
        }
    }
}
impl selecting_import_directive::_S0 {
    pub fn new(
        ((((open_brace_char, selected_imports), close_brace_char), from), import_path): (
            (((char, Box<selecting_import_directive::_S1>), char), usize),
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
impl selecting_import_directive::_S1 {
    pub fn new((selected_imports, comma_chars): (Vec<selected_import::N>, Vec<char>)) -> Self {
        Self {
            selected_imports,
            comma_chars,
        }
    }
}
impl simple_import_directive::_S0 {
    pub fn new(
        (import_path, _s2s): (import_path::N, Vec<Box<simple_import_directive::_S2>>),
    ) -> Self {
        Self { import_path, _s2s }
    }
}
impl simple_import_directive::_S2 {
    pub fn new((r#as, identifier): (usize, identifier::N)) -> Self {
        Self { r#as, identifier }
    }
}
impl star_import_directive::_S0 {
    pub fn new(
        ((((star_char, r#as), identifier), from), import_path): (
            (((char, usize), identifier::N), usize),
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
impl argument_list::_S0 {
    pub fn new(
        ((open_paren_char, _c2), close_paren_char): ((char, Option<Box<argument_list::_C2>>), char),
    ) -> Self {
        Self {
            open_paren_char,
            _c2,
            close_paren_char,
        }
    }
}
impl catch_clause::_S0 {
    pub fn new(((catch, _s2), block): ((usize, Option<Box<catch_clause::_S2>>), block::N)) -> Self {
        Self { catch, _s2, block }
    }
}
impl catch_clause::_S2 {
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
impl function_type::_S0 {
    pub fn new(
        (((function, parameter_list), _2), _s3): (
            ((usize, parameter_list::N), Vec<usize>),
            Option<Box<function_type::_S3>>,
        ),
    ) -> Self {
        Self {
            function,
            parameter_list,
            _2,
            _s3,
        }
    }
}
impl function_type::_S3 {
    pub fn new((returns, non_empty_parameter_list): (usize, non_empty_parameter_list::N)) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl import_directive::_S0 {
    pub fn new(
        ((import, _c1), semicolon_char): ((usize, Box<import_directive::_C1>), char),
    ) -> Self {
        Self {
            import,
            _c1,
            semicolon_char,
        }
    }
}
impl yul_assignment::_S0 {
    pub fn new((yul_path, _c1): (yul_path::N, Box<yul_assignment::_C1>)) -> Self {
        Self { yul_path, _c1 }
    }
}
impl yul_assignment::_S3 {
    pub fn new(
        ((_s5s, colon_equal), yul_function_call): (
            (Vec<Box<yul_assignment::_S5>>, usize),
            yul_function_call::N,
        ),
    ) -> Self {
        Self {
            _s5s,
            colon_equal,
            yul_function_call,
        }
    }
}
impl yul_assignment::_S5 {
    pub fn new((comma_char, yul_path): (char, yul_path::N)) -> Self {
        Self {
            comma_char,
            yul_path,
        }
    }
}
impl yul_assignment::_S2 {
    pub fn new((colon_equal, yul_expression): (usize, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl yul_for_statement::_S0 {
    pub fn new(
        ((((r#for, yul_block_0), yul_expression), yul_block_1), yul_block_2): (
            (((usize, yul_block::N), yul_expression::N), yul_block::N),
            yul_block::N,
        ),
    ) -> Self {
        Self {
            r#for,
            yul_block_0,
            yul_expression,
            yul_block_1,
            yul_block_2,
        }
    }
}
impl yul_if_statement::_S0 {
    pub fn new(
        ((r#if, yul_expression), yul_block): ((usize, yul_expression::N), yul_block::N),
    ) -> Self {
        Self {
            r#if,
            yul_expression,
            yul_block,
        }
    }
}
impl yul_switch_statement::_S0 {
    pub fn new(
        ((switch, yul_expression), _c1): (
            (usize, yul_expression::N),
            Box<yul_switch_statement::_C1>,
        ),
    ) -> Self {
        Self {
            switch,
            yul_expression,
            _c1,
        }
    }
}
impl yul_switch_statement::_S7 {
    pub fn new((default, yul_block): (usize, yul_block::N)) -> Self {
        Self { default, yul_block }
    }
}
impl yul_switch_statement::_S2 {
    pub fn new(
        (_s4s, _s6): (
            Vec<Box<yul_switch_statement::_S4>>,
            Option<Box<yul_switch_statement::_S6>>,
        ),
    ) -> Self {
        Self { _s4s, _s6 }
    }
}
impl yul_switch_statement::_S6 {
    pub fn new((default, yul_block): (usize, yul_block::N)) -> Self {
        Self { default, yul_block }
    }
}
impl yul_switch_statement::_S4 {
    pub fn new(((case, yul_literal), yul_block): ((usize, yul_literal::N), yul_block::N)) -> Self {
        Self {
            case,
            yul_literal,
            yul_block,
        }
    }
}
impl yul_variable_declaration::_S0 {
    pub fn new(
        ((r#let, yul_identifier), _c2): (
            (usize, yul_identifier::N),
            Option<Box<yul_variable_declaration::_C2>>,
        ),
    ) -> Self {
        Self {
            r#let,
            yul_identifier,
            _c2,
        }
    }
}
impl yul_variable_declaration::_S4 {
    pub fn new(
        (_s6, _s8): (
            Option<Box<yul_variable_declaration::_S6>>,
            Option<Box<yul_variable_declaration::_S8>>,
        ),
    ) -> Self {
        Self { _s6, _s8 }
    }
}
impl yul_variable_declaration::_S8 {
    pub fn new((colon_equal, yul_function_call): (usize, yul_function_call::N)) -> Self {
        Self {
            colon_equal,
            yul_function_call,
        }
    }
}
impl yul_variable_declaration::_S6 {
    pub fn new((comma_char, yul_identifier): (char, yul_identifier::N)) -> Self {
        Self {
            comma_char,
            yul_identifier,
        }
    }
}
impl yul_variable_declaration::_S3 {
    pub fn new((colon_equal, yul_expression): (usize, yul_expression::N)) -> Self {
        Self {
            colon_equal,
            yul_expression,
        }
    }
}
impl inheritance_specifier::_S0 {
    pub fn new(
        (identifier_path, argument_list): (identifier_path::N, Option<argument_list::N>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl modifier_invocation::_S0 {
    pub fn new(
        (identifier_path, argument_list): (identifier_path::N, Option<argument_list::N>),
    ) -> Self {
        Self {
            identifier_path,
            argument_list,
        }
    }
}
impl type_name::_S0 {
    pub fn new((_c1, _s3s): (Box<type_name::_C1>, Vec<Box<type_name::_S3>>)) -> Self {
        Self { _c1, _s3s }
    }
}
impl type_name::_S3 {
    pub fn new(
        ((open_bracket_char, expression), close_bracket_char): (
            (char, Option<expression::N>),
            char,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            close_bracket_char,
        }
    }
}
impl error_parameter::_S0 {
    pub fn new((type_name, identifier): (type_name::N, Option<identifier::N>)) -> Self {
        Self {
            type_name,
            identifier,
        }
    }
}
impl event_parameter::_S0 {
    pub fn new(
        ((type_name, indexed), identifier): ((type_name::N, Option<usize>), Option<identifier::N>),
    ) -> Self {
        Self {
            type_name,
            indexed,
            identifier,
        }
    }
}
impl inheritance_specifier_list::_S0 {
    pub fn new(
        (is, inheritance_specifiers): (usize, Box<inheritance_specifier_list::_S1>),
    ) -> Self {
        Self {
            is,
            inheritance_specifiers,
        }
    }
}
impl inheritance_specifier_list::_S1 {
    pub fn new(
        (inheritance_specifiers, comma_chars): (Vec<inheritance_specifier::N>, Vec<char>),
    ) -> Self {
        Self {
            inheritance_specifiers,
            comma_chars,
        }
    }
}
impl primary_expression::_S7 {
    pub fn new(
        ((open_bracket_char, expressions), close_bracket_char): (
            (char, Box<primary_expression::_S8>),
            char,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expressions,
            close_bracket_char,
        }
    }
}
impl primary_expression::_S8 {
    pub fn new((expressions, comma_chars): (Vec<expression::N>, Vec<char>)) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}
impl primary_expression::_S4 {
    pub fn new(
        ((open_paren_char, expressions), close_paren_char): (
            (char, Box<primary_expression::_S5>),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            expressions,
            close_paren_char,
        }
    }
}
impl primary_expression::_S5 {
    pub fn new((expressions, comma_chars): (Vec<Option<expression::N>>, Vec<char>)) -> Self {
        Self {
            expressions,
            comma_chars,
        }
    }
}
impl primary_expression::_S3 {
    pub fn new((new, type_name): (usize, type_name::N)) -> Self {
        Self { new, type_name }
    }
}
impl primary_expression::_S2 {
    pub fn new(
        (((r#type, open_paren_char), type_name), close_paren_char): (
            ((usize, char), type_name::N),
            char,
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
impl primary_expression::_S1 {
    pub fn new((payable, argument_list): (usize, argument_list::N)) -> Self {
        Self {
            payable,
            argument_list,
        }
    }
}
impl struct_definition::_S0 {
    pub fn new(
        ((((r#struct, identifier), open_brace_char), _s2s), close_brace_char): (
            (
                ((usize, identifier::N), char),
                Vec<Box<struct_definition::_S2>>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            r#struct,
            identifier,
            open_brace_char,
            _s2s,
            close_brace_char,
        }
    }
}
impl struct_definition::_S2 {
    pub fn new(
        ((type_name, identifier), semicolon_char): ((type_name::N, identifier::N), char),
    ) -> Self {
        Self {
            type_name,
            identifier,
            semicolon_char,
        }
    }
}
impl using_directive::_S0 {
    pub fn new(
        (((((using, _c1), r#for), _c4), global), semicolon_char): (
            (
                (
                    ((usize, Box<using_directive::_C1>), usize),
                    Box<using_directive::_C4>,
                ),
                Option<usize>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            using,
            _c1,
            r#for,
            _c4,
            global,
            semicolon_char,
        }
    }
}
impl using_directive::_S2 {
    pub fn new(
        ((open_brace_char, identifier_paths), close_brace_char): (
            (char, Box<using_directive::_S3>),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            identifier_paths,
            close_brace_char,
        }
    }
}
impl using_directive::_S3 {
    pub fn new((identifier_paths, comma_chars): (Vec<identifier_path::N>, Vec<char>)) -> Self {
        Self {
            identifier_paths,
            comma_chars,
        }
    }
}
impl variable_declaration::_S0 {
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
impl yul_block::_S0 {
    pub fn new(
        ((open_brace_char, yul_statements), close_brace_char): (
            (char, Vec<yul_statement::N>),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            yul_statements,
            close_brace_char,
        }
    }
}
impl assembly_statement::_S0 {
    pub fn new(
        (((assembly, double_quote_evmasm_double_quote), assembly_flags), yul_block): (
            ((usize, Option<usize>), Option<assembly_flags::N>),
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
impl error_definition::_S0 {
    pub fn new(
        (
            ((((error, identifier), open_paren_char), error_parameters), close_paren_char),
            semicolon_char,
        ): (
            (
                (
                    ((usize, identifier::N), char),
                    Option<Box<error_definition::_S1>>,
                ),
                char,
            ),
            char,
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
impl error_definition::_S1 {
    pub fn new((error_parameters, comma_chars): (Vec<error_parameter::N>, Vec<char>)) -> Self {
        Self {
            error_parameters,
            comma_chars,
        }
    }
}
impl event_definition::_S0 {
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
                        ((usize, identifier::N), char),
                        Option<Box<event_definition::_S1>>,
                    ),
                    char,
                ),
                Option<usize>,
            ),
            char,
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
impl event_definition::_S1 {
    pub fn new((event_parameters, comma_chars): (Vec<event_parameter::N>, Vec<char>)) -> Self {
        Self {
            event_parameters,
            comma_chars,
        }
    }
}
impl index_access_expression::_S0 {
    pub fn new(
        (primary_expression, _s2s): (
            primary_expression::N,
            Vec<Box<index_access_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            primary_expression,
            _s2s,
        }
    }
}
impl index_access_expression::_S2 {
    pub fn new(
        (((open_bracket_char, expression), _s5), close_bracket_char): (
            (
                (char, Option<expression::N>),
                Option<Box<index_access_expression::_S5>>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_bracket_char,
            expression,
            _s5,
            close_bracket_char,
        }
    }
}
impl index_access_expression::_S5 {
    pub fn new((colon_char, expression): (char, Option<expression::N>)) -> Self {
        Self {
            colon_char,
            expression,
        }
    }
}
impl variable_declaration_tuple::_S0 {
    pub fn new(
        ((((open_paren_char, comma_chars), variable_declaration), _s3s), close_paren_char): (
            (
                ((char, Vec<char>), variable_declaration::N),
                Vec<Box<variable_declaration_tuple::_S3>>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            open_paren_char,
            comma_chars,
            variable_declaration,
            _s3s,
            close_paren_char,
        }
    }
}
impl variable_declaration_tuple::_S3 {
    pub fn new(
        (comma_char, variable_declaration): (char, Option<variable_declaration::N>),
    ) -> Self {
        Self {
            comma_char,
            variable_declaration,
        }
    }
}
impl member_access_expression::_S0 {
    pub fn new(
        (index_access_expression, _s2s): (
            index_access_expression::N,
            Vec<Box<member_access_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            index_access_expression,
            _s2s,
        }
    }
}
impl member_access_expression::_S2 {
    pub fn new((period_char, _c3): (char, Box<member_access_expression::_C3>)) -> Self {
        Self { period_char, _c3 }
    }
}
impl function_call_options_expression::_S0 {
    pub fn new(
        (member_access_expression, _s2s): (
            member_access_expression::N,
            Vec<Box<function_call_options_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            member_access_expression,
            _s2s,
        }
    }
}
impl function_call_options_expression::_S2 {
    pub fn new(
        ((open_brace_char, named_arguments), close_brace_char): (
            (char, Box<function_call_options_expression::_S3>),
            char,
        ),
    ) -> Self {
        Self {
            open_brace_char,
            named_arguments,
            close_brace_char,
        }
    }
}
impl function_call_options_expression::_S3 {
    pub fn new((named_arguments, comma_chars): (Vec<named_argument::N>, Vec<char>)) -> Self {
        Self {
            named_arguments,
            comma_chars,
        }
    }
}
impl function_call_expression::_S0 {
    pub fn new(
        (function_call_options_expression, argument_lists): (
            function_call_options_expression::N,
            Vec<argument_list::N>,
        ),
    ) -> Self {
        Self {
            function_call_options_expression,
            argument_lists,
        }
    }
}
impl unary_prefix_expression::_S0 {
    pub fn new((_0, function_call_expression): (usize, function_call_expression::N)) -> Self {
        Self {
            _0,
            function_call_expression,
        }
    }
}
impl unary_suffix_expression::_S0 {
    pub fn new((unary_prefix_expression, _1): (unary_prefix_expression::N, usize)) -> Self {
        Self {
            unary_prefix_expression,
            _1,
        }
    }
}
impl exp_expression::_S0 {
    pub fn new(
        ((unary_suffix_expression, star_star), expression): (
            (unary_suffix_expression::N, usize),
            expression::N,
        ),
    ) -> Self {
        Self {
            unary_suffix_expression,
            star_star,
            expression,
        }
    }
}
impl mul_div_mod_expression::_S0 {
    pub fn new(
        (exp_expression, _s2s): (exp_expression::N, Vec<Box<mul_div_mod_expression::_S2>>),
    ) -> Self {
        Self {
            exp_expression,
            _s2s,
        }
    }
}
impl mul_div_mod_expression::_S2 {
    pub fn new((_0, exp_expression): (char, exp_expression::N)) -> Self {
        Self { _0, exp_expression }
    }
}
impl add_sub_expression::_S0 {
    pub fn new(
        (mul_div_mod_expression, _s2s): (
            mul_div_mod_expression::N,
            Vec<Box<add_sub_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            mul_div_mod_expression,
            _s2s,
        }
    }
}
impl add_sub_expression::_S2 {
    pub fn new((_0, mul_div_mod_expression): (char, mul_div_mod_expression::N)) -> Self {
        Self {
            _0,
            mul_div_mod_expression,
        }
    }
}
impl shift_expression::_S0 {
    pub fn new(
        (add_sub_expression, _s2s): (add_sub_expression::N, Vec<Box<shift_expression::_S2>>),
    ) -> Self {
        Self {
            add_sub_expression,
            _s2s,
        }
    }
}
impl shift_expression::_S2 {
    pub fn new((_0, add_sub_expression): (usize, add_sub_expression::N)) -> Self {
        Self {
            _0,
            add_sub_expression,
        }
    }
}
impl bit_and_expression::_S0 {
    pub fn new(
        (shift_expression, _s2s): (shift_expression::N, Vec<Box<bit_and_expression::_S2>>),
    ) -> Self {
        Self {
            shift_expression,
            _s2s,
        }
    }
}
impl bit_and_expression::_S2 {
    pub fn new((ampersand_char, shift_expression): (char, shift_expression::N)) -> Self {
        Self {
            ampersand_char,
            shift_expression,
        }
    }
}
impl bit_x_or_expression::_S0 {
    pub fn new(
        (bit_and_expression, _s2s): (bit_and_expression::N, Vec<Box<bit_x_or_expression::_S2>>),
    ) -> Self {
        Self {
            bit_and_expression,
            _s2s,
        }
    }
}
impl bit_x_or_expression::_S2 {
    pub fn new((caret_char, bit_and_expression): (char, bit_and_expression::N)) -> Self {
        Self {
            caret_char,
            bit_and_expression,
        }
    }
}
impl bit_or_expression::_S0 {
    pub fn new(
        (bit_x_or_expression, _s2s): (bit_x_or_expression::N, Vec<Box<bit_or_expression::_S2>>),
    ) -> Self {
        Self {
            bit_x_or_expression,
            _s2s,
        }
    }
}
impl bit_or_expression::_S2 {
    pub fn new((bar_char, bit_x_or_expression): (char, bit_x_or_expression::N)) -> Self {
        Self {
            bar_char,
            bit_x_or_expression,
        }
    }
}
impl order_comparison_expression::_S0 {
    pub fn new(
        (bit_or_expression, _s2s): (
            bit_or_expression::N,
            Vec<Box<order_comparison_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            bit_or_expression,
            _s2s,
        }
    }
}
impl order_comparison_expression::_S2 {
    pub fn new((_0, bit_or_expression): (usize, bit_or_expression::N)) -> Self {
        Self {
            _0,
            bit_or_expression,
        }
    }
}
impl equality_comparison_expression::_S0 {
    pub fn new(
        (order_comparison_expression, _s2s): (
            order_comparison_expression::N,
            Vec<Box<equality_comparison_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            order_comparison_expression,
            _s2s,
        }
    }
}
impl equality_comparison_expression::_S2 {
    pub fn new((_0, order_comparison_expression): (usize, order_comparison_expression::N)) -> Self {
        Self {
            _0,
            order_comparison_expression,
        }
    }
}
impl and_expression::_S0 {
    pub fn new(
        (equality_comparison_expression, _s2s): (
            equality_comparison_expression::N,
            Vec<Box<and_expression::_S2>>,
        ),
    ) -> Self {
        Self {
            equality_comparison_expression,
            _s2s,
        }
    }
}
impl and_expression::_S2 {
    pub fn new(
        (ampersand_ampersand, equality_comparison_expression): (
            usize,
            equality_comparison_expression::N,
        ),
    ) -> Self {
        Self {
            ampersand_ampersand,
            equality_comparison_expression,
        }
    }
}
impl or_expression::_S0 {
    pub fn new((and_expression, _s2s): (and_expression::N, Vec<Box<or_expression::_S2>>)) -> Self {
        Self {
            and_expression,
            _s2s,
        }
    }
}
impl or_expression::_S2 {
    pub fn new((bar_bar, and_expression): (usize, and_expression::N)) -> Self {
        Self {
            bar_bar,
            and_expression,
        }
    }
}
impl conditional_expression::_S0 {
    pub fn new(
        ((((or_expression, question_char), expression_0), colon_char), expression_1): (
            (((or_expression::N, char), expression::N), char),
            expression::N,
        ),
    ) -> Self {
        Self {
            or_expression,
            question_char,
            expression_0,
            colon_char,
            expression_1,
        }
    }
}
impl assignment_expression::_S0 {
    pub fn new(
        ((conditional_expression, _1), expression): (
            (conditional_expression::N, usize),
            expression::N,
        ),
    ) -> Self {
        Self {
            conditional_expression,
            _1,
            expression,
        }
    }
}
impl constant_definition::_S0 {
    pub fn new(
        (((((type_name, constant), identifier), equal_char), expression), semicolon_char): (
            (
                (((type_name::N, usize), identifier::N), char),
                expression::N,
            ),
            char,
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
impl do_while_statement::_S0 {
    pub fn new(
        (
            (((((r#do, statement), r#while), open_paren_char), expression), close_paren_char),
            semicolon_char,
        ): (
            (
                ((((usize, statement::N), usize), char), expression::N),
                char,
            ),
            char,
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
impl emit_statement::_S0 {
    pub fn new(
        (((emit, expression), argument_list), semicolon_char): (
            ((usize, expression::N), argument_list::N),
            char,
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
impl expression_statement::_S0 {
    pub fn new((expression, semicolon_char): (expression::N, char)) -> Self {
        Self {
            expression,
            semicolon_char,
        }
    }
}
impl if_statement::_S0 {
    pub fn new(
        (((((r#if, open_paren_char), expression), close_paren_char), statement), _s2): (
            ((((usize, char), expression::N), char), statement::N),
            Option<Box<if_statement::_S2>>,
        ),
    ) -> Self {
        Self {
            r#if,
            open_paren_char,
            expression,
            close_paren_char,
            statement,
            _s2,
        }
    }
}
impl if_statement::_S2 {
    pub fn new((r#else, statement): (usize, statement::N)) -> Self {
        Self { r#else, statement }
    }
}
impl return_statement::_S0 {
    pub fn new(
        ((r#return, expression), semicolon_char): ((usize, Option<expression::N>), char),
    ) -> Self {
        Self {
            r#return,
            expression,
            semicolon_char,
        }
    }
}
impl revert_statement::_S0 {
    pub fn new(
        (((revert, expression), argument_list), semicolon_char): (
            ((usize, expression::N), argument_list::N),
            char,
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
impl state_variable_declaration::_S0 {
    pub fn new(
        ((((type_name, state_variable_attributes), identifier), _s3), semicolon_char): (
            (
                (
                    (type_name::N, Vec<state_variable_attribute::N>),
                    identifier::N,
                ),
                Option<Box<state_variable_declaration::_S3>>,
            ),
            char,
        ),
    ) -> Self {
        Self {
            type_name,
            state_variable_attributes,
            identifier,
            _s3,
            semicolon_char,
        }
    }
}
impl state_variable_declaration::_S3 {
    pub fn new((equal_char, expression): (char, expression::N)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl try_statement::_S0 {
    pub fn new(
        (((((r#try, expression), _s2), block), catch_clause), catch_clauses): (
            (
                (
                    ((usize, expression::N), Option<Box<try_statement::_S2>>),
                    block::N,
                ),
                catch_clause::N,
            ),
            Vec<catch_clause::N>,
        ),
    ) -> Self {
        Self {
            r#try,
            expression,
            _s2,
            block,
            catch_clause,
            catch_clauses,
        }
    }
}
impl try_statement::_S2 {
    pub fn new((returns, non_empty_parameter_list): (usize, non_empty_parameter_list::N)) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl variable_declaration_statement::_S0 {
    pub fn new((_c1, semicolon_char): (Box<variable_declaration_statement::_C1>, char)) -> Self {
        Self {
            _c1,
            semicolon_char,
        }
    }
}
impl variable_declaration_statement::_S5 {
    pub fn new(
        ((variable_declaration_tuple, equal_char), expression): (
            (variable_declaration_tuple::N, char),
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
impl variable_declaration_statement::_S2 {
    pub fn new(
        (variable_declaration, _s4): (
            variable_declaration::N,
            Option<Box<variable_declaration_statement::_S4>>,
        ),
    ) -> Self {
        Self {
            variable_declaration,
            _s4,
        }
    }
}
impl variable_declaration_statement::_S4 {
    pub fn new((equal_char, expression): (char, expression::N)) -> Self {
        Self {
            equal_char,
            expression,
        }
    }
}
impl while_statement::_S0 {
    pub fn new(
        ((((r#while, open_paren_char), expression), close_paren_char), statement): (
            (((usize, char), expression::N), char),
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
impl for_statement::_S0 {
    pub fn new(
        ((((((r#for, open_paren_char), _c1), _c2), expression), close_paren_char), statement): (
            (
                (
                    (
                        ((usize, char), Box<for_statement::_C1>),
                        Box<for_statement::_C2>,
                    ),
                    Option<expression::N>,
                ),
                char,
            ),
            statement::N,
        ),
    ) -> Self {
        Self {
            r#for,
            open_paren_char,
            _c1,
            _c2,
            expression,
            close_paren_char,
            statement,
        }
    }
}
impl block::_S0 {
    pub fn new(
        ((open_brace_char, _c2s), close_brace_char): ((char, Vec<Box<block::_C2>>), char),
    ) -> Self {
        Self {
            open_brace_char,
            _c2s,
            close_brace_char,
        }
    }
}
impl constructor_definition::_S0 {
    pub fn new(
        (((constructor, parameter_list), constructor_attributes), block): (
            ((usize, parameter_list::N), Vec<constructor_attribute::N>),
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
impl fallback_function_definition::_S0 {
    pub fn new(
        ((((fallback, parameter_list), fallback_function_attributes), _s3), _c4): (
            (
                (
                    (usize, parameter_list::N),
                    Vec<fallback_function_attribute::N>,
                ),
                Option<Box<fallback_function_definition::_S3>>,
            ),
            Box<fallback_function_definition::_C4>,
        ),
    ) -> Self {
        Self {
            fallback,
            parameter_list,
            fallback_function_attributes,
            _s3,
            _c4,
        }
    }
}
impl fallback_function_definition::_S3 {
    pub fn new((returns, non_empty_parameter_list): (usize, non_empty_parameter_list::N)) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl function_definition::_S0 {
    pub fn new(
        (((((function, _c1), parameter_list), function_attributes), _s4), _c5): (
            (
                (
                    ((usize, Box<function_definition::_C1>), parameter_list::N),
                    Vec<function_attribute::N>,
                ),
                Option<Box<function_definition::_S4>>,
            ),
            Box<function_definition::_C5>,
        ),
    ) -> Self {
        Self {
            function,
            _c1,
            parameter_list,
            function_attributes,
            _s4,
            _c5,
        }
    }
}
impl function_definition::_S4 {
    pub fn new((returns, non_empty_parameter_list): (usize, non_empty_parameter_list::N)) -> Self {
        Self {
            returns,
            non_empty_parameter_list,
        }
    }
}
impl modifier_definition::_S0 {
    pub fn new(
        ((((modifier, identifier), parameter_list), method_attributes), _c3): (
            (
                ((usize, identifier::N), Option<parameter_list::N>),
                Vec<method_attribute::N>,
            ),
            Box<modifier_definition::_C3>,
        ),
    ) -> Self {
        Self {
            modifier,
            identifier,
            parameter_list,
            method_attributes,
            _c3,
        }
    }
}
impl receive_function_definition::_S0 {
    pub fn new(
        ((((receive, open_paren_char), close_paren_char), receive_function_attributes), _c2): (
            (((usize, char), char), Vec<receive_function_attribute::N>),
            Box<receive_function_definition::_C2>,
        ),
    ) -> Self {
        Self {
            receive,
            open_paren_char,
            close_paren_char,
            receive_function_attributes,
            _c2,
        }
    }
}
impl contract_definition::_S0 {
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
                        ((Option<usize>, usize), identifier::N),
                        Option<inheritance_specifier_list::N>,
                    ),
                    char,
                ),
                Vec<contract_body_element::N>,
            ),
            char,
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
impl interface_definition::_S0 {
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
                        (usize, identifier::N),
                        Option<inheritance_specifier_list::N>,
                    ),
                    char,
                ),
                Vec<contract_body_element::N>,
            ),
            char,
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
impl library_definition::_S0 {
    pub fn new(
        ((((library, identifier), open_brace_char), contract_body_elements), close_brace_char): (
            (
                ((usize, identifier::N), char),
                Vec<contract_body_element::N>,
            ),
            char,
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
impl source_unit::_S0 {
    pub fn new(
        ((ignore, _c2s), end_marker): ((ignore::N, Vec<Box<source_unit::_C2>>), ()),
    ) -> Self {
        Self {
            ignore,
            _c2s,
            end_marker,
        }
    }
}
