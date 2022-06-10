use super::parser_interface::*;
use super::tree_interface::*;
use chumsky::prelude::*;
use chumsky::primitive::Just;
use chumsky::Parser;
#[allow(unused_imports)]
use syntax_schema::chumsky::combinators::*;
#[allow(dead_code)]
fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
    let mut expressions = vec![e];
    let mut separators = vec![];
    for (s, e) in es.into_iter() {
        separators.push(s);
        expressions.push(e);
    }
    (expressions, separators)
}
#[allow(dead_code)]
#[inline]
fn terminal(str: &str) -> Just<char, &str, ErrorType> {
    just(str)
}
impl Parsers {
    pub fn new() -> Self {
        let mut block_parser = Recursive::declare();
        let mut expression_parser = Recursive::declare();
        let mut ignore_parser = Recursive::declare();
        let mut statement_parser = Recursive::declare();
        let mut type_name_parser = Recursive::declare();
        let mut yul_block_parser = Recursive::declare();
        let mut yul_expression_parser = Recursive::declare();
        let add_sub_operator_parser = filter(|&c: &char| c == '+' || c == '-').boxed();
        let assignment_operator_parser = choice::<_, ErrorType>((
            terminal("%=").map(|_| 2usize),
            terminal("&=").map(|_| 2usize),
            terminal("*=").map(|_| 2usize),
            terminal("+=").map(|_| 2usize),
            terminal("-=").map(|_| 2usize),
            terminal("/=").map(|_| 2usize),
            terminal("<<=").map(|_| 3usize),
            terminal("=").map(|_| 1usize),
            terminal(">>").ignore_then(choice((
                terminal("=").map(|_| 3usize),
                terminal(">=").map(|_| 4usize),
            ))),
            terminal("^=").map(|_| 2usize),
            terminal("|=").map(|_| 2usize),
        ))
        .boxed();
        let break_statement_parser = terminal("break")
            .map(|_| 5usize)
            .then(just(';'))
            .map(|v| Box::new(break_statement::_S0::new(v)))
            .boxed();
        let comment_parser = terminal("/*")
            .map(|_| 2usize)
            .then(
                choice((
                    filter(|&c: &char| c != '*').map(|v| Box::new(comment::_C2::StarChar(v))),
                    just('*')
                        .repeated()
                        .at_least(1usize)
                        .then(filter(|&c: &char| c != '*' && c != '/'))
                        .map(|v| Box::new(comment::_S3::new(v)))
                        .map(|v| Box::new(comment::_C2::_S3(v))),
                ))
                .repeated(),
            )
            .then(just('*').repeated().at_least(1usize))
            .then(just('/'))
            .map(|v| Box::new(comment::_S0::new(v)))
            .boxed();
        let continue_statement_parser = terminal("continue")
            .map(|_| 8usize)
            .then(just(';'))
            .map(|v| Box::new(continue_statement::_S0::new(v)))
            .boxed();
        let data_location_parser = choice::<_, ErrorType>((
            terminal("calldata").map(|_| 8usize),
            terminal("memory").map(|_| 6usize),
            terminal("storage").map(|_| 7usize),
        ))
        .boxed();
        let equality_comparison_operator_parser = choice::<_, ErrorType>((
            terminal("!=").map(|_| 2usize),
            terminal("==").map(|_| 2usize),
        ))
        .boxed();
        let line_comment_parser = terminal("//")
            .map(|_| 2usize)
            .then(filter(|&c: &char| c != '\n' && c != '\r').repeated())
            .map(|v| Box::new(line_comment::_S0::new(v)))
            .boxed();
        let mul_div_mod_operator_parser =
            filter(|&c: &char| c == '*' || c == '/' || c == '%').boxed();
        let order_comparison_operator_parser = choice::<_, ErrorType>((
            terminal("<").ignore_then(choice((
                terminal("=").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
            terminal(">").ignore_then(choice((
                terminal("=").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
        ))
        .boxed();
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(just(',').then(expression_parser.clone()).repeated())
            .map(repetition_mapper)
            .map(|v| Box::new(positional_argument_list::_S0::new(v)))
            .boxed();
        let shift_operator_parser = choice::<_, ErrorType>((
            terminal("<<").map(|_| 2usize),
            terminal(">>").ignore_then(choice((
                terminal(">").map(|_| 3usize),
                empty().map(|_| 2usize),
            ))),
        ))
        .boxed();
        let state_mutability_specifier_parser = choice::<_, ErrorType>((
            terminal("p").ignore_then(choice((
                terminal("ayable").map(|_| 7usize),
                terminal("ure").map(|_| 4usize),
            ))),
            terminal("view").map(|_| 4usize),
        ))
        .boxed();
        let unary_prefix_operator_parser = choice::<_, ErrorType>((
            terminal("!").map(|_| 1usize),
            terminal("++").map(|_| 2usize),
            terminal("-").ignore_then(choice((
                terminal("-").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
            terminal("delete").map(|_| 6usize),
            terminal("~").map(|_| 1usize),
        ))
        .boxed();
        let unary_suffix_operator_parser = choice::<_, ErrorType>((
            terminal("++").map(|_| 2usize),
            terminal("--").map(|_| 2usize),
        ))
        .boxed();
        let unchecked_block_parser = terminal("unchecked")
            .map(|_| 9usize)
            .then(block_parser.clone())
            .map(|v| Box::new(unchecked_block::_S0::new(v)))
            .boxed();
        let visibility_specifier_parser = choice::<_, ErrorType>((
            terminal("external").map(|_| 8usize),
            terminal("internal").map(|_| 8usize),
            terminal("p").ignore_then(choice((
                terminal("rivate").map(|_| 7usize),
                terminal("ublic").map(|_| 6usize),
            ))),
        ))
        .boxed();
        let whitespace_parser =
            filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n').boxed();
        let yul_break_statement_parser = terminal("break").map(|_| 5usize).boxed();
        let yul_continue_statement_parser = terminal("continue").map(|_| 8usize).boxed();
        let yul_leave_statement_parser = terminal("leave").map(|_| 5usize).boxed();
        ignore_parser.define(
            choice((
                choice::<_, ErrorType>((
                    terminal("\t").map(|_| 1usize),
                    terminal("\n").map(|_| 1usize),
                    terminal("\r").map(|_| 1usize),
                    terminal(" ").map(|_| 1usize),
                ))
                .map(|v| Box::new(ignore::_C1::_0(v))),
                comment_parser
                    .clone()
                    .map(|v| Box::new(ignore::_C1::Comment(v))),
                line_comment_parser
                    .clone()
                    .map(|v| Box::new(ignore::_C1::LineComment(v))),
            ))
            .repeated()
            .boxed(),
        );
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
        .boxed();
        let boolean_literal_parser = choice::<_, ErrorType>((
            terminal("false").map(|_| 5usize),
            terminal("true").map(|_| 4usize),
        ))
        .boxed();
        let decimal_integer_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .then(
                just('_')
                    .or_not()
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')))
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(decimal_integer::_S0::new(v)))
            .boxed();
        let fixed_bytes_type_parser = terminal("bytes")
            .map(|_| 5usize)
            .then(choice::<_, ErrorType>((
                terminal("1").ignore_then(choice((
                    terminal("0").map(|_| 2usize),
                    terminal("1").map(|_| 2usize),
                    terminal("2").map(|_| 2usize),
                    terminal("3").map(|_| 2usize),
                    terminal("4").map(|_| 2usize),
                    terminal("5").map(|_| 2usize),
                    terminal("6").map(|_| 2usize),
                    terminal("7").map(|_| 2usize),
                    terminal("8").map(|_| 2usize),
                    terminal("9").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("2").ignore_then(choice((
                    terminal("0").map(|_| 2usize),
                    terminal("1").map(|_| 2usize),
                    terminal("2").map(|_| 2usize),
                    terminal("3").map(|_| 2usize),
                    terminal("4").map(|_| 2usize),
                    terminal("5").map(|_| 2usize),
                    terminal("6").map(|_| 2usize),
                    terminal("7").map(|_| 2usize),
                    terminal("8").map(|_| 2usize),
                    terminal("9").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("3").ignore_then(choice((
                    terminal("0").map(|_| 2usize),
                    terminal("1").map(|_| 2usize),
                    terminal("2").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("4").map(|_| 1usize),
                terminal("5").map(|_| 1usize),
                terminal("6").map(|_| 1usize),
                terminal("7").map(|_| 1usize),
                terminal("8").map(|_| 1usize),
                terminal("9").map(|_| 1usize),
            )))
            .map(|v| Box::new(fixed_bytes_type::_S0::new(v)))
            .boxed();
        let fixed_type_parser = terminal("fixed")
            .map(|_| 5usize)
            .then(
                filter(|&c: &char| ('1' <= c && c <= '9'))
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')).repeated())
                    .then(just('x'))
                    .then(filter(|&c: &char| ('1' <= c && c <= '9')))
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')).repeated())
                    .map(|v| Box::new(fixed_type::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(fixed_type::_S0::new(v)))
            .boxed();
        let hex_character_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .boxed();
        let identifier_start_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .boxed();
        let number_unit_parser = choice::<_, ErrorType>((
            terminal("days").map(|_| 4usize),
            terminal("ether").map(|_| 5usize),
            terminal("gwei").map(|_| 4usize),
            terminal("hours").map(|_| 5usize),
            terminal("minutes").map(|_| 7usize),
            terminal("seconds").map(|_| 7usize),
            terminal("we").ignore_then(choice((
                terminal("eks").map(|_| 5usize),
                terminal("i").map(|_| 3usize),
            ))),
            terminal("years").map(|_| 5usize),
        ))
        .boxed();
        let pragma_directive_parser = terminal("pragma")
            .map(|_| 6usize)
            .then(filter(|&c: &char| c != ';'))
            .then(filter(|&c: &char| c != ';').repeated())
            .then(just(';'))
            .map(|v| Box::new(pragma_directive::_S0::new(v)))
            .boxed();
        let reserved_keyword_parser = choice::<_, ErrorType>((
            terminal("a").ignore_then(choice((
                terminal("fter").map(|_| 5usize),
                terminal("lias").map(|_| 5usize),
                terminal("pply").map(|_| 5usize),
                terminal("uto").map(|_| 4usize),
            ))),
            terminal("byte").map(|_| 4usize),
            terminal("c").ignore_then(choice((
                terminal("ase").map(|_| 4usize),
                terminal("opyof").map(|_| 6usize),
            ))),
            terminal("def").ignore_then(choice((
                terminal("ault").map(|_| 7usize),
                terminal("ine").map(|_| 6usize),
            ))),
            terminal("final").map(|_| 5usize),
            terminal("i").ignore_then(choice((
                terminal("mplements").map(|_| 10usize),
                terminal("n").ignore_then(choice((
                    terminal("line").map(|_| 6usize),
                    empty().map(|_| 2usize),
                ))),
            ))),
            terminal("let").map(|_| 3usize),
            terminal("m").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("cro").map(|_| 5usize),
                    terminal("tch").map(|_| 5usize),
                ))),
                terminal("utable").map(|_| 7usize),
            ))),
            terminal("null").map(|_| 4usize),
            terminal("of").map(|_| 2usize),
            terminal("p").ignore_then(choice((
                terminal("artial").map(|_| 7usize),
                terminal("romise").map(|_| 7usize),
            ))),
            terminal("re").ignore_then(choice((
                terminal("ference").map(|_| 9usize),
                terminal("locatable").map(|_| 11usize),
            ))),
            terminal("s").ignore_then(choice((
                terminal("ealed").map(|_| 6usize),
                terminal("izeof").map(|_| 6usize),
                terminal("tatic").map(|_| 6usize),
                terminal("upports").map(|_| 8usize),
                terminal("witch").map(|_| 6usize),
            ))),
            terminal("type").ignore_then(choice((
                terminal("def").map(|_| 7usize),
                terminal("of").map(|_| 6usize),
            ))),
            terminal("var").map(|_| 3usize),
        ))
        .boxed();
        let signed_integer_type_parser = terminal("int")
            .map(|_| 3usize)
            .then(choice::<_, ErrorType>((
                terminal("1").ignore_then(choice((
                    terminal("04").map(|_| 3usize),
                    terminal("12").map(|_| 3usize),
                    terminal("2").ignore_then(choice((
                        terminal("0").map(|_| 3usize),
                        terminal("8").map(|_| 3usize),
                    ))),
                    terminal("36").map(|_| 3usize),
                    terminal("44").map(|_| 3usize),
                    terminal("52").map(|_| 3usize),
                    terminal("6").ignore_then(choice((
                        terminal("0").map(|_| 3usize),
                        terminal("8").map(|_| 3usize),
                        empty().map(|_| 2usize),
                    ))),
                    terminal("76").map(|_| 3usize),
                    terminal("84").map(|_| 3usize),
                    terminal("92").map(|_| 3usize),
                ))),
                terminal("2").ignore_then(choice((
                    terminal("0").ignore_then(choice((
                        terminal("0").map(|_| 3usize),
                        terminal("8").map(|_| 3usize),
                    ))),
                    terminal("16").map(|_| 3usize),
                    terminal("24").map(|_| 3usize),
                    terminal("32").map(|_| 3usize),
                    terminal("4").ignore_then(choice((
                        terminal("0").map(|_| 3usize),
                        terminal("8").map(|_| 3usize),
                        empty().map(|_| 2usize),
                    ))),
                    terminal("56").map(|_| 3usize),
                ))),
                terminal("32").map(|_| 2usize),
                terminal("4").ignore_then(choice((
                    terminal("0").map(|_| 2usize),
                    terminal("8").map(|_| 2usize),
                ))),
                terminal("56").map(|_| 2usize),
                terminal("64").map(|_| 2usize),
                terminal("72").map(|_| 2usize),
                terminal("8").ignore_then(choice((
                    terminal("0").map(|_| 2usize),
                    terminal("8").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("96").map(|_| 2usize),
            )))
            .map(|v| Box::new(signed_integer_type::_S0::new(v)))
            .boxed();
        let yul_decimal_number_literal_parser = choice((
            terminal("0")
                .map(|_| 1usize)
                .map(|v| Box::new(yul_decimal_number_literal::_C0::Zero(v))),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .then(filter(|&c: &char| ('0' <= c && c <= '9')).repeated())
                .map(|v| Box::new(yul_decimal_number_literal::_S1::new(v)))
                .map(|v| Box::new(yul_decimal_number_literal::_C0::_S1(v))),
        ))
        .boxed();
        let yul_evm_builtin_function_name_parser = choice::<_, ErrorType>((
            terminal("Blockhash").map(|_| 9usize),
            terminal("a").ignore_then(choice((
                terminal("dd").ignore_then(choice((
                    terminal("mod").map(|_| 6usize),
                    terminal("ress").map(|_| 7usize),
                    empty().map(|_| 3usize),
                ))),
                terminal("nd").map(|_| 3usize),
            ))),
            terminal("b").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("lance").map(|_| 7usize),
                    terminal("sefee").map(|_| 7usize),
                ))),
                terminal("yte").map(|_| 4usize),
            ))),
            terminal("c").ignore_then(choice((
                terminal("all").ignore_then(choice((
                    terminal("code").map(|_| 8usize),
                    terminal("data").ignore_then(choice((
                        terminal("copy").map(|_| 12usize),
                        terminal("load").map(|_| 12usize),
                        terminal("size").map(|_| 12usize),
                    ))),
                    terminal("er").map(|_| 6usize),
                    terminal("value").map(|_| 9usize),
                    empty().map(|_| 4usize),
                ))),
                terminal("hainid").map(|_| 7usize),
                terminal("oinbase").map(|_| 8usize),
                terminal("reate").ignore_then(choice((
                    terminal("2").map(|_| 7usize),
                    empty().map(|_| 6usize),
                ))),
            ))),
            terminal("d").ignore_then(choice((
                terminal("elegatecall").map(|_| 12usize),
                terminal("i").ignore_then(choice((
                    terminal("fficulty").map(|_| 10usize),
                    terminal("v").map(|_| 3usize),
                ))),
            ))),
            terminal("e").ignore_then(choice((
                terminal("q").map(|_| 2usize),
                terminal("x").ignore_then(choice((
                    terminal("p").map(|_| 3usize),
                    terminal("tcode").ignore_then(choice((
                        terminal("copy").map(|_| 11usize),
                        terminal("hash").map(|_| 11usize),
                        terminal("size").map(|_| 11usize),
                    ))),
                ))),
            ))),
            terminal("g").ignore_then(choice((
                terminal("as").ignore_then(choice((
                    terminal("limit").map(|_| 8usize),
                    terminal("price").map(|_| 8usize),
                    empty().map(|_| 3usize),
                ))),
                terminal("t").map(|_| 2usize),
            ))),
            terminal("i").ignore_then(choice((
                terminal("nvalid").map(|_| 7usize),
                terminal("szero").map(|_| 6usize),
            ))),
            terminal("keccak256").map(|_| 9usize),
            terminal("l").ignore_then(choice((
                terminal("og").ignore_then(choice((
                    terminal("0").map(|_| 4usize),
                    terminal("1").map(|_| 4usize),
                    terminal("2").map(|_| 4usize),
                    terminal("3").map(|_| 4usize),
                    terminal("4").map(|_| 4usize),
                ))),
                terminal("t").map(|_| 2usize),
            ))),
            terminal("m").ignore_then(choice((
                terminal("load").map(|_| 5usize),
                terminal("od").map(|_| 3usize),
                terminal("s").ignore_then(choice((
                    terminal("ize").map(|_| 5usize),
                    terminal("tore").ignore_then(choice((
                        terminal("8").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("ul").ignore_then(choice((
                    terminal("mod").map(|_| 6usize),
                    empty().map(|_| 3usize),
                ))),
            ))),
            terminal("n").ignore_then(choice((
                terminal("ot").map(|_| 3usize),
                terminal("umber").map(|_| 6usize),
            ))),
            terminal("or").ignore_then(choice((
                terminal("igin").map(|_| 6usize),
                empty().map(|_| 2usize),
            ))),
            terminal("pop").map(|_| 3usize),
            terminal("re").ignore_then(choice((
                terminal("turn").ignore_then(choice((
                    terminal("data").ignore_then(choice((
                        terminal("copy").map(|_| 14usize),
                        terminal("size").map(|_| 14usize),
                    ))),
                    empty().map(|_| 6usize),
                ))),
                terminal("vert").map(|_| 6usize),
            ))),
            terminal("s").ignore_then(choice((
                terminal("ar").map(|_| 3usize),
                terminal("div").map(|_| 4usize),
                terminal("elf").ignore_then(choice((
                    terminal("balance").map(|_| 11usize),
                    terminal("destruct").map(|_| 12usize),
                ))),
                terminal("gt").map(|_| 3usize),
                terminal("h").ignore_then(choice((
                    terminal("l").map(|_| 3usize),
                    terminal("r").map(|_| 3usize),
                ))),
                terminal("ignextend").map(|_| 10usize),
                terminal("l").ignore_then(choice((
                    terminal("oad").map(|_| 5usize),
                    terminal("t").map(|_| 3usize),
                ))),
                terminal("mod").map(|_| 4usize),
                terminal("store").map(|_| 6usize),
                terminal("t").ignore_then(choice((
                    terminal("aticcall").map(|_| 10usize),
                    terminal("op").map(|_| 4usize),
                ))),
                terminal("ub").map(|_| 3usize),
            ))),
            terminal("timestamp").map(|_| 9usize),
            terminal("xor").map(|_| 3usize),
        ))
        .boxed();
        let yul_hex_literal_parser = terminal("0x")
            .map(|_| 2usize)
            .then(filter(|&c: &char| {
                ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
            }))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .repeated(),
            )
            .map(|v| Box::new(yul_hex_literal::_S0::new(v)))
            .boxed();
        let yul_keyword_parser = choice::<_, ErrorType>((
            terminal("break").map(|_| 5usize),
            terminal("c").ignore_then(choice((
                terminal("ase").map(|_| 4usize),
                terminal("ontinue").map(|_| 8usize),
            ))),
            terminal("default").map(|_| 7usize),
            terminal("f").ignore_then(choice((
                terminal("or").map(|_| 3usize),
                terminal("unction").map(|_| 8usize),
            ))),
            terminal("hex").map(|_| 3usize),
            terminal("if").map(|_| 2usize),
            terminal("le").ignore_then(choice((
                terminal("ave").map(|_| 5usize),
                terminal("t").map(|_| 3usize),
            ))),
            terminal("switch").map(|_| 6usize),
        ))
        .boxed();
        let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
            .then(just('-').or_not())
            .then(decimal_integer_parser.clone())
            .map(|v| Box::new(decimal_exponent::_S0::new(v)))
            .boxed();
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.'))
            .then(decimal_integer_parser.clone())
            .map(|v| Box::new(decimal_float::_S0::new(v)))
            .boxed();
        let hex_byte_escape_parser = just('x')
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .repeated()
                .exactly(2usize),
            )
            .map(|v| Box::new(hex_byte_escape::_S0::new(v)))
            .boxed();
        let hex_number_parser = just('0')
            .then(just('x'))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .then(
                    just('_')
                        .or_not()
                        .then(filter(|&c: &char| {
                            ('0' <= c && c <= '9')
                                || ('a' <= c && c <= 'f')
                                || ('A' <= c && c <= 'F')
                        }))
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(hex_number::_S1::new(v))),
            )
            .map(|v| Box::new(hex_number::_S0::new(v)))
            .boxed();
        let identifier_part_parser = filter(|&c: &char| {
            c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')
                || ('0' <= c && c <= '9')
        })
        .boxed();
        let possibly_separated_pairs_of_hex_digits_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .repeated()
        .exactly(2usize)
        .then(
            just('_')
                .or_not()
                .then(
                    filter(|&c: &char| {
                        ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                    })
                    .repeated()
                    .exactly(2usize),
                )
                .repeated(),
        )
        .map(repetition_mapper)
        .map(|v| Box::new(possibly_separated_pairs_of_hex_digits::_S0::new(v)))
        .boxed();
        let ufixed_type_parser = just('u')
            .then(fixed_type_parser.clone())
            .map(|v| Box::new(ufixed_type::_S0::new(v)))
            .boxed();
        let unicode_escape_parser = just('u')
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .repeated()
                .exactly(4usize),
            )
            .map(|v| Box::new(unicode_escape::_S0::new(v)))
            .boxed();
        let unsigned_integer_type_parser = just('u')
            .then(signed_integer_type_parser.clone())
            .map(|v| Box::new(unsigned_integer_type::_S0::new(v)))
            .boxed();
        let yul_reserved_word_parser = choice::<_, ErrorType>((
            terminal("Blockhash").map(|_| 9usize),
            terminal("a").ignore_then(choice((
                terminal("dd").ignore_then(choice((
                    terminal("mod").map(|_| 6usize),
                    terminal("ress").map(|_| 7usize),
                    empty().map(|_| 3usize),
                ))),
                terminal("nd").map(|_| 3usize),
            ))),
            terminal("b").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("lance").map(|_| 7usize),
                    terminal("sefee").map(|_| 7usize),
                ))),
                terminal("reak").map(|_| 5usize),
                terminal("yte").map(|_| 4usize),
            ))),
            terminal("c").ignore_then(choice((
                terminal("a").ignore_then(choice((
                    terminal("ll").ignore_then(choice((
                        terminal("code").map(|_| 8usize),
                        terminal("data").ignore_then(choice((
                            terminal("copy").map(|_| 12usize),
                            terminal("load").map(|_| 12usize),
                            terminal("size").map(|_| 12usize),
                        ))),
                        terminal("er").map(|_| 6usize),
                        terminal("value").map(|_| 9usize),
                        empty().map(|_| 4usize),
                    ))),
                    terminal("se").map(|_| 4usize),
                ))),
                terminal("hainid").map(|_| 7usize),
                terminal("o").ignore_then(choice((
                    terminal("inbase").map(|_| 8usize),
                    terminal("ntinue").map(|_| 8usize),
                ))),
                terminal("reate").ignore_then(choice((
                    terminal("2").map(|_| 7usize),
                    empty().map(|_| 6usize),
                ))),
            ))),
            terminal("d").ignore_then(choice((
                terminal("e").ignore_then(choice((
                    terminal("fault").map(|_| 7usize),
                    terminal("legatecall").map(|_| 12usize),
                ))),
                terminal("i").ignore_then(choice((
                    terminal("fficulty").map(|_| 10usize),
                    terminal("v").map(|_| 3usize),
                ))),
            ))),
            terminal("e").ignore_then(choice((
                terminal("q").map(|_| 2usize),
                terminal("x").ignore_then(choice((
                    terminal("p").map(|_| 3usize),
                    terminal("tcode").ignore_then(choice((
                        terminal("copy").map(|_| 11usize),
                        terminal("hash").map(|_| 11usize),
                        terminal("size").map(|_| 11usize),
                    ))),
                ))),
            ))),
            terminal("f").ignore_then(choice((
                terminal("alse").map(|_| 5usize),
                terminal("or").map(|_| 3usize),
                terminal("unction").map(|_| 8usize),
            ))),
            terminal("g").ignore_then(choice((
                terminal("as").ignore_then(choice((
                    terminal("limit").map(|_| 8usize),
                    terminal("price").map(|_| 8usize),
                    empty().map(|_| 3usize),
                ))),
                terminal("t").map(|_| 2usize),
            ))),
            terminal("hex").map(|_| 3usize),
            terminal("i").ignore_then(choice((
                terminal("f").map(|_| 2usize),
                terminal("nvalid").map(|_| 7usize),
                terminal("szero").map(|_| 6usize),
            ))),
            terminal("keccak256").map(|_| 9usize),
            terminal("l").ignore_then(choice((
                terminal("e").ignore_then(choice((
                    terminal("ave").map(|_| 5usize),
                    terminal("t").map(|_| 3usize),
                ))),
                terminal("og").ignore_then(choice((
                    terminal("0").map(|_| 4usize),
                    terminal("1").map(|_| 4usize),
                    terminal("2").map(|_| 4usize),
                    terminal("3").map(|_| 4usize),
                    terminal("4").map(|_| 4usize),
                ))),
                terminal("t").map(|_| 2usize),
            ))),
            terminal("m").ignore_then(choice((
                terminal("load").map(|_| 5usize),
                terminal("od").map(|_| 3usize),
                terminal("s").ignore_then(choice((
                    terminal("ize").map(|_| 5usize),
                    terminal("tore").ignore_then(choice((
                        terminal("8").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("ul").ignore_then(choice((
                    terminal("mod").map(|_| 6usize),
                    empty().map(|_| 3usize),
                ))),
            ))),
            terminal("n").ignore_then(choice((
                terminal("ot").map(|_| 3usize),
                terminal("umber").map(|_| 6usize),
            ))),
            terminal("or").ignore_then(choice((
                terminal("igin").map(|_| 6usize),
                empty().map(|_| 2usize),
            ))),
            terminal("pop").map(|_| 3usize),
            terminal("re").ignore_then(choice((
                terminal("turn").ignore_then(choice((
                    terminal("data").ignore_then(choice((
                        terminal("copy").map(|_| 14usize),
                        terminal("size").map(|_| 14usize),
                    ))),
                    empty().map(|_| 6usize),
                ))),
                terminal("vert").map(|_| 6usize),
            ))),
            terminal("s").ignore_then(choice((
                terminal("ar").map(|_| 3usize),
                terminal("div").map(|_| 4usize),
                terminal("elf").ignore_then(choice((
                    terminal("balance").map(|_| 11usize),
                    terminal("destruct").map(|_| 12usize),
                ))),
                terminal("gt").map(|_| 3usize),
                terminal("h").ignore_then(choice((
                    terminal("l").map(|_| 3usize),
                    terminal("r").map(|_| 3usize),
                ))),
                terminal("ignextend").map(|_| 10usize),
                terminal("l").ignore_then(choice((
                    terminal("oad").map(|_| 5usize),
                    terminal("t").map(|_| 3usize),
                ))),
                terminal("mod").map(|_| 4usize),
                terminal("store").map(|_| 6usize),
                terminal("t").ignore_then(choice((
                    terminal("aticcall").map(|_| 10usize),
                    terminal("op").map(|_| 4usize),
                ))),
                terminal("ub").map(|_| 3usize),
                terminal("witch").map(|_| 6usize),
            ))),
            terminal("t").ignore_then(choice((
                terminal("imestamp").map(|_| 9usize),
                terminal("rue").map(|_| 4usize),
            ))),
            terminal("xor").map(|_| 3usize),
        ))
        .boxed();
        let decimal_number_parser = choice((
            decimal_integer_parser
                .clone()
                .map(|v| Box::new(decimal_number::_C1::DecimalInteger(v))),
            decimal_float_parser
                .clone()
                .map(|v| Box::new(decimal_number::_C1::DecimalFloat(v))),
        ))
        .then(decimal_exponent_parser.clone().or_not())
        .map(|v| Box::new(decimal_number::_S0::new(v)))
        .boxed();
        let elementary_type_parser = choice((
            choice::<_, ErrorType>((
                terminal("b").ignore_then(choice((
                    terminal("ool").map(|_| 4usize),
                    terminal("ytes").map(|_| 5usize),
                ))),
                terminal("string").map(|_| 6usize),
            ))
            .map(|v| Box::new(elementary_type::_C0::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_C0::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_C0::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_C0::FixedBytesType(v))),
            fixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_C0::FixedType(v))),
            ufixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_C0::UfixedType(v))),
        ))
        .boxed();
        let escape_sequence_parser = just('\\')
            .then(choice((
                choice::<_, ErrorType>((
                    terminal("\n").map(|_| 1usize),
                    terminal("\r").map(|_| 1usize),
                    terminal("\"").map(|_| 1usize),
                    terminal("'").map(|_| 1usize),
                    terminal("\\").map(|_| 1usize),
                    terminal("n").map(|_| 1usize),
                    terminal("r").map(|_| 1usize),
                    terminal("t").map(|_| 1usize),
                ))
                .map(|v| Box::new(escape_sequence::_C1::_0(v))),
                hex_byte_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_C1::HexByteEscape(v))),
                unicode_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_C1::UnicodeEscape(v))),
            )))
            .map(|v| Box::new(escape_sequence::_S0::new(v)))
            .boxed();
        let hex_string_literal_parser = terminal("hex")
            .map(|_| 3usize)
            .then(choice((
                just('"')
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('"'))
                    .map(|v| Box::new(hex_string_literal::_S2::new(v)))
                    .map(|v| Box::new(hex_string_literal::_C1::_S2(v))),
                just('\'')
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('\''))
                    .map(|v| Box::new(hex_string_literal::_S4::new(v)))
                    .map(|v| Box::new(hex_string_literal::_C1::_S4(v))),
            )))
            .map(|v| Box::new(hex_string_literal::_S0::new(v)))
            .boxed();
        let keyword_parser = choice((
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("bstract").map(|_| 8usize),
                    terminal("ddress").map(|_| 7usize),
                    terminal("nonymous").map(|_| 9usize),
                    terminal("s").ignore_then(choice((
                        terminal("sembly").map(|_| 8usize),
                        empty().map(|_| 2usize),
                    ))),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("ool").map(|_| 4usize),
                    terminal("reak").map(|_| 5usize),
                    terminal("ytes").map(|_| 5usize),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lldata").map(|_| 8usize),
                        terminal("tch").map(|_| 5usize),
                    ))),
                    terminal("on").ignore_then(choice((
                        terminal("st").ignore_then(choice((
                            terminal("ant").map(|_| 8usize),
                            terminal("ructor").map(|_| 11usize),
                        ))),
                        terminal("t").ignore_then(choice((
                            terminal("inue").map(|_| 8usize),
                            terminal("ract").map(|_| 8usize),
                        ))),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("elete").map(|_| 6usize),
                    terminal("o").map(|_| 2usize),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("lse").map(|_| 4usize),
                    terminal("mit").map(|_| 4usize),
                    terminal("num").map(|_| 4usize),
                    terminal("vent").map(|_| 5usize),
                    terminal("xternal").map(|_| 8usize),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("al").ignore_then(choice((
                        terminal("lback").map(|_| 8usize),
                        terminal("se").map(|_| 5usize),
                    ))),
                    terminal("or").map(|_| 3usize),
                    terminal("unction").map(|_| 8usize),
                ))),
                terminal("hex").map(|_| 3usize),
                terminal("i").ignore_then(choice((
                    terminal("f").map(|_| 2usize),
                    terminal("m").ignore_then(choice((
                        terminal("mutable").map(|_| 9usize),
                        terminal("port").map(|_| 6usize),
                    ))),
                    terminal("n").ignore_then(choice((
                        terminal("dexed").map(|_| 7usize),
                        terminal("ter").ignore_then(choice((
                            terminal("face").map(|_| 9usize),
                            terminal("nal").map(|_| 8usize),
                        ))),
                    ))),
                    terminal("s").map(|_| 2usize),
                ))),
                terminal("library").map(|_| 7usize),
                terminal("m").ignore_then(choice((
                    terminal("apping").map(|_| 7usize),
                    terminal("emory").map(|_| 6usize),
                    terminal("odifier").map(|_| 8usize),
                ))),
                terminal("new").map(|_| 3usize),
                terminal("override").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("r").ignore_then(choice((
                        terminal("agma").map(|_| 6usize),
                        terminal("ivate").map(|_| 7usize),
                    ))),
                    terminal("u").ignore_then(choice((
                        terminal("blic").map(|_| 6usize),
                        terminal("re").map(|_| 4usize),
                    ))),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ceive").map(|_| 7usize),
                    terminal("turn").ignore_then(choice((
                        terminal("s").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("st").ignore_then(choice((
                    terminal("orage").map(|_| 7usize),
                    terminal("r").ignore_then(choice((
                        terminal("ing").map(|_| 6usize),
                        terminal("uct").map(|_| 6usize),
                    ))),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("r").ignore_then(choice((
                        terminal("ue").map(|_| 4usize),
                        terminal("y").map(|_| 3usize),
                    ))),
                    terminal("ype").map(|_| 4usize),
                ))),
                terminal("u").ignore_then(choice((
                    terminal("nchecked").map(|_| 9usize),
                    terminal("sing").map(|_| 5usize),
                ))),
                terminal("vi").ignore_then(choice((
                    terminal("ew").map(|_| 4usize),
                    terminal("rtual").map(|_| 7usize),
                ))),
                terminal("while").map(|_| 5usize),
            ))
            .map(|v| Box::new(keyword::_C0::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_C0::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_C0::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(keyword::_C0::FixedBytesType(v))),
            choice::<_, ErrorType>((
                terminal("fixed").map(|_| 5usize),
                terminal("ufixed").map(|_| 6usize),
            ))
            .map(|v| Box::new(keyword::_C0::_4(v))),
        ))
        .boxed();
        let raw_identifier_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .then(
            filter(|&c: &char| {
                c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')
            })
            .repeated(),
        )
        .map(|v| Box::new(raw_identifier::_S0::new(v)))
        .boxed();
        let double_quoted_ascii_string_literal_parser = just('"')
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '"' && c != '\\')
                        .map(|v| Box::new(double_quoted_ascii_string_literal::_C2::_0(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_ascii_string_literal::_C2::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"'))
            .map(|v| Box::new(double_quoted_ascii_string_literal::_S0::new(v)))
            .boxed();
        let double_quoted_unicode_string_literal_parser = terminal("unicode\"")
            .map(|_| 8usize)
            .then(
                choice((
                    filter(|&c: &char| c != '"' && c != '\\' && c != '\n' && c != '\r')
                        .map(|v| Box::new(double_quoted_unicode_string_literal::_C2::_0(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_unicode_string_literal::_C2::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"'))
            .map(|v| Box::new(double_quoted_unicode_string_literal::_S0::new(v)))
            .boxed();
        let elementary_type_with_payable_parser = choice((
            terminal("address")
                .map(|_| 7usize)
                .then(terminal("payable").map(|_| 7usize).or_not())
                .map(|v| Box::new(elementary_type_with_payable::_S1::new(v)))
                .map(|v| Box::new(elementary_type_with_payable::_C0::_S1(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_with_payable::_C0::ElementaryType(v))),
        ))
        .boxed();
        let elementary_type_without_payable_parser = choice((
            terminal("address")
                .map(|_| 7usize)
                .map(|v| Box::new(elementary_type_without_payable::_C0::Address(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_without_payable::_C0::ElementaryType(v))),
        ))
        .boxed();
        let numeric_literal_parser = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_C1::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_C1::HexNumber(v))),
        ))
        .then(
            choice::<_, ErrorType>((
                terminal("days").map(|_| 4usize),
                terminal("ether").map(|_| 5usize),
                terminal("gwei").map(|_| 4usize),
                terminal("hours").map(|_| 5usize),
                terminal("minutes").map(|_| 7usize),
                terminal("seconds").map(|_| 7usize),
                terminal("we").ignore_then(choice((
                    terminal("eks").map(|_| 5usize),
                    terminal("i").map(|_| 3usize),
                ))),
                terminal("years").map(|_| 5usize),
            ))
            .or_not(),
        )
        .map(|v| Box::new(numeric_literal::_S0::new(v)))
        .boxed();
        let reserved_word_parser = choice((
            keyword_parser
                .clone()
                .map(|v| Box::new(reserved_word::_C0::Keyword(v))),
            choice::<_, ErrorType>((
                terminal("a").ignore_then(choice((
                    terminal("fter").map(|_| 5usize),
                    terminal("lias").map(|_| 5usize),
                    terminal("pply").map(|_| 5usize),
                    terminal("uto").map(|_| 4usize),
                ))),
                terminal("byte").map(|_| 4usize),
                terminal("c").ignore_then(choice((
                    terminal("ase").map(|_| 4usize),
                    terminal("opyof").map(|_| 6usize),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("ays").map(|_| 4usize),
                    terminal("ef").ignore_then(choice((
                        terminal("ault").map(|_| 7usize),
                        terminal("ine").map(|_| 6usize),
                    ))),
                ))),
                terminal("ether").map(|_| 5usize),
                terminal("f").ignore_then(choice((
                    terminal("alse").map(|_| 5usize),
                    terminal("inal").map(|_| 5usize),
                ))),
                terminal("gwei").map(|_| 4usize),
                terminal("hours").map(|_| 5usize),
                terminal("i").ignore_then(choice((
                    terminal("mplements").map(|_| 10usize),
                    terminal("n").ignore_then(choice((
                        terminal("line").map(|_| 6usize),
                        empty().map(|_| 2usize),
                    ))),
                ))),
                terminal("let").map(|_| 3usize),
                terminal("m").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("cro").map(|_| 5usize),
                        terminal("tch").map(|_| 5usize),
                    ))),
                    terminal("inutes").map(|_| 7usize),
                    terminal("utable").map(|_| 7usize),
                ))),
                terminal("null").map(|_| 4usize),
                terminal("of").map(|_| 2usize),
                terminal("p").ignore_then(choice((
                    terminal("artial").map(|_| 7usize),
                    terminal("romise").map(|_| 7usize),
                ))),
                terminal("re").ignore_then(choice((
                    terminal("ference").map(|_| 9usize),
                    terminal("locatable").map(|_| 11usize),
                ))),
                terminal("s").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("aled").map(|_| 6usize),
                        terminal("conds").map(|_| 7usize),
                    ))),
                    terminal("izeof").map(|_| 6usize),
                    terminal("tatic").map(|_| 6usize),
                    terminal("upports").map(|_| 8usize),
                    terminal("witch").map(|_| 6usize),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("rue").map(|_| 4usize),
                    terminal("ype").ignore_then(choice((
                        terminal("def").map(|_| 7usize),
                        terminal("of").map(|_| 6usize),
                    ))),
                ))),
                terminal("var").map(|_| 3usize),
                terminal("we").ignore_then(choice((
                    terminal("eks").map(|_| 5usize),
                    terminal("i").map(|_| 3usize),
                ))),
                terminal("years").map(|_| 5usize),
            ))
            .map(|v| Box::new(reserved_word::_C0::_1(v))),
        ))
        .boxed();
        let single_quoted_ascii_string_literal_parser = just('\'')
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '\'' && c != '\\')
                        .map(|v| Box::new(single_quoted_ascii_string_literal::_C2::_0(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_ascii_string_literal::_C2::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\''))
            .map(|v| Box::new(single_quoted_ascii_string_literal::_S0::new(v)))
            .boxed();
        let single_quoted_unicode_string_literal_parser = terminal("unicode'")
            .map(|_| 8usize)
            .then(
                choice((
                    filter(|&c: &char| c != '\'' && c != '\\' && c != '\n' && c != '\r')
                        .map(|v| Box::new(single_quoted_unicode_string_literal::_C2::_0(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_unicode_string_literal::_C2::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\''))
            .map(|v| Box::new(single_quoted_unicode_string_literal::_S0::new(v)))
            .boxed();
        let yul_identifier_parser = raw_identifier_parser
            .clone()
            .excluding(choice::<_, ErrorType>((
                terminal("Blockhash").map(|_| 9usize),
                terminal("a").ignore_then(choice((
                    terminal("dd").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        terminal("ress").map(|_| 7usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("nd").map(|_| 3usize),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lance").map(|_| 7usize),
                        terminal("sefee").map(|_| 7usize),
                    ))),
                    terminal("reak").map(|_| 5usize),
                    terminal("yte").map(|_| 4usize),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("ll").ignore_then(choice((
                            terminal("code").map(|_| 8usize),
                            terminal("data").ignore_then(choice((
                                terminal("copy").map(|_| 12usize),
                                terminal("load").map(|_| 12usize),
                                terminal("size").map(|_| 12usize),
                            ))),
                            terminal("er").map(|_| 6usize),
                            terminal("value").map(|_| 9usize),
                            empty().map(|_| 4usize),
                        ))),
                        terminal("se").map(|_| 4usize),
                    ))),
                    terminal("hainid").map(|_| 7usize),
                    terminal("o").ignore_then(choice((
                        terminal("inbase").map(|_| 8usize),
                        terminal("ntinue").map(|_| 8usize),
                    ))),
                    terminal("reate").ignore_then(choice((
                        terminal("2").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("fault").map(|_| 7usize),
                        terminal("legatecall").map(|_| 12usize),
                    ))),
                    terminal("i").ignore_then(choice((
                        terminal("fficulty").map(|_| 10usize),
                        terminal("v").map(|_| 3usize),
                    ))),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("q").map(|_| 2usize),
                    terminal("x").ignore_then(choice((
                        terminal("p").map(|_| 3usize),
                        terminal("tcode").ignore_then(choice((
                            terminal("copy").map(|_| 11usize),
                            terminal("hash").map(|_| 11usize),
                            terminal("size").map(|_| 11usize),
                        ))),
                    ))),
                ))),
                terminal("f").ignore_then(choice((
                    terminal("alse").map(|_| 5usize),
                    terminal("or").map(|_| 3usize),
                    terminal("unction").map(|_| 8usize),
                ))),
                terminal("g").ignore_then(choice((
                    terminal("as").ignore_then(choice((
                        terminal("limit").map(|_| 8usize),
                        terminal("price").map(|_| 8usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("hex").map(|_| 3usize),
                terminal("i").ignore_then(choice((
                    terminal("f").map(|_| 2usize),
                    terminal("nvalid").map(|_| 7usize),
                    terminal("szero").map(|_| 6usize),
                ))),
                terminal("keccak256").map(|_| 9usize),
                terminal("l").ignore_then(choice((
                    terminal("e").ignore_then(choice((
                        terminal("ave").map(|_| 5usize),
                        terminal("t").map(|_| 3usize),
                    ))),
                    terminal("og").ignore_then(choice((
                        terminal("0").map(|_| 4usize),
                        terminal("1").map(|_| 4usize),
                        terminal("2").map(|_| 4usize),
                        terminal("3").map(|_| 4usize),
                        terminal("4").map(|_| 4usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("m").ignore_then(choice((
                    terminal("load").map(|_| 5usize),
                    terminal("od").map(|_| 3usize),
                    terminal("s").ignore_then(choice((
                        terminal("ize").map(|_| 5usize),
                        terminal("tore").ignore_then(choice((
                            terminal("8").map(|_| 7usize),
                            empty().map(|_| 6usize),
                        ))),
                    ))),
                    terminal("ul").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        empty().map(|_| 3usize),
                    ))),
                ))),
                terminal("n").ignore_then(choice((
                    terminal("ot").map(|_| 3usize),
                    terminal("umber").map(|_| 6usize),
                ))),
                terminal("or").ignore_then(choice((
                    terminal("igin").map(|_| 6usize),
                    empty().map(|_| 2usize),
                ))),
                terminal("pop").map(|_| 3usize),
                terminal("re").ignore_then(choice((
                    terminal("turn").ignore_then(choice((
                        terminal("data").ignore_then(choice((
                            terminal("copy").map(|_| 14usize),
                            terminal("size").map(|_| 14usize),
                        ))),
                        empty().map(|_| 6usize),
                    ))),
                    terminal("vert").map(|_| 6usize),
                ))),
                terminal("s").ignore_then(choice((
                    terminal("ar").map(|_| 3usize),
                    terminal("div").map(|_| 4usize),
                    terminal("elf").ignore_then(choice((
                        terminal("balance").map(|_| 11usize),
                        terminal("destruct").map(|_| 12usize),
                    ))),
                    terminal("gt").map(|_| 3usize),
                    terminal("h").ignore_then(choice((
                        terminal("l").map(|_| 3usize),
                        terminal("r").map(|_| 3usize),
                    ))),
                    terminal("ignextend").map(|_| 10usize),
                    terminal("l").ignore_then(choice((
                        terminal("oad").map(|_| 5usize),
                        terminal("t").map(|_| 3usize),
                    ))),
                    terminal("mod").map(|_| 4usize),
                    terminal("store").map(|_| 6usize),
                    terminal("t").ignore_then(choice((
                        terminal("aticcall").map(|_| 10usize),
                        terminal("op").map(|_| 4usize),
                    ))),
                    terminal("ub").map(|_| 3usize),
                    terminal("witch").map(|_| 6usize),
                ))),
                terminal("t").ignore_then(choice((
                    terminal("imestamp").map(|_| 9usize),
                    terminal("rue").map(|_| 4usize),
                ))),
                terminal("xor").map(|_| 3usize),
            )))
            .boxed();
        let ascii_string_literal_parser = choice((
            single_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_C0::SingleQuotedAsciiStringLiteral(v))),
            double_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_C0::DoubleQuotedAsciiStringLiteral(v))),
        ))
        .boxed();
        let assembly_flags_parser = just('(')
            .then(
                double_quoted_ascii_string_literal_parser
                    .clone()
                    .then(
                        just(',')
                            .then(double_quoted_ascii_string_literal_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(assembly_flags::_S1::new(v))),
            )
            .then(just(')'))
            .map(|v| Box::new(assembly_flags::_S0::new(v)))
            .boxed();
        let identifier_parser = raw_identifier_parser
            .clone()
            .excluding(reserved_word_parser.clone())
            .boxed();
        let unicode_string_literal_parser = choice((
            single_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_C0::SingleQuotedUnicodeStringLiteral(v))
                }),
            double_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_C0::DoubleQuotedUnicodeStringLiteral(v))
                }),
        ))
        .boxed();
        let yul_function_call_parser = choice((
            yul_identifier_parser
                .clone()
                .map(|v| Box::new(yul_function_call::_C1::YulIdentifier(v))),
            choice::<_, ErrorType>((
                terminal("Blockhash").map(|_| 9usize),
                terminal("a").ignore_then(choice((
                    terminal("dd").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        terminal("ress").map(|_| 7usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("nd").map(|_| 3usize),
                ))),
                terminal("b").ignore_then(choice((
                    terminal("a").ignore_then(choice((
                        terminal("lance").map(|_| 7usize),
                        terminal("sefee").map(|_| 7usize),
                    ))),
                    terminal("yte").map(|_| 4usize),
                ))),
                terminal("c").ignore_then(choice((
                    terminal("all").ignore_then(choice((
                        terminal("code").map(|_| 8usize),
                        terminal("data").ignore_then(choice((
                            terminal("copy").map(|_| 12usize),
                            terminal("load").map(|_| 12usize),
                            terminal("size").map(|_| 12usize),
                        ))),
                        terminal("er").map(|_| 6usize),
                        terminal("value").map(|_| 9usize),
                        empty().map(|_| 4usize),
                    ))),
                    terminal("hainid").map(|_| 7usize),
                    terminal("oinbase").map(|_| 8usize),
                    terminal("reate").ignore_then(choice((
                        terminal("2").map(|_| 7usize),
                        empty().map(|_| 6usize),
                    ))),
                ))),
                terminal("d").ignore_then(choice((
                    terminal("elegatecall").map(|_| 12usize),
                    terminal("i").ignore_then(choice((
                        terminal("fficulty").map(|_| 10usize),
                        terminal("v").map(|_| 3usize),
                    ))),
                ))),
                terminal("e").ignore_then(choice((
                    terminal("q").map(|_| 2usize),
                    terminal("x").ignore_then(choice((
                        terminal("p").map(|_| 3usize),
                        terminal("tcode").ignore_then(choice((
                            terminal("copy").map(|_| 11usize),
                            terminal("hash").map(|_| 11usize),
                            terminal("size").map(|_| 11usize),
                        ))),
                    ))),
                ))),
                terminal("g").ignore_then(choice((
                    terminal("as").ignore_then(choice((
                        terminal("limit").map(|_| 8usize),
                        terminal("price").map(|_| 8usize),
                        empty().map(|_| 3usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("i").ignore_then(choice((
                    terminal("nvalid").map(|_| 7usize),
                    terminal("szero").map(|_| 6usize),
                ))),
                terminal("keccak256").map(|_| 9usize),
                terminal("l").ignore_then(choice((
                    terminal("og").ignore_then(choice((
                        terminal("0").map(|_| 4usize),
                        terminal("1").map(|_| 4usize),
                        terminal("2").map(|_| 4usize),
                        terminal("3").map(|_| 4usize),
                        terminal("4").map(|_| 4usize),
                    ))),
                    terminal("t").map(|_| 2usize),
                ))),
                terminal("m").ignore_then(choice((
                    terminal("load").map(|_| 5usize),
                    terminal("od").map(|_| 3usize),
                    terminal("s").ignore_then(choice((
                        terminal("ize").map(|_| 5usize),
                        terminal("tore").ignore_then(choice((
                            terminal("8").map(|_| 7usize),
                            empty().map(|_| 6usize),
                        ))),
                    ))),
                    terminal("ul").ignore_then(choice((
                        terminal("mod").map(|_| 6usize),
                        empty().map(|_| 3usize),
                    ))),
                ))),
                terminal("n").ignore_then(choice((
                    terminal("ot").map(|_| 3usize),
                    terminal("umber").map(|_| 6usize),
                ))),
                terminal("or").ignore_then(choice((
                    terminal("igin").map(|_| 6usize),
                    empty().map(|_| 2usize),
                ))),
                terminal("pop").map(|_| 3usize),
                terminal("re").ignore_then(choice((
                    terminal("turn").ignore_then(choice((
                        terminal("data").ignore_then(choice((
                            terminal("copy").map(|_| 14usize),
                            terminal("size").map(|_| 14usize),
                        ))),
                        empty().map(|_| 6usize),
                    ))),
                    terminal("vert").map(|_| 6usize),
                ))),
                terminal("s").ignore_then(choice((
                    terminal("ar").map(|_| 3usize),
                    terminal("div").map(|_| 4usize),
                    terminal("elf").ignore_then(choice((
                        terminal("balance").map(|_| 11usize),
                        terminal("destruct").map(|_| 12usize),
                    ))),
                    terminal("gt").map(|_| 3usize),
                    terminal("h").ignore_then(choice((
                        terminal("l").map(|_| 3usize),
                        terminal("r").map(|_| 3usize),
                    ))),
                    terminal("ignextend").map(|_| 10usize),
                    terminal("l").ignore_then(choice((
                        terminal("oad").map(|_| 5usize),
                        terminal("t").map(|_| 3usize),
                    ))),
                    terminal("mod").map(|_| 4usize),
                    terminal("store").map(|_| 6usize),
                    terminal("t").ignore_then(choice((
                        terminal("aticcall").map(|_| 10usize),
                        terminal("op").map(|_| 4usize),
                    ))),
                    terminal("ub").map(|_| 3usize),
                ))),
                terminal("timestamp").map(|_| 9usize),
                terminal("xor").map(|_| 3usize),
            ))
            .map(|v| Box::new(yul_function_call::_C1::_1(v))),
        ))
        .then(just('('))
        .then(
            yul_expression_parser
                .clone()
                .then(just(',').then(yul_expression_parser.clone()).repeated())
                .map(repetition_mapper)
                .map(|v| Box::new(yul_function_call::_S2::new(v)))
                .or_not(),
        )
        .then(just(')'))
        .map(|v| Box::new(yul_function_call::_S0::new(v)))
        .boxed();
        let yul_function_definition_parser = terminal("function")
            .map(|_| 8usize)
            .then(yul_identifier_parser.clone())
            .then(just('('))
            .then(
                yul_identifier_parser
                    .clone()
                    .then(just(',').then(yul_identifier_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(yul_function_definition::_S1::new(v)))
                    .or_not(),
            )
            .then(just(')'))
            .then(
                terminal("->")
                    .map(|_| 2usize)
                    .then(
                        yul_identifier_parser
                            .clone()
                            .then(just(',').then(yul_identifier_parser.clone()).repeated())
                            .map(repetition_mapper)
                            .map(|v| Box::new(yul_function_definition::_S4::new(v))),
                    )
                    .map(|v| Box::new(yul_function_definition::_S3::new(v)))
                    .or_not(),
            )
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_function_definition::_S0::new(v)))
            .boxed();
        let yul_path_parser = yul_identifier_parser
            .clone()
            .then(
                just('.')
                    .then(choice((
                        yul_identifier_parser
                            .clone()
                            .map(|v| Box::new(yul_path::_C3::YulIdentifier(v))),
                        choice::<_, ErrorType>((
                            terminal("Blockhash").map(|_| 9usize),
                            terminal("a").ignore_then(choice((
                                terminal("dd").ignore_then(choice((
                                    terminal("mod").map(|_| 6usize),
                                    terminal("ress").map(|_| 7usize),
                                    empty().map(|_| 3usize),
                                ))),
                                terminal("nd").map(|_| 3usize),
                            ))),
                            terminal("b").ignore_then(choice((
                                terminal("a").ignore_then(choice((
                                    terminal("lance").map(|_| 7usize),
                                    terminal("sefee").map(|_| 7usize),
                                ))),
                                terminal("yte").map(|_| 4usize),
                            ))),
                            terminal("c").ignore_then(choice((
                                terminal("all").ignore_then(choice((
                                    terminal("code").map(|_| 8usize),
                                    terminal("data").ignore_then(choice((
                                        terminal("copy").map(|_| 12usize),
                                        terminal("load").map(|_| 12usize),
                                        terminal("size").map(|_| 12usize),
                                    ))),
                                    terminal("er").map(|_| 6usize),
                                    terminal("value").map(|_| 9usize),
                                    empty().map(|_| 4usize),
                                ))),
                                terminal("hainid").map(|_| 7usize),
                                terminal("oinbase").map(|_| 8usize),
                                terminal("reate").ignore_then(choice((
                                    terminal("2").map(|_| 7usize),
                                    empty().map(|_| 6usize),
                                ))),
                            ))),
                            terminal("d").ignore_then(choice((
                                terminal("elegatecall").map(|_| 12usize),
                                terminal("i").ignore_then(choice((
                                    terminal("fficulty").map(|_| 10usize),
                                    terminal("v").map(|_| 3usize),
                                ))),
                            ))),
                            terminal("e").ignore_then(choice((
                                terminal("q").map(|_| 2usize),
                                terminal("x").ignore_then(choice((
                                    terminal("p").map(|_| 3usize),
                                    terminal("tcode").ignore_then(choice((
                                        terminal("copy").map(|_| 11usize),
                                        terminal("hash").map(|_| 11usize),
                                        terminal("size").map(|_| 11usize),
                                    ))),
                                ))),
                            ))),
                            terminal("g").ignore_then(choice((
                                terminal("as").ignore_then(choice((
                                    terminal("limit").map(|_| 8usize),
                                    terminal("price").map(|_| 8usize),
                                    empty().map(|_| 3usize),
                                ))),
                                terminal("t").map(|_| 2usize),
                            ))),
                            terminal("i").ignore_then(choice((
                                terminal("nvalid").map(|_| 7usize),
                                terminal("szero").map(|_| 6usize),
                            ))),
                            terminal("keccak256").map(|_| 9usize),
                            terminal("l").ignore_then(choice((
                                terminal("og").ignore_then(choice((
                                    terminal("0").map(|_| 4usize),
                                    terminal("1").map(|_| 4usize),
                                    terminal("2").map(|_| 4usize),
                                    terminal("3").map(|_| 4usize),
                                    terminal("4").map(|_| 4usize),
                                ))),
                                terminal("t").map(|_| 2usize),
                            ))),
                            terminal("m").ignore_then(choice((
                                terminal("load").map(|_| 5usize),
                                terminal("od").map(|_| 3usize),
                                terminal("s").ignore_then(choice((
                                    terminal("ize").map(|_| 5usize),
                                    terminal("tore").ignore_then(choice((
                                        terminal("8").map(|_| 7usize),
                                        empty().map(|_| 6usize),
                                    ))),
                                ))),
                                terminal("ul").ignore_then(choice((
                                    terminal("mod").map(|_| 6usize),
                                    empty().map(|_| 3usize),
                                ))),
                            ))),
                            terminal("n").ignore_then(choice((
                                terminal("ot").map(|_| 3usize),
                                terminal("umber").map(|_| 6usize),
                            ))),
                            terminal("or").ignore_then(choice((
                                terminal("igin").map(|_| 6usize),
                                empty().map(|_| 2usize),
                            ))),
                            terminal("pop").map(|_| 3usize),
                            terminal("re").ignore_then(choice((
                                terminal("turn").ignore_then(choice((
                                    terminal("data").ignore_then(choice((
                                        terminal("copy").map(|_| 14usize),
                                        terminal("size").map(|_| 14usize),
                                    ))),
                                    empty().map(|_| 6usize),
                                ))),
                                terminal("vert").map(|_| 6usize),
                            ))),
                            terminal("s").ignore_then(choice((
                                terminal("ar").map(|_| 3usize),
                                terminal("div").map(|_| 4usize),
                                terminal("elf").ignore_then(choice((
                                    terminal("balance").map(|_| 11usize),
                                    terminal("destruct").map(|_| 12usize),
                                ))),
                                terminal("gt").map(|_| 3usize),
                                terminal("h").ignore_then(choice((
                                    terminal("l").map(|_| 3usize),
                                    terminal("r").map(|_| 3usize),
                                ))),
                                terminal("ignextend").map(|_| 10usize),
                                terminal("l").ignore_then(choice((
                                    terminal("oad").map(|_| 5usize),
                                    terminal("t").map(|_| 3usize),
                                ))),
                                terminal("mod").map(|_| 4usize),
                                terminal("store").map(|_| 6usize),
                                terminal("t").ignore_then(choice((
                                    terminal("aticcall").map(|_| 10usize),
                                    terminal("op").map(|_| 4usize),
                                ))),
                                terminal("ub").map(|_| 3usize),
                            ))),
                            terminal("timestamp").map(|_| 9usize),
                            terminal("xor").map(|_| 3usize),
                        ))
                        .map(|v| Box::new(yul_path::_C3::_1(v))),
                    )))
                    .map(|v| Box::new(yul_path::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(yul_path::_S0::new(v)))
            .boxed();
        let enum_definition_parser = terminal("enum")
            .map(|_| 4usize)
            .then(identifier_parser.clone())
            .then(just('{'))
            .then(
                identifier_parser
                    .clone()
                    .then(just(',').then(identifier_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(enum_definition::_S1::new(v))),
            )
            .then(just('}'))
            .map(|v| Box::new(enum_definition::_S0::new(v)))
            .boxed();
        let identifier_path_parser = identifier_parser
            .clone()
            .then(just('.').then(identifier_parser.clone()).repeated())
            .map(repetition_mapper)
            .map(|v| Box::new(identifier_path::_S0::new(v)))
            .boxed();
        let import_path_parser = ascii_string_literal_parser.clone().boxed();
        let literal_parser = choice((
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_C0::AsciiStringLiteral(v))),
            unicode_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_C0::UnicodeStringLiteral(v))),
            numeric_literal_parser
                .clone()
                .map(|v| Box::new(literal::_C0::NumericLiteral(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_C0::HexStringLiteral(v))),
            choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            ))
            .map(|v| Box::new(literal::_C0::_4(v))),
        ))
        .boxed();
        let named_argument_parser = identifier_parser
            .clone()
            .then(just(':'))
            .then(expression_parser.clone())
            .map(|v| Box::new(named_argument::_S0::new(v)))
            .boxed();
        let parameter_declaration_parser = type_name_parser
            .clone()
            .then(
                choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                ))
                .or_not(),
            )
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(parameter_declaration::_S0::new(v)))
            .boxed();
        let selected_import_parser = identifier_parser
            .clone()
            .then(
                terminal("as")
                    .map(|_| 2usize)
                    .then(identifier_parser.clone())
                    .map(|v| Box::new(selected_import::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(selected_import::_S0::new(v)))
            .boxed();
        let user_defined_value_type_definition_parser = terminal("type")
            .map(|_| 4usize)
            .then(identifier_parser.clone())
            .then(terminal("is").map(|_| 2usize))
            .then(elementary_type_with_payable_parser.clone())
            .then(just(';'))
            .map(|v| Box::new(user_defined_value_type_definition::_S0::new(v)))
            .boxed();
        let yul_literal_parser = choice((
            yul_decimal_number_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_C0::YulDecimalNumberLiteral(v))),
            yul_hex_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_C0::YulHexLiteral(v))),
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_C0::AsciiStringLiteral(v))),
            choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            ))
            .map(|v| Box::new(yul_literal::_C0::_3(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_C0::HexStringLiteral(v))),
        ))
        .boxed();
        let mapping_type_parser = terminal("mapping")
            .map(|_| 7usize)
            .then(just('('))
            .then(choice((
                elementary_type_without_payable_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_C1::ElementaryTypeWithoutPayable(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_C1::IdentifierPath(v))),
            )))
            .then(terminal("=>").map(|_| 2usize))
            .then(type_name_parser.clone())
            .then(just(')'))
            .map(|v| Box::new(mapping_type::_S0::new(v)))
            .boxed();
        let named_argument_list_parser = just('{')
            .then(
                named_argument_parser
                    .clone()
                    .then(just(',').then(named_argument_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(named_argument_list::_S1::new(v)))
                    .or_not(),
            )
            .then(just('}'))
            .map(|v| Box::new(named_argument_list::_S0::new(v)))
            .boxed();
        let non_empty_parameter_list_parser = just('(')
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        just(',')
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(non_empty_parameter_list::_S1::new(v))),
            )
            .then(just(')'))
            .map(|v| Box::new(non_empty_parameter_list::_S0::new(v)))
            .boxed();
        let override_specifier_parser = terminal("override")
            .map(|_| 8usize)
            .then(
                just('(')
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(
                                just(',')
                                    .then(identifier_path_parser.clone())
                                    .repeated()
                                    .at_most(1usize - 1),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(override_specifier::_S3::new(v))),
                    )
                    .then(just(')'))
                    .map(|v| Box::new(override_specifier::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(override_specifier::_S0::new(v)))
            .boxed();
        let parameter_list_parser = just('(')
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        just(',')
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(parameter_list::_S1::new(v)))
                    .or_not(),
            )
            .then(just(')'))
            .map(|v| Box::new(parameter_list::_S0::new(v)))
            .boxed();
        let selecting_import_directive_parser = just('{')
            .then(
                selected_import_parser
                    .clone()
                    .then(just(',').then(selected_import_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(selecting_import_directive::_S1::new(v))),
            )
            .then(just('}'))
            .then(terminal("from").map(|_| 4usize))
            .then(import_path_parser.clone())
            .map(|v| Box::new(selecting_import_directive::_S0::new(v)))
            .boxed();
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(
                terminal("as")
                    .map(|_| 2usize)
                    .then(identifier_parser.clone())
                    .map(|v| Box::new(simple_import_directive::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(simple_import_directive::_S0::new(v)))
            .boxed();
        let star_import_directive_parser = just('*')
            .then(terminal("as").map(|_| 2usize))
            .then(identifier_parser.clone())
            .then(terminal("from").map(|_| 4usize))
            .then(import_path_parser.clone())
            .map(|v| Box::new(star_import_directive::_S0::new(v)))
            .boxed();
        yul_expression_parser.define(
            choice((
                yul_path_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_C0::YulPath(v))),
                yul_function_call_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_C0::YulFunctionCall(v))),
                yul_literal_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::_C0::YulLiteral(v))),
            ))
            .boxed(),
        );
        let argument_list_parser = just('(')
            .then(
                choice((
                    positional_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_C2::PositionalArgumentList(v))),
                    named_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_C2::NamedArgumentList(v))),
                ))
                .or_not(),
            )
            .then(just(')'))
            .map(|v| Box::new(argument_list::_S0::new(v)))
            .boxed();
        let catch_clause_parser = terminal("catch")
            .map(|_| 5usize)
            .then(
                identifier_parser
                    .clone()
                    .or_not()
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(catch_clause::_S2::new(v)))
                    .or_not(),
            )
            .then(block_parser.clone())
            .map(|v| Box::new(catch_clause::_S0::new(v)))
            .boxed();
        let function_type_parser = terminal("function")
            .map(|_| 8usize)
            .then(parameter_list_parser.clone())
            .then(
                choice::<_, ErrorType>((
                    terminal("external").map(|_| 8usize),
                    terminal("internal").map(|_| 8usize),
                    terminal("p").ignore_then(choice((
                        terminal("ayable").map(|_| 7usize),
                        terminal("rivate").map(|_| 7usize),
                        terminal("u").ignore_then(choice((
                            terminal("blic").map(|_| 6usize),
                            terminal("re").map(|_| 4usize),
                        ))),
                    ))),
                    terminal("view").map(|_| 4usize),
                ))
                .repeated(),
            )
            .then(
                terminal("returns")
                    .map(|_| 7usize)
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_type::_S3::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(function_type::_S0::new(v)))
            .boxed();
        let import_directive_parser = terminal("import")
            .map(|_| 6usize)
            .then(choice((
                simple_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_C1::SimpleImportDirective(v))),
                star_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_C1::StarImportDirective(v))),
                selecting_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_C1::SelectingImportDirective(v))),
            )))
            .then(just(';'))
            .map(|v| Box::new(import_directive::_S0::new(v)))
            .boxed();
        let method_attribute_parser = choice((
            terminal("virtual")
                .map(|_| 7usize)
                .map(|v| Box::new(method_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(method_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();
        let state_variable_attribute_parser = choice((
            choice::<_, ErrorType>((
                terminal("constant").map(|_| 8usize),
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("rivate").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            ))
            .map(|v| Box::new(state_variable_attribute::_C0::_0(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(state_variable_attribute::_C0::OverrideSpecifier(v))),
            terminal("immutable")
                .map(|_| 9usize)
                .map(|v| Box::new(state_variable_attribute::_C0::Immutable(v))),
        ))
        .boxed();
        let yul_assignment_parser = yul_path_parser
            .clone()
            .then(choice((
                terminal(":=")
                    .map(|_| 2usize)
                    .then(yul_expression_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S2::new(v)))
                    .map(|v| Box::new(yul_assignment::_C1::_S2(v))),
                just(',')
                    .then(yul_path_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S5::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(terminal(":=").map(|_| 2usize))
                    .then(yul_function_call_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S3::new(v)))
                    .map(|v| Box::new(yul_assignment::_C1::_S3(v))),
            )))
            .map(|v| Box::new(yul_assignment::_S0::new(v)))
            .boxed();
        let yul_for_statement_parser = terminal("for")
            .map(|_| 3usize)
            .then(yul_block_parser.clone())
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_for_statement::_S0::new(v)))
            .boxed();
        let yul_if_statement_parser = terminal("if")
            .map(|_| 2usize)
            .then(yul_expression_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_if_statement::_S0::new(v)))
            .boxed();
        let yul_switch_statement_parser = terminal("switch")
            .map(|_| 6usize)
            .then(yul_expression_parser.clone())
            .then(choice((
                terminal("case")
                    .map(|_| 4usize)
                    .then(yul_literal_parser.clone())
                    .then(yul_block_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_S4::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(
                        terminal("default")
                            .map(|_| 7usize)
                            .then(yul_block_parser.clone())
                            .map(|v| Box::new(yul_switch_statement::_S6::new(v)))
                            .or_not(),
                    )
                    .map(|v| Box::new(yul_switch_statement::_S2::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_C1::_S2(v))),
                terminal("default")
                    .map(|_| 7usize)
                    .then(yul_block_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_S7::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_C1::_S7(v))),
            )))
            .map(|v| Box::new(yul_switch_statement::_S0::new(v)))
            .boxed();
        let yul_variable_declaration_parser = terminal("let")
            .map(|_| 3usize)
            .then(yul_identifier_parser.clone())
            .then(
                choice((
                    terminal(":=")
                        .map(|_| 2usize)
                        .then(yul_expression_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_S3::new(v)))
                        .map(|v| Box::new(yul_variable_declaration::_C2::_S3(v))),
                    just(',')
                        .then(yul_identifier_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_S6::new(v)))
                        .or_not()
                        .then(
                            terminal(":=")
                                .map(|_| 2usize)
                                .then(yul_function_call_parser.clone())
                                .map(|v| Box::new(yul_variable_declaration::_S8::new(v)))
                                .or_not(),
                        )
                        .map(|v| Box::new(yul_variable_declaration::_S4::new(v)))
                        .map(|v| Box::new(yul_variable_declaration::_C2::_S4(v))),
                ))
                .or_not(),
            )
            .map(|v| Box::new(yul_variable_declaration::_S0::new(v)))
            .boxed();
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(inheritance_specifier::_S0::new(v)))
            .boxed();
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(modifier_invocation::_S0::new(v)))
            .boxed();
        type_name_parser.define(
            choice((
                elementary_type_with_payable_parser
                    .clone()
                    .map(|v| Box::new(type_name::_C1::ElementaryTypeWithPayable(v))),
                function_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_C1::FunctionType(v))),
                mapping_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_C1::MappingType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(type_name::_C1::IdentifierPath(v))),
            ))
            .then(
                just('[')
                    .then(expression_parser.clone().or_not())
                    .then(just(']'))
                    .map(|v| Box::new(type_name::_S3::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(type_name::_S0::new(v)))
            .boxed(),
        );
        let yul_statement_parser = choice((
            yul_block_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulBlock(v))),
            yul_variable_declaration_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulVariableDeclaration(v))),
            yul_function_definition_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulFunctionDefinition(v))),
            yul_assignment_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulAssignment(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulFunctionCall(v))),
            yul_if_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulIfStatement(v))),
            yul_for_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulForStatement(v))),
            yul_switch_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_C0::YulSwitchStatement(v))),
            choice::<_, ErrorType>((
                terminal("break").map(|_| 5usize),
                terminal("continue").map(|_| 8usize),
                terminal("leave").map(|_| 5usize),
            ))
            .map(|v| Box::new(yul_statement::_C0::_8(v))),
        ))
        .boxed();
        let constructor_attribute_parser = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(constructor_attribute::_C0::ModifierInvocation(v))),
            choice::<_, ErrorType>((
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            ))
            .map(|v| Box::new(constructor_attribute::_C0::_1(v))),
        ))
        .boxed();
        let error_parameter_parser = type_name_parser
            .clone()
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(error_parameter::_S0::new(v)))
            .boxed();
        let event_parameter_parser = type_name_parser
            .clone()
            .then(terminal("indexed").map(|_| 7usize).or_not())
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(event_parameter::_S0::new(v)))
            .boxed();
        let fallback_function_attribute_parser = choice((
            choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ure").map(|_| 4usize),
                ))),
                terminal("view").map(|_| 4usize),
            ))
            .map(|v| Box::new(fallback_function_attribute::_C0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_C0::ModifierInvocation(v))),
            terminal("virtual")
                .map(|_| 7usize)
                .map(|v| Box::new(fallback_function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();
        let function_attribute_parser = choice((
            choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("rivate").map(|_| 7usize),
                    terminal("u").ignore_then(choice((
                        terminal("blic").map(|_| 6usize),
                        terminal("re").map(|_| 4usize),
                    ))),
                ))),
                terminal("view").map(|_| 4usize),
            ))
            .map(|v| Box::new(function_attribute::_C0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(function_attribute::_C0::ModifierInvocation(v))),
            terminal("virtual")
                .map(|_| 7usize)
                .map(|v| Box::new(function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();
        let inheritance_specifier_list_parser = terminal("is")
            .map(|_| 2usize)
            .then(
                inheritance_specifier_parser
                    .clone()
                    .then(
                        just(',')
                            .then(inheritance_specifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(inheritance_specifier_list::_S1::new(v))),
            )
            .map(|v| Box::new(inheritance_specifier_list::_S0::new(v)))
            .boxed();
        let primary_expression_parser = choice((
            terminal("payable")
                .map(|_| 7usize)
                .then(argument_list_parser.clone())
                .map(|v| Box::new(primary_expression::_S1::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S1(v))),
            terminal("type")
                .map(|_| 4usize)
                .then(just('('))
                .then(type_name_parser.clone())
                .then(just(')'))
                .map(|v| Box::new(primary_expression::_S2::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S2(v))),
            terminal("new")
                .map(|_| 3usize)
                .then(type_name_parser.clone())
                .map(|v| Box::new(primary_expression::_S3::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S3(v))),
            just('(')
                .then(
                    expression_parser
                        .clone()
                        .or_not()
                        .then(
                            just(',')
                                .then(expression_parser.clone().or_not())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(primary_expression::_S5::new(v))),
                )
                .then(just(')'))
                .map(|v| Box::new(primary_expression::_S4::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S4(v))),
            just('[')
                .then(
                    expression_parser
                        .clone()
                        .then(just(',').then(expression_parser.clone()).repeated())
                        .map(repetition_mapper)
                        .map(|v| Box::new(primary_expression::_S8::new(v))),
                )
                .then(just(']'))
                .map(|v| Box::new(primary_expression::_S7::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S7(v))),
            identifier_parser
                .clone()
                .map(|v| Box::new(primary_expression::_C0::Identifier(v))),
            literal_parser
                .clone()
                .map(|v| Box::new(primary_expression::_C0::Literal(v))),
            elementary_type_without_payable_parser
                .clone()
                .map(|v| Box::new(primary_expression::_C0::ElementaryTypeWithoutPayable(v))),
        ))
        .boxed();
        let receive_function_attribute_parser = choice((
            choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("payable").map(|_| 7usize),
            ))
            .map(|v| Box::new(receive_function_attribute::_C0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_C0::ModifierInvocation(v))),
            terminal("virtual")
                .map(|_| 7usize)
                .map(|v| Box::new(receive_function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();
        let struct_definition_parser = terminal("struct")
            .map(|_| 6usize)
            .then(identifier_parser.clone())
            .then(just('{'))
            .then(
                type_name_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(just(';'))
                    .map(|v| Box::new(struct_definition::_S2::new(v)))
                    .repeated()
                    .at_least(1usize),
            )
            .then(just('}'))
            .map(|v| Box::new(struct_definition::_S0::new(v)))
            .boxed();
        let using_directive_parser = terminal("using")
            .map(|_| 5usize)
            .then(choice((
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_C1::IdentifierPath(v))),
                just('{')
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(just(',').then(identifier_path_parser.clone()).repeated())
                            .map(repetition_mapper)
                            .map(|v| Box::new(using_directive::_S3::new(v))),
                    )
                    .then(just('}'))
                    .map(|v| Box::new(using_directive::_S2::new(v)))
                    .map(|v| Box::new(using_directive::_C1::_S2(v))),
            )))
            .then(terminal("for").map(|_| 3usize))
            .then(choice((
                terminal("*")
                    .map(|_| 1usize)
                    .map(|v| Box::new(using_directive::_C4::Star(v))),
                type_name_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_C4::TypeName(v))),
            )))
            .then(terminal("global").map(|_| 6usize).or_not())
            .then(just(';'))
            .map(|v| Box::new(using_directive::_S0::new(v)))
            .boxed();
        let variable_declaration_parser = type_name_parser
            .clone()
            .then(
                choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                ))
                .or_not(),
            )
            .then(identifier_parser.clone())
            .map(|v| Box::new(variable_declaration::_S0::new(v)))
            .boxed();
        yul_block_parser.define(
            just('{')
                .then(yul_statement_parser.clone().repeated())
                .then(just('}'))
                .map(|v| Box::new(yul_block::_S0::new(v)))
                .boxed(),
        );
        let assembly_statement_parser = terminal("assembly")
            .map(|_| 8usize)
            .then(terminal("\"evmasm\"").map(|_| 8usize).or_not())
            .then(assembly_flags_parser.clone().or_not())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(assembly_statement::_S0::new(v)))
            .boxed();
        let directive_parser = choice((
            pragma_directive_parser
                .clone()
                .map(|v| Box::new(directive::_C0::PragmaDirective(v))),
            import_directive_parser
                .clone()
                .map(|v| Box::new(directive::_C0::ImportDirective(v))),
            using_directive_parser
                .clone()
                .map(|v| Box::new(directive::_C0::UsingDirective(v))),
        ))
        .boxed();
        let error_definition_parser = terminal("error")
            .map(|_| 5usize)
            .then(identifier_parser.clone())
            .then(just('('))
            .then(
                error_parameter_parser
                    .clone()
                    .then(just(',').then(error_parameter_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(error_definition::_S1::new(v)))
                    .or_not(),
            )
            .then(just(')'))
            .then(just(';'))
            .map(|v| Box::new(error_definition::_S0::new(v)))
            .boxed();
        let event_definition_parser = terminal("event")
            .map(|_| 5usize)
            .then(identifier_parser.clone())
            .then(just('('))
            .then(
                event_parameter_parser
                    .clone()
                    .then(just(',').then(event_parameter_parser.clone()).repeated())
                    .map(repetition_mapper)
                    .map(|v| Box::new(event_definition::_S1::new(v)))
                    .or_not(),
            )
            .then(just(')'))
            .then(terminal("anonymous").map(|_| 9usize).or_not())
            .then(just(';'))
            .map(|v| Box::new(event_definition::_S0::new(v)))
            .boxed();
        let index_access_expression_parser = primary_expression_parser
            .clone()
            .then(
                just('[')
                    .then(expression_parser.clone().or_not())
                    .then(
                        just(':')
                            .then(expression_parser.clone().or_not())
                            .map(|v| Box::new(index_access_expression::_S5::new(v)))
                            .or_not(),
                    )
                    .then(just(']'))
                    .map(|v| Box::new(index_access_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(index_access_expression::_S0::new(v)))
            .boxed();
        let variable_declaration_tuple_parser = just('(')
            .then(just(',').repeated())
            .then(variable_declaration_parser.clone())
            .then(
                just(',')
                    .then(variable_declaration_parser.clone().or_not())
                    .map(|v| Box::new(variable_declaration_tuple::_S3::new(v)))
                    .repeated(),
            )
            .then(just(')'))
            .map(|v| Box::new(variable_declaration_tuple::_S0::new(v)))
            .boxed();
        let member_access_expression_parser = index_access_expression_parser
            .clone()
            .then(
                just('.')
                    .then(choice((
                        identifier_parser
                            .clone()
                            .map(|v| Box::new(member_access_expression::_C3::Identifier(v))),
                        terminal("address")
                            .map(|_| 7usize)
                            .map(|v| Box::new(member_access_expression::_C3::Address(v))),
                    )))
                    .map(|v| Box::new(member_access_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(member_access_expression::_S0::new(v)))
            .boxed();
        let function_call_options_expression_parser = member_access_expression_parser
            .clone()
            .then(
                just('{')
                    .then(
                        named_argument_parser
                            .clone()
                            .then(just(',').then(named_argument_parser.clone()).repeated())
                            .map(repetition_mapper)
                            .map(|v| Box::new(function_call_options_expression::_S3::new(v))),
                    )
                    .then(just('}'))
                    .map(|v| Box::new(function_call_options_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(function_call_options_expression::_S0::new(v)))
            .boxed();
        let function_call_expression_parser = function_call_options_expression_parser
            .clone()
            .then(argument_list_parser.clone().repeated())
            .map(|v| Box::new(function_call_expression::_S0::new(v)))
            .boxed();
        let unary_prefix_expression_parser = choice::<_, ErrorType>((
            terminal("!").map(|_| 1usize),
            terminal("++").map(|_| 2usize),
            terminal("-").ignore_then(choice((
                terminal("-").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
            terminal("delete").map(|_| 6usize),
            terminal("~").map(|_| 1usize),
        ))
        .then(function_call_expression_parser.clone())
        .map(|v| Box::new(unary_prefix_expression::_S0::new(v)))
        .boxed();
        let unary_suffix_expression_parser = unary_prefix_expression_parser
            .clone()
            .then(choice::<_, ErrorType>((
                terminal("++").map(|_| 2usize),
                terminal("--").map(|_| 2usize),
            )))
            .map(|v| Box::new(unary_suffix_expression::_S0::new(v)))
            .boxed();
        let exp_expression_parser = unary_suffix_expression_parser
            .clone()
            .then(terminal("**").map(|_| 2usize))
            .then(expression_parser.clone())
            .map(|v| Box::new(exp_expression::_S0::new(v)))
            .boxed();
        let mul_div_mod_expression_parser = exp_expression_parser
            .clone()
            .then(
                filter(|&c: &char| c == '*' || c == '/' || c == '%')
                    .then(exp_expression_parser.clone())
                    .map(|v| Box::new(mul_div_mod_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(mul_div_mod_expression::_S0::new(v)))
            .boxed();
        let add_sub_expression_parser = mul_div_mod_expression_parser
            .clone()
            .then(
                filter(|&c: &char| c == '+' || c == '-')
                    .then(mul_div_mod_expression_parser.clone())
                    .map(|v| Box::new(add_sub_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(add_sub_expression::_S0::new(v)))
            .boxed();
        let shift_expression_parser = add_sub_expression_parser
            .clone()
            .then(
                choice::<_, ErrorType>((
                    terminal("<<").map(|_| 2usize),
                    terminal(">>").ignore_then(choice((
                        terminal(">").map(|_| 3usize),
                        empty().map(|_| 2usize),
                    ))),
                ))
                .then(add_sub_expression_parser.clone())
                .map(|v| Box::new(shift_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(shift_expression::_S0::new(v)))
            .boxed();
        let bit_and_expression_parser = shift_expression_parser
            .clone()
            .then(
                just('&')
                    .then(shift_expression_parser.clone())
                    .map(|v| Box::new(bit_and_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_and_expression::_S0::new(v)))
            .boxed();
        let bit_x_or_expression_parser = bit_and_expression_parser
            .clone()
            .then(
                just('^')
                    .then(bit_and_expression_parser.clone())
                    .map(|v| Box::new(bit_x_or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_x_or_expression::_S0::new(v)))
            .boxed();
        let bit_or_expression_parser = bit_x_or_expression_parser
            .clone()
            .then(
                just('|')
                    .then(bit_x_or_expression_parser.clone())
                    .map(|v| Box::new(bit_or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_or_expression::_S0::new(v)))
            .boxed();
        let order_comparison_expression_parser = bit_or_expression_parser
            .clone()
            .then(
                choice::<_, ErrorType>((
                    terminal("<").ignore_then(choice((
                        terminal("=").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                    terminal(">").ignore_then(choice((
                        terminal("=").map(|_| 2usize),
                        empty().map(|_| 1usize),
                    ))),
                ))
                .then(bit_or_expression_parser.clone())
                .map(|v| Box::new(order_comparison_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(order_comparison_expression::_S0::new(v)))
            .boxed();
        let equality_comparison_expression_parser = order_comparison_expression_parser
            .clone()
            .then(
                choice::<_, ErrorType>((
                    terminal("!=").map(|_| 2usize),
                    terminal("==").map(|_| 2usize),
                ))
                .then(order_comparison_expression_parser.clone())
                .map(|v| Box::new(equality_comparison_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(equality_comparison_expression::_S0::new(v)))
            .boxed();
        let and_expression_parser = equality_comparison_expression_parser
            .clone()
            .then(
                terminal("&&")
                    .map(|_| 2usize)
                    .then(equality_comparison_expression_parser.clone())
                    .map(|v| Box::new(and_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(and_expression::_S0::new(v)))
            .boxed();
        let or_expression_parser = and_expression_parser
            .clone()
            .then(
                terminal("||")
                    .map(|_| 2usize)
                    .then(and_expression_parser.clone())
                    .map(|v| Box::new(or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(or_expression::_S0::new(v)))
            .boxed();
        let conditional_expression_parser = or_expression_parser
            .clone()
            .then(just('?'))
            .then(expression_parser.clone())
            .then(just(':'))
            .then(expression_parser.clone())
            .map(|v| Box::new(conditional_expression::_S0::new(v)))
            .boxed();
        let assignment_expression_parser = conditional_expression_parser
            .clone()
            .then(choice::<_, ErrorType>((
                terminal("%=").map(|_| 2usize),
                terminal("&=").map(|_| 2usize),
                terminal("*=").map(|_| 2usize),
                terminal("+=").map(|_| 2usize),
                terminal("-=").map(|_| 2usize),
                terminal("/=").map(|_| 2usize),
                terminal("<<=").map(|_| 3usize),
                terminal("=").map(|_| 1usize),
                terminal(">>").ignore_then(choice((
                    terminal("=").map(|_| 3usize),
                    terminal(">=").map(|_| 4usize),
                ))),
                terminal("^=").map(|_| 2usize),
                terminal("|=").map(|_| 2usize),
            )))
            .then(expression_parser.clone())
            .map(|v| Box::new(assignment_expression::_S0::new(v)))
            .boxed();
        expression_parser.define(assignment_expression_parser.clone().boxed());
        let constant_definition_parser = type_name_parser
            .clone()
            .then(terminal("constant").map(|_| 8usize))
            .then(identifier_parser.clone())
            .then(just('='))
            .then(expression_parser.clone())
            .then(just(';'))
            .map(|v| Box::new(constant_definition::_S0::new(v)))
            .boxed();
        let do_while_statement_parser = terminal("do")
            .map(|_| 2usize)
            .then(statement_parser.clone())
            .then(terminal("while").map(|_| 5usize))
            .then(just('('))
            .then(expression_parser.clone())
            .then(just(')'))
            .then(just(';'))
            .map(|v| Box::new(do_while_statement::_S0::new(v)))
            .boxed();
        let emit_statement_parser = terminal("emit")
            .map(|_| 4usize)
            .then(expression_parser.clone())
            .then(argument_list_parser.clone())
            .then(just(';'))
            .map(|v| Box::new(emit_statement::_S0::new(v)))
            .boxed();
        let expression_statement_parser = expression_parser
            .clone()
            .then(just(';'))
            .map(|v| Box::new(expression_statement::_S0::new(v)))
            .boxed();
        let if_statement_parser = terminal("if")
            .map(|_| 2usize)
            .then(just('('))
            .then(expression_parser.clone())
            .then(just(')'))
            .then(statement_parser.clone())
            .then(
                terminal("else")
                    .map(|_| 4usize)
                    .then(statement_parser.clone())
                    .map(|v| Box::new(if_statement::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(if_statement::_S0::new(v)))
            .boxed();
        let return_statement_parser = terminal("return")
            .map(|_| 6usize)
            .then(expression_parser.clone().or_not())
            .then(just(';'))
            .map(|v| Box::new(return_statement::_S0::new(v)))
            .boxed();
        let revert_statement_parser = terminal("revert")
            .map(|_| 6usize)
            .then(expression_parser.clone())
            .then(argument_list_parser.clone())
            .then(just(';'))
            .map(|v| Box::new(revert_statement::_S0::new(v)))
            .boxed();
        let state_variable_declaration_parser = type_name_parser
            .clone()
            .then(state_variable_attribute_parser.clone().repeated())
            .then(identifier_parser.clone())
            .then(
                just('=')
                    .then(expression_parser.clone())
                    .map(|v| Box::new(state_variable_declaration::_S3::new(v)))
                    .or_not(),
            )
            .then(just(';'))
            .map(|v| Box::new(state_variable_declaration::_S0::new(v)))
            .boxed();
        let try_statement_parser = terminal("try")
            .map(|_| 3usize)
            .then(expression_parser.clone())
            .then(
                terminal("returns")
                    .map(|_| 7usize)
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(try_statement::_S2::new(v)))
                    .or_not(),
            )
            .then(block_parser.clone())
            .then(catch_clause_parser.clone())
            .then(catch_clause_parser.clone().repeated())
            .map(|v| Box::new(try_statement::_S0::new(v)))
            .boxed();
        let variable_declaration_statement_parser = choice((
            variable_declaration_parser
                .clone()
                .then(
                    just('=')
                        .then(expression_parser.clone())
                        .map(|v| Box::new(variable_declaration_statement::_S4::new(v)))
                        .or_not(),
                )
                .map(|v| Box::new(variable_declaration_statement::_S2::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_C1::_S2(v))),
            variable_declaration_tuple_parser
                .clone()
                .then(just('='))
                .then(expression_parser.clone())
                .map(|v| Box::new(variable_declaration_statement::_S5::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_C1::_S5(v))),
        ))
        .then(just(';'))
        .map(|v| Box::new(variable_declaration_statement::_S0::new(v)))
        .boxed();
        let while_statement_parser = terminal("while")
            .map(|_| 5usize)
            .then(just('('))
            .then(expression_parser.clone())
            .then(just(')'))
            .then(statement_parser.clone())
            .map(|v| Box::new(while_statement::_S0::new(v)))
            .boxed();
        let simple_statement_parser = choice((
            variable_declaration_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_C0::VariableDeclarationStatement(v))),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_C0::ExpressionStatement(v))),
        ))
        .boxed();
        let for_statement_parser = terminal("for")
            .map(|_| 3usize)
            .then(just('('))
            .then(choice((
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_C1::SimpleStatement(v))),
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(for_statement::_C1::Semicolon(v))),
            )))
            .then(choice((
                expression_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_C2::ExpressionStatement(v))),
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(for_statement::_C2::Semicolon(v))),
            )))
            .then(expression_parser.clone().or_not())
            .then(just(')'))
            .then(statement_parser.clone())
            .map(|v| Box::new(for_statement::_S0::new(v)))
            .boxed();
        statement_parser.define(
            choice((
                block_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::Block(v))),
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::SimpleStatement(v))),
                if_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::IfStatement(v))),
                for_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::ForStatement(v))),
                while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::WhileStatement(v))),
                do_while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::DoWhileStatement(v))),
                continue_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::ContinueStatement(v))),
                break_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::BreakStatement(v))),
                try_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::TryStatement(v))),
                return_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::ReturnStatement(v))),
                emit_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::EmitStatement(v))),
                revert_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::RevertStatement(v))),
                assembly_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::_C0::AssemblyStatement(v))),
            ))
            .boxed(),
        );
        block_parser.define(
            just('{')
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::_C2::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::_C2::UncheckedBlock(v))),
                    ))
                    .repeated(),
                )
                .then(just('}'))
                .map(|v| Box::new(block::_S0::new(v)))
                .boxed(),
        );
        let constructor_definition_parser = terminal("constructor")
            .map(|_| 11usize)
            .then(parameter_list_parser.clone())
            .then(constructor_attribute_parser.clone().repeated())
            .then(block_parser.clone())
            .map(|v| Box::new(constructor_definition::_S0::new(v)))
            .boxed();
        let fallback_function_definition_parser = terminal("fallback")
            .map(|_| 8usize)
            .then(parameter_list_parser.clone())
            .then(fallback_function_attribute_parser.clone().repeated())
            .then(
                terminal("returns")
                    .map(|_| 7usize)
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(fallback_function_definition::_S3::new(v)))
                    .or_not(),
            )
            .then(choice((
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(fallback_function_definition::_C4::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(fallback_function_definition::_C4::Block(v))),
            )))
            .map(|v| Box::new(fallback_function_definition::_S0::new(v)))
            .boxed();
        let function_definition_parser = terminal("function")
            .map(|_| 8usize)
            .then(choice((
                identifier_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_C1::Identifier(v))),
                choice::<_, ErrorType>((
                    terminal("fallback").map(|_| 8usize),
                    terminal("receive").map(|_| 7usize),
                ))
                .map(|v| Box::new(function_definition::_C1::_1(v))),
            )))
            .then(parameter_list_parser.clone())
            .then(function_attribute_parser.clone().repeated())
            .then(
                terminal("returns")
                    .map(|_| 7usize)
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_definition::_S4::new(v)))
                    .or_not(),
            )
            .then(choice((
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(function_definition::_C5::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_C5::Block(v))),
            )))
            .map(|v| Box::new(function_definition::_S0::new(v)))
            .boxed();
        let modifier_definition_parser = terminal("modifier")
            .map(|_| 8usize)
            .then(identifier_parser.clone())
            .then(parameter_list_parser.clone().or_not())
            .then(method_attribute_parser.clone().repeated())
            .then(choice((
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(modifier_definition::_C3::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(modifier_definition::_C3::Block(v))),
            )))
            .map(|v| Box::new(modifier_definition::_S0::new(v)))
            .boxed();
        let receive_function_definition_parser = terminal("receive")
            .map(|_| 7usize)
            .then(just('('))
            .then(just(')'))
            .then(receive_function_attribute_parser.clone().repeated())
            .then(choice((
                terminal(";")
                    .map(|_| 1usize)
                    .map(|v| Box::new(receive_function_definition::_C2::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(receive_function_definition::_C2::Block(v))),
            )))
            .map(|v| Box::new(receive_function_definition::_S0::new(v)))
            .boxed();
        let contract_body_element_parser = choice((
            using_directive_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::UsingDirective(v))),
            constructor_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::ConstructorDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::FunctionDefinition(v))),
            fallback_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::FallbackFunctionDefinition(v))),
            receive_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::ReceiveFunctionDefinition(v))),
            modifier_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::ModifierDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::EnumDefinition(v))),
            user_defined_value_type_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::_C0::UserDefinedValueTypeDefinition(
                    v,
                ))
            }),
            event_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::EventDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::ErrorDefinition(v))),
            state_variable_declaration_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_C0::StateVariableDeclaration(v))),
        ))
        .boxed();
        let contract_definition_parser = terminal("abstract")
            .map(|_| 8usize)
            .or_not()
            .then(terminal("contract").map(|_| 8usize))
            .then(identifier_parser.clone())
            .then(inheritance_specifier_list_parser.clone().or_not())
            .then(just('{'))
            .then(contract_body_element_parser.clone().repeated())
            .then(just('}'))
            .map(|v| Box::new(contract_definition::_S0::new(v)))
            .boxed();
        let interface_definition_parser = terminal("interface")
            .map(|_| 9usize)
            .then(identifier_parser.clone())
            .then(inheritance_specifier_list_parser.clone().or_not())
            .then(just('{'))
            .then(contract_body_element_parser.clone().repeated())
            .then(just('}'))
            .map(|v| Box::new(interface_definition::_S0::new(v)))
            .boxed();
        let library_definition_parser = terminal("library")
            .map(|_| 7usize)
            .then(identifier_parser.clone())
            .then(just('{'))
            .then(contract_body_element_parser.clone().repeated())
            .then(just('}'))
            .map(|v| Box::new(library_definition::_S0::new(v)))
            .boxed();
        let definition_parser = choice((
            contract_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::ContractDefinition(v))),
            interface_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::InterfaceDefinition(v))),
            library_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::LibraryDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::FunctionDefinition(v))),
            constant_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::ConstantDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::EnumDefinition(v))),
            user_defined_value_type_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::UserDefinedValueTypeDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(definition::_C0::ErrorDefinition(v))),
        ))
        .boxed();
        let source_unit_parser = ignore_parser
            .clone()
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_C2::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_C2::Definition(v))),
                ))
                .repeated(),
            )
            .then(end())
            .map(|v| Box::new(source_unit::_S0::new(v)))
            .boxed();
        Self {
            add_sub_operator: add_sub_operator_parser.boxed(),
            assignment_operator: assignment_operator_parser.boxed(),
            break_statement: break_statement_parser.boxed(),
            comment: comment_parser.boxed(),
            continue_statement: continue_statement_parser.boxed(),
            data_location: data_location_parser.boxed(),
            equality_comparison_operator: equality_comparison_operator_parser.boxed(),
            line_comment: line_comment_parser.boxed(),
            mul_div_mod_operator: mul_div_mod_operator_parser.boxed(),
            order_comparison_operator: order_comparison_operator_parser.boxed(),
            positional_argument_list: positional_argument_list_parser.boxed(),
            shift_operator: shift_operator_parser.boxed(),
            state_mutability_specifier: state_mutability_specifier_parser.boxed(),
            unary_prefix_operator: unary_prefix_operator_parser.boxed(),
            unary_suffix_operator: unary_suffix_operator_parser.boxed(),
            unchecked_block: unchecked_block_parser.boxed(),
            visibility_specifier: visibility_specifier_parser.boxed(),
            whitespace: whitespace_parser.boxed(),
            yul_break_statement: yul_break_statement_parser.boxed(),
            yul_continue_statement: yul_continue_statement_parser.boxed(),
            yul_leave_statement: yul_leave_statement_parser.boxed(),
            ignore: ignore_parser.boxed(),
            ascii_escape: ascii_escape_parser.boxed(),
            boolean_literal: boolean_literal_parser.boxed(),
            decimal_integer: decimal_integer_parser.boxed(),
            fixed_bytes_type: fixed_bytes_type_parser.boxed(),
            fixed_type: fixed_type_parser.boxed(),
            hex_character: hex_character_parser.boxed(),
            identifier_start: identifier_start_parser.boxed(),
            number_unit: number_unit_parser.boxed(),
            pragma_directive: pragma_directive_parser.boxed(),
            reserved_keyword: reserved_keyword_parser.boxed(),
            signed_integer_type: signed_integer_type_parser.boxed(),
            yul_decimal_number_literal: yul_decimal_number_literal_parser.boxed(),
            yul_evm_builtin_function_name: yul_evm_builtin_function_name_parser.boxed(),
            yul_hex_literal: yul_hex_literal_parser.boxed(),
            yul_keyword: yul_keyword_parser.boxed(),
            decimal_exponent: decimal_exponent_parser.boxed(),
            decimal_float: decimal_float_parser.boxed(),
            hex_byte_escape: hex_byte_escape_parser.boxed(),
            hex_number: hex_number_parser.boxed(),
            identifier_part: identifier_part_parser.boxed(),
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser
                .boxed(),
            ufixed_type: ufixed_type_parser.boxed(),
            unicode_escape: unicode_escape_parser.boxed(),
            unsigned_integer_type: unsigned_integer_type_parser.boxed(),
            yul_reserved_word: yul_reserved_word_parser.boxed(),
            decimal_number: decimal_number_parser.boxed(),
            elementary_type: elementary_type_parser.boxed(),
            escape_sequence: escape_sequence_parser.boxed(),
            hex_string_literal: hex_string_literal_parser.boxed(),
            keyword: keyword_parser.boxed(),
            raw_identifier: raw_identifier_parser.boxed(),
            double_quoted_ascii_string_literal: double_quoted_ascii_string_literal_parser.boxed(),
            double_quoted_unicode_string_literal: double_quoted_unicode_string_literal_parser
                .boxed(),
            elementary_type_with_payable: elementary_type_with_payable_parser.boxed(),
            elementary_type_without_payable: elementary_type_without_payable_parser.boxed(),
            numeric_literal: numeric_literal_parser.boxed(),
            reserved_word: reserved_word_parser.boxed(),
            single_quoted_ascii_string_literal: single_quoted_ascii_string_literal_parser.boxed(),
            single_quoted_unicode_string_literal: single_quoted_unicode_string_literal_parser
                .boxed(),
            yul_identifier: yul_identifier_parser.boxed(),
            ascii_string_literal: ascii_string_literal_parser.boxed(),
            assembly_flags: assembly_flags_parser.boxed(),
            identifier: identifier_parser.boxed(),
            unicode_string_literal: unicode_string_literal_parser.boxed(),
            yul_function_call: yul_function_call_parser.boxed(),
            yul_function_definition: yul_function_definition_parser.boxed(),
            yul_path: yul_path_parser.boxed(),
            enum_definition: enum_definition_parser.boxed(),
            identifier_path: identifier_path_parser.boxed(),
            import_path: import_path_parser.boxed(),
            literal: literal_parser.boxed(),
            named_argument: named_argument_parser.boxed(),
            parameter_declaration: parameter_declaration_parser.boxed(),
            selected_import: selected_import_parser.boxed(),
            user_defined_value_type_definition: user_defined_value_type_definition_parser.boxed(),
            yul_literal: yul_literal_parser.boxed(),
            mapping_type: mapping_type_parser.boxed(),
            named_argument_list: named_argument_list_parser.boxed(),
            non_empty_parameter_list: non_empty_parameter_list_parser.boxed(),
            override_specifier: override_specifier_parser.boxed(),
            parameter_list: parameter_list_parser.boxed(),
            selecting_import_directive: selecting_import_directive_parser.boxed(),
            simple_import_directive: simple_import_directive_parser.boxed(),
            star_import_directive: star_import_directive_parser.boxed(),
            yul_expression: yul_expression_parser.boxed(),
            argument_list: argument_list_parser.boxed(),
            catch_clause: catch_clause_parser.boxed(),
            function_type: function_type_parser.boxed(),
            import_directive: import_directive_parser.boxed(),
            method_attribute: method_attribute_parser.boxed(),
            state_variable_attribute: state_variable_attribute_parser.boxed(),
            yul_assignment: yul_assignment_parser.boxed(),
            yul_for_statement: yul_for_statement_parser.boxed(),
            yul_if_statement: yul_if_statement_parser.boxed(),
            yul_switch_statement: yul_switch_statement_parser.boxed(),
            yul_variable_declaration: yul_variable_declaration_parser.boxed(),
            inheritance_specifier: inheritance_specifier_parser.boxed(),
            modifier_invocation: modifier_invocation_parser.boxed(),
            type_name: type_name_parser.boxed(),
            yul_statement: yul_statement_parser.boxed(),
            constructor_attribute: constructor_attribute_parser.boxed(),
            error_parameter: error_parameter_parser.boxed(),
            event_parameter: event_parameter_parser.boxed(),
            fallback_function_attribute: fallback_function_attribute_parser.boxed(),
            function_attribute: function_attribute_parser.boxed(),
            inheritance_specifier_list: inheritance_specifier_list_parser.boxed(),
            primary_expression: primary_expression_parser.boxed(),
            receive_function_attribute: receive_function_attribute_parser.boxed(),
            struct_definition: struct_definition_parser.boxed(),
            using_directive: using_directive_parser.boxed(),
            variable_declaration: variable_declaration_parser.boxed(),
            yul_block: yul_block_parser.boxed(),
            assembly_statement: assembly_statement_parser.boxed(),
            directive: directive_parser.boxed(),
            error_definition: error_definition_parser.boxed(),
            event_definition: event_definition_parser.boxed(),
            index_access_expression: index_access_expression_parser.boxed(),
            variable_declaration_tuple: variable_declaration_tuple_parser.boxed(),
            member_access_expression: member_access_expression_parser.boxed(),
            function_call_options_expression: function_call_options_expression_parser.boxed(),
            function_call_expression: function_call_expression_parser.boxed(),
            unary_prefix_expression: unary_prefix_expression_parser.boxed(),
            unary_suffix_expression: unary_suffix_expression_parser.boxed(),
            exp_expression: exp_expression_parser.boxed(),
            mul_div_mod_expression: mul_div_mod_expression_parser.boxed(),
            add_sub_expression: add_sub_expression_parser.boxed(),
            shift_expression: shift_expression_parser.boxed(),
            bit_and_expression: bit_and_expression_parser.boxed(),
            bit_x_or_expression: bit_x_or_expression_parser.boxed(),
            bit_or_expression: bit_or_expression_parser.boxed(),
            order_comparison_expression: order_comparison_expression_parser.boxed(),
            equality_comparison_expression: equality_comparison_expression_parser.boxed(),
            and_expression: and_expression_parser.boxed(),
            or_expression: or_expression_parser.boxed(),
            conditional_expression: conditional_expression_parser.boxed(),
            assignment_expression: assignment_expression_parser.boxed(),
            expression: expression_parser.boxed(),
            constant_definition: constant_definition_parser.boxed(),
            do_while_statement: do_while_statement_parser.boxed(),
            emit_statement: emit_statement_parser.boxed(),
            expression_statement: expression_statement_parser.boxed(),
            if_statement: if_statement_parser.boxed(),
            return_statement: return_statement_parser.boxed(),
            revert_statement: revert_statement_parser.boxed(),
            state_variable_declaration: state_variable_declaration_parser.boxed(),
            try_statement: try_statement_parser.boxed(),
            variable_declaration_statement: variable_declaration_statement_parser.boxed(),
            while_statement: while_statement_parser.boxed(),
            simple_statement: simple_statement_parser.boxed(),
            for_statement: for_statement_parser.boxed(),
            statement: statement_parser.boxed(),
            block: block_parser.boxed(),
            constructor_definition: constructor_definition_parser.boxed(),
            fallback_function_definition: fallback_function_definition_parser.boxed(),
            function_definition: function_definition_parser.boxed(),
            modifier_definition: modifier_definition_parser.boxed(),
            receive_function_definition: receive_function_definition_parser.boxed(),
            contract_body_element: contract_body_element_parser.boxed(),
            contract_definition: contract_definition_parser.boxed(),
            interface_definition: interface_definition_parser.boxed(),
            library_definition: library_definition_parser.boxed(),
            definition: definition_parser.boxed(),
            source_unit: source_unit_parser.boxed(),
        }
    }
}
