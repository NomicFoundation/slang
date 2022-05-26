#[allow(unused_imports)]
use super::builder;
use chumsky::{prelude::*, Parser};
#[allow(unused_imports)]
use syntax_schema::chumsky::combinators::NomicParser;

pub type ErrorType = Simple<char>;
pub type SourceUnitParserResultType = ();

pub fn create_source_unit_parser(
) -> impl Parser<char, SourceUnitParserResultType, Error = ErrorType> {
    let mut block_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut type_name_parser = Recursive::declare();
    let mut yul_block_parser = Recursive::declare();
    let mut yul_expression_parser = Recursive::declare();
    let ascii_escape_parser = filter(|&c: &char| {
        c == 'n'
            || c == 'r'
            || c == 't'
            || c == '\''
            || c == '"'
            || c == '\\'
            || c == '\n'
            || c == '\r'
    })
    .map(builder::to_char);
    let boolean_literal_parser = choice::<_, ErrorType>((just("false"), just("true"))).ignored();
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(filter(|&c: &char| (c != '*' && c != '/')))
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
        .ignored();
    let decimal_integer_parser = filter(|&c: &char| c.is_ascii_digit())
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .ignored();
    let fixed_bytes_type_parser = just("bytes")
        .ignore_then(choice::<_, ErrorType>((
            choice::<_, ErrorType>((
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
            )),
            choice::<_, ErrorType>((
                just("26"),
                just("27"),
                just("28"),
                just("29"),
                just("30"),
                just("31"),
                just("32"),
                just("1"),
                just("2"),
                just("3"),
                just("4"),
                just("5"),
                just("6"),
                just("7"),
                just("8"),
                just("9"),
            )),
        )))
        .map(builder::to_str);
    let fixed_type_parser = just("fixed")
        .ignore_then(
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .then_ignore(just('x'))
                .then(filter(|&c: &char| ('1' <= c && c <= '9')))
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .or_not(),
        )
        .map(builder::to_str);
    let hex_character_parser =
        filter(|&c: &char| c.is_ascii_digit() || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F'))
            .ignored();
    let identifier_start_parser = filter(|&c: &char| {
        c == '_' || c == '$' || c.is_ascii_lowercase() || c.is_ascii_uppercase()
    });
    let line_comment_parser = just("//")
        .ignore_then(filter(|&c: &char| (c != '\n' && c != '\r')).repeated())
        .ignored();
    let number_unit_parser = choice::<_, ErrorType>((
        just("seconds"),
        just("minutes"),
        just("ether"),
        just("hours"),
        just("weeks"),
        just("years"),
        just("gwei"),
        just("days"),
        just("wei"),
    ))
    .ignored();
    let reserved_keyword_parser = choice::<_, ErrorType>((
        choice::<_, ErrorType>((
            just("relocatable"),
            just("implements"),
            just("reference"),
            just("supports"),
            just("default"),
            just("mutable"),
            just("partial"),
            just("promise"),
            just("typedef"),
            just("copyof"),
            just("define"),
            just("inline"),
            just("sealed"),
            just("sizeof"),
            just("static"),
            just("switch"),
        )),
        choice::<_, ErrorType>((
            just("typeof"),
            just("after"),
            just("alias"),
            just("apply"),
            just("final"),
            just("macro"),
            just("match"),
            just("auto"),
            just("byte"),
            just("case"),
            just("null"),
            just("let"),
            just("var"),
            just("in"),
            just("of"),
        )),
    ))
    .ignored();
    let signed_integer_type_parser = just("int")
        .ignore_then(choice::<_, ErrorType>((
            choice::<_, ErrorType>((
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
            )),
            choice::<_, ErrorType>((
                just("232"),
                just("240"),
                just("248"),
                just("256"),
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
                just("8"),
            )),
        )))
        .map(builder::to_str);
    let whitespace_parser =
        filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n').ignored();
    let yul_decimal_number_literal_parser = choice::<_, ErrorType>((
        just('0').map(builder::to_str).ignored(),
        filter(|&c: &char| ('1' <= c && c <= '9'))
            .chain(filter(|&c: &char| c.is_ascii_digit()).repeated())
            .map(builder::to_str)
            .ignored(),
    ))
    .ignored();
    let yul_evm_builtin_function_name_parser = choice::<_, ErrorType>((
        choice::<_, ErrorType>((
            just("returndatasize"),
            just("returndatacopy"),
            just("calldataload"),
            just("calldatasize"),
            just("calldatacopy"),
            just("delegatecall"),
            just("selfdestruct"),
            just("selfbalance"),
            just("extcodesize"),
            just("extcodecopy"),
            just("extcodehash"),
            just("signextend"),
            just("staticcall"),
            just("difficulty"),
            just("keccak256"),
            just("callvalue"),
        )),
        choice::<_, ErrorType>((
            just("blockhash"),
            just("timestamp"),
            just("callcode"),
            just("gasprice"),
            just("coinbase"),
            just("gaslimit"),
            just("mstore8"),
            just("address"),
            just("balance"),
            just("create2"),
            just("invalid"),
            just("chainid"),
            just("basefee"),
            just("iszero"),
            just("addmod"),
            just("mulmod"),
        )),
        choice::<_, ErrorType>((
            just("mstore"),
            just("sstore"),
            just("caller"),
            just("create"),
            just("return"),
            just("revert"),
            just("origin"),
            just("number"),
            just("mload"),
            just("sload"),
            just("msize"),
            just("stop"),
            just("sdiv"),
            just("smod"),
            just("byte"),
            just("call"),
        )),
        choice::<_, ErrorType>((
            just("log0"),
            just("log1"),
            just("log2"),
            just("log3"),
            just("log4"),
            just("add"),
            just("sub"),
            just("mul"),
            just("div"),
            just("mod"),
            just("exp"),
            just("not"),
            just("slt"),
            just("sgt"),
            just("and"),
            just("xor"),
        )),
        choice::<_, ErrorType>((
            just("shl"),
            just("shr"),
            just("sar"),
            just("pop"),
            just("gas"),
            just("lt"),
            just("gt"),
            just("eq"),
            just("or"),
        )),
    ))
    .ignored();
    let yul_hex_literal_parser = just("0x")
        .ignore_then(filter(|&c: &char| {
            c.is_ascii_digit() || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        }))
        .then(
            filter(|&c: &char| {
                c.is_ascii_digit() || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
            })
            .repeated(),
        )
        .ignored();
    let yul_keyword_parser = choice::<_, ErrorType>((
        just("continue"),
        just("function"),
        just("default"),
        just("switch"),
        just("break"),
        just("leave"),
        just("case"),
        just("for"),
        just("let"),
        just("hex"),
        just("if"),
    ))
    .ignored();
    let pragma_directive_parser = just("pragma")
        .ignore_then(filter(|&c: &char| c != ';'))
        .then(filter(|&c: &char| c != ';').repeated())
        .then_ignore(just(';'))
        .ignored();
    let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
        .then(just('-').or_not())
        .then_ignore(decimal_integer_parser.clone())
        .ignored();
    let decimal_float_parser = decimal_integer_parser
        .clone()
        .or_not()
        .then_ignore(just('.'))
        .then_ignore(decimal_integer_parser.clone())
        .ignored();
    let hex_byte_escape_parser = just('x')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(2usize)
                .at_most(2usize),
        )
        .map(builder::to_char);
    let hex_number_parser = just('0')
        .ignore_then(just('x'))
        .ignore_then(
            hex_character_parser
                .clone()
                .separated_by(just('_').or_not())
                .at_least(1usize),
        )
        .ignored();
    let ignore_parser = choice::<_, ErrorType>((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
        line_comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored();
    let identifier_part_parser = choice::<_, ErrorType>((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));
    let possibly_separated_pairs_of_hex_digits_parser = hex_character_parser
        .clone()
        .repeated()
        .at_least(2usize)
        .at_most(2usize)
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .ignored();
    let ufixed_type_parser = just('u')
        .ignore_then(fixed_type_parser.clone())
        .map(builder::to_str);
    let unicode_escape_parser = just('u')
        .ignore_then(
            hex_character_parser
                .clone()
                .repeated()
                .at_least(4usize)
                .at_most(4usize),
        )
        .map(builder::to_char);
    let unsigned_integer_type_parser = just('u')
        .ignore_then(signed_integer_type_parser.clone())
        .map(builder::to_str);
    let yul_reserved_word_parser = choice::<_, ErrorType>((
        yul_keyword_parser.clone().ignored(),
        yul_evm_builtin_function_name_parser.clone().ignored(),
        boolean_literal_parser.clone().ignored(),
    ))
    .ignored();
    let decimal_number_parser = choice::<_, ErrorType>((
        decimal_integer_parser.clone().ignored(),
        decimal_float_parser.clone().ignored(),
    ))
    .then(decimal_exponent_parser.clone().or_not())
    .ignored();
    let escape_sequence_parser = just('\\').ignore_then(choice::<_, ErrorType>((
        ascii_escape_parser.clone(),
        hex_byte_escape_parser.clone(),
        unicode_escape_parser.clone(),
    )));
    let hex_string_literal_parser = just("hex")
        .ignore_then(choice::<_, ErrorType>((
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
        .ignored();
    let keyword_parser = choice::<_, ErrorType>((
        choice::<_, ErrorType>((
            just("pragma").ignored(),
            just("abstract").ignored(),
            just("anonymous").ignored(),
            just("address").ignored(),
            just("as").ignored(),
            just("assembly").ignored(),
            just("bool").ignored(),
            just("break").ignored(),
            just("bytes").ignored(),
            just("calldata").ignored(),
            just("catch").ignored(),
            just("constant").ignored(),
            just("constructor").ignored(),
            just("continue").ignored(),
            just("contract").ignored(),
            just("delete").ignored(),
        )),
        choice::<_, ErrorType>((
            just("do").ignored(),
            just("else").ignored(),
            just("emit").ignored(),
            just("enum").ignored(),
            just("event").ignored(),
            just("external").ignored(),
            just("fallback").ignored(),
            just("false").ignored(),
            just("for").ignored(),
            just("function").ignored(),
            just("hex").ignored(),
            just("if").ignored(),
            just("immutable").ignored(),
            just("import").ignored(),
            just("indexed").ignored(),
            just("interface").ignored(),
        )),
        choice::<_, ErrorType>((
            just("internal").ignored(),
            just("is").ignored(),
            just("library").ignored(),
            just("mapping").ignored(),
            just("memory").ignored(),
            just("modifier").ignored(),
            just("new").ignored(),
            just("override").ignored(),
            just("payable").ignored(),
            just("private").ignored(),
            just("public").ignored(),
            just("pure").ignored(),
            just("receive").ignored(),
            just("return").ignored(),
            just("returns").ignored(),
            just("storage").ignored(),
        )),
        choice::<_, ErrorType>((
            just("string").ignored(),
            just("struct").ignored(),
            just("true").ignored(),
            just("try").ignored(),
            just("type").ignored(),
            just("unchecked").ignored(),
            just("using").ignored(),
            just("view").ignored(),
            just("virtual").ignored(),
            just("while").ignored(),
            signed_integer_type_parser.clone().ignored(),
            unsigned_integer_type_parser.clone().ignored(),
            fixed_bytes_type_parser.clone().ignored(),
            just("fixed").ignored(),
            just("ufixed").ignored(),
        )),
    ))
    .ignored();
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_part_parser.clone().repeated());
    let add_sub_operator_parser = filter(|&c: &char| c == '+' || c == '-').ignored().boxed();
    let assignment_operator_parser = choice::<_, ErrorType>((
        just(">>>="),
        just("<<="),
        just(">>="),
        just("|="),
        just("^="),
        just("&="),
        just("+="),
        just("-="),
        just("*="),
        just("/="),
        just("%="),
        just("="),
    ))
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
        choice::<_, ErrorType>((just("calldata"), just("storage"), just("memory")))
            .ignored()
            .boxed();
    let elementary_type_parser = choice::<_, ErrorType>((
        just("bool").then_ignore(ignore_parser.clone()),
        just("string").then_ignore(ignore_parser.clone()),
        just("bytes").then_ignore(ignore_parser.clone()),
        signed_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        unsigned_integer_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        fixed_bytes_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
        fixed_type_parser.clone().then_ignore(ignore_parser.clone()),
        ufixed_type_parser
            .clone()
            .then_ignore(ignore_parser.clone()),
    ))
    .boxed();
    let equality_comparison_operator_parser = choice::<_, ErrorType>((just("=="), just("!=")))
        .ignored()
        .boxed();
    let mul_div_mod_operator_parser = filter(|&c: &char| c == '*' || c == '/' || c == '%')
        .ignored()
        .boxed();
    let order_comparison_operator_parser =
        choice::<_, ErrorType>((just("<="), just(">="), just("<"), just(">")))
            .ignored()
            .boxed();
    let positional_argument_list_parser = expression_parser
        .clone()
        .separated_by(just(',').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .ignored()
        .boxed();
    let shift_operator_parser = choice::<_, ErrorType>((just(">>>"), just("<<"), just(">>")))
        .ignored()
        .boxed();
    let state_mutability_specifier_parser =
        choice::<_, ErrorType>((just("payable"), just("pure"), just("view"))).boxed();
    let unary_prefix_operator_parser = choice::<_, ErrorType>((
        just("delete"),
        just("++"),
        just("--"),
        just("!"),
        just("~"),
        just("-"),
    ))
    .ignored()
    .boxed();
    let unary_suffix_operator_parser = choice::<_, ErrorType>((just("++"), just("--")))
        .ignored()
        .boxed();
    let unchecked_block_parser = just("unchecked")
        .then_ignore(ignore_parser.clone())
        .ignore_then(block_parser.clone())
        .ignored()
        .boxed();
    let visibility_specifier_parser = choice::<_, ErrorType>((
        just("internal"),
        just("external"),
        just("private"),
        just("public"),
    ))
    .boxed();
    let yul_break_statement_parser = just("break")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let yul_continue_statement_parser = just("continue")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let yul_leave_statement_parser = just("leave")
        .then_ignore(ignore_parser.clone())
        .ignored()
        .boxed();
    let double_quoted_ascii_string_literal_parser = just('"')
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| (' ' <= c && c <= '~') && (c != '"' && c != '\\')).ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('"'))
        .ignored();
    let double_quoted_unicode_string_literal_parser = just("unicode\"")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| (c != '"' && c != '\\' && c != '\n' && c != '\r')).ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('"'))
        .ignored();
    let reserved_word_parser = choice::<_, ErrorType>((
        keyword_parser.clone().ignored(),
        reserved_keyword_parser.clone().ignored(),
        number_unit_parser.clone().ignored(),
        boolean_literal_parser.clone().ignored(),
    ))
    .ignored();
    let single_quoted_ascii_string_literal_parser = just('\'')
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| (' ' <= c && c <= '~') && (c != '\'' && c != '\\')).ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('\''))
        .ignored();
    let single_quoted_unicode_string_literal_parser = just("unicode'")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| (c != '\'' && c != '\\' && c != '\n' && c != '\r')).ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('\''))
        .ignored();
    let yul_identifier_parser = raw_identifier_parser
        .clone()
        .excluding(yul_reserved_word_parser.clone())
        .ignored();
    let elementary_type_with_payable_parser = choice::<_, ErrorType>((
        just("address")
            .then_ignore(ignore_parser.clone())
            .ignore_then(just("payable").then_ignore(ignore_parser.clone()).or_not())
            .map(builder::to_str),
        elementary_type_parser.clone(),
    ))
    .boxed();
    let elementary_type_without_payable_parser = choice::<_, ErrorType>((
        just("address").then_ignore(ignore_parser.clone()),
        elementary_type_parser.clone(),
    ))
    .boxed();
    let numeric_literal_parser = choice::<_, ErrorType>((
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
    let ascii_string_literal_parser = choice::<_, ErrorType>((
        single_quoted_ascii_string_literal_parser.clone().ignored(),
        double_quoted_ascii_string_literal_parser.clone().ignored(),
    ))
    .ignored();
    let identifier_parser = raw_identifier_parser
        .clone()
        .excluding(reserved_word_parser.clone())
        .ignored();
    let unicode_string_literal_parser = choice::<_, ErrorType>((
        single_quoted_unicode_string_literal_parser
            .clone()
            .ignored(),
        double_quoted_unicode_string_literal_parser
            .clone()
            .ignored(),
    ))
    .ignored();
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
    let yul_function_call_parser = choice::<_, ErrorType>((
        yul_identifier_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        yul_evm_builtin_function_name_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
    ))
    .then_ignore(just('(').then_ignore(ignore_parser.clone()))
    .then(
        yul_expression_parser
            .clone()
            .separated_by(just(',').then_ignore(ignore_parser.clone())),
    )
    .then_ignore(just(')').then_ignore(ignore_parser.clone()))
    .ignored()
    .boxed();
    let yul_function_definition_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            yul_identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone()),
        )
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(
            yul_identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .separated_by(just(',').then_ignore(ignore_parser.clone())),
        )
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then(
            just("->")
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    yul_identifier_parser
                        .clone()
                        .then_ignore(ignore_parser.clone())
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .or_not(),
        )
        .then_ignore(yul_block_parser.clone())
        .ignored()
        .boxed();
    let yul_path_parser = yul_identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            just('.')
                .then_ignore(ignore_parser.clone())
                .ignore_then(choice::<_, ErrorType>((
                    yul_identifier_parser
                        .clone()
                        .then_ignore(ignore_parser.clone())
                        .ignored(),
                    yul_evm_builtin_function_name_parser
                        .clone()
                        .then_ignore(ignore_parser.clone())
                        .ignored(),
                )))
                .repeated(),
        )
        .ignored()
        .boxed();
    let identifier_path_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .separated_by(just('.').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::to_str)
        .boxed();
    let ascii_string_literal_parser = ascii_string_literal_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .repeated()
        .at_least(1usize)
        .ignored()
        .boxed();
    let enum_definition_parser = just("enum")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('{').then_ignore(ignore_parser.clone()))
        .ignore_then(
            identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .separated_by(just(',').then_ignore(ignore_parser.clone()))
                .at_least(1usize),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let named_argument_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(':').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignored()
        .boxed();
    let parameter_declaration_parser = type_name_parser
        .clone()
        .ignore_then(data_location_parser.clone().or_not())
        .then(
            identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .or_not(),
        )
        .ignored()
        .boxed();
    let selected_import_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
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
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just("is").then_ignore(ignore_parser.clone()))
        .ignore_then(elementary_type_with_payable_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let import_path_parser = ascii_string_literal_parser.clone().ignored().boxed();
    let literal_parser = choice::<_, ErrorType>((
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
        .ignore_then(choice::<_, ErrorType>((
            elementary_type_without_payable_parser.clone().ignored(),
            identifier_path_parser.clone().ignored(),
        )))
        .then_ignore(just("=>").then_ignore(ignore_parser.clone()))
        .then_ignore(type_name_parser.clone())
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
        .map(builder::to_str)
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
    let yul_literal_parser = choice::<_, ErrorType>((
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
            choice::<_, ErrorType>((
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
                .then_ignore(ignore_parser.clone())
                .or_not()
                .then_ignore(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then_ignore(block_parser.clone())
        .ignored()
        .boxed();
    let function_type_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .ignore_then(
            choice::<_, ErrorType>((
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
    let method_attribute_parser = choice::<_, ErrorType>((
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
        .then_ignore(import_path_parser.clone())
        .ignored()
        .boxed();
    let simple_import_directive_parser = import_path_parser
        .clone()
        .ignore_then(
            just("as")
                .then_ignore(ignore_parser.clone())
                .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
                .repeated(),
        )
        .ignored()
        .boxed();
    let star_import_directive_parser = just('*')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just("as").then_ignore(ignore_parser.clone()))
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just("from").then_ignore(ignore_parser.clone()))
        .ignore_then(import_path_parser.clone())
        .ignored()
        .boxed();
    let state_variable_attribute_parser = choice::<_, ErrorType>((
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
    yul_expression_parser.define(
        choice::<_, ErrorType>((
            yul_path_parser.clone().ignored(),
            yul_function_call_parser.clone().ignored(),
            yul_literal_parser.clone().ignored(),
        ))
        .ignored()
        .boxed(),
    );
    let import_directive_parser = just("import")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, ErrorType>((
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
        .map(builder::to_str)
        .boxed();
    type_name_parser.define(
        choice::<_, ErrorType>((
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
    let yul_assignment_parser = yul_path_parser
        .clone()
        .ignore_then(choice::<_, ErrorType>((
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
                .then_ignore(yul_function_call_parser.clone())
                .ignored(),
        )))
        .ignored()
        .boxed();
    let yul_for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .ignored()
        .boxed();
    let yul_if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .ignore_then(yul_block_parser.clone())
        .ignored()
        .boxed();
    let yul_switch_statement_parser = just("switch")
        .then_ignore(ignore_parser.clone())
        .ignore_then(yul_expression_parser.clone())
        .ignore_then(choice::<_, ErrorType>((
            just("case")
                .then_ignore(ignore_parser.clone())
                .ignore_then(yul_literal_parser.clone())
                .ignore_then(yul_block_parser.clone())
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
    let yul_variable_declaration_parser = just("let")
        .then_ignore(ignore_parser.clone())
        .ignore_then(
            yul_identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone()),
        )
        .ignore_then(
            choice::<_, ErrorType>((
                just(":=")
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(yul_expression_parser.clone())
                    .ignored(),
                just(',')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(
                        yul_identifier_parser
                            .clone()
                            .then_ignore(ignore_parser.clone()),
                    )
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
    let constructor_attribute_parser = choice::<_, ErrorType>((
        modifier_invocation_parser.clone().ignored(),
        just("payable").then_ignore(ignore_parser.clone()).ignored(),
        just("internal")
            .then_ignore(ignore_parser.clone())
            .ignored(),
        just("public").then_ignore(ignore_parser.clone()).ignored(),
    ))
    .ignored()
    .boxed();
    let error_parameter_parser = type_name_parser
        .clone()
        .ignore_then(
            identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .or_not(),
        )
        .ignored()
        .boxed();
    let event_parameter_parser = type_name_parser
        .clone()
        .ignore_then(just("indexed").then_ignore(ignore_parser.clone()).or_not())
        .then(
            identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .or_not(),
        )
        .ignored()
        .boxed();
    let fallback_function_attribute_parser = choice::<_, ErrorType>((
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
    let function_attribute_parser = choice::<_, ErrorType>((
        visibility_specifier_parser.clone().ignored(),
        state_mutability_specifier_parser.clone().ignored(),
        modifier_invocation_parser.clone().ignored(),
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
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
    let primary_expression_parser = choice::<_, ErrorType>((
        just("payable")
            .then_ignore(ignore_parser.clone())
            .ignore_then(argument_list_parser.clone())
            .ignored(),
        just("type")
            .then_ignore(ignore_parser.clone())
            .ignore_then(just('(').then_ignore(ignore_parser.clone()))
            .ignore_then(type_name_parser.clone())
            .ignore_then(just(')').then_ignore(ignore_parser.clone()))
            .ignored(),
        just("new")
            .then_ignore(ignore_parser.clone())
            .ignore_then(type_name_parser.clone())
            .ignored(),
        just('(')
            .then_ignore(ignore_parser.clone())
            .ignore_then(
                expression_parser
                    .clone()
                    .or_not()
                    .separated_by(just(',').then_ignore(ignore_parser.clone()))
                    .at_least(1usize),
            )
            .then_ignore(just(')').then_ignore(ignore_parser.clone()))
            .ignored(),
        just('[')
            .then_ignore(ignore_parser.clone())
            .ignore_then(
                expression_parser
                    .clone()
                    .separated_by(just(',').then_ignore(ignore_parser.clone()))
                    .at_least(1usize),
            )
            .then_ignore(just(']').then_ignore(ignore_parser.clone()))
            .ignored(),
        identifier_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .ignored(),
        literal_parser.clone().ignored(),
        elementary_type_without_payable_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let receive_function_attribute_parser = choice::<_, ErrorType>((
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
    let struct_definition_parser = just("struct")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('{').then_ignore(ignore_parser.clone()))
        .ignore_then(
            type_name_parser
                .clone()
                .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
                .ignore_then(just(';').then_ignore(ignore_parser.clone()))
                .repeated()
                .at_least(1usize),
        )
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let using_directive_parser = just("using")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, ErrorType>((
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
        .then(choice::<_, ErrorType>((
            just('*').then_ignore(ignore_parser.clone()).ignored(),
            type_name_parser.clone().ignored(),
        )))
        .then(just("global").then_ignore(ignore_parser.clone()).or_not())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let variable_declaration_parser = type_name_parser
        .clone()
        .ignore_then(data_location_parser.clone().or_not())
        .then_ignore(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let yul_statement_parser = choice::<_, ErrorType>((
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
    let directive_parser = choice::<_, ErrorType>((
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
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(
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
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(
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
    let index_access_expression_parser = primary_expression_parser
        .clone()
        .ignore_then(
            just('[')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone().or_not())
                .then(
                    just(':')
                        .then_ignore(ignore_parser.clone())
                        .ignore_then(expression_parser.clone().or_not())
                        .or_not(),
                )
                .then_ignore(just(']').then_ignore(ignore_parser.clone()))
                .repeated(),
        )
        .ignored()
        .boxed();
    let variable_declaration_tuple_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(just(',').then_ignore(ignore_parser.clone()).repeated())
        .then_ignore(variable_declaration_parser.clone())
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
        .then_ignore(yul_block_parser.clone())
        .ignored()
        .boxed();
    let member_access_expression_parser = index_access_expression_parser
        .clone()
        .ignore_then(
            just('.')
                .then_ignore(ignore_parser.clone())
                .ignore_then(choice::<_, ErrorType>((
                    identifier_parser
                        .clone()
                        .then_ignore(ignore_parser.clone())
                        .ignored(),
                    just("address").then_ignore(ignore_parser.clone()).ignored(),
                )))
                .repeated(),
        )
        .ignored()
        .boxed();
    let function_call_options_expression_parser = member_access_expression_parser
        .clone()
        .ignore_then(
            just('{')
                .then_ignore(ignore_parser.clone())
                .ignore_then(
                    named_argument_parser
                        .clone()
                        .separated_by(just(',').then_ignore(ignore_parser.clone()))
                        .at_least(1usize),
                )
                .then_ignore(just('}').then_ignore(ignore_parser.clone()))
                .repeated(),
        )
        .ignored()
        .boxed();
    let function_call_expression_parser = function_call_options_expression_parser
        .clone()
        .ignore_then(argument_list_parser.clone().repeated())
        .ignored()
        .boxed();
    let unary_prefix_expression_parser = unary_prefix_operator_parser
        .clone()
        .ignore_then(function_call_expression_parser.clone())
        .ignored()
        .boxed();
    let unary_suffix_expression_parser = unary_prefix_expression_parser
        .clone()
        .ignore_then(unary_suffix_operator_parser.clone())
        .ignored()
        .boxed();
    let exp_expression_parser = unary_suffix_expression_parser
        .clone()
        .ignore_then(just("**").then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignored()
        .boxed();
    let mul_div_mod_expression_parser = exp_expression_parser
        .clone()
        .ignore_then(
            mul_div_mod_operator_parser
                .clone()
                .ignore_then(exp_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let add_sub_expression_parser = mul_div_mod_expression_parser
        .clone()
        .ignore_then(
            add_sub_operator_parser
                .clone()
                .ignore_then(mul_div_mod_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let shift_expression_parser = add_sub_expression_parser
        .clone()
        .ignore_then(
            shift_operator_parser
                .clone()
                .ignore_then(add_sub_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let bit_and_expression_parser = shift_expression_parser
        .clone()
        .ignore_then(
            just('&')
                .then_ignore(ignore_parser.clone())
                .ignore_then(shift_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let bit_xor_expression_parser = bit_and_expression_parser
        .clone()
        .ignore_then(
            just('^')
                .then_ignore(ignore_parser.clone())
                .ignore_then(bit_and_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let bit_or_expression_parser = bit_xor_expression_parser
        .clone()
        .ignore_then(
            just('|')
                .then_ignore(ignore_parser.clone())
                .ignore_then(bit_xor_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let order_comparison_expression_parser = bit_or_expression_parser
        .clone()
        .ignore_then(
            order_comparison_operator_parser
                .clone()
                .ignore_then(bit_or_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let equality_comparison_expression_parser = order_comparison_expression_parser
        .clone()
        .ignore_then(
            equality_comparison_operator_parser
                .clone()
                .ignore_then(order_comparison_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let and_expression_parser = equality_comparison_expression_parser
        .clone()
        .ignore_then(
            just("&&")
                .then_ignore(ignore_parser.clone())
                .ignore_then(equality_comparison_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let or_expression_parser = and_expression_parser
        .clone()
        .ignore_then(
            just("||")
                .then_ignore(ignore_parser.clone())
                .ignore_then(and_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let conditional_expression_parser = or_expression_parser
        .clone()
        .ignore_then(just('?').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignore_then(just(':').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignored()
        .boxed();
    let assignment_expression_parser = conditional_expression_parser
        .clone()
        .ignore_then(assignment_operator_parser.clone())
        .ignore_then(expression_parser.clone())
        .ignored()
        .boxed();
    expression_parser.define(assignment_expression_parser.clone().ignored().boxed());
    let constant_definition_parser = type_name_parser
        .clone()
        .ignore_then(just("constant").then_ignore(ignore_parser.clone()))
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('=').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let do_while_statement_parser = just("do")
        .then_ignore(ignore_parser.clone())
        .ignore_then(statement_parser.clone())
        .ignore_then(just("while").then_ignore(ignore_parser.clone()))
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let emit_statement_parser = just("emit")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .ignore_then(argument_list_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let expression_statement_parser = expression_parser
        .clone()
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let if_statement_parser = just("if")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(statement_parser.clone())
        .ignore_then(
            just("else")
                .then_ignore(ignore_parser.clone())
                .ignore_then(statement_parser.clone())
                .or_not(),
        )
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
        .ignore_then(argument_list_parser.clone())
        .ignore_then(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let state_variable_declaration_parser = type_name_parser
        .clone()
        .ignore_then(state_variable_attribute_parser.clone().repeated())
        .then_ignore(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .then(
            just('=')
                .then_ignore(ignore_parser.clone())
                .ignore_then(expression_parser.clone())
                .or_not(),
        )
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let try_statement_parser = just("try")
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .ignore_then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then_ignore(block_parser.clone())
        .then_ignore(catch_clause_parser.clone())
        .then(catch_clause_parser.clone().repeated())
        .ignored()
        .boxed();
    let variable_declaration_statement_parser = choice::<_, ErrorType>((
        variable_declaration_parser
            .clone()
            .ignore_then(
                just('=')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(expression_parser.clone())
                    .or_not(),
            )
            .ignored(),
        variable_declaration_tuple_parser
            .clone()
            .ignore_then(just('=').then_ignore(ignore_parser.clone()))
            .ignore_then(expression_parser.clone())
            .ignored(),
    ))
    .then_ignore(just(';').then_ignore(ignore_parser.clone()))
    .ignored()
    .boxed();
    let while_statement_parser = just("while")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(expression_parser.clone())
        .ignore_then(just(')').then_ignore(ignore_parser.clone()))
        .ignore_then(statement_parser.clone())
        .ignored()
        .boxed();
    let simple_statement_parser = choice::<_, ErrorType>((
        variable_declaration_statement_parser.clone().ignored(),
        expression_statement_parser.clone().ignored(),
    ))
    .ignored()
    .boxed();
    let for_statement_parser = just("for")
        .then_ignore(ignore_parser.clone())
        .ignore_then(just('(').then_ignore(ignore_parser.clone()))
        .ignore_then(choice::<_, ErrorType>((
            simple_statement_parser.clone().ignored(),
            just(';').then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then(choice::<_, ErrorType>((
            expression_statement_parser.clone().ignored(),
            just(';').then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then(expression_parser.clone().or_not())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()))
        .then_ignore(statement_parser.clone())
        .ignored()
        .boxed();
    statement_parser.define(
        choice::<_, ErrorType>((
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
                choice::<_, ErrorType>((
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
        .ignore_then(constructor_attribute_parser.clone().repeated())
        .then_ignore(block_parser.clone())
        .ignored()
        .boxed();
    let fallback_function_definition_parser = just("fallback")
        .then_ignore(ignore_parser.clone())
        .ignore_then(parameter_list_parser.clone())
        .ignore_then(fallback_function_attribute_parser.clone().repeated())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(choice::<_, ErrorType>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let function_definition_parser = just("function")
        .then_ignore(ignore_parser.clone())
        .ignore_then(choice::<_, ErrorType>((
            identifier_parser
                .clone()
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("fallback")
                .then_ignore(ignore_parser.clone())
                .ignored(),
            just("receive").then_ignore(ignore_parser.clone()).ignored(),
        )))
        .then_ignore(parameter_list_parser.clone())
        .then(function_attribute_parser.clone().repeated())
        .then(
            just("returns")
                .then_ignore(ignore_parser.clone())
                .ignore_then(non_empty_parameter_list_parser.clone())
                .or_not(),
        )
        .then(choice::<_, ErrorType>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let modifier_definition_parser = just("modifier")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(parameter_list_parser.clone().or_not())
        .then(method_attribute_parser.clone().repeated())
        .then(choice::<_, ErrorType>((
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
        .then(choice::<_, ErrorType>((
            just(';').then_ignore(ignore_parser.clone()).ignored(),
            block_parser.clone().ignored(),
        )))
        .ignored()
        .boxed();
    let contract_body_element_parser = choice::<_, ErrorType>((
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
        .then_ignore(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let interface_definition_parser = just("interface")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(inheritance_specifier_list_parser.clone().or_not())
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let library_definition_parser = just("library")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just('{').then_ignore(ignore_parser.clone()))
        .ignore_then(contract_body_element_parser.clone().repeated())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .ignored()
        .boxed();
    let definition_parser = choice::<_, ErrorType>((
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
        .ignore_then(
            choice::<_, ErrorType>((
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
