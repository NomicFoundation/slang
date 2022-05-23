use chumsky::{prelude::*, Parser};

use super::builder;
pub type SourceUnitParserResultType = ();

pub fn create_source_unit_parser(
) -> impl Parser<char, SourceUnitParserResultType, Error = Simple<char>> {
    let mut yul_block_parser = Recursive::declare();
    let mut block_parser = Recursive::declare();
    let mut type_name_parser = Recursive::declare();
    let mut yul_expression_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
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
    .map(builder::ascii_escape);
    let boolean_literal_parser =
        choice((just("true"), just("false"))).map(builder::boolean_literal);
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
        .map(builder::comment);
    let decimal_integer_parser = filter(|&c: &char| c.is_ascii_digit())
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .map(builder::decimal_integer);
    let fixed_parser = just("fixed")
        .ignore_then(
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .then_ignore(just('x'))
                .then(filter(|&c: &char| ('1' <= c && c <= '9')))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .or_not(),
        )
        .map(builder::fixed);
    let fixed_bytes_parser = just("bytes")
        .ignore_then(choice((
            just('1'),
            just('2'),
            just('3'),
            just('4'),
            just('5'),
            just('6'),
            just('7'),
            just('8'),
            just('9'),
            just("10"),
            just("11"),
            just("12"),
            just("13"),
            just("14"),
            just("15"),
            just("16"),
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
        )))
        .map(builder::fixed_bytes);
    let hex_character_parser = choice((
        filter(|&c: &char| c.is_ascii_digit()),
        filter(|&c: &char| ('a' <= c && c <= 'f')),
        filter(|&c: &char| ('A' <= c && c <= 'F')),
    ))
    .map(builder::hex_character);
    let identifier_start_parser = choice((
        just('_'),
        just('$'),
        filter(|&c: &char| c.is_ascii_lowercase()),
        filter(|&c: &char| c.is_ascii_uppercase()),
    ))
    .map(builder::identifier_start);
    let line_comment_parser = just("//")
        .ignore_then(none_of("\n\r").repeated())
        .map(builder::line_comment);
    let number_unit_parser = choice((
        just("wei"),
        just("gwei"),
        just("ether"),
        just("seconds"),
        just("minutes"),
        just("hours"),
        just("days"),
        just("weeks"),
        just("years"),
    ))
    .map(builder::number_unit);
    let signed_integer_type_parser = just("int")
        .ignore_then(choice((
            just('8'),
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
        )))
        .map(builder::signed_integer_type);
    let whitespace_parser =
        choice((just(' '), just('\t'), just('\r'), just('\n'))).map(builder::whitespace);
    let yul_decimal_number_literal_parser = choice((
        just('0'),
        filter(|&c: &char| ('1' <= c && c <= '9'))
            .then(filter(|&c: &char| c.is_ascii_digit()).repeated()),
    ))
    .map(builder::yul_decimal_number_literal);
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
        .map(builder::yul_hex_literal);
    let pragma_directive_parser = just("pragma")
        .ignore_then(filter(|&c: &char| c != ';'))
        .then(filter(|&c: &char| c != ';').repeated())
        .then_ignore(just(';'))
        .map(builder::pragma_directive);
    let decimal_exponent_parser = choice((just('e'), just('E')))
        .then(just('-').or_not())
        .then(decimal_integer_parser.clone())
        .map(builder::decimal_exponent);
    let decimal_float_parser = decimal_integer_parser
        .clone()
        .or_not()
        .then_ignore(just('.'))
        .then(decimal_integer_parser.clone())
        .map(builder::decimal_float);
    let hex_byte_escape_parser = just('x')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(2usize)
                .at_most(2usize),
        )
        .map(builder::hex_byte_escape);
    let hex_number_parser = just('0')
        .ignore_then(just('x'))
        .ignore_then(
            hex_character_parser
                .clone()
                .separated_by(just('_').or_not())
                .at_least(1usize),
        )
        .map(builder::hex_number);
    let ignore_parser = choice((
        whitespace_parser.clone(),
        comment_parser.clone(),
        line_comment_parser.clone(),
    ))
    .repeated()
    .map(builder::ignore);
    let identifier_part_parser = choice((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ))
    .map(builder::identifier_part);
    let possibly_separated_pairs_of_hex_digits_parser = hex_character_parser
        .clone()
        .repeated()
        .at_least(2usize)
        .at_most(2usize)
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .map(builder::possibly_separated_pairs_of_hex_digits);
    let ufixed_parser = just('u')
        .ignore_then(fixed_parser.clone())
        .map(builder::ufixed);
    let unicode_escape_parser = just('u')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(4usize)
                .at_most(4usize),
        )
        .map(builder::unicode_escape);
    let unsigned_integer_type_parser = just('u')
        .ignore_then(signed_integer_type_parser.clone())
        .map(builder::unsigned_integer_type);
    let decimal_number_parser =
        choice((decimal_integer_parser.clone(), decimal_float_parser.clone()))
            .then(decimal_exponent_parser.clone().or_not())
            .map(builder::decimal_number);
    let escape_sequence_parser = just('\\')
        .ignore_then(choice((
            ascii_escape_parser.clone(),
            hex_byte_escape_parser.clone(),
            unicode_escape_parser.clone(),
        )))
        .map(builder::escape_sequence);
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
        .map(builder::hex_string_literal);
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .then(identifier_part_parser.clone().repeated())
        .map(builder::raw_identifier);
    let break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::break_statement);
    let continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::continue_statement);
    let data_location_parser = choice((
        just("memory").then_ignore(ignore_parser.clone()),
        just("storage").then_ignore(ignore_parser.clone()),
        just("calldata").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::data_location);
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
    .map(builder::elementary_type);
    let inline_array_expression_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            expression_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(builder::inline_array_expression);
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
    .map(builder::keyword);
    let positional_argument_list_parser = expression_parser
        .clone()
        .separated_by(just(',').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::positional_argument_list);
    let reserved_keyword_parser = choice((
        just("after").then_ignore(ignore_parser.clone()),
        just("alias").then_ignore(ignore_parser.clone()),
        just("apply").then_ignore(ignore_parser.clone()),
        just("auto").then_ignore(ignore_parser.clone()),
        just("byte").then_ignore(ignore_parser.clone()),
        just("case").then_ignore(ignore_parser.clone()),
        just("copyof").then_ignore(ignore_parser.clone()),
        just("default").then_ignore(ignore_parser.clone()),
        just("define").then_ignore(ignore_parser.clone()),
        just("final").then_ignore(ignore_parser.clone()),
        just("implements").then_ignore(ignore_parser.clone()),
        just("in").then_ignore(ignore_parser.clone()),
        just("inline").then_ignore(ignore_parser.clone()),
        just("let").then_ignore(ignore_parser.clone()),
        just("macro").then_ignore(ignore_parser.clone()),
        just("match").then_ignore(ignore_parser.clone()),
        just("mutable").then_ignore(ignore_parser.clone()),
        just("null").then_ignore(ignore_parser.clone()),
        just("of").then_ignore(ignore_parser.clone()),
        just("partial").then_ignore(ignore_parser.clone()),
        just("promise").then_ignore(ignore_parser.clone()),
        just("reference").then_ignore(ignore_parser.clone()),
        just("relocatable").then_ignore(ignore_parser.clone()),
        just("sealed").then_ignore(ignore_parser.clone()),
        just("sizeof").then_ignore(ignore_parser.clone()),
        just("static").then_ignore(ignore_parser.clone()),
        just("supports").then_ignore(ignore_parser.clone()),
        just("switch").then_ignore(ignore_parser.clone()),
        just("typedef").then_ignore(ignore_parser.clone()),
        just("typeof").then_ignore(ignore_parser.clone()),
        just("var").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::reserved_keyword);
    let state_mutability_specifier_parser = choice((
        just("pure").then_ignore(ignore_parser.clone()),
        just("view").then_ignore(ignore_parser.clone()),
        just("payable").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::state_mutability_specifier);
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
        .map(builder::tuple_expression);
    let unchecked_block_parser = just("unchecked")
        .then_ignore(ignore_parser.clone())
        .ignore_then(block_parser.clone())
        .map(builder::unchecked_block);
    let visibility_specifier_parser = choice((
        just("internal").then_ignore(ignore_parser.clone()),
        just("external").then_ignore(ignore_parser.clone()),
        just("private").then_ignore(ignore_parser.clone()),
        just("public").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::visibility_specifier);
    let yul_break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_break_statement);
    let yul_continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_continue_statement);
    let yul_evm_builtin_function_name_parser = choice((
        just("stop").then_ignore(ignore_parser.clone()),
        just("add").then_ignore(ignore_parser.clone()),
        just("sub").then_ignore(ignore_parser.clone()),
        just("mul").then_ignore(ignore_parser.clone()),
        just("div").then_ignore(ignore_parser.clone()),
        just("sdiv").then_ignore(ignore_parser.clone()),
        just("mod").then_ignore(ignore_parser.clone()),
        just("smod").then_ignore(ignore_parser.clone()),
        just("exp").then_ignore(ignore_parser.clone()),
        just("not").then_ignore(ignore_parser.clone()),
        just("lt").then_ignore(ignore_parser.clone()),
        just("gt").then_ignore(ignore_parser.clone()),
        just("slt").then_ignore(ignore_parser.clone()),
        just("sgt").then_ignore(ignore_parser.clone()),
        just("eq").then_ignore(ignore_parser.clone()),
        just("iszero").then_ignore(ignore_parser.clone()),
        just("and").then_ignore(ignore_parser.clone()),
        just("or").then_ignore(ignore_parser.clone()),
        just("xor").then_ignore(ignore_parser.clone()),
        just("byte").then_ignore(ignore_parser.clone()),
        just("shl").then_ignore(ignore_parser.clone()),
        just("shr").then_ignore(ignore_parser.clone()),
        just("sar").then_ignore(ignore_parser.clone()),
        just("addmod").then_ignore(ignore_parser.clone()),
        just("mulmod").then_ignore(ignore_parser.clone()),
        just("signextend").then_ignore(ignore_parser.clone()),
        just("keccak256").then_ignore(ignore_parser.clone()),
        just("pop").then_ignore(ignore_parser.clone()),
        just("mload").then_ignore(ignore_parser.clone()),
        just("mstore").then_ignore(ignore_parser.clone()),
        just("mstore8").then_ignore(ignore_parser.clone()),
        just("sload").then_ignore(ignore_parser.clone()),
        just("sstore").then_ignore(ignore_parser.clone()),
        just("msize").then_ignore(ignore_parser.clone()),
        just("gas").then_ignore(ignore_parser.clone()),
        just("address").then_ignore(ignore_parser.clone()),
        just("balance").then_ignore(ignore_parser.clone()),
        just("selfbalance").then_ignore(ignore_parser.clone()),
        just("caller").then_ignore(ignore_parser.clone()),
        just("callvalue").then_ignore(ignore_parser.clone()),
        just("calldataload").then_ignore(ignore_parser.clone()),
        just("calldatasize").then_ignore(ignore_parser.clone()),
        just("calldatacopy").then_ignore(ignore_parser.clone()),
        just("extcodesize").then_ignore(ignore_parser.clone()),
        just("extcodecopy").then_ignore(ignore_parser.clone()),
        just("returndatasize").then_ignore(ignore_parser.clone()),
        just("returndatacopy").then_ignore(ignore_parser.clone()),
        just("extcodehash").then_ignore(ignore_parser.clone()),
        just("create").then_ignore(ignore_parser.clone()),
        just("create2").then_ignore(ignore_parser.clone()),
        just("call").then_ignore(ignore_parser.clone()),
        just("callcode").then_ignore(ignore_parser.clone()),
        just("delegatecall").then_ignore(ignore_parser.clone()),
        just("staticcall").then_ignore(ignore_parser.clone()),
        just("return").then_ignore(ignore_parser.clone()),
        just("revert").then_ignore(ignore_parser.clone()),
        just("selfdestruct").then_ignore(ignore_parser.clone()),
        just("invalid").then_ignore(ignore_parser.clone()),
        just("log0").then_ignore(ignore_parser.clone()),
        just("log1").then_ignore(ignore_parser.clone()),
        just("log2").then_ignore(ignore_parser.clone()),
        just("log3").then_ignore(ignore_parser.clone()),
        just("log4").then_ignore(ignore_parser.clone()),
        just("chainid").then_ignore(ignore_parser.clone()),
        just("origin").then_ignore(ignore_parser.clone()),
        just("gasprice").then_ignore(ignore_parser.clone()),
        just("blockhash").then_ignore(ignore_parser.clone()),
        just("coinbase").then_ignore(ignore_parser.clone()),
        just("timestamp").then_ignore(ignore_parser.clone()),
        just("number").then_ignore(ignore_parser.clone()),
        just("difficulty").then_ignore(ignore_parser.clone()),
        just("gaslimit").then_ignore(ignore_parser.clone()),
        just("basefee").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::yul_evm_builtin_function_name);
    let yul_keyword_parser = choice((
        just("break").then_ignore(ignore_parser.clone()),
        just("case").then_ignore(ignore_parser.clone()),
        just("continue").then_ignore(ignore_parser.clone()),
        just("default").then_ignore(ignore_parser.clone()),
        just("for").then_ignore(ignore_parser.clone()),
        just("function").then_ignore(ignore_parser.clone()),
        just("if").then_ignore(ignore_parser.clone()),
        just("leave").then_ignore(ignore_parser.clone()),
        just("let").then_ignore(ignore_parser.clone()),
        just("switch").then_ignore(ignore_parser.clone()),
        just("hex").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::yul_keyword);
    let yul_leave_statement_parser = just("leave")
        .then_ignore(ignore_parser.clone())
        .map(builder::yul_leave_statement);
    let double_quoted_ascii_string_literal_parser = just('"')
        .ignore_then(choice((todo(), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('"'))
        .map(builder::double_quoted_ascii_string_literal);
    let double_quoted_unicode_string_literal_parser = just("unicode\"")
        .ignore_then(choice((none_of("\"\\\n\r"), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('"'))
        .map(builder::double_quoted_unicode_string_literal);
    let single_quoted_ascii_string_literal_parser = just('\'')
        .ignore_then(choice((todo(), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('\''))
        .map(builder::single_quoted_ascii_string_literal);
    let single_quoted_unicode_string_literal_parser = just("unicode'")
        .ignore_then(choice((none_of("'\\\n\r"), escape_sequence_parser.clone())).repeated())
        .then_ignore(just('\''))
        .map(builder::single_quoted_unicode_string_literal);
    let elementary_type_with_payable_parser = choice((
        just("address")
            .then_ignore(ignore_parser.clone())
            .ignore_then(just("payable").then_ignore(ignore_parser.clone()).or_not()),
        elementary_type_parser.clone(),
    ))
    .map(builder::elementary_type_with_payable);
    let elementary_type_without_payable_parser = choice((
        just("address").then_ignore(ignore_parser.clone()),
        elementary_type_parser.clone(),
    ))
    .map(builder::elementary_type_without_payable);
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
    .map(builder::numeric_literal);
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
    .map(builder::reserved_word);
    let yul_reserved_word_parser = choice((
        yul_keyword_parser.clone(),
        yul_evm_builtin_function_name_parser.clone(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .map(builder::yul_reserved_word);
    let ascii_string_literal_parser = choice((
        single_quoted_ascii_string_literal_parser.clone(),
        double_quoted_ascii_string_literal_parser.clone(),
    ))
    .map(builder::ascii_string_literal);
    let unicode_string_literal_parser = choice((
        single_quoted_unicode_string_literal_parser.clone(),
        double_quoted_unicode_string_literal_parser.clone(),
    ))
    .map(builder::unicode_string_literal);
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
        .map(builder::assembly_flags);
    let identifier_parser = todo().map(builder::identifier);
    let yul_identifier_parser = todo().map(builder::yul_identifier);
    let ascii_string_literal_parser = ascii_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .map(builder::ascii_string_literal);
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
        .map(builder::enum_definition);
    let identifier_path_parser = identifier_parser
        .clone()
        .separated_by(just('.').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::identifier_path);
    let named_argument_parser = identifier_parser
        .clone()
        .then_ignore(just(':').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .map(builder::named_argument);
    let parameter_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone().or_not())
        .map(builder::parameter_declaration);
    let selected_import_parser = identifier_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .or_not(),
        )
        .map(builder::selected_import);
    let unicode_string_literal_parser = unicode_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .map(builder::unicode_string_literal);
    let user_defined_value_type_definition_parser = just("type")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("is").then_ignore(ignore_parser.clone()))
        .then(elementary_type_with_payable_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::user_defined_value_type_definition);
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
    .map(builder::yul_function_call);
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
        .map(builder::yul_function_definition);
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
        .map(builder::yul_path);
    let import_path_parser = ascii_string_literal_parser
        .clone()
        .map(builder::import_path);
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
    .map(builder::literal);
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
        .map(builder::mapping_type);
    let named_argument_list_parser = just('{')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            named_argument_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::named_argument_list);
    let non_empty_parameter_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            parameter_declaration_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::non_empty_parameter_list);
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
        .map(builder::override_specifier);
    let parameter_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            parameter_declaration_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .map(builder::parameter_list);
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
    .map(builder::yul_literal);
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
        .map(builder::argument_list);
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
        .map(builder::catch_clause);
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
        .map(builder::function_type);
    let method_attribute_parser = choice((
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::method_attribute);
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
        .map(builder::selecting_import_directive);
    let simple_import_directive_parser = import_path_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .repeated(),
        )
        .map(builder::simple_import_directive);
    let star_import_directive_parser = just('*')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just("as").then_ignore(ignore_parser.clone()))
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("from").then_ignore(ignore_parser.clone()))
        .then(import_path_parser.clone())
        .map(builder::star_import_directive);
    let state_variable_attribute_parser = choice((
        just("public").then_ignore(ignore_parser.clone()),
        just("private").then_ignore(ignore_parser.clone()),
        just("internal").then_ignore(ignore_parser.clone()),
        just("constant").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
        just("immutable").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::state_variable_attribute);
    yul_expression_parser.define(
        choice((
            yul_path_parser.clone(),
            yul_function_call_parser.clone(),
            yul_literal_parser.clone(),
        ))
        .map(builder::yul_expression),
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
            choice((
                just("++").then_ignore(ignore_parser.clone()),
                just("--").then_ignore(ignore_parser.clone()),
                just('!').then_ignore(ignore_parser.clone()),
                just('~').then_ignore(ignore_parser.clone()),
                just("delete").then_ignore(ignore_parser.clone()),
                just('-').then_ignore(ignore_parser.clone()),
            ))
            .then(expression_parser.clone()),
            expression_parser.clone().then(choice((
                just("++").then_ignore(ignore_parser.clone()),
                just("--").then_ignore(ignore_parser.clone()),
            ))),
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
                .then(choice((
                    just("<<").then_ignore(ignore_parser.clone()),
                    just(">>").then_ignore(ignore_parser.clone()),
                    just(">>>").then_ignore(ignore_parser.clone()),
                )))
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
                .then(choice((
                    just('<').then_ignore(ignore_parser.clone()),
                    just('>').then_ignore(ignore_parser.clone()),
                    just("<=").then_ignore(ignore_parser.clone()),
                    just(">=").then_ignore(ignore_parser.clone()),
                )))
                .then(expression_parser.clone()),
            expression_parser
                .clone()
                .then(choice((
                    just("==").then_ignore(ignore_parser.clone()),
                    just("!=").then_ignore(ignore_parser.clone()),
                )))
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
                .then(choice((
                    just('=').then_ignore(ignore_parser.clone()),
                    just("|=").then_ignore(ignore_parser.clone()),
                    just("^=").then_ignore(ignore_parser.clone()),
                    just("&=").then_ignore(ignore_parser.clone()),
                    just("<<=").then_ignore(ignore_parser.clone()),
                    just(">>=").then_ignore(ignore_parser.clone()),
                    just(">>>=").then_ignore(ignore_parser.clone()),
                    just("+=").then_ignore(ignore_parser.clone()),
                    just("-=").then_ignore(ignore_parser.clone()),
                    just("*=").then_ignore(ignore_parser.clone()),
                    just("/=").then_ignore(ignore_parser.clone()),
                    just("%=").then_ignore(ignore_parser.clone()),
                )))
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
        .map(builder::expression),
    );
    let import_directive_parser = just("import")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice((
            simple_import_directive_parser.clone(),
            star_import_directive_parser.clone(),
            selecting_import_directive_parser.clone(),
        )))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::import_directive);
    let inheritance_specifier_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .map(builder::inheritance_specifier);
    let modifier_invocation_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .map(builder::modifier_invocation);
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
        .map(builder::yul_assignment);
    let yul_for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .then(yul_block_parser.clone())
        .map(builder::yul_for_statement);
    let yul_if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .map(builder::yul_if_statement);
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
        .map(builder::yul_switch_statement);
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
        .map(builder::yul_variable_declaration);
    let constructor_attribute_parser = choice((
        modifier_invocation_parser.clone(),
        just("payable").then_ignore(ignore_parser.clone()),
        just("internal").then_ignore(ignore_parser.clone()),
        just("public").then_ignore(ignore_parser.clone()),
    ))
    .map(builder::constructor_attribute);
    let do_while_statement_parser = just("do")
        .then_ignore(ignore_parser.clone())
        .ignore_then(statement_parser.clone())
        .then_ignore(just("while").then_ignore(ignore_parser.clone()))
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::do_while_statement);
    let emit_statement_parser = just("emit")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::emit_statement);
    let expression_statement_parser = expression_parser
        .clone()
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::expression_statement);
    let fallback_function_attribute_parser = choice((
        just("external").then_ignore(ignore_parser.clone()),
        state_mutability_specifier_parser.clone(),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::fallback_function_attribute);
    let function_attribute_parser = choice((
        visibility_specifier_parser.clone(),
        state_mutability_specifier_parser.clone(),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::function_attribute);
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
        .map(builder::if_statement);
    let inheritance_specifier_list_parser = just("is")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            inheritance_specifier_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .map(builder::inheritance_specifier_list);
    let receive_function_attribute_parser = choice((
        just("external").then_ignore(ignore_parser.clone()),
        just("payable").then_ignore(ignore_parser.clone()),
        modifier_invocation_parser.clone(),
        just("virtual").then_ignore(ignore_parser.clone()),
        override_specifier_parser.clone(),
    ))
    .map(builder::receive_function_attribute);
    let return_statement_parser = just("return")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone().or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::return_statement);
    let revert_statement_parser = just("revert")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::revert_statement);
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
        .map(builder::try_statement);
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
        .map(builder::type_name),
    );
    let while_statement_parser = just("while")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .map(builder::while_statement);
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
    .map(builder::yul_statement);
    let constant_definition_parser = type_name_parser
        .clone()
        .then_ignore(just("constant").then_ignore(ignore_parser.clone()))
        .then(identifier_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::constant_definition);
    let error_parameter_parser = type_name_parser
        .clone()
        .then(identifier_parser.clone().or_not())
        .map(builder::error_parameter);
    let event_parameter_parser = type_name_parser
        .clone()
        .then(just("indexed").then_ignore(ignore_parser.clone()).or_not())
        .then(identifier_parser.clone().or_not())
        .map(builder::event_parameter);
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
        .map(builder::state_variable_declaration);
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
        .map(builder::struct_definition);
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
        .map(builder::using_directive);
    let variable_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone())
        .map(builder::variable_declaration);
    yul_block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(yul_statement_parser.clone().repeated())
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .map(builder::yul_block),
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
        .map(builder::assembly_statement);
    let directive_parser = choice((
        pragma_directive_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        import_directive_parser.clone(),
        using_directive_parser.clone(),
    ))
    .map(builder::directive);
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
        .map(builder::error_definition);
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
        .map(builder::event_definition);
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
        .map(builder::variable_declaration_tuple);
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
    .map(builder::variable_declaration_statement);
    let simple_statement_parser = choice((
        variable_declaration_statement_parser.clone(),
        expression_statement_parser.clone(),
    ))
    .map(builder::simple_statement);
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
        .map(builder::for_statement);
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
        .map(builder::statement),
    );
    block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(
                choice((statement_parser.clone(), unchecked_block_parser.clone())).repeated(),
            )
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .map(builder::block),
    );
    let constructor_definition_parser = just("constructor")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(constructor_attribute_parser.clone().repeated())
        .then(block_parser.clone())
        .map(builder::constructor_definition);
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
        .map(builder::fallback_function_definition);
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
        .map(builder::function_definition);
    let modifier_definition_parser = just("modifier")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(parameter_list_parser.clone().or_not())
        .then(method_attribute_parser.clone().repeated())
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::modifier_definition);
    let receive_function_definition_parser = just("receive")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(receive_function_attribute_parser.clone().repeated())
        .then(choice((
            just(';').then_ignore(ignore_parser.clone()),
            block_parser.clone(),
        )))
        .map(builder::receive_function_definition);
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
    .map(builder::contract_body_element);
    let contract_definition_parser = just("abstract")
        .then_ignore(ignore_parser.clone())
        .or_not()
        .then_ignore(just("contract").then_ignore(ignore_parser.clone()))
        .then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::contract_definition);
    let interface_definition_parser = just("interface")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::interface_definition);
    let library_definition_parser = just("library")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::library_definition);
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
    .map(builder::definition);
    let source_unit_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then(choice((directive_parser.clone(), definition_parser.clone())).repeated())
        .then_ignore(end())
        .map(builder::source_unit);
    source_unit_parser.recover_with(skip_then_retry_until([]))
}
