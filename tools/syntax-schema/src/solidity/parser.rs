use chumsky::{prelude::*, Parser};

use super::builder;
pub type SourceUnitParserResultType = ();

pub fn create_source_unit_parser(
) -> impl Parser<char, SourceUnitParserResultType, Error = Simple<char>> {
    let mut statement_parser = Recursive::declare();
    let mut yul_expression_parser = Recursive::declare();
    let mut block_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut type_name_parser = Recursive::declare();
    let mut yul_block_parser = Recursive::declare();
    let ascii_escape_parser = choice((
        just('n'),
        just('r'),
        just('t'),
        just('\''),
        just('"'),
        just('\\'),
        just('\n'),
        just('\r'),
    ))
    .map(builder::ascii_escape)
    .boxed();
    let boolean_literal_parser = choice((choice((just("true"), just("false")))))
        .map(builder::boolean_literal)
        .boxed();
    let comment_parser = just("/*")
        .ignore_then(
            choice((
                filter(|&c: &char| c != '*'),
                just('*').repeated().at_least(1usize).then(none_of("*/")),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
        .map(builder::comment)
        .boxed();
    let decimal_integer_parser = filter(|&c: &char| c.is_ascii_digit())
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .map(builder::decimal_integer)
        .boxed();
    let fixed_parser = just("fixed")
        .ignore_then(
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .then_ignore(just('x'))
                .then(filter(|&c: &char| ('1' <= c && c <= '9')))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .or_not(),
        )
        .map(builder::fixed)
        .boxed();
    let fixed_bytes_parser = just("bytes")
        .ignore_then(choice((
            choice((
                just("1"),
                just("2"),
                just("3"),
                just("4"),
                just("5"),
                just("6"),
                just("7"),
                just("8"),
                just("9"),
                just("10"),
                just("11"),
                just("12"),
                just("13"),
                just("14"),
                just("15"),
                just("16"),
            )),
            choice((
                just("17"),
                just("18"),
                just("19"),
                just("20"),
                just("21"),
                just("22"),
                just("23"),
                just("24"),
                just("25"),
                just("26"),
                just("27"),
                just("28"),
                just("29"),
                just("30"),
                just("31"),
                just("32"),
            )),
        )))
        .map(builder::fixed_bytes)
        .boxed();
    let hex_character_parser = choice((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ))
    .map(builder::hex_character)
    .boxed();
    let identifier_start_parser = choice((
        just('_'),
        just('$'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ))
    .map(builder::identifier_start)
    .boxed();
    let line_comment_parser = just("//")
        .ignore_then(none_of("\n\r").repeated())
        .map(builder::line_comment)
        .boxed();
    let number_unit_parser = choice(
        (choice((
            just("wei"),
            just("gwei"),
            just("ether"),
            just("seconds"),
            just("minutes"),
            just("hours"),
            just("days"),
            just("weeks"),
            just("years"),
        ))),
    )
    .map(builder::number_unit)
    .boxed();
    let signed_integer_type_parser = just("int")
        .ignore_then(choice((
            choice((
                just("8"),
                just("16"),
                just("24"),
                just("32"),
                just("40"),
                just("48"),
                just("56"),
                just("64"),
                just("72"),
                just("80"),
                just("88"),
                just("96"),
                just("104"),
                just("112"),
                just("120"),
                just("128"),
            )),
            choice((
                just("136"),
                just("144"),
                just("152"),
                just("160"),
                just("168"),
                just("176"),
                just("184"),
                just("192"),
                just("200"),
                just("208"),
                just("216"),
                just("224"),
                just("232"),
                just("240"),
                just("248"),
                just("256"),
            )),
        )))
        .map(builder::signed_integer_type)
        .boxed();
    let whitespace_parser = choice((just(' '), just('\t'), just('\r'), just('\n')))
        .map(builder::whitespace)
        .boxed();
    let yul_decimal_number_literal_parser = choice((
        just('0'),
        filter(|&c: &char| ('1' <= c && c <= '9'))
            .then(filter(|&c: &char| c.is_ascii_digit()).repeated()),
    ))
    .map(builder::yul_decimal_number_literal)
    .boxed();
    let yul_hex_literal_parser = just("0x")
        .ignore_then(choice((
            filter(|&c: &char| c.is_ascii_digit()),
            filter(|&c: &char| ('a' <= c && c <= 'f')),
            filter(|&c: &char| ('A' <= c && c <= 'F')),
        )))
        .then(
            choice((
                filter(|&c: &char| c.is_ascii_digit()),
                filter(|&c: &char| ('a' <= c && c <= 'f')),
                filter(|&c: &char| ('A' <= c && c <= 'F')),
            ))
            .repeated(),
        )
        .map(builder::yul_hex_literal)
        .boxed();
    let pragma_directive_parser = just("pragma")
        .ignore_then(filter(|&c: &char| c != ';'))
        .then(filter(|&c: &char| c != ';').repeated())
        .then_ignore(just(';'))
        .map(builder::pragma_directive)
        .boxed();
    let decimal_exponent_parser = choice((just('e'), just('E')))
        .then(just('-').or_not())
        .then(decimal_integer_parser.clone())
        .map(builder::decimal_exponent)
        .boxed();
    let decimal_float_parser = decimal_integer_parser
        .clone()
        .or_not()
        .then_ignore(just('.'))
        .then(decimal_integer_parser.clone())
        .map(builder::decimal_float)
        .boxed();
    let hex_byte_escape_parser = just('x')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(2usize)
                .at_most(2usize),
        )
        .map(builder::hex_byte_escape)
        .boxed();
    let hex_number_parser = just('0')
        .ignore_then(just('x'))
        .ignore_then(
            hex_character_parser
                .clone()
                .separated_by(just('_').or_not())
                .at_least(1usize),
        )
        .map(builder::hex_number)
        .boxed();
    let ignore_parser = choice((
        whitespace_parser.clone(),
        comment_parser.clone(),
        line_comment_parser.clone(),
    ))
    .repeated()
    .map(builder::ignore)
    .boxed();
    let identifier_part_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ))
    .map(builder::identifier_part)
    .boxed();
    let possibly_separated_pairs_of_hex_digits_parser = hex_character_parser
        .clone()
        .repeated()
        .at_least(2usize)
        .at_most(2usize)
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .map(builder::possibly_separated_pairs_of_hex_digits)
        .boxed();
    let ufixed_parser = just('u')
        .ignore_then(fixed_parser.clone())
        .map(builder::ufixed)
        .boxed();
    let unicode_escape_parser = just('u')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(4usize)
                .at_most(4usize),
        )
        .map(builder::unicode_escape)
        .boxed();
    let unsigned_integer_type_parser = just('u')
        .ignore_then(signed_integer_type_parser.clone())
        .map(builder::unsigned_integer_type)
        .boxed();
    let decimal_number_parser =
        choice((decimal_integer_parser.clone(), decimal_float_parser.clone()))
            .then(decimal_exponent_parser.clone().or_not())
            .map(builder::decimal_number)
            .boxed();
    let escape_sequence_parser = just('\\')
        .ignore_then(choice((
            ascii_escape_parser.clone(),
            hex_byte_escape_parser.clone(),
            unicode_escape_parser.clone(),
        )))
        .map(builder::escape_sequence)
        .boxed();
    let hex_string_literal_parser = just("hex")
        .ignore_then(choice((
            just('"')
                .ignore_then(
                    possibly_separated_pairs_of_hex_digits_parser
                        .clone()
                        .or_not(),
                )
                .then_ignore(just('"')),
            just('\'')
                .ignore_then(
                    possibly_separated_pairs_of_hex_digits_parser
                        .clone()
                        .or_not(),
                )
                .then_ignore(just('\'')),
        )))
        .map(builder::hex_string_literal)
        .boxed();
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .then(identifier_part_parser.clone().repeated())
        .map(builder::raw_identifier)
        .boxed();
    let break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::break_statement)
        .boxed();
    let continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::continue_statement)
        .boxed();
    let data_location_parser =
        choice((choice((just("memory"), just("storage"), just("calldata")))))
            .map(builder::data_location)
            .boxed();
    let elementary_type_parser = choice((
        just("bool").then_ignore(ignore_parser.clone()),
        just("string").then_ignore(ignore_parser.clone()),
        just("bytes").then_ignore(ignore_parser.clone()),
        signed_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        unsigned_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        fixed_bytes_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        fixed_parser.clone().then_ignore(ignore_parser.clone()),
        ufixed_parser.clone().then_ignore(ignore_parser.clone()),
    ))
    .map(builder::elementary_type)
    .boxed();
    let inline_array_expression_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            expression_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(builder::inline_array_expression)
        .boxed();
    let keyword_parser = choice((
        just("pragma").then_ignore(ignore_parser.clone()),
        just("abstract").then_ignore(ignore_parser.clone()),
        just("anonymous").then_ignore(ignore_parser.clone()),
        just("address").then_ignore(ignore_parser.clone()),
        just("as").then_ignore(ignore_parser.clone()),
        just("assembly").then_ignore(ignore_parser.clone()),
        just("bool").then_ignore(ignore_parser.clone()),
        just("break").then_ignore(ignore_parser.clone()),
        just("bytes").then_ignore(ignore_parser.clone()),
        just("calldata").then_ignore(ignore_parser.clone()),
        just("catch").then_ignore(ignore_parser.clone()),
        just("constant").then_ignore(ignore_parser.clone()),
        just("constructor").then_ignore(ignore_parser.clone()),
        just("continue").then_ignore(ignore_parser.clone()),
        just("contract").then_ignore(ignore_parser.clone()),
        just("delete").then_ignore(ignore_parser.clone()),
        just("do").then_ignore(ignore_parser.clone()),
        just("else").then_ignore(ignore_parser.clone()),
        just("emit").then_ignore(ignore_parser.clone()),
        just("enum").then_ignore(ignore_parser.clone()),
        just("event").then_ignore(ignore_parser.clone()),
        just("external").then_ignore(ignore_parser.clone()),
        just("fallback").then_ignore(ignore_parser.clone()),
        just("false").then_ignore(ignore_parser.clone()),
        just("for").then_ignore(ignore_parser.clone()),
        just("function").then_ignore(ignore_parser.clone()),
        just("hex").then_ignore(ignore_parser.clone()),
        just("if").then_ignore(ignore_parser.clone()),
        just("immutable").then_ignore(ignore_parser.clone()),
        just("import").then_ignore(ignore_parser.clone()),
        just("indexed").then_ignore(ignore_parser.clone()),
        just("interface").then_ignore(ignore_parser.clone()),
        just("internal").then_ignore(ignore_parser.clone()),
        just("is").then_ignore(ignore_parser.clone()),
        just("library").then_ignore(ignore_parser.clone()),
        just("mapping").then_ignore(ignore_parser.clone()),
        just("memory").then_ignore(ignore_parser.clone()),
        just("modifier").then_ignore(ignore_parser.clone()),
        just("new").then_ignore(ignore_parser.clone()),
        just("override").then_ignore(ignore_parser.clone()),
        just("payable").then_ignore(ignore_parser.clone()),
        just("private").then_ignore(ignore_parser.clone()),
        just("public").then_ignore(ignore_parser.clone()),
        just("pure").then_ignore(ignore_parser.clone()),
        just("receive").then_ignore(ignore_parser.clone()),
        just("return").then_ignore(ignore_parser.clone()),
        just("returns").then_ignore(ignore_parser.clone()),
        just("storage").then_ignore(ignore_parser.clone()),
        just("string").then_ignore(ignore_parser.clone()),
        just("struct").then_ignore(ignore_parser.clone()),
        just("true").then_ignore(ignore_parser.clone()),
        just("try").then_ignore(ignore_parser.clone()),
        just("type").then_ignore(ignore_parser.clone()),
        just("unchecked").then_ignore(ignore_parser.clone()),
        just("using").then_ignore(ignore_parser.clone()),
        just("view").then_ignore(ignore_parser.clone()),
        just("virtual").then_ignore(ignore_parser.clone()),
        just("while").then_ignore(ignore_parser.clone()),
        signed_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        unsigned_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        fixed_bytes_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        just("fixed").then_ignore(ignore_parser.clone()),
        just("ufixed").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::keyword)
    .boxed();
    let positional_argument_list_parser = expression_parser
        .clone()
        .separated_by(just(',').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::positional_argument_list)
        .boxed();
    let reserved_keyword_parser = choice((
        choice((
            just("after"),
            just("alias"),
            just("apply"),
            just("auto"),
            just("byte"),
            just("case"),
            just("copyof"),
            just("default"),
            just("define"),
            just("final"),
            just("implements"),
            just("in"),
            just("inline"),
            just("let"),
            just("macro"),
            just("match"),
        )),
        choice((
            just("mutable"),
            just("null"),
            just("of"),
            just("partial"),
            just("promise"),
            just("reference"),
            just("relocatable"),
            just("sealed"),
            just("sizeof"),
            just("static"),
            just("supports"),
            just("switch"),
            just("typedef"),
            just("typeof"),
            just("var"),
        )),
    ))
    .map(builder::reserved_keyword)
    .boxed();
    let state_mutability_specifier_parser =
        choice((choice((just("pure"), just("view"), just("payable")))))
            .map(builder::state_mutability_specifier)
            .boxed();
    let tuple_expression_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            expression_parser
                .clone()
                .or_not()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::tuple_expression)
        .boxed();
    let unchecked_block_parser = just("unchecked")
        .then_ignore(ignore_parser.clone())
        .ignore_then(block_parser.clone())
        .map(builder::unchecked_block)
        .boxed();
    let visibility_specifier_parser = choice(
        (choice((
            just("internal"),
            just("external"),
            just("private"),
            just("public"),
        ))),
    )
    .map(builder::visibility_specifier)
    .boxed();
    let yul_break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_break_statement)
        .boxed();
    let yul_continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_continue_statement)
        .boxed();
    let yul_evm_builtin_function_name_parser = choice((
        choice((
            just("stop"),
            just("add"),
            just("sub"),
            just("mul"),
            just("div"),
            just("sdiv"),
            just("mod"),
            just("smod"),
            just("exp"),
            just("not"),
            just("lt"),
            just("gt"),
            just("slt"),
            just("sgt"),
            just("eq"),
            just("iszero"),
        )),
        choice((
            just("and"),
            just("or"),
            just("xor"),
            just("byte"),
            just("shl"),
            just("shr"),
            just("sar"),
            just("addmod"),
            just("mulmod"),
            just("signextend"),
            just("keccak256"),
            just("pop"),
            just("mload"),
            just("mstore"),
            just("mstore8"),
            just("sload"),
        )),
        choice((
            just("sstore"),
            just("msize"),
            just("gas"),
            just("address"),
            just("balance"),
            just("selfbalance"),
            just("caller"),
            just("callvalue"),
            just("calldataload"),
            just("calldatasize"),
            just("calldatacopy"),
            just("extcodesize"),
            just("extcodecopy"),
            just("returndatasize"),
            just("returndatacopy"),
            just("extcodehash"),
        )),
        choice((
            just("create"),
            just("create2"),
            just("call"),
            just("callcode"),
            just("delegatecall"),
            just("staticcall"),
            just("return"),
            just("revert"),
            just("selfdestruct"),
            just("invalid"),
            just("log0"),
            just("log1"),
            just("log2"),
            just("log3"),
            just("log4"),
            just("chainid"),
        )),
        choice((
            just("origin"),
            just("gasprice"),
            just("blockhash"),
            just("coinbase"),
            just("timestamp"),
            just("number"),
            just("difficulty"),
            just("gaslimit"),
            just("basefee"),
        )),
    ))
    .map(builder::yul_evm_builtin_function_name)
    .boxed();
    let yul_keyword_parser = choice(
        (choice((
            just("break"),
            just("case"),
            just("continue"),
            just("default"),
            just("for"),
            just("function"),
            just("if"),
            just("leave"),
            just("let"),
            just("switch"),
            just("hex"),
        ))),
    )
    .map(builder::yul_keyword)
    .boxed();
    let yul_leave_statement_parser = just("leave")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_leave_statement)
        .boxed();
    let double_quoted_ascii_string_literal_parser = just('"')
        .ignore_then(choice((todo(), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('"'))
        .map(builder::double_quoted_ascii_string_literal)
        .boxed();
    let double_quoted_unicode_string_literal_parser = just("unicode\"")
        .ignore_then(choice((none_of("\"\\\n\r"), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('"'))
        .map(builder::double_quoted_unicode_string_literal)
        .boxed();
    let single_quoted_ascii_string_literal_parser = just('\'')
        .ignore_then(choice((todo(), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('\''))
        .map(builder::single_quoted_ascii_string_literal)
        .boxed();
    let single_quoted_unicode_string_literal_parser = just("unicode'")
        .ignore_then(choice((none_of("'\\\n\r"), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('\''))
        .map(builder::single_quoted_unicode_string_literal)
        .boxed();
    let elementary_type_with_payable_parser = choice((
        just("address")
            .then_ignore(ignore_parser.clone())
            .ignore_then(just("payable").then_ignore(ignore_parser.clone()).or_not()),
        elementary_type_parser.clone(),
    ))
    .map(builder::elementary_type_with_payable)
    .boxed();
    let elementary_type_without_payable_parser = choice((
        just("address").then_ignore(ignore_parser.clone()),
        elementary_type_parser.clone(),
    ))
    .map(builder::elementary_type_without_payable)
    .boxed();
    let numeric_literal_parser = choice((
        decimal_number_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        hex_number_parser.clone().then_ignore(ignore_parser.clone()),
    ))
    .then(
        number_unit_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .or_not(),
    )
    .map(builder::numeric_literal)
    .boxed();
    let reserved_word_parser = choice((
        keyword_parser.clone(),
        reserved_keyword_parser.clone(),
        number_unit_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .map(builder::reserved_word)
    .boxed();
    let yul_reserved_word_parser = choice((
        yul_keyword_parser.clone(),
        yul_evm_builtin_function_name_parser.clone(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .map(builder::yul_reserved_word)
    .boxed();
    let ascii_string_literal_parser = choice((
        single_quoted_ascii_string_literal_parser.clone(),
        double_quoted_ascii_string_literal_parser.clone(),
    ))
    .map(builder::ascii_string_literal)
    .boxed();
    let unicode_string_literal_parser = choice((
        single_quoted_unicode_string_literal_parser.clone(),
        double_quoted_unicode_string_literal_parser.clone(),
    ))
    .map(builder::unicode_string_literal)
    .boxed();
    let assembly_flags_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            double_quoted_ascii_string_literal_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::assembly_flags)
        .boxed();
    let identifier_parser = todo().map(builder::identifier).boxed();
    let yul_identifier_parser = todo().map(builder::yul_identifier).boxed();
    let ascii_string_literal_parser = ascii_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .map(builder::ascii_string_literal)
        .boxed();
    let enum_definition_parser = just("enum")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(
            identifier_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::enum_definition)
        .boxed();
    let identifier_path_parser = identifier_parser
        .clone()
        .separated_by(just('.').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::identifier_path)
        .boxed();
    let named_argument_parser = identifier_parser
        .clone()
        .then_ignore(just(':').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .map(builder::named_argument)
        .boxed();
    let parameter_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone().or_not())
        .map(builder::parameter_declaration)
        .boxed();
    let selected_import_parser = identifier_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .or_not(),
        )
        .map(builder::selected_import)
        .boxed();
    let unicode_string_literal_parser = unicode_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .map(builder::unicode_string_literal)
        .boxed();
    let user_defined_value_type_definition_parser = just("type")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("is").then_ignore(ignore_parser.clone()))
        .then(elementary_type_with_payable_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::user_defined_value_type_definition)
        .boxed();
    let yul_function_call_parser = choice((
        yul_identifier_parser.clone(),
        yul_evm_builtin_function_name_parser.clone(),
    ))
    .then_ignore(just('(').then_ignore(ignore_parser.clone()))
    .then(
        yul_expression_parser
            .clone()
            .separated_by(just(',').then_ignore(ignore_parser.clone())),
    )
    .then_ignore(just(')').then_ignore(ignore_parser.clone()))
    .map(builder::yul_function_call)
    .boxed();
    let yul_function_definition_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_identifier_parser.clone())
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(
            yul_identifier_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(
            just("->")
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    yul_identifier_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .or_not(),
        )
        .then(yul_block_parser.clone())
        .map(builder::yul_function_definition)
        .boxed();
    let yul_path_parser = yul_identifier_parser
        .clone()
        .then(
            just('.')
                .then_ignore(ignore_parser.clone())
                .ignore_then(choice((
                    yul_identifier_parser.clone(),
                    yul_evm_builtin_function_name_parser.clone(),
                )))
                .repeated(),
        )
        .map(builder::yul_path)
        .boxed();
    let import_path_parser = ascii_string_literal_parser
        .clone()
        .map(builder::import_path)
        .boxed();
    let literal_parser = choice((
        ascii_string_literal_parser.clone(),
        unicode_string_literal_parser.clone(),
        numeric_literal_parser.clone(),
        hex_string_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .map(builder::literal)
    .boxed();
    let mapping_type_parser = just("mapping")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(choice((
            elementary_type_without_payable_parser.clone(),
            identifier_path_parser.clone(),
        )))
        .then_ignore(just("=>").then_ignore(ignore_parser.clone()))
        .then(type_name_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::mapping_type)
        .boxed();
    let named_argument_list_parser = just('{')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            named_argument_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::named_argument_list)
        .boxed();
    let non_empty_parameter_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            parameter_declaration_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::non_empty_parameter_list)
        .boxed();
    let override_specifier_parser = just("override")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            just('(')
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    identifier_path_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize)
                        .at_most(1usize),
                )
                .then_ignore(just(')').then_ignore(ignore_parser.clone()))
                .or_not(),
        )
        .map(builder::override_specifier)
        .boxed();
    let parameter_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            parameter_declaration_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::parameter_list)
        .boxed();
    let yul_literal_parser = choice((
        yul_decimal_number_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        yul_hex_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        ascii_string_literal_parser.clone(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        hex_string_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .map(builder::yul_literal)
    .boxed();
    let argument_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            choice((
                positional_argument_list_parser.clone(),
                named_argument_list_parser.clone(),
            ))
            .or_not(),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::argument_list)
        .boxed();
    let catch_clause_parser = just("catch")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            identifier_parser
                .clone()
                .or_not()
                .then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(block_parser.clone())
        .map(builder::catch_clause)
        .boxed();
    let function_type_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(
            choice((
                visibility_specifier_parser.clone(),
                state_mutability_specifier_parser.clone(),
            ))
            .repeated(),
        )
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .map(builder::function_type)
        .boxed();
    let method_attribute_parser = choice((
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::method_attribute)
    .boxed();
    let selecting_import_directive_parser = just('{')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            selected_import_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .then_ignore(just("from").then_ignore(ignore_parser.clone()))
        .then(import_path_parser.clone())
        .map(builder::selecting_import_directive)
        .boxed();
    let simple_import_directive_parser = import_path_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .repeated(),
        )
        .map(builder::simple_import_directive)
        .boxed();
    let star_import_directive_parser = just('*')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just("as").then_ignore(ignore_parser.clone()))
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("from").then_ignore(ignore_parser.clone()))
        .then(import_path_parser.clone())
        .map(builder::star_import_directive)
        .boxed();
    let state_variable_attribute_parser = choice((
        just("public").then_ignore(ignore_parser.clone()),
        just("private").then_ignore(ignore_parser.clone()),
        just("internal").then_ignore(ignore_parser.clone()),
        just("constant").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
        just("immutable").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::state_variable_attribute)
    .boxed();
    yul_expression_parser.define(
        choice((
            yul_path_parser.clone(),
            yul_function_call_parser.clone(),
            yul_literal_parser.clone(),
        ))
        .map(builder::yul_expression)
        .boxed(),
    );
    expression_parser.define(
        choice((
            expression_parser
                .clone()
                .then_ignore(just('[').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone().or_not())
                .then(
                    just(':')
                        .then_ignore(ignore_parser.clone())
                        .ignore_then(expression_parser.clone().or_not())
                        .or_not(),
                )
                .then_ignore(just(']').then_ignore(ignore_parser.clone())),
            expression_parser
                .clone()
                .then_ignore(just('.').then_ignore(ignore_parser.clone()))
                .then(choice((
                    identifier_parser.clone(),
                    just("address").then_ignore(ignore_parser.clone()),
                ))),
            expression_parser
                .clone()
                .then_ignore(just('{').then_ignore(ignore_parser.clone()))
                .then(
                    named_argument_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .then_ignore(just('}').then_ignore(ignore_parser.clone())),
            expression_parser.clone().then(argument_list_parser.clone()),
            just("payable")
                .then_ignore(ignore_parser.clone())
                .ignore_then(argument_list_parser.clone()),
            just("type")
                .then_ignore(ignore_parser.clone())
                .ignore_then(just('(').then_ignore(ignore_parser.clone()))
                .ignore_then(type_name_parser.clone())
                .then_ignore(just(')').then_ignore(ignore_parser.clone())),
            choice(
                (choice((
                    just("++"),
                    just("--"),
                    just("!"),
                    just("~"),
                    just("delete"),
                    just("-"),
                ))),
            )
            .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((choice((just("++"), just("--")))))),
            expression_parser
                .clone()
                .then_ignore(just("**").then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((
                    just('*').then_ignore(ignore_parser.clone()),
                    just('/').then_ignore(ignore_parser.clone()),
                    just('%').then_ignore(ignore_parser.clone()),
                )))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((
                    just('+').then_ignore(ignore_parser.clone()),
                    just('-').then_ignore(ignore_parser.clone()),
                )))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((choice((just("<<"), just(">>"), just(">>>"))))))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just('&').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just('^').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just('|').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice(
                    (choice((just("<"), just(">"), just("<="), just(">=")))),
                ))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((choice((just("=="), just("!="))))))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just("&&").then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just("||").then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then_ignore(just('?').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone())
                .then_ignore(just(':').then_ignore(ignore_parser.clone()))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice(
                    (choice((
                        just("="),
                        just("|="),
                        just("^="),
                        just("&="),
                        just("<<="),
                        just(">>="),
                        just(">>>="),
                        just("+="),
                        just("-="),
                        just("*="),
                        just("/="),
                        just("%="),
                    ))),
                ))
                .then(expression_parser.clone()),
            just("new")
                .then_ignore(ignore_parser.clone())
                .ignore_then(type_name_parser.clone()),
            tuple_expression_parser.clone(),
            inline_array_expression_parser.clone(),
            choice((
                identifier_parser.clone(),
                literal_parser.clone(),
                elementary_type_without_payable_parser.clone(),
            )),
        ))
        .map(builder::expression)
        .boxed(),
    );
    let import_directive_parser = just("import")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice((
            simple_import_directive_parser.clone(),
            star_import_directive_parser.clone(),
            selecting_import_directive_parser.clone(),
        )))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::import_directive)
        .boxed();
    let inheritance_specifier_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .map(builder::inheritance_specifier)
        .boxed();
    let modifier_invocation_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .map(builder::modifier_invocation)
        .boxed();
    let yul_assignment_parser = yul_path_parser
        .clone()
        .then(choice((
            just(":=")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_expression_parser.clone()),
            just(',')
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_path_parser.clone())
                .repeated()
                .at_least(1usize)
                .then_ignore(just(":=").then_ignore(ignore_parser.clone()))
                .then(yul_function_call_parser.clone()),
        )))
        .map(builder::yul_assignment)
        .boxed();
    let yul_for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .then(yul_block_parser.clone())
        .map(builder::yul_for_statement)
        .boxed();
    let yul_if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .map(builder::yul_if_statement)
        .boxed();
    let yul_switch_statement_parser = just("switch")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .then(choice((
            just("case")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_literal_parser.clone())
                .then(yul_block_parser.clone())
                .repeated()
                .at_least(1usize)
                .then(
                    just("default")
                        .then_ignore(ignore_parser.clone())
                        .ignore_then(yul_block_parser.clone())
                        .or_not(),
                ),
            just("default")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_block_parser.clone()),
        )))
        .map(builder::yul_switch_statement)
        .boxed();
    let yul_variable_declaration_parser = just("let")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_identifier_parser.clone())
        .then(
            choice((
                just(":=")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(yul_expression_parser.clone()),
                just(',')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(yul_identifier_parser.clone())
                    .or_not()
                    .then(
                        just(":=")
                            .then_ignore(ignore_parser.clone())
                            .ignore_then(yul_function_call_parser.clone())
                            .or_not(),
                    ),
            ))
            .or_not(),
        )
        .map(builder::yul_variable_declaration)
        .boxed();
    let constructor_attribute_parser = choice((
        modifier_invocation_parser.clone(),
        just("payable").then_ignore(ignore_parser.clone()),
        just("internal").then_ignore(ignore_parser.clone()),
        just("public").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::constructor_attribute)
    .boxed();
    let do_while_statement_parser = just("do")
        .then_ignore(ignore_parser.clone())
        .ignore_then(statement_parser.clone())
        .then_ignore(just("while").then_ignore(ignore_parser.clone()))
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::do_while_statement)
        .boxed();
    let emit_statement_parser = just("emit")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::emit_statement)
        .boxed();
    let expression_statement_parser = expression_parser
        .clone()
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::expression_statement)
        .boxed();
    let fallback_function_attribute_parser = choice((
        just("external").then_ignore(ignore_parser.clone()),
        state_mutability_specifier_parser.clone(),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::fallback_function_attribute)
    .boxed();
    let function_attribute_parser = choice((
        visibility_specifier_parser.clone(),
        state_mutability_specifier_parser.clone(),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::function_attribute)
    .boxed();
    let if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .then(
            just("else")
                .then_ignore(ignore_parser.clone())
                .ignore_then(statement_parser.clone())
                .or_not(),
        )
        .map(builder::if_statement)
        .boxed();
    let inheritance_specifier_list_parser = just("is")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            inheritance_specifier_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .map(builder::inheritance_specifier_list)
        .boxed();
    let receive_function_attribute_parser = choice((
        just("external").then_ignore(ignore_parser.clone()),
        just("payable").then_ignore(ignore_parser.clone()),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::receive_function_attribute)
    .boxed();
    let return_statement_parser = just("return")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone().or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::return_statement)
        .boxed();
    let revert_statement_parser = just("revert")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::revert_statement)
        .boxed();
    let try_statement_parser = just("try")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(block_parser.clone())
        .then(catch_clause_parser.clone())
        .then(catch_clause_parser.clone().repeated())
        .map(builder::try_statement)
        .boxed();
    type_name_parser.define(
        choice((
            elementary_type_with_payable_parser.clone(),
            function_type_parser.clone(),
            mapping_type_parser.clone(),
            identifier_path_parser.clone(),
        ))
        .then(
            just('[')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone().or_not())
                .then_ignore(just(']').then_ignore(ignore_parser.clone()))
                .repeated(),
        )
        .map(builder::type_name)
        .boxed(),
    );
    let while_statement_parser = just("while")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .map(builder::while_statement)
        .boxed();
    let yul_statement_parser = choice((
        yul_block_parser.clone(),
        yul_variable_declaration_parser.clone(),
        yul_function_definition_parser.clone(),
        yul_assignment_parser.clone(),
        yul_function_call_parser.clone(),
        yul_if_statement_parser.clone(),
        yul_for_statement_parser.clone(),
        yul_switch_statement_parser.clone(),
        yul_leave_statement_parser.clone(),
        yul_break_statement_parser.clone(),
        yul_continue_statement_parser.clone(),
    ))
    .map(builder::yul_statement)
    .boxed();
    let constant_definition_parser = type_name_parser
        .clone()
        .then_ignore(just("constant").then_ignore(ignore_parser.clone()))
        .then(identifier_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::constant_definition)
        .boxed();
    let error_parameter_parser = type_name_parser
        .clone()
        .then(identifier_parser.clone().or_not())
        .map(builder::error_parameter)
        .boxed();
    let event_parameter_parser = type_name_parser
        .clone()
        .then(just("indexed").then_ignore(ignore_parser.clone()).or_not())
        .then(identifier_parser.clone().or_not())
        .map(builder::event_parameter)
        .boxed();
    let state_variable_declaration_parser = type_name_parser
        .clone()
        .then(state_variable_attribute_parser.clone().repeated())
        .then(identifier_parser.clone())
        .then(
            just('=')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone())
                .or_not(),
        )
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::state_variable_declaration)
        .boxed();
    let struct_definition_parser = just("struct")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(
            type_name_parser
                .clone()
                .then(identifier_parser.clone())
                .then_ignore(just(';').then_ignore(ignore_parser.clone()))
                .repeated()
                .at_least(1usize),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::struct_definition)
        .boxed();
    let using_directive_parser = just("using")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice((
            identifier_path_parser.clone(),
            just('{')
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    identifier_path_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .then_ignore(just('}').then_ignore(ignore_parser.clone())),
        )))
        .then_ignore(just("for").then_ignore(ignore_parser.clone()))
        .then(choice((
            just('*').then_ignore(ignore_parser.clone()),
            type_name_parser.clone(),
        )))
        .then(just("global").then_ignore(ignore_parser.clone()).or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::using_directive)
        .boxed();
    let variable_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone())
        .map(builder::variable_declaration)
        .boxed();
    yul_block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(yul_statement_parser.clone().repeated())
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .map(builder::yul_block)
            .boxed(),
    );
    let assembly_statement_parser = just("assembly")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            just("\"evmasm\"")
                .then_ignore(ignore_parser.clone())
                .or_not(),
        )
        .then(assembly_flags_parser.clone().or_not())
        .then(yul_block_parser.clone())
        .map(builder::assembly_statement)
        .boxed();
    let directive_parser = choice((
        pragma_directive_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        import_directive_parser.clone(),
        using_directive_parser.clone(),
    ))
    .map(builder::directive)
    .boxed();
    let error_definition_parser = just("error")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(
            error_parameter_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::error_definition)
        .boxed();
    let event_definition_parser = just("event")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(
            event_parameter_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(
            just("anonymous")
                .then_ignore(ignore_parser.clone())
                .or_not(),
        )
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::event_definition)
        .boxed();
    let variable_declaration_tuple_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(',').then_ignore(ignore_parser.clone()).repeated())
        .then(variable_declaration_parser.clone())
        .then(
            just(',')
                .then_ignore(ignore_parser.clone())
                .ignore_then(variable_declaration_parser.clone().or_not())
                .repeated(),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::variable_declaration_tuple)
        .boxed();
    let variable_declaration_statement_parser = choice((
        variable_declaration_parser.clone().then(
            just('=')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone())
                .or_not(),
        ),
        variable_declaration_tuple_parser
            .clone()
            .then_ignore(just('=').then_ignore(ignore_parser.clone()))
            .then(expression_parser.clone()),
    ))
    .then_ignore(just(';').then_ignore(ignore_parser.clone()))
    .map(builder::variable_declaration_statement)
    .boxed();
    let simple_statement_parser = choice((
        variable_declaration_statement_parser.clone(),
        expression_statement_parser.clone(),
    ))
    .map(builder::simple_statement)
    .boxed();
    let for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(choice((
            simple_statement_parser.clone(),
            just(';').then_ignore(ignore_parser.clone()),
        )))
        .then(choice((
            expression_statement_parser.clone(),
            just(';').then_ignore(ignore_parser.clone()),
        )))
        .then(expression_parser.clone().or_not())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .map(builder::for_statement)
        .boxed();
    statement_parser.define(
        choice((
            block_parser.clone(),
            simple_statement_parser.clone(),
            if_statement_parser.clone(),
            for_statement_parser.clone(),
            while_statement_parser.clone(),
            do_while_statement_parser.clone(),
            continue_statement_parser.clone(),
            break_statement_parser.clone(),
            try_statement_parser.clone(),
            return_statement_parser.clone(),
            emit_statement_parser.clone(),
            revert_statement_parser.clone(),
            assembly_statement_parser.clone(),
        ))
        .map(builder::statement)
        .boxed(),
    );
    block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(
                choice((statement_parser.clone(), unchecked_block_parser.clone())).repeated(),
            )
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .map(builder::block)
            .boxed(),
    );
    let constructor_definition_parser = just("constructor")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(constructor_attribute_parser.clone().repeated())
        .then(block_parser.clone())
        .map(builder::constructor_definition)
        .boxed();
    let fallback_function_definition_parser = just("fallback")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(fallback_function_attribute_parser.clone().repeated())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::fallback_function_definition)
        .boxed();
    let function_definition_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice((
            identifier_parser.clone(),
            just("fallback").then_ignore(ignore_parser.clone()),
            just("receive").then_ignore(ignore_parser.clone()),
        )))
        .then(parameter_list_parser.clone())
        .then(function_attribute_parser.clone().repeated())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::function_definition)
        .boxed();
    let modifier_definition_parser = just("modifier")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(parameter_list_parser.clone().or_not())
        .then(method_attribute_parser.clone().repeated())
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::modifier_definition)
        .boxed();
    let receive_function_definition_parser = just("receive")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(receive_function_attribute_parser.clone().repeated())
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::receive_function_definition)
        .boxed();
    let contract_body_element_parser = choice((
        using_directive_parser.clone(),
        constructor_definition_parser.clone(),
        function_definition_parser.clone(),
        fallback_function_definition_parser.clone(),
        receive_function_definition_parser.clone(),
        modifier_definition_parser.clone(),
        struct_definition_parser.clone(),
        enum_definition_parser.clone(),
        user_defined_value_type_definition_parser.clone(),
        event_definition_parser.clone(),
        error_definition_parser.clone(),
        state_variable_declaration_parser.clone(),
    ))
    .map(builder::contract_body_element)
    .boxed();
    let contract_definition_parser = just("abstract")
        .then_ignore(ignore_parser.clone())
        .or_not()
        .then_ignore(just("contract").then_ignore(ignore_parser.clone()))
        .then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::contract_definition)
        .boxed();
    let interface_definition_parser = just("interface")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::interface_definition)
        .boxed();
    let library_definition_parser = just("library")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::library_definition)
        .boxed();
    let definition_parser = choice((
        contract_definition_parser.clone(),
        interface_definition_parser.clone(),
        library_definition_parser.clone(),
        function_definition_parser.clone(),
        constant_definition_parser.clone(),
        struct_definition_parser.clone(),
        enum_definition_parser.clone(),
        user_defined_value_type_definition_parser.clone(),
        error_definition_parser.clone(),
    ))
    .map(builder::definition)
    .boxed();
    let source_unit_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then(choice((directive_parser.clone(), definition_parser.clone())).repeated())
        .then_ignore(end())
        .map(builder::source_unit)
        .boxed();
    source_unit_parser.recover_with(skip_then_retry_until([]))
}
