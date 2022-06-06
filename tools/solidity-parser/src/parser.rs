use chumsky::{prelude::*, Parser};

pub type ErrorType = Simple<char>;
use super::builder;
use syntax_schema::chumsky::combinators::NomicParser;

pub type SourceUnitParserResultType = ();

pub fn create_source_unit_parser(
) -> impl Parser<char, SourceUnitParserResultType, Error = ErrorType> {
    let mut ascii_string_literal_parser = Recursive::declare();
    let mut block_parser = Recursive::declare();
    let mut expression_parser = Recursive::declare();
    let mut statement_parser = Recursive::declare();
    let mut type_name_parser = Recursive::declare();
    let mut unicode_string_literal_parser = Recursive::declare();
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
    ascii_string_literal_parser.define(
        ascii_string_literal_parser
            .clone()
            .repeated()
            .at_least(1usize)
            .ignored()
            .boxed(),
    );
    let boolean_literal_parser =
        choice::<_, ErrorType>((just("false").ignored(), just("true").ignored())).ignored();
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| c != '*').ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(filter(|&c: &char| c != '*' && c != '/'))
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
        .ignore_then(
            choice::<_, ErrorType>((
                just("1")
                    .then(choice((
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        just("3").ignored(),
                        just("4").ignored(),
                        just("5").ignored(),
                        just("6").ignored(),
                        just("7").ignored(),
                        just("8").ignored(),
                        just("9").ignored(),
                        empty(),
                    )))
                    .ignored(),
                just("2")
                    .then(choice((
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        just("3").ignored(),
                        just("4").ignored(),
                        just("5").ignored(),
                        just("6").ignored(),
                        just("7").ignored(),
                        just("8").ignored(),
                        just("9").ignored(),
                        empty(),
                    )))
                    .ignored(),
                just("3")
                    .then(choice((
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        empty(),
                    )))
                    .ignored(),
                just("4").ignored(),
                just("5").ignored(),
                just("6").ignored(),
                just("7").ignored(),
                just("8").ignored(),
                just("9").ignored(),
            ))
            .then_ignore(filter(|&c: &char| !c.is_ascii_digit()).rewind()),
        )
        .map(builder::to_str);
    let fixed_type_parser = just("fixed")
        .ignore_then(
            filter(|&c: &char| '1' <= c && c <= '9')
                .then(filter(|&c: &char| c.is_ascii_digit()).repeated())
                .then_ignore(just('x'))
                .then(filter(|&c: &char| '1' <= c && c <= '9'))
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
        .ignore_then(filter(|&c: &char| c != '\n' && c != '\r').repeated())
        .ignored();
    let number_unit_parser = choice::<_, ErrorType>((
        just("days").ignored(),
        just("ether").ignored(),
        just("gwei").ignored(),
        just("hours").ignored(),
        just("minutes").ignored(),
        just("seconds").ignored(),
        just("we")
            .then(choice((just("eks").ignored(), just("i").ignored())))
            .ignored(),
        just("years").ignored(),
    ))
    .ignored();
    let pragma_directive_parser = just("pragma")
        .ignore_then(filter(|&c: &char| c != ';'))
        .then(filter(|&c: &char| c != ';').repeated())
        .then_ignore(just(';'))
        .ignored();
    let reserved_keyword_parser = choice::<_, ErrorType>((
        just("a")
            .then(choice((
                just("fter").ignored(),
                just("lias").ignored(),
                just("pply").ignored(),
                just("uto").ignored(),
            )))
            .ignored(),
        just("byte").ignored(),
        just("c")
            .then(choice((just("ase").ignored(), just("opyof").ignored())))
            .ignored(),
        just("def")
            .then(choice((just("ault").ignored(), just("ine").ignored())))
            .ignored(),
        just("final").ignored(),
        just("i")
            .then(choice((
                just("mplements").ignored(),
                just("n")
                    .then(choice((just("line").ignored(), empty())))
                    .ignored(),
            )))
            .ignored(),
        just("let").ignored(),
        just("m")
            .then(choice((
                just("a")
                    .then(choice((just("cro").ignored(), just("tch").ignored())))
                    .ignored(),
                just("utable").ignored(),
            )))
            .ignored(),
        just("null").ignored(),
        just("of").ignored(),
        just("p")
            .then(choice((just("artial").ignored(), just("romise").ignored())))
            .ignored(),
        just("re")
            .then(choice((
                just("ference").ignored(),
                just("locatable").ignored(),
            )))
            .ignored(),
        just("s")
            .then(choice((
                just("ealed").ignored(),
                just("izeof").ignored(),
                just("tatic").ignored(),
                just("upports").ignored(),
                just("witch").ignored(),
            )))
            .ignored(),
        just("type")
            .then(choice((just("def").ignored(), just("of").ignored())))
            .ignored(),
        just("var").ignored(),
    ))
    .ignored();
    let signed_integer_type_parser = just("int")
        .ignore_then(
            choice::<_, ErrorType>((
                just("1")
                    .then(choice((
                        just("04").ignored(),
                        just("12").ignored(),
                        just("2")
                            .then(choice((just("0").ignored(), just("8").ignored())))
                            .ignored(),
                        just("36").ignored(),
                        just("44").ignored(),
                        just("52").ignored(),
                        just("6")
                            .then(choice((just("0").ignored(), just("8").ignored(), empty())))
                            .ignored(),
                        just("76").ignored(),
                        just("84").ignored(),
                        just("92").ignored(),
                    )))
                    .ignored(),
                just("2")
                    .then(choice((
                        just("0")
                            .then(choice((just("0").ignored(), just("8").ignored())))
                            .ignored(),
                        just("16").ignored(),
                        just("24").ignored(),
                        just("32").ignored(),
                        just("4")
                            .then(choice((just("0").ignored(), just("8").ignored(), empty())))
                            .ignored(),
                        just("56").ignored(),
                    )))
                    .ignored(),
                just("32").ignored(),
                just("4")
                    .then(choice((just("0").ignored(), just("8").ignored())))
                    .ignored(),
                just("56").ignored(),
                just("64").ignored(),
                just("72").ignored(),
                just("8")
                    .then(choice((just("0").ignored(), just("8").ignored(), empty())))
                    .ignored(),
                just("96").ignored(),
            ))
            .then_ignore(filter(|&c: &char| !c.is_ascii_digit()).rewind()),
        )
        .map(builder::to_str);
    unicode_string_literal_parser.define(
        unicode_string_literal_parser
            .clone()
            .repeated()
            .at_least(1usize)
            .ignored()
            .boxed(),
    );
    let whitespace_parser =
        filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n').ignored();
    let yul_decimal_number_literal_parser = choice::<_, ErrorType>((
        just('0').map(builder::to_str).ignored(),
        filter(|&c: &char| '1' <= c && c <= '9')
            .chain(filter(|&c: &char| c.is_ascii_digit()).repeated())
            .map(builder::to_str)
            .ignored(),
    ))
    .ignored();
    let yul_evm_builtin_function_name_parser = choice::<_, ErrorType>((
        choice::<_, ErrorType>((
            just("Blockhash").ignored(),
            just("a")
                .then(choice((
                    just("dd")
                        .then(choice((
                            just("mod").ignored(),
                            just("ress").ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("nd").ignored(),
                )))
                .ignored(),
            just("b")
                .then(choice((
                    just("a")
                        .then(choice((just("lance").ignored(), just("sefee").ignored())))
                        .ignored(),
                    just("yte").ignored(),
                )))
                .ignored(),
            just("c")
                .then(choice((
                    just("all")
                        .then(choice((
                            just("code").ignored(),
                            just("data")
                                .then(choice((
                                    just("copy").ignored(),
                                    just("load").ignored(),
                                    just("size").ignored(),
                                )))
                                .ignored(),
                            just("er").ignored(),
                            just("value").ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("hainid").ignored(),
                    just("oinbase").ignored(),
                    just("reate")
                        .then(choice((just("2").ignored(), empty())))
                        .ignored(),
                )))
                .ignored(),
            just("d")
                .then(choice((
                    just("elegatecall").ignored(),
                    just("i")
                        .then(choice((just("fficulty").ignored(), just("v").ignored())))
                        .ignored(),
                )))
                .ignored(),
            just("e")
                .then(choice((
                    just("q").ignored(),
                    just("x")
                        .then(choice((
                            just("p").ignored(),
                            just("tcode")
                                .then(choice((
                                    just("copy").ignored(),
                                    just("hash").ignored(),
                                    just("size").ignored(),
                                )))
                                .ignored(),
                        )))
                        .ignored(),
                )))
                .ignored(),
            just("g")
                .then(choice((
                    just("as")
                        .then(choice((
                            just("limit").ignored(),
                            just("price").ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("t").ignored(),
                )))
                .ignored(),
            just("i")
                .then(choice((just("nvalid").ignored(), just("szero").ignored())))
                .ignored(),
            just("keccak256").ignored(),
            just("l")
                .then(choice((
                    just("og")
                        .then(choice((
                            just("0").ignored(),
                            just("1").ignored(),
                            just("2").ignored(),
                            just("3").ignored(),
                            just("4").ignored(),
                        )))
                        .ignored(),
                    just("t").ignored(),
                )))
                .ignored(),
            just("m")
                .then(choice((
                    just("load").ignored(),
                    just("od").ignored(),
                    just("s")
                        .then(choice((
                            just("ize").ignored(),
                            just("tore")
                                .then(choice((just("8").ignored(), empty())))
                                .ignored(),
                        )))
                        .ignored(),
                    just("ul")
                        .then(choice((just("mod").ignored(), empty())))
                        .ignored(),
                )))
                .ignored(),
            just("n")
                .then(choice((just("ot").ignored(), just("umber").ignored())))
                .ignored(),
            just("or")
                .then(choice((just("igin").ignored(), empty())))
                .ignored(),
            just("pop").ignored(),
            just("re")
                .then(choice((
                    just("turn")
                        .then(choice((
                            just("data")
                                .then(choice((just("copy").ignored(), just("size").ignored())))
                                .ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("vert").ignored(),
                )))
                .ignored(),
            just("s")
                .then(choice((
                    just("ar").ignored(),
                    just("div").ignored(),
                    just("elf")
                        .then(choice((
                            just("balance").ignored(),
                            just("destruct").ignored(),
                        )))
                        .ignored(),
                    just("gt").ignored(),
                    just("h")
                        .then(choice((just("l").ignored(), just("r").ignored())))
                        .ignored(),
                    just("ignextend").ignored(),
                    just("l")
                        .then(choice((just("oad").ignored(), just("t").ignored())))
                        .ignored(),
                    just("mod").ignored(),
                    just("store").ignored(),
                    just("t")
                        .then(choice((just("aticcall").ignored(), just("op").ignored())))
                        .ignored(),
                    just("ub").ignored(),
                )))
                .ignored(),
        )),
        choice::<_, ErrorType>((just("timestamp").ignored(), just("xor").ignored())),
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
        just("break").ignored(),
        just("c")
            .then(choice((just("ase").ignored(), just("ontinue").ignored())))
            .ignored(),
        just("default").ignored(),
        just("f")
            .then(choice((just("or").ignored(), just("unction").ignored())))
            .ignored(),
        just("hex").ignored(),
        just("if").ignored(),
        just("le")
            .then(choice((just("ave").ignored(), just("t").ignored())))
            .ignored(),
        just("switch").ignored(),
    ))
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
        .ignore_then(hex_character_parser.clone().repeated().exactly(2usize))
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
    let import_path_parser = ascii_string_literal_parser.clone().ignored().boxed();
    let possibly_separated_pairs_of_hex_digits_parser = hex_character_parser
        .clone()
        .repeated()
        .exactly(2usize)
        .separated_by(just('_').or_not())
        .at_least(1usize)
        .ignored();
    let ufixed_type_parser = just('u')
        .ignore_then(fixed_type_parser.clone())
        .map(builder::to_str);
    let unicode_escape_parser = just('u')
        .ignore_then(hex_character_parser.clone().repeated().exactly(4usize))
        .map(builder::to_char);
    let unsigned_integer_type_parser = just('u')
        .ignore_then(signed_integer_type_parser.clone())
        .map(builder::to_str);
    let yul_reserved_word_parser = choice::<_, ErrorType>((
        choice::<_, ErrorType>((
            just("Blockhash").ignored(),
            just("a")
                .then(choice((
                    just("dd")
                        .then(choice((
                            just("mod").ignored(),
                            just("ress").ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("nd").ignored(),
                )))
                .ignored(),
            just("b")
                .then(choice((
                    just("a")
                        .then(choice((just("lance").ignored(), just("sefee").ignored())))
                        .ignored(),
                    just("reak").ignored(),
                    just("yte").ignored(),
                )))
                .ignored(),
            just("c")
                .then(choice((
                    just("a")
                        .then(choice((
                            just("ll")
                                .then(choice((
                                    just("code").ignored(),
                                    just("data")
                                        .then(choice((
                                            just("copy").ignored(),
                                            just("load").ignored(),
                                            just("size").ignored(),
                                        )))
                                        .ignored(),
                                    just("er").ignored(),
                                    just("value").ignored(),
                                    empty(),
                                )))
                                .ignored(),
                            just("se").ignored(),
                        )))
                        .ignored(),
                    just("hainid").ignored(),
                    just("o")
                        .then(choice((just("inbase").ignored(), just("ntinue").ignored())))
                        .ignored(),
                    just("reate")
                        .then(choice((just("2").ignored(), empty())))
                        .ignored(),
                )))
                .ignored(),
            just("d")
                .then(choice((
                    just("e")
                        .then(choice((
                            just("fault").ignored(),
                            just("legatecall").ignored(),
                        )))
                        .ignored(),
                    just("i")
                        .then(choice((just("fficulty").ignored(), just("v").ignored())))
                        .ignored(),
                )))
                .ignored(),
            just("e")
                .then(choice((
                    just("q").ignored(),
                    just("x")
                        .then(choice((
                            just("p").ignored(),
                            just("tcode")
                                .then(choice((
                                    just("copy").ignored(),
                                    just("hash").ignored(),
                                    just("size").ignored(),
                                )))
                                .ignored(),
                        )))
                        .ignored(),
                )))
                .ignored(),
            just("f")
                .then(choice((
                    just("alse").ignored(),
                    just("or").ignored(),
                    just("unction").ignored(),
                )))
                .ignored(),
            just("g")
                .then(choice((
                    just("as")
                        .then(choice((
                            just("limit").ignored(),
                            just("price").ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("t").ignored(),
                )))
                .ignored(),
            just("hex").ignored(),
            just("i")
                .then(choice((
                    just("f").ignored(),
                    just("nvalid").ignored(),
                    just("szero").ignored(),
                )))
                .ignored(),
            just("keccak256").ignored(),
            just("l")
                .then(choice((
                    just("e")
                        .then(choice((just("ave").ignored(), just("t").ignored())))
                        .ignored(),
                    just("og")
                        .then(choice((
                            just("0").ignored(),
                            just("1").ignored(),
                            just("2").ignored(),
                            just("3").ignored(),
                            just("4").ignored(),
                        )))
                        .ignored(),
                    just("t").ignored(),
                )))
                .ignored(),
            just("m")
                .then(choice((
                    just("load").ignored(),
                    just("od").ignored(),
                    just("s")
                        .then(choice((
                            just("ize").ignored(),
                            just("tore")
                                .then(choice((just("8").ignored(), empty())))
                                .ignored(),
                        )))
                        .ignored(),
                    just("ul")
                        .then(choice((just("mod").ignored(), empty())))
                        .ignored(),
                )))
                .ignored(),
            just("n")
                .then(choice((just("ot").ignored(), just("umber").ignored())))
                .ignored(),
            just("or")
                .then(choice((just("igin").ignored(), empty())))
                .ignored(),
            just("pop").ignored(),
        )),
        choice::<_, ErrorType>((
            just("re")
                .then(choice((
                    just("turn")
                        .then(choice((
                            just("data")
                                .then(choice((just("copy").ignored(), just("size").ignored())))
                                .ignored(),
                            empty(),
                        )))
                        .ignored(),
                    just("vert").ignored(),
                )))
                .ignored(),
            just("s")
                .then(choice((
                    just("ar").ignored(),
                    just("div").ignored(),
                    just("elf")
                        .then(choice((
                            just("balance").ignored(),
                            just("destruct").ignored(),
                        )))
                        .ignored(),
                    just("gt").ignored(),
                    just("h")
                        .then(choice((just("l").ignored(), just("r").ignored())))
                        .ignored(),
                    just("ignextend").ignored(),
                    just("l")
                        .then(choice((just("oad").ignored(), just("t").ignored())))
                        .ignored(),
                    just("mod").ignored(),
                    just("store").ignored(),
                    just("t")
                        .then(choice((just("aticcall").ignored(), just("op").ignored())))
                        .ignored(),
                    just("ub").ignored(),
                    just("witch").ignored(),
                )))
                .ignored(),
            just("t")
                .then(choice((just("imestamp").ignored(), just("rue").ignored())))
                .ignored(),
            just("xor").ignored(),
        )),
    ))
    .ignored();
    let add_sub_operator_parser = filter(|&c: &char| c == '+' || c == '-').ignored().boxed();
    let assignment_operator_parser = choice::<_, ErrorType>((
        just("%=").ignored(),
        just("&=").ignored(),
        just("*=").ignored(),
        just("+=").ignored(),
        just("-=").ignored(),
        just("/=").ignored(),
        just("<<=").ignored(),
        just("=").ignored(),
        just(">>")
            .then(choice((just("=").ignored(), just(">=").ignored())))
            .ignored(),
        just("^=").ignored(),
        just("|=").ignored(),
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
    let data_location_parser = choice::<_, ErrorType>((
        just("calldata").ignored(),
        just("memory").ignored(),
        just("storage").ignored(),
    ))
    .ignored()
    .boxed();
    let decimal_number_parser = choice::<_, ErrorType>((
        decimal_integer_parser.clone().ignored(),
        decimal_float_parser.clone().ignored(),
    ))
    .then(decimal_exponent_parser.clone().or_not())
    .ignored();
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
    let equality_comparison_operator_parser =
        choice::<_, ErrorType>((just("!=").ignored(), just("==").ignored()))
            .ignored()
            .boxed();
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
    let mul_div_mod_operator_parser = filter(|&c: &char| c == '*' || c == '/' || c == '%')
        .ignored()
        .boxed();
    let order_comparison_operator_parser = choice::<_, ErrorType>((
        just("<")
            .then(choice((just("=").ignored(), empty())))
            .ignored(),
        just(">")
            .then(choice((just("=").ignored(), empty())))
            .ignored(),
    ))
    .ignored()
    .boxed();
    let positional_argument_list_parser = expression_parser
        .clone()
        .separated_by(just(',').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .ignored()
        .boxed();
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_part_parser.clone().repeated());
    let shift_operator_parser = choice::<_, ErrorType>((
        just("<<").ignored(),
        just(">>")
            .then(choice((just(">").ignored(), empty())))
            .ignored(),
    ))
    .ignored()
    .boxed();
    let state_mutability_specifier_parser = choice::<_, ErrorType>((
        just("p")
            .then(choice((just("ayable").ignored(), just("ure").ignored())))
            .ignored(),
        just("view").ignored(),
    ))
    .boxed();
    let unary_prefix_operator_parser = choice::<_, ErrorType>((
        just("!").ignored(),
        just("++").ignored(),
        just("-")
            .then(choice((just("-").ignored(), empty())))
            .ignored(),
        just("delete").ignored(),
        just("~").ignored(),
    ))
    .ignored()
    .boxed();
    let unary_suffix_operator_parser =
        choice::<_, ErrorType>((just("++").ignored(), just("--").ignored()))
            .ignored()
            .boxed();
    let unchecked_block_parser = just("unchecked")
        .then_ignore(ignore_parser.clone())
        .ignore_then(block_parser.clone())
        .ignored()
        .boxed();
    let visibility_specifier_parser = choice::<_, ErrorType>((
        just("external").ignored(),
        just("internal").ignored(),
        just("p")
            .then(choice((just("rivate").ignored(), just("ublic").ignored())))
            .ignored(),
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
                filter(|&c: &char| ' ' <= c && c <= '~' && c != '"' && c != '\\').ignored(),
                escape_sequence_parser.clone().ignored(),
            ))
            .repeated(),
        )
        .then_ignore(just('"'))
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
    let reserved_word_parser = choice::<_, ErrorType>((
        keyword_parser.clone().ignored(),
        reserved_keyword_parser.clone().ignored(),
        number_unit_parser.clone().ignored(),
        boolean_literal_parser.clone().ignored(),
    ))
    .ignored();
    let yul_identifier_parser = raw_identifier_parser
        .clone()
        .excluding(yul_reserved_word_parser.clone())
        .ignored();
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
    let identifier_parser = raw_identifier_parser
        .clone()
        .excluding(reserved_word_parser.clone())
        .ignored();
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
    let identifier_path_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .separated_by(just('.').then_ignore(ignore_parser.clone()))
        .at_least(1usize)
        .map(builder::to_str)
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
    let user_defined_value_type_definition_parser = just("type")
        .then_ignore(ignore_parser.clone())
        .ignore_then(identifier_parser.clone().then_ignore(ignore_parser.clone()))
        .ignore_then(just("is").then_ignore(ignore_parser.clone()))
        .ignore_then(elementary_type_with_payable_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
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
                        .exactly(1usize),
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
                just("external").ignored(),
                just("internal").ignored(),
                just("p")
                    .then(choice((
                        just("ayable").ignored(),
                        just("rivate").ignored(),
                        just("u")
                            .then(choice((just("blic").ignored(), just("re").ignored())))
                            .ignored(),
                    )))
                    .ignored(),
                just("view").ignored(),
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
    let method_attribute_parser = choice::<_, ErrorType>((
        just("virtual").then_ignore(ignore_parser.clone()).ignored(),
        override_specifier_parser.clone().ignored(),
    ))
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
    let bit_x_or_expression_parser = bit_and_expression_parser
        .clone()
        .ignore_then(
            just('^')
                .then_ignore(ignore_parser.clone())
                .ignore_then(bit_and_expression_parser.clone())
                .repeated(),
        )
        .ignored()
        .boxed();
    let bit_or_expression_parser = bit_x_or_expression_parser
        .clone()
        .ignore_then(
            just('|')
                .then_ignore(ignore_parser.clone())
                .ignore_then(bit_x_or_expression_parser.clone())
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
