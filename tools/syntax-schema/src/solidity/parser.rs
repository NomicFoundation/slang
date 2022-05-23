use chumsky::{prelude::*, Parser};

// use super::builder;
pub type SourceUnitParserResultType = ();

pub fn create_source_unit_parser(
) -> impl Parser<char, SourceUnitParserResultType, Error = Simple<char>> {
    let mut block_parser = Recursive::declare();
    let mut yul_function_call_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut type_name_parser = Recursive::declare();
    let mut yul_block_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let ascii_escape_parser = choice::<_, Simple<char>>((
        just('n').ignored(),
        just('r').ignored(),
        just('t').ignored(),
        just('\'').ignored(),
        just('"').ignored(),
        just('\\').ignored(),
        just('\n').ignored(),
        just('\r').ignored(),
    ))
    .ignored()
    .boxed();
    let boolean_literal_parser = choice::<_, Simple<char>>((just("true"), just("false")))
        .ignored()
        .boxed();
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, Simple<char>>((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(none_of("*/"))
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
        .ignored()
        .boxed();
    let decimal_integer_parser = filter(|&c: &char| c.is_ascii_digit())
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .ignored()
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
        .ignored()
        .boxed();
    let fixed_bytes_parser = just("bytes")
        .ignore_then(choice::<_, Simple<char>>((
            choice::<_, Simple<char>>((
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
            choice::<_, Simple<char>>((
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
        .ignored()
        .boxed();
    let hex_character_parser = choice::<_, Simple<char>>((
        filter(|&c: &char| c.is_ascii_digit()).ignored(),
        filter(|&c: &char| ('a' <= c && c <= 'f')).ignored(),
        filter(|&c: &char| ('A' <= c && c <= 'F')).ignored(),
    ))
    .ignored()
    .boxed();
    let identifier_start_parser = choice::<_, Simple<char>>((
        just('_').ignored(),
        just('$').ignored(),
        filter(|&c: &char| c.is_ascii_lowercase()).ignored(),
        filter(|&c: &char| c.is_ascii_uppercase()).ignored(),
    ))
    .ignored()
    .boxed();
    let line_comment_parser = just("//")
        .ignore_then(none_of("\n\r").repeated())
        .ignored()
        .boxed();
    let number_unit_parser = choice::<_, Simple<char>>((
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
    .ignored()
    .boxed();
    let signed_integer_type_parser = just("int")
        .ignore_then(choice::<_, Simple<char>>((
            choice::<_, Simple<char>>((
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
            choice::<_, Simple<char>>((
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
        .ignored()
        .boxed();
    let whitespace_parser = choice::<_, Simple<char>>((
        just(' ').ignored(),
        just('\t').ignored(),
        just('\r').ignored(),
        just('\n').ignored(),
    ))
    .ignored()
    .boxed();
    let yul_decimal_number_literal_parser = choice::<_, Simple<char>>((
        just('0').ignored(),
        filter(|&c: &char| ('1' <= c && c <= '9'))
            .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let yul_hex_literal_parser = just("0x")
        .ignore_then(choice::<_, Simple<char>>((
            filter(|&c: &char| c.is_ascii_digit()).ignored(),
            filter(|&c: &char| ('a' <= c && c <= 'f')).ignored(),
            filter(|&c: &char| ('A' <= c && c <= 'F')).ignored(),
        )))
        .then(
            choice::<_, Simple<char>>((
                filter(|&c: &char| c.is_ascii_digit()).ignored(),
                filter(|&c: &char| ('a' <= c && c <= 'f')).ignored(),
                filter(|&c: &char| ('A' <= c && c <= 'F')).ignored(),
            ))
            .repeated(),
        )
        .ignored()
        .boxed();
    let pragma_directive_parser = just("pragma")
        .ignore_then(filter(|&c: &char| c != ';'))
        .then(filter(|&c: &char| c != ';').repeated())
        .then_ignore(just(';'))
        .ignored()
        .boxed();
    let decimal_exponent_parser =
        choice::<_, Simple<char>>((just('e').ignored(), just('E').ignored()))
            .then(just('-').or_not())
            .then(decimal_integer_parser.clone())
            .ignored()
            .boxed();
    let decimal_float_parser = decimal_integer_parser
        .clone()
        .or_not()
        .then_ignore(just('.'))
        .then(decimal_integer_parser.clone())
        .ignored()
        .boxed();
    let hex_byte_escape_parser = just('x')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(2usize)
                .at_most(2usize),
        )
        .ignored()
        .boxed();
    let hex_number_parser = just('0')
        .ignore_then(just('x'))
        .ignore_then(
            hex_character_parser
                .clone()
                .separated_by(just('_').or_not())
                .at_least(1usize),
        )
        .ignored()
        .boxed();
    let ignore_parser = choice::<_, Simple<char>>((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
        line_comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored()
    .boxed();
    let identifier_part_parser = choice::<_, Simple<char>>((
        identifier_start_parser.clone().ignored(),
        filter(|&c: &char| c.is_ascii_digit()).ignored(),
    ))
    .ignored()
    .boxed();
    let possibly_separated_pairs_of_hex_digits_parser = hex_character_parser
        .clone()
        .repeated()
        .at_least(2usize)
        .at_most(2usize)
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .ignored()
        .boxed();
    let ufixed_parser = just('u')
        .ignore_then(fixed_parser.clone())
        .ignored()
        .boxed();
    let unicode_escape_parser = just('u')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(4usize)
                .at_most(4usize),
        )
        .ignored()
        .boxed();
    let unsigned_integer_type_parser = just('u')
        .ignore_then(signed_integer_type_parser.clone())
        .ignored()
        .boxed();
    let decimal_number_parser = choice::<_, Simple<char>>((
        decimal_integer_parser.clone().ignored(),
        decimal_float_parser.clone().ignored(),
    ))
    .then(decimal_exponent_parser.clone().or_not())
    .ignored()
    .boxed();
    let escape_sequence_parser = just('\\')
        .ignore_then(choice::<_, Simple<char>>((
            ascii_escape_parser.clone().ignored(),
            hex_byte_escape_parser.clone().ignored(),
            unicode_escape_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let hex_string_literal_parser = just("hex")
        .ignore_then(choice::<_, Simple<char>>((
            just('"')
                .ignore_then(
                    possibly_separated_pairs_of_hex_digits_parser
                        .clone()
                        .or_not(),
                )
                .then_ignore(just('"'))
                .ignored(),
            just('\'')
                .ignore_then(
                    possibly_separated_pairs_of_hex_digits_parser
                        .clone()
                        .or_not(),
                )
                .then_ignore(just('\''))
                .ignored(),
        )))
        .ignored()
        .boxed();
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .then(identifier_part_parser.clone().repeated())
        .ignored()
        .boxed();
    let break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let data_location_parser =
        choice::<_, Simple<char>>((just("memory"), just("storage"), just("calldata")))
            .ignored()
            .boxed();
    let elementary_type_parser = choice::<_, Simple<char>>((
        just("bool").then_ignore(ignore_parser.clone()).ignored(),
        just("string").then_ignore(ignore_parser.clone()).ignored(),
        just("bytes").then_ignore(ignore_parser.clone()).ignored(),
        signed_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        unsigned_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        fixed_bytes_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        fixed_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        ufixed_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
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
        .ignored()
        .boxed();
    let keyword_parser = choice::<_, Simple<char>>((
        choice::<_, Simple<char>>((
            just("pragma").then_ignore(ignore_parser.clone()).ignored(),
            just("abstract")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("anonymous")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("address").then_ignore(ignore_parser.clone()).ignored(),
            just("as").then_ignore(ignore_parser.clone()).ignored(),
            just("assembly")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("bool").then_ignore(ignore_parser.clone()).ignored(),
            just("break").then_ignore(ignore_parser.clone()).ignored(),
            just("bytes").then_ignore(ignore_parser.clone()).ignored(),
            just("calldata")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("catch").then_ignore(ignore_parser.clone()).ignored(),
            just("constant")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("constructor")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("continue")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("contract")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("delete").then_ignore(ignore_parser.clone()).ignored(),
        )),
        choice::<_, Simple<char>>((
            just("do").then_ignore(ignore_parser.clone()).ignored(),
            just("else").then_ignore(ignore_parser.clone()).ignored(),
            just("emit").then_ignore(ignore_parser.clone()).ignored(),
            just("enum").then_ignore(ignore_parser.clone()).ignored(),
            just("event").then_ignore(ignore_parser.clone()).ignored(),
            just("external")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("fallback")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("false").then_ignore(ignore_parser.clone()).ignored(),
            just("for").then_ignore(ignore_parser.clone()).ignored(),
            just("function")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("hex").then_ignore(ignore_parser.clone()).ignored(),
            just("if").then_ignore(ignore_parser.clone()).ignored(),
            just("immutable")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("import").then_ignore(ignore_parser.clone()).ignored(),
            just("indexed").then_ignore(ignore_parser.clone()).ignored(),
            just("interface")
                .then_ignore(ignore_parser.clone())
                .ignored(),
        )),
        choice::<_, Simple<char>>((
            just("internal")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("is").then_ignore(ignore_parser.clone()).ignored(),
            just("library").then_ignore(ignore_parser.clone()).ignored(),
            just("mapping").then_ignore(ignore_parser.clone()).ignored(),
            just("memory").then_ignore(ignore_parser.clone()).ignored(),
            just("modifier")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("new").then_ignore(ignore_parser.clone()).ignored(),
            just("override")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("payable").then_ignore(ignore_parser.clone()).ignored(),
            just("private").then_ignore(ignore_parser.clone()).ignored(),
            just("public").then_ignore(ignore_parser.clone()).ignored(),
            just("pure").then_ignore(ignore_parser.clone()).ignored(),
            just("receive").then_ignore(ignore_parser.clone()).ignored(),
            just("return").then_ignore(ignore_parser.clone()).ignored(),
            just("returns").then_ignore(ignore_parser.clone()).ignored(),
            just("storage").then_ignore(ignore_parser.clone()).ignored(),
        )),
        choice::<_, Simple<char>>((
            just("string").then_ignore(ignore_parser.clone()).ignored(),
            just("struct").then_ignore(ignore_parser.clone()).ignored(),
            just("true").then_ignore(ignore_parser.clone()).ignored(),
            just("try").then_ignore(ignore_parser.clone()).ignored(),
            just("type").then_ignore(ignore_parser.clone()).ignored(),
            just("unchecked")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("using").then_ignore(ignore_parser.clone()).ignored(),
            just("view").then_ignore(ignore_parser.clone()).ignored(),
            just("virtual").then_ignore(ignore_parser.clone()).ignored(),
            just("while").then_ignore(ignore_parser.clone()).ignored(),
            signed_integer_type_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .ignored(),
            unsigned_integer_type_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .ignored(),
            fixed_bytes_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("fixed").then_ignore(ignore_parser.clone()).ignored(),
            just("ufixed").then_ignore(ignore_parser.clone()).ignored(),
        )),
    ))
    .ignored()
    .boxed();
    let positional_argument_list_parser = expression_parser
        .clone()
        .separated_by(just(',').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .ignored()
        .boxed();
    let reserved_keyword_parser = choice::<_, Simple<char>>((
        choice::<_, Simple<char>>((
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
        choice::<_, Simple<char>>((
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
    .ignored()
    .boxed();
    let state_mutability_specifier_parser =
        choice::<_, Simple<char>>((just("pure"), just("view"), just("payable")))
            .ignored()
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
        .ignored()
        .boxed();
    let unchecked_block_parser = just("unchecked")
        .then_ignore(ignore_parser.clone())
        .ignore_then(block_parser.clone())
        .ignored()
        .boxed();
    let visibility_specifier_parser = choice::<_, Simple<char>>((
        just("internal"),
        just("external"),
        just("private"),
        just("public"),
    ))
    .ignored()
    .boxed();
    let yul_break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let yul_continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let yul_evm_builtin_function_name_parser = choice::<_, Simple<char>>((
        choice::<_, Simple<char>>((
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
        choice::<_, Simple<char>>((
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
        choice::<_, Simple<char>>((
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
        choice::<_, Simple<char>>((
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
        choice::<_, Simple<char>>((
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
    .ignored()
    .boxed();
    let yul_keyword_parser = choice::<_, Simple<char>>((
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
    ))
    .ignored()
    .boxed();
    let yul_leave_statement_parser = just("leave")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let double_quoted_ascii_string_literal_parser = just('"')
        .ignore_then(
            choice::<_, Simple<char>>((
                just("todo()").ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('"'))
        .ignored()
        .boxed();
    let double_quoted_unicode_string_literal_parser = just("unicode\"")
        .ignore_then(
            choice::<_, Simple<char>>((
                none_of("\"\\\n\r").ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('"'))
        .ignored()
        .boxed();
    let single_quoted_ascii_string_literal_parser = just('\'')
        .ignore_then(
            choice::<_, Simple<char>>((
                just("todo()").ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('\''))
        .ignored()
        .boxed();
    let single_quoted_unicode_string_literal_parser = just("unicode'")
        .ignore_then(
            choice::<_, Simple<char>>((
                none_of("'\\\n\r").ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('\''))
        .ignored()
        .boxed();
    let elementary_type_with_payable_parser = choice::<_, Simple<char>>((
        just("address")
            .then_ignore(ignore_parser.clone())
            .ignore_then(just("payable").then_ignore(ignore_parser.clone()).or_not())
            .ignored(),
        elementary_type_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let elementary_type_without_payable_parser = choice::<_, Simple<char>>((
        just("address").then_ignore(ignore_parser.clone()).ignored(),
        elementary_type_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let numeric_literal_parser = choice::<_, Simple<char>>((
        decimal_number_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        hex_number_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .then(
        number_unit_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .or_not(),
    )
    .ignored()
    .boxed();
    let reserved_word_parser = choice::<_, Simple<char>>((
        keyword_parser.clone().ignored(),
        reserved_keyword_parser.clone().ignored(),
        number_unit_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let yul_reserved_word_parser = choice::<_, Simple<char>>((
        yul_keyword_parser.clone().ignored(),
        yul_evm_builtin_function_name_parser.clone().ignored(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let ascii_string_literal_parser = choice::<_, Simple<char>>((
        single_quoted_ascii_string_literal_parser.clone().ignored(),
        double_quoted_ascii_string_literal_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let unicode_string_literal_parser = choice::<_, Simple<char>>((
        single_quoted_unicode_string_literal_parser
            .clone()
            .ignored(),
        double_quoted_unicode_string_literal_parser
            .clone()
            .ignored(),
    ))
    .ignored()
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
        .ignored()
        .boxed();
    let identifier_parser = just("todo()").ignored().boxed();
    let yul_identifier_parser = just("todo()").ignored().boxed();
    let ascii_string_literal_parser = ascii_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .ignored()
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
        .ignored()
        .boxed();
    let identifier_path_parser = identifier_parser
        .clone()
        .separated_by(just('.').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .ignored()
        .boxed();
    let named_argument_parser = identifier_parser
        .clone()
        .then_ignore(just(':').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .ignored()
        .boxed();
    let parameter_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone().or_not())
        .ignored()
        .boxed();
    let selected_import_parser = identifier_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .or_not(),
        )
        .ignored()
        .boxed();
    let unicode_string_literal_parser = unicode_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .ignored()
        .boxed();
    let user_defined_value_type_definition_parser = just("type")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("is").then_ignore(ignore_parser.clone()))
        .then(elementary_type_with_payable_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
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
        .ignored()
        .boxed();
    let yul_path_parser = yul_identifier_parser
        .clone()
        .then(
            just('.')
                .then_ignore(ignore_parser.clone())
                .ignore_then(choice::<_, Simple<char>>((
                    yul_identifier_parser.clone().ignored(),
                    yul_evm_builtin_function_name_parser.clone().ignored(),
                )))
                .repeated(),
        )
        .ignored()
        .boxed();
    let import_path_parser = ascii_string_literal_parser.clone().ignored().boxed();
    let literal_parser = choice::<_, Simple<char>>((
        ascii_string_literal_parser.clone().ignored(),
        unicode_string_literal_parser.clone().ignored(),
        numeric_literal_parser.clone().ignored(),
        hex_string_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let mapping_type_parser = just("mapping")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(choice::<_, Simple<char>>((
            elementary_type_without_payable_parser.clone().ignored(),
            identifier_path_parser.clone().ignored(),
        )))
        .then_ignore(just("=>").then_ignore(ignore_parser.clone()))
        .then(type_name_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let named_argument_list_parser = just('{')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            named_argument_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
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
        .ignored()
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
        .ignored()
        .boxed();
    let parameter_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            parameter_declaration_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let yul_literal_parser = choice::<_, Simple<char>>((
        yul_decimal_number_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        yul_hex_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        ascii_string_literal_parser.clone().ignored(),
        boolean_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        hex_string_literal_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let argument_list_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            choice::<_, Simple<char>>((
                positional_argument_list_parser.clone().ignored(),
                named_argument_list_parser.clone().ignored(),
            ))
            .or_not(),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .ignored()
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
        .ignored()
        .boxed();
    let function_type_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(
            choice::<_, Simple<char>>((
                visibility_specifier_parser.clone().ignored(),
                state_mutability_specifier_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .ignored()
        .boxed();
    let method_attribute_parser = choice::<_, Simple<char>>((
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
    .ignored()
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
        .ignored()
        .boxed();
    let simple_import_directive_parser = import_path_parser
        .clone()
        .then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let star_import_directive_parser = just('*')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just("as").then_ignore(ignore_parser.clone()))
        .ignore_then(identifier_parser.clone())
        .then_ignore(just("from").then_ignore(ignore_parser.clone()))
        .then(import_path_parser.clone())
        .ignored()
        .boxed();
    let state_variable_attribute_parser = choice::<_, Simple<char>>((
        just("public").then_ignore(ignore_parser.clone()).ignored(),
        just("private").then_ignore(ignore_parser.clone()).ignored(),
        just("internal")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        just("constant")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        override_specifier_parser.clone().ignored(),
        just("immutable")
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .ignored()
    .boxed();
    let yul_expression_parser = choice::<_, Simple<char>>((
        yul_path_parser.clone().ignored(),
        yul_function_call_parser.clone().ignored(),
        yul_literal_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    expression_parser.define(
        choice::<_, Simple<char>>((
            choice::<_, Simple<char>>((
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
                    .then_ignore(just(']').then_ignore(ignore_parser.clone()))
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('.').then_ignore(ignore_parser.clone()))
                    .then(choice::<_, Simple<char>>((
                        identifier_parser.clone().ignored(),
                        just("address").then_ignore(ignore_parser.clone()).ignored(),
                    )))
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('{').then_ignore(ignore_parser.clone()))
                    .then(
                        named_argument_parser
                            .clone()
                            .separated_by(just(',').then_ignore(ignore_parser.clone()))
                            .at_least(1usize),
                    )
                    .then_ignore(just('}').then_ignore(ignore_parser.clone()))
                    .ignored(),
                expression_parser
                    .clone()
                    .then(argument_list_parser.clone())
                    .ignored(),
                just("payable")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(argument_list_parser.clone())
                    .ignored(),
                just("type")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(just('(').then_ignore(ignore_parser.clone()))
                    .ignore_then(type_name_parser.clone())
                    .then_ignore(just(')').then_ignore(ignore_parser.clone()))
                    .ignored(),
                choice::<_, Simple<char>>((
                    just("++"),
                    just("--"),
                    just("!"),
                    just("~"),
                    just("delete"),
                    just("-"),
                ))
                .then(expression_parser.clone())
                .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((just("++"), just("--"))))
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just("**").then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((
                        just('*').then_ignore(ignore_parser.clone()).ignored(),
                        just('/').then_ignore(ignore_parser.clone()).ignored(),
                        just('%').then_ignore(ignore_parser.clone()).ignored(),
                    )))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((
                        just('+').then_ignore(ignore_parser.clone()).ignored(),
                        just('-').then_ignore(ignore_parser.clone()).ignored(),
                    )))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((
                        just("<<"),
                        just(">>"),
                        just(">>>"),
                    )))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('&').then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('^').then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('|').then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((
                        just("<"),
                        just(">"),
                        just("<="),
                        just(">="),
                    )))
                    .then(expression_parser.clone())
                    .ignored(),
            )),
            choice::<_, Simple<char>>((
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((just("=="), just("!="))))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just("&&").then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just("||").then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then_ignore(just('?').then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .then_ignore(just(':').then_ignore(ignore_parser.clone()))
                    .then(expression_parser.clone())
                    .ignored(),
                expression_parser
                    .clone()
                    .then(choice::<_, Simple<char>>((
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
                    )))
                    .then(expression_parser.clone())
                    .ignored(),
                just("new")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(type_name_parser.clone())
                    .ignored(),
                tuple_expression_parser.clone().ignored(),
                inline_array_expression_parser.clone().ignored(),
                choice::<_, Simple<char>>((
                    identifier_parser.clone().ignored(),
                    literal_parser.clone().ignored(),
                    elementary_type_without_payable_parser.clone().ignored(),
                ))
                .ignored(),
            )),
        ))
        .ignored()
        .boxed(),
    );
    let import_directive_parser = just("import")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, Simple<char>>((
            simple_import_directive_parser.clone().ignored(),
            star_import_directive_parser.clone().ignored(),
            selecting_import_directive_parser.clone().ignored(),
        )))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let inheritance_specifier_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .ignored()
        .boxed();
    let modifier_invocation_parser = identifier_path_parser
        .clone()
        .then(argument_list_parser.clone().or_not())
        .ignored()
        .boxed();
    let yul_for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .then(yul_block_parser.clone())
        .ignored()
        .boxed();
    yul_function_call_parser.define(
        choice::<_, Simple<char>>((
            yul_identifier_parser.clone().ignored(),
            yul_evm_builtin_function_name_parser.clone().ignored(),
        ))
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(
            yul_expression_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed(),
    );
    let yul_if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .then(yul_block_parser.clone())
        .ignored()
        .boxed();
    let yul_switch_statement_parser = just("switch")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .then(choice::<_, Simple<char>>((
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
                )
                .ignored(),
            just("default")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_block_parser.clone())
                .ignored(),
        )))
        .ignored()
        .boxed();
    let constructor_attribute_parser = choice::<_, Simple<char>>((
        modifier_invocation_parser.clone().ignored(),
        just("payable").then_ignore(ignore_parser.clone()).ignored(),
        just("internal")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        just("public").then_ignore(ignore_parser.clone()).ignored(),
    ))
    .ignored()
    .boxed();
    let do_while_statement_parser = just("do")
        .then_ignore(ignore_parser.clone())
        .ignore_then(statement_parser.clone())
        .then_ignore(just("while").then_ignore(ignore_parser.clone()))
        .then_ignore(just('(').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let emit_statement_parser = just("emit")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let expression_statement_parser = expression_parser
        .clone()
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let fallback_function_attribute_parser = choice::<_, Simple<char>>((
        just("external")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        state_mutability_specifier_parser.clone().ignored(),
        modifier_invocation_parser.clone().ignored(),
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let function_attribute_parser = choice::<_, Simple<char>>((
        visibility_specifier_parser.clone().ignored(),
        state_mutability_specifier_parser.clone().ignored(),
        modifier_invocation_parser.clone().ignored(),
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
    .ignored()
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
        .ignored()
        .boxed();
    let inheritance_specifier_list_parser = just("is")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            inheritance_specifier_parser
                .clone()
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .ignored()
        .boxed();
    let receive_function_attribute_parser = choice::<_, Simple<char>>((
        just("external")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        just("payable").then_ignore(ignore_parser.clone()).ignored(),
        modifier_invocation_parser.clone().ignored(),
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let return_statement_parser = just("return")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone().or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let revert_statement_parser = just("revert")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
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
        .ignored()
        .boxed();
    type_name_parser.define(
        choice::<_, Simple<char>>((
            elementary_type_with_payable_parser.clone().ignored(),
            function_type_parser.clone().ignored(),
            mapping_type_parser.clone().ignored(),
            identifier_path_parser.clone().ignored(),
        ))
        .then(
            just('[')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone().or_not())
                .then_ignore(just(']').then_ignore(ignore_parser.clone()))
                .repeated(),
        )
        .ignored()
        .boxed(),
    );
    let while_statement_parser = just("while")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .ignored()
        .boxed();
    let yul_assignment_parser = yul_path_parser
        .clone()
        .then(choice::<_, Simple<char>>((
            just(":=")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_expression_parser.clone())
                .ignored(),
            just(',')
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_path_parser.clone())
                .repeated()
                .at_least(1usize)
                .then_ignore(just(":=").then_ignore(ignore_parser.clone()))
                .then(yul_function_call_parser.clone())
                .ignored(),
        )))
        .ignored()
        .boxed();
    let yul_variable_declaration_parser = just("let")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_identifier_parser.clone())
        .then(
            choice::<_, Simple<char>>((
                just(":=")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(yul_expression_parser.clone())
                    .ignored(),
                just(',')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(yul_identifier_parser.clone())
                    .or_not()
                    .then(
                        just(":=")
                            .then_ignore(ignore_parser.clone())
                            .ignore_then(yul_function_call_parser.clone())
                            .or_not(),
                    )
                    .ignored(),
            ))
            .or_not(),
        )
        .ignored()
        .boxed();
    let constant_definition_parser = type_name_parser
        .clone()
        .then_ignore(just("constant").then_ignore(ignore_parser.clone()))
        .then(identifier_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let error_parameter_parser = type_name_parser
        .clone()
        .then(identifier_parser.clone().or_not())
        .ignored()
        .boxed();
    let event_parameter_parser = type_name_parser
        .clone()
        .then(just("indexed").then_ignore(ignore_parser.clone()).or_not())
        .then(identifier_parser.clone().or_not())
        .ignored()
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
        .ignored()
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
        .ignored()
        .boxed();
    let using_directive_parser = just("using")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, Simple<char>>((
            identifier_path_parser.clone().ignored(),
            just('{')
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    identifier_path_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .then_ignore(just('}').then_ignore(ignore_parser.clone()))
                .ignored(),
        )))
        .then_ignore(just("for").then_ignore(ignore_parser.clone()))
        .then(choice::<_, Simple<char>>((
            just('*').then_ignore(ignore_parser.clone()).ignored(),
            type_name_parser.clone().ignored(),
        )))
        .then(just("global").then_ignore(ignore_parser.clone()).or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let variable_declaration_parser = type_name_parser
        .clone()
        .then(data_location_parser.clone().or_not())
        .then(identifier_parser.clone())
        .ignored()
        .boxed();
    let yul_statement_parser = choice::<_, Simple<char>>((
        yul_block_parser.clone().ignored(),
        yul_variable_declaration_parser.clone().ignored(),
        yul_function_definition_parser.clone().ignored(),
        yul_assignment_parser.clone().ignored(),
        yul_function_call_parser.clone().ignored(),
        yul_if_statement_parser.clone().ignored(),
        yul_for_statement_parser.clone().ignored(),
        yul_switch_statement_parser.clone().ignored(),
        yul_leave_statement_parser.clone().ignored(),
        yul_break_statement_parser.clone().ignored(),
        yul_continue_statement_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let directive_parser = choice::<_, Simple<char>>((
        pragma_directive_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        import_directive_parser.clone().ignored(),
        using_directive_parser.clone().ignored(),
    ))
    .ignored()
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
        .ignored()
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
        .ignored()
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
        .ignored()
        .boxed();
    yul_block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(yul_statement_parser.clone().repeated())
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .ignored()
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
        .ignored()
        .boxed();
    let variable_declaration_statement_parser = choice::<_, Simple<char>>((
        variable_declaration_parser
            .clone()
            .then(
                just('=')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(expression_parser.clone())
                    .or_not(),
            )
            .ignored(),
        variable_declaration_tuple_parser
            .clone()
            .then_ignore(just('=').then_ignore(ignore_parser.clone()))
            .then(expression_parser.clone())
            .ignored(),
    ))
    .then_ignore(just(';').then_ignore(ignore_parser.clone()))
    .ignored()
    .boxed();
    let simple_statement_parser = choice::<_, Simple<char>>((
        variable_declaration_statement_parser.clone().ignored(),
        expression_statement_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(choice::<_, Simple<char>>((
            simple_statement_parser.clone().ignored(),
            just(';').then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then(choice::<_, Simple<char>>((
            expression_statement_parser.clone().ignored(),
            just(';').then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then(expression_parser.clone().or_not())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(statement_parser.clone())
        .ignored()
        .boxed();
    statement_parser.define(
        choice::<_, Simple<char>>((
            block_parser.clone().ignored(),
            simple_statement_parser.clone().ignored(),
            if_statement_parser.clone().ignored(),
            for_statement_parser.clone().ignored(),
            while_statement_parser.clone().ignored(),
            do_while_statement_parser.clone().ignored(),
            continue_statement_parser.clone().ignored(),
            break_statement_parser.clone().ignored(),
            try_statement_parser.clone().ignored(),
            return_statement_parser.clone().ignored(),
            emit_statement_parser.clone().ignored(),
            revert_statement_parser.clone().ignored(),
            assembly_statement_parser.clone().ignored(),
        ))
        .ignored()
        .boxed(),
    );
    block_parser.define(
        just('{')
            .then_ignore(ignore_parser.clone())
            .ignore_then(
                choice::<_, Simple<char>>((
                    statement_parser.clone().ignored(),
                    unchecked_block_parser.clone().ignored(),
                ))
                .repeated(),
            )
            .then_ignore(just('}').then_ignore(ignore_parser.clone()))
            .ignored()
            .boxed(),
    );
    let constructor_definition_parser = just("constructor")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .then(constructor_attribute_parser.clone().repeated())
        .then(block_parser.clone())
        .ignored()
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
        .then(choice::<_, Simple<char>>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let function_definition_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, Simple<char>>((
            identifier_parser.clone().ignored(),
            just("fallback")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("receive").then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then(parameter_list_parser.clone())
        .then(function_attribute_parser.clone().repeated())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(choice::<_, Simple<char>>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let modifier_definition_parser = just("modifier")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(parameter_list_parser.clone().or_not())
        .then(method_attribute_parser.clone().repeated())
        .then(choice::<_, Simple<char>>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let receive_function_definition_parser = just("receive")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(receive_function_attribute_parser.clone().repeated())
        .then(choice::<_, Simple<char>>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let contract_body_element_parser = choice::<_, Simple<char>>((
        using_directive_parser.clone().ignored(),
        constructor_definition_parser.clone().ignored(),
        function_definition_parser.clone().ignored(),
        fallback_function_definition_parser.clone().ignored(),
        receive_function_definition_parser.clone().ignored(),
        modifier_definition_parser.clone().ignored(),
        struct_definition_parser.clone().ignored(),
        enum_definition_parser.clone().ignored(),
        user_defined_value_type_definition_parser.clone().ignored(),
        event_definition_parser.clone().ignored(),
        error_definition_parser.clone().ignored(),
        state_variable_declaration_parser.clone().ignored(),
    ))
    .ignored()
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
        .ignored()
        .boxed();
    let interface_definition_parser = just("interface")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let library_definition_parser = just("library")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let definition_parser = choice::<_, Simple<char>>((
        contract_definition_parser.clone().ignored(),
        interface_definition_parser.clone().ignored(),
        library_definition_parser.clone().ignored(),
        function_definition_parser.clone().ignored(),
        constant_definition_parser.clone().ignored(),
        struct_definition_parser.clone().ignored(),
        enum_definition_parser.clone().ignored(),
        user_defined_value_type_definition_parser.clone().ignored(),
        error_definition_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let source_unit_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then(
            choice::<_, Simple<char>>((
                directive_parser.clone().ignored(),
                definition_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(end())
        .ignored()
        .boxed();
    source_unit_parser.recover_with(skip_then_retry_until([]))
}
