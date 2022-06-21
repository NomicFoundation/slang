use super::parser_interface::*;
use super::tree_interface::*;
use chumsky::prelude::*;
use chumsky::primitive::Just;
use chumsky::Parser;
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
fn difference<M, MO, S, SO>(minuend: M, subtrahend: S) -> impl Parser<char, MO, Error = ErrorType>
where
    M: Clone + Parser<char, MO, Error = ErrorType>,
    S: Parser<char, SO, Error = ErrorType>,
{
    let minuend_end = minuend.clone().map_with_span(|_, span| span.end).rewind();
    let subtrahend_end = subtrahend
        .map_with_span(|_, span| span.end)
        .rewind()
        .or_else(|_| Ok(0));
    minuend_end
        .then(subtrahend_end)
        .validate(|(m, s), span, emit| {
            if m == s {
                emit(Simple::custom(span, "subtrahend matches minuend"))
            }
        })
        .ignore_then(minuend)
}
#[allow(dead_code)]
#[inline]
fn terminal(str: &str) -> Just<char, &str, ErrorType> {
    just(str)
}

impl Parsers {
    pub fn new() -> Self {
        let mut block_parser_unmapped = Recursive::declare();
        let mut block_parser = Recursive::declare();

        let mut expression_parser_unmapped = Recursive::declare();
        let mut expression_parser = Recursive::declare();

        let mut statement_parser_unmapped = Recursive::declare();
        let mut statement_parser = Recursive::declare();

        let mut type_name_parser_unmapped = Recursive::declare();
        let mut type_name_parser = Recursive::declare();

        let mut yul_block_parser_unmapped = Recursive::declare();
        let mut yul_block_parser = Recursive::declare();

        let mut yul_expression_parser_unmapped = Recursive::declare();
        let mut yul_expression_parser = Recursive::declare();

        // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
        let ascii_escape_parser_unmapped = filter(|&c: &char| {
            c == 'n'
                || c == 'r'
                || c == 't'
                || c == '\''
                || c == '"'
                || c == '\\'
                || c == '\n'
                || c == '\r'
        })
        .map(|_| FixedTerminal::<1>());
        let ascii_escape_parser = ascii_escape_parser_unmapped.boxed();

        // «BooleanLiteral» = 'true' | 'false' ;
        let boolean_literal_parser_unmapped = choice::<_, ErrorType>((
            terminal("false").map(|_| 5usize),
            terminal("true").map(|_| 4usize),
        ));
        let boolean_literal_parser = boolean_literal_parser_unmapped.boxed();

        // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let comment_parser_unmapped = terminal("/*")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedTerminal::<1>())
                        .map(|v| Box::new(comment::_T2::NotStarChar(v))),
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .then(
                            filter(|&c: &char| c != '*' && c != '/').map(|_| FixedTerminal::<1>()),
                        )
                        .map(|v| Box::new(comment::_T3::new(v)))
                        .map(|v| Box::new(comment::_T2::_T3(v))),
                ))
                .repeated(),
            )
            .then(
                just('*')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
            )
            .then(terminal("*/").ignored().map(|_| FixedTerminal::<2usize>()))
            .map(|v| Box::new(comment::_T0::new(v)));
        let comment_parser = comment_parser_unmapped.boxed();

        // «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
        let decimal_integer_parser_unmapped = filter(|&c: &char| ('0' <= c && c <= '9'))
            .map(|_| FixedTerminal::<1>())
            .then(
                just('_')
                    .map(|_| FixedTerminal::<1>())
                    .or_not()
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')).map(|_| FixedTerminal::<1>()))
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(decimal_integer::_T0::new(v)));
        let decimal_integer_parser = decimal_integer_parser_unmapped.boxed();

        // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
        let fixed_bytes_type_parser_unmapped = terminal("bytes")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
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
            .map(|v| Box::new(fixed_bytes_type::_T0::new(v)));
        let fixed_bytes_type_parser = fixed_bytes_type_parser_unmapped.boxed();

        // «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
        let fixed_type_parser_unmapped = terminal("fixed")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(
                filter(|&c: &char| ('1' <= c && c <= '9'))
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .map(|_| FixedTerminal::<1>())
                            .repeated()
                            .map(|v| v.len()),
                    )
                    .then(just('x').map(|_| FixedTerminal::<1>()))
                    .then(filter(|&c: &char| ('1' <= c && c <= '9')).map(|_| FixedTerminal::<1>()))
                    .then(
                        filter(|&c: &char| ('0' <= c && c <= '9'))
                            .map(|_| FixedTerminal::<1>())
                            .repeated()
                            .map(|v| v.len()),
                    )
                    .map(|v| Box::new(fixed_type::_T1::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(fixed_type::_T0::new(v)));
        let fixed_type_parser = fixed_type_parser_unmapped.boxed();

        // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
        let hex_character_parser_unmapped = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .map(|_| FixedTerminal::<1>());
        let hex_character_parser = hex_character_parser_unmapped.boxed();

        // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
        let identifier_start_parser_unmapped = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .map(|_| FixedTerminal::<1>());
        let identifier_start_parser = identifier_start_parser_unmapped.boxed();

        // «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
        let line_comment_parser_unmapped = terminal("//")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                filter(|&c: &char| c != '\n' && c != '\r')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
            )
            .map(|v| Box::new(line_comment::_T0::new(v)));
        let line_comment_parser = line_comment_parser_unmapped.boxed();

        // «NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;
        let number_unit_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let number_unit_parser = number_unit_parser_unmapped.boxed();

        // «PragmaDirective» = 'pragma' 1…*{ ¬';' } ';' ;
        let pragma_directive_parser_unmapped = terminal("pragma")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(
                filter(|&c: &char| c != ';')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .at_least(1usize)
                    .map(|v| v.len()),
            )
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(pragma_directive::_T0::new(v)));
        let pragma_directive_parser = pragma_directive_parser_unmapped.boxed();

        // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
        let reserved_keyword_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let reserved_keyword_parser = reserved_keyword_parser_unmapped.boxed();

        // «SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;
        let signed_integer_type_parser_unmapped = terminal("int")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
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
            .map(|v| Box::new(signed_integer_type::_T0::new(v)));
        let signed_integer_type_parser = signed_integer_type_parser_unmapped.boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
        let whitespace_parser_unmapped =
            filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n')
                .map(|_| FixedTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| v.len());
        let whitespace_parser = whitespace_parser_unmapped.boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser_unmapped = choice((
            just('0')
                .map(|_| FixedTerminal::<1>())
                .map(|v| Box::new(yul_decimal_number_literal::_T0::ZeroChar(v))),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .map(|_| FixedTerminal::<1>())
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .map(|v| v.len()),
                )
                .map(|v| Box::new(yul_decimal_number_literal::_T1::new(v)))
                .map(|v| Box::new(yul_decimal_number_literal::_T0::_T1(v))),
        ));
        let yul_decimal_number_literal_parser = yul_decimal_number_literal_parser_unmapped.boxed();

        // «YulEVMBuiltinFunctionName» = 'stop' | 'add' | 'sub' | 'mul' | 'div' | 'sdiv' | 'mod' | 'smod' | 'exp' | 'not' | 'lt' | 'gt' | 'slt' | 'sgt' | 'eq' | 'iszero' | 'and' | 'or' | 'xor' | 'byte' | 'shl' | 'shr' | 'sar' | 'addmod' | 'mulmod' | 'signextend' | 'keccak256' | 'pop' | 'mload' | 'mstore' | 'mstore8' | 'sload' | 'sstore' | 'msize' | 'gas' | 'address' | 'balance' | 'selfbalance' | 'caller' | 'callvalue' | 'calldataload' | 'calldatasize' | 'calldatacopy' | 'extcodesize' | 'extcodecopy' | 'returndatasize' | 'returndatacopy' | 'extcodehash' | 'create' | 'create2' | 'call' | 'callcode' | 'delegatecall' | 'staticcall' | 'return' | 'revert' | 'selfdestruct' | 'invalid' | 'log0' | 'log1' | 'log2' | 'log3' | 'log4' | 'chainid' | 'origin' | 'gasprice' | 'Blockhash' | 'coinbase' | 'timestamp' | 'number' | 'difficulty' | 'gaslimit' | 'basefee' ;
        let yul_evm_builtin_function_name_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let yul_evm_builtin_function_name_parser =
            yul_evm_builtin_function_name_parser_unmapped.boxed();

        // «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
        let yul_hex_literal_parser_unmapped = terminal("0x")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>())
                .repeated()
                .at_least(1usize)
                .map(|v| v.len()),
            )
            .map(|v| Box::new(yul_hex_literal::_T0::new(v)));
        let yul_hex_literal_parser = yul_hex_literal_parser_unmapped.boxed();

        // «YulKeyword» = 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'if' | 'leave' | 'let' | 'switch' | 'hex' ;
        let yul_keyword_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let yul_keyword_parser = yul_keyword_parser_unmapped.boxed();

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        let decimal_exponent_parser_unmapped = filter(|&c: &char| c == 'e' || c == 'E')
            .map(|_| FixedTerminal::<1>())
            .then(just('-').map(|_| FixedTerminal::<1>()).or_not())
            .then(decimal_integer_parser.clone())
            .map(|v| Box::new(decimal_exponent::_T0::new(v)));
        let decimal_exponent_parser = decimal_exponent_parser_unmapped.boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser_unmapped = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.').map(|_| FixedTerminal::<1>()))
            .then(decimal_integer_parser.clone())
            .map(|v| Box::new(decimal_float::_T0::new(v)));
        let decimal_float_parser = decimal_float_parser_unmapped.boxed();

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        let hex_byte_escape_parser_unmapped = just('x')
            .map(|_| FixedTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>())
                .repeated()
                .exactly(2usize)
                .map(|v| v.len()),
            )
            .map(|v| Box::new(hex_byte_escape::_T0::new(v)));
        let hex_byte_escape_parser = hex_byte_escape_parser_unmapped.boxed();

        // «HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;
        let hex_number_parser_unmapped = just('0')
            .map(|_| FixedTerminal::<1>())
            .then(just('x').map(|_| FixedTerminal::<1>()))
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>())
                .then(
                    just('_')
                        .map(|_| FixedTerminal::<1>())
                        .or_not()
                        .then(
                            filter(|&c: &char| {
                                ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')
                            })
                            .map(|_| FixedTerminal::<1>()),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(hex_number::_T1::new(v))),
            )
            .map(|v| Box::new(hex_number::_T0::new(v)));
        let hex_number_parser = hex_number_parser_unmapped.boxed();

        // «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
        let ignore_parser_unmapped = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(ignore::_T1::Whitespace(v))),
            comment_parser
                .clone()
                .map(|v| Box::new(ignore::_T1::Comment(v))),
            line_comment_parser
                .clone()
                .map(|v| Box::new(ignore::_T1::LineComment(v))),
        ))
        .repeated();
        let ignore_parser = ignore_parser_unmapped.boxed();

        // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
        let identifier_part_parser_unmapped = filter(|&c: &char| {
            c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')
                || ('0' <= c && c <= '9')
        })
        .map(|_| FixedTerminal::<1>());
        let identifier_part_parser = identifier_part_parser_unmapped.boxed();

        // «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
        let possibly_separated_pairs_of_hex_digits_parser_unmapped = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .map(|_| FixedTerminal::<1>())
        .repeated()
        .exactly(2usize)
        .map(|v| v.len())
        .then(
            just('_')
                .map(|_| FixedTerminal::<1>())
                .or_not()
                .then(
                    filter(|&c: &char| {
                        ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                    })
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .exactly(2usize)
                    .map(|v| v.len()),
                )
                .repeated(),
        )
        .map(repetition_mapper)
        .map(|v| Box::new(possibly_separated_pairs_of_hex_digits::_T0::new(v)));
        let possibly_separated_pairs_of_hex_digits_parser =
            possibly_separated_pairs_of_hex_digits_parser_unmapped.boxed();

        // «UfixedType» = 'u' «FixedType» ;
        let ufixed_type_parser_unmapped = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(fixed_type_parser.clone())
            .map(|v| Box::new(ufixed_type::_T0::new(v)));
        let ufixed_type_parser = ufixed_type_parser_unmapped.boxed();

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        let unicode_escape_parser_unmapped = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>())
                .repeated()
                .exactly(4usize)
                .map(|v| v.len()),
            )
            .map(|v| Box::new(unicode_escape::_T0::new(v)));
        let unicode_escape_parser = unicode_escape_parser_unmapped.boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser_unmapped = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(signed_integer_type_parser.clone())
            .map(|v| Box::new(unsigned_integer_type::_T0::new(v)));
        let unsigned_integer_type_parser = unsigned_integer_type_parser_unmapped.boxed();

        // «YulReservedWord» = «YulKeyword» | «YulEVMBuiltinFunctionName» | «BooleanLiteral» ;
        let yul_reserved_word_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let yul_reserved_word_parser = yul_reserved_word_parser_unmapped.boxed();

        // AddSubOperator = '+' | '-' ;
        let add_sub_operator_parser_unmapped =
            filter(|&c: &char| c == '+' || c == '-').map(|_| FixedTerminal::<1>());
        let add_sub_operator_parser = add_sub_operator_parser_unmapped.boxed();

        // AssignmentOperator = '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ;
        let assignment_operator_parser_unmapped = choice::<_, ErrorType>((
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
        ));
        let assignment_operator_parser = assignment_operator_parser_unmapped.boxed();

        // BreakStatement = 'break' ';' ;
        let break_statement_parser_unmapped = terminal("break")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(break_statement::_T0::new(v)));
        let break_statement_parser = break_statement_parser_unmapped.boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser_unmapped = terminal("continue")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(continue_statement::_T0::new(v)));
        let continue_statement_parser = continue_statement_parser_unmapped.boxed();

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        let data_location_parser_unmapped = choice::<_, ErrorType>((
            terminal("calldata").map(|_| 8usize),
            terminal("memory").map(|_| 6usize),
            terminal("storage").map(|_| 7usize),
        ));
        let data_location_parser = data_location_parser_unmapped.boxed();

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        let decimal_number_parser_unmapped = choice((
            decimal_integer_parser
                .clone()
                .map(|v| Box::new(decimal_number::_T1::DecimalInteger(v))),
            decimal_float_parser
                .clone()
                .map(|v| Box::new(decimal_number::_T1::DecimalFloat(v))),
        ))
        .then(decimal_exponent_parser.clone().or_not())
        .map(|v| Box::new(decimal_number::_T0::new(v)));
        let decimal_number_parser = decimal_number_parser_unmapped.boxed();

        // ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
        let elementary_type_parser_unmapped = choice((
            choice::<_, ErrorType>((
                terminal("b").ignore_then(choice((
                    terminal("ool").map(|_| 4usize),
                    terminal("ytes").map(|_| 5usize),
                ))),
                terminal("string").map(|_| 6usize),
            ))
            .map(|v| Box::new(elementary_type::_T0::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::FixedBytesType(v))),
            fixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::FixedType(v))),
            ufixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::_T0::UfixedType(v))),
        ));
        let elementary_type_parser = elementary_type_parser_unmapped.boxed();

        // EqualityComparisonOperator = '==' | '!=' ;
        let equality_comparison_operator_parser_unmapped =
            choice::<_, ErrorType>((terminal("!=").ignored(), terminal("==").ignored()))
                .map(|_| FixedTerminal::<2usize>());
        let equality_comparison_operator_parser =
            equality_comparison_operator_parser_unmapped.boxed();

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser_unmapped = just('\\')
            .map(|_| FixedTerminal::<1>())
            .then(choice((
                filter(|&c: &char| {
                    c == 'n'
                        || c == 'r'
                        || c == 't'
                        || c == '\''
                        || c == '"'
                        || c == '\\'
                        || c == '\n'
                        || c == '\r'
                })
                .map(|_| FixedTerminal::<1>())
                .map(|v| Box::new(escape_sequence::_T1::_0(v))),
                hex_byte_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_T1::HexByteEscape(v))),
                unicode_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::_T1::UnicodeEscape(v))),
            )))
            .map(|v| Box::new(escape_sequence::_T0::new(v)));
        let escape_sequence_parser = escape_sequence_parser_unmapped.boxed();

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        let hex_string_literal_parser_unmapped = terminal("hex")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(choice((
                just('"')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('"').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(hex_string_literal::_T2::new(v)))
                    .map(|v| Box::new(hex_string_literal::_T1::_T2(v))),
                just('\'')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('\'').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(hex_string_literal::_T3::new(v)))
                    .map(|v| Box::new(hex_string_literal::_T1::_T3(v))),
            )))
            .map(|v| Box::new(hex_string_literal::_T0::new(v)));
        let hex_string_literal_parser = hex_string_literal_parser_unmapped.boxed();

        // «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
        let keyword_parser_unmapped = choice((
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
            .map(|v| Box::new(keyword::_T0::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(keyword::_T0::FixedBytesType(v))),
            choice::<_, ErrorType>((
                terminal("fixed").map(|_| 5usize),
                terminal("ufixed").map(|_| 6usize),
            ))
            .map(|v| Box::new(keyword::_T0::_4(v))),
        ));
        let keyword_parser = keyword_parser_unmapped.boxed();

        // MulDivModOperator = '*' | '/' | '%' ;
        let mul_div_mod_operator_parser_unmapped =
            filter(|&c: &char| c == '*' || c == '/' || c == '%').map(|_| FixedTerminal::<1>());
        let mul_div_mod_operator_parser = mul_div_mod_operator_parser_unmapped.boxed();

        // OrderComparisonOperator = '<' | '>' | '<=' | '>=' ;
        let order_comparison_operator_parser_unmapped = choice::<_, ErrorType>((
            terminal("<").ignore_then(choice((
                terminal("=").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
            terminal(">").ignore_then(choice((
                terminal("=").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
        ));
        let order_comparison_operator_parser = order_comparison_operator_parser_unmapped.boxed();

        // PositionalArgumentList = 1…*{ Expression / ',' } ;
        let positional_argument_list_parser_unmapped = expression_parser
            .clone()
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(expression_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(positional_argument_list::_T0::new(v)));
        let positional_argument_list_parser = positional_argument_list_parser_unmapped.boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        let raw_identifier_parser_unmapped = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .map(|_| FixedTerminal::<1>())
        .then(
            filter(|&c: &char| {
                c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')
            })
            .map(|_| FixedTerminal::<1>())
            .repeated()
            .map(|v| v.len()),
        )
        .map(|v| Box::new(raw_identifier::_T0::new(v)));
        let raw_identifier_parser = raw_identifier_parser_unmapped.boxed();

        // ShiftOperator = '<<' | '>>' | '>>>' ;
        let shift_operator_parser_unmapped = choice::<_, ErrorType>((
            terminal("<<").map(|_| 2usize),
            terminal(">>").ignore_then(choice((
                terminal(">").map(|_| 3usize),
                empty().map(|_| 2usize),
            ))),
        ));
        let shift_operator_parser = shift_operator_parser_unmapped.boxed();

        // StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;
        let state_mutability_specifier_parser_unmapped = choice::<_, ErrorType>((
            terminal("p").ignore_then(choice((
                terminal("ayable").map(|_| 7usize),
                terminal("ure").map(|_| 4usize),
            ))),
            terminal("view").map(|_| 4usize),
        ));
        let state_mutability_specifier_parser = state_mutability_specifier_parser_unmapped.boxed();

        // UnaryPrefixOperator = '++' | '--' | '!' | '~' | 'delete' | '-' ;
        let unary_prefix_operator_parser_unmapped = choice::<_, ErrorType>((
            terminal("!").map(|_| 1usize),
            terminal("++").map(|_| 2usize),
            terminal("-").ignore_then(choice((
                terminal("-").map(|_| 2usize),
                empty().map(|_| 1usize),
            ))),
            terminal("delete").map(|_| 6usize),
            terminal("~").map(|_| 1usize),
        ));
        let unary_prefix_operator_parser = unary_prefix_operator_parser_unmapped.boxed();

        // UnarySuffixOperator = '++' | '--' ;
        let unary_suffix_operator_parser_unmapped =
            choice::<_, ErrorType>((terminal("++").ignored(), terminal("--").ignored()))
                .map(|_| FixedTerminal::<2usize>());
        let unary_suffix_operator_parser = unary_suffix_operator_parser_unmapped.boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser_unmapped = terminal("unchecked")
            .ignored()
            .map(|_| FixedTerminal::<9usize>())
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(unchecked_block::_T0::new(v)));
        let unchecked_block_parser = unchecked_block_parser_unmapped.boxed();

        // VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;
        let visibility_specifier_parser_unmapped = choice::<_, ErrorType>((
            terminal("external").map(|_| 8usize),
            terminal("internal").map(|_| 8usize),
            terminal("p").ignore_then(choice((
                terminal("rivate").map(|_| 7usize),
                terminal("ublic").map(|_| 6usize),
            ))),
        ));
        let visibility_specifier_parser = visibility_specifier_parser_unmapped.boxed();

        // YulBreakStatement = 'break' ;
        let yul_break_statement_parser_unmapped = terminal("break")
            .ignored()
            .map(|_| FixedTerminal::<5usize>());
        let yul_break_statement_parser = yul_break_statement_parser_unmapped.boxed();

        // YulContinueStatement = 'continue' ;
        let yul_continue_statement_parser_unmapped = terminal("continue")
            .ignored()
            .map(|_| FixedTerminal::<8usize>());
        let yul_continue_statement_parser = yul_continue_statement_parser_unmapped.boxed();

        // YulLeaveStatement = 'leave' ;
        let yul_leave_statement_parser_unmapped = terminal("leave")
            .ignored()
            .map(|_| FixedTerminal::<5usize>());
        let yul_leave_statement_parser = yul_leave_statement_parser_unmapped.boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser_unmapped = just('"')
            .map(|_| FixedTerminal::<1>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '"' && c != '\\')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .map(|v| Box::new(double_quoted_ascii_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(double_quoted_ascii_string_literal::_T0::new(v)));
        let double_quoted_ascii_string_literal_parser =
            double_quoted_ascii_string_literal_parser_unmapped.boxed();

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser_unmapped = terminal("unicode\"")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '"' && c != '\\' && c != '\n' && c != '\r')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .map(|v| Box::new(double_quoted_unicode_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(double_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('"').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(double_quoted_unicode_string_literal::_T0::new(v)));
        let double_quoted_unicode_string_literal_parser =
            double_quoted_unicode_string_literal_parser_unmapped.boxed();

        // ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
        let elementary_type_with_payable_parser_unmapped = choice((
            terminal("address")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .then(
                    terminal("payable")
                        .ignored()
                        .map(|_| FixedTerminal::<7usize>())
                        .or_not(),
                )
                .map(|v| Box::new(elementary_type_with_payable::_T1::new(v)))
                .map(|v| Box::new(elementary_type_with_payable::_T0::_T1(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_with_payable::_T0::ElementaryType(v))),
        ));
        let elementary_type_with_payable_parser =
            elementary_type_with_payable_parser_unmapped.boxed();

        // ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
        let elementary_type_without_payable_parser_unmapped = choice((
            terminal("address")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(elementary_type_without_payable::_T0::Address(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_without_payable::_T0::ElementaryType(v))),
        ));
        let elementary_type_without_payable_parser =
            elementary_type_without_payable_parser_unmapped.boxed();

        // NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser_unmapped = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_T1::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_T1::HexNumber(v))),
        ))
        .then(ignore_parser.clone())
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
        .map(|v| Box::new(numeric_literal::_T0::new(v)));
        let numeric_literal_parser = numeric_literal_parser_unmapped.boxed();

        // «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
        let reserved_word_parser_unmapped = choice((
            keyword_parser
                .clone()
                .map(|v| Box::new(reserved_word::_T0::Keyword(v))),
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
            .map(|v| Box::new(reserved_word::_T0::_1(v))),
        ));
        let reserved_word_parser = reserved_word_parser_unmapped.boxed();

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser_unmapped = just('\'')
            .map(|_| FixedTerminal::<1>())
            .then(
                choice((
                    filter(|&c: &char| (' ' <= c && c <= '~') && c != '\'' && c != '\\')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .map(|v| Box::new(single_quoted_ascii_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_ascii_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(single_quoted_ascii_string_literal::_T0::new(v)));
        let single_quoted_ascii_string_literal_parser =
            single_quoted_ascii_string_literal_parser_unmapped.boxed();

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser_unmapped = terminal("unicode'")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '\'' && c != '\\' && c != '\n' && c != '\r')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .map(|v| Box::new(single_quoted_unicode_string_literal::Run::Chars(v))),
                    escape_sequence_parser.clone().map(|v| {
                        Box::new(single_quoted_unicode_string_literal::Run::EscapeSequence(v))
                    }),
                ))
                .repeated(),
            )
            .then(just('\'').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(single_quoted_unicode_string_literal::_T0::new(v)));
        let single_quoted_unicode_string_literal_parser =
            single_quoted_unicode_string_literal_parser_unmapped.boxed();

        // «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
        let yul_identifier_parser_unmapped = difference(
            raw_identifier_parser.clone(),
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
            )),
        );
        let yul_identifier_parser = yul_identifier_parser_unmapped.boxed();

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        let ascii_string_literal_parser_unmapped = choice((
            single_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_T0::SingleQuotedAsciiStringLiteral(v))),
            double_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_T0::DoubleQuotedAsciiStringLiteral(v))),
        ));
        let ascii_string_literal_parser = ascii_string_literal_parser_unmapped.boxed();

        // AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
        let assembly_flags_parser_unmapped = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                double_quoted_ascii_string_literal_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(double_quoted_ascii_string_literal_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(assembly_flags::_T1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(assembly_flags::_T0::new(v)));
        let assembly_flags_parser = assembly_flags_parser_unmapped.boxed();

        // «Identifier» = «RawIdentifier» - «ReservedWord» ;
        let identifier_parser_unmapped =
            difference(raw_identifier_parser.clone(), reserved_word_parser.clone());
        let identifier_parser = identifier_parser_unmapped.boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        let unicode_string_literal_parser_unmapped = choice((
            single_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_T0::SingleQuotedUnicodeStringLiteral(v))
                }),
            double_quoted_unicode_string_literal_parser
                .clone()
                .map(|v| {
                    Box::new(unicode_string_literal::_T0::DoubleQuotedUnicodeStringLiteral(v))
                }),
        ));
        let unicode_string_literal_parser = unicode_string_literal_parser_unmapped.boxed();

        // YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
        let yul_function_call_parser_unmapped = choice((
            yul_identifier_parser
                .clone()
                .map(|v| Box::new(yul_function_call::_T1::YulIdentifier(v))),
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
            .map(|v| Box::new(yul_function_call::_T1::_1(v))),
        ))
        .then(ignore_parser.clone())
        .then(just('(').map(|_| FixedTerminal::<1>()))
        .then(ignore_parser.clone())
        .then(
            yul_expression_parser
                .clone()
                .then(
                    just(',')
                        .map(|_| FixedTerminal::<1>())
                        .then(yul_expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(yul_function_call::_T2::new(v)))
                .or_not(),
        )
        .then(ignore_parser.clone())
        .then(just(')').map(|_| FixedTerminal::<1>()))
        .map(|v| Box::new(yul_function_call::_T0::new(v)));
        let yul_function_call_parser = yul_function_call_parser_unmapped.boxed();

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
        let yul_function_definition_parser_unmapped = terminal("function")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(yul_identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                yul_identifier_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(yul_identifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(yul_function_definition::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                terminal("->")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(
                        yul_identifier_parser
                            .clone()
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(yul_identifier_parser.clone())
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(yul_function_definition::_T3::new(v))),
                    )
                    .map(|v| Box::new(yul_function_definition::_T2::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_function_definition::_T0::new(v)));
        let yul_function_definition_parser = yul_function_definition_parser_unmapped.boxed();

        // YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
        let yul_path_parser_unmapped = yul_identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('.')
                    .map(|_| FixedTerminal::<1>())
                    .then(choice((
                        yul_identifier_parser
                            .clone()
                            .map(|v| Box::new(yul_path::_T3::YulIdentifier(v))),
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
                        .map(|v| Box::new(yul_path::_T3::_1(v))),
                    )))
                    .map(|v| Box::new(yul_path::_T2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(yul_path::_T0::new(v)));
        let yul_path_parser = yul_path_parser_unmapped.boxed();

        // EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
        let enum_definition_parser_unmapped = terminal("enum")
            .ignored()
            .map(|_| FixedTerminal::<4usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                identifier_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(identifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(enum_definition::_T1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(enum_definition::_T0::new(v)));
        let enum_definition_parser = enum_definition_parser_unmapped.boxed();

        // IdentifierPath = 1…*{ «Identifier» / '.' } ;
        let identifier_path_parser_unmapped = identifier_parser
            .clone()
            .then(
                just('.')
                    .map(|_| FixedTerminal::<1>())
                    .then(identifier_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(identifier_path::_T0::new(v)));
        let identifier_path_parser = identifier_path_parser_unmapped.boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser_unmapped = ascii_string_literal_parser.clone();
        let import_path_parser = import_path_parser_unmapped.boxed();

        // Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
        let literal_parser_unmapped = choice((
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_T0::AsciiStringLiteral(v))),
            unicode_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_T0::UnicodeStringLiteral(v))),
            numeric_literal_parser
                .clone()
                .map(|v| Box::new(literal::_T0::NumericLiteral(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::_T0::HexStringLiteral(v))),
            choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            ))
            .map(|v| Box::new(literal::_T0::_4(v))),
        ));
        let literal_parser = literal_parser_unmapped.boxed();

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser_unmapped = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just(':').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .map(|v| Box::new(named_argument::_T0::new(v)));
        let named_argument_parser = named_argument_parser_unmapped.boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                ))
                .or_not(),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(parameter_declaration::_T0::new(v)));
        let parameter_declaration_parser = parameter_declaration_parser_unmapped.boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser_unmapped = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("as")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(identifier_parser.clone())
                    .map(|v| Box::new(selected_import::_T1::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(selected_import::_T0::new(v)));
        let selected_import_parser = selected_import_parser_unmapped.boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
        let user_defined_value_type_definition_parser_unmapped = terminal("type")
            .ignored()
            .map(|_| FixedTerminal::<4usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(terminal("is").ignored().map(|_| FixedTerminal::<2usize>()))
            .then(ignore_parser.clone())
            .then(elementary_type_with_payable_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(user_defined_value_type_definition::_T0::new(v)));
        let user_defined_value_type_definition_parser =
            user_defined_value_type_definition_parser_unmapped.boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        let yul_literal_parser_unmapped = choice((
            yul_decimal_number_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_T0::YulDecimalNumberLiteral(v))),
            yul_hex_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_T0::YulHexLiteral(v))),
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_T0::AsciiStringLiteral(v))),
            choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            ))
            .map(|v| Box::new(yul_literal::_T0::_3(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::_T0::HexStringLiteral(v))),
        ));
        let yul_literal_parser = yul_literal_parser_unmapped.boxed();

        // MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser_unmapped = terminal("mapping")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(choice((
                elementary_type_without_payable_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_T1::ElementaryTypeWithoutPayable(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_T1::IdentifierPath(v))),
            )))
            .then(ignore_parser.clone())
            .then(terminal("=>").ignored().map(|_| FixedTerminal::<2usize>()))
            .then(ignore_parser.clone())
            .then(type_name_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(mapping_type::_T0::new(v)));
        let mapping_type_parser = mapping_type_parser_unmapped.boxed();

        // NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
        let named_argument_list_parser_unmapped = just('{')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                named_argument_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(named_argument_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(named_argument_list::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(named_argument_list::_T0::new(v)));
        let named_argument_list_parser = named_argument_list_parser_unmapped.boxed();

        // NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
        let non_empty_parameter_list_parser_unmapped = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(non_empty_parameter_list::_T1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(non_empty_parameter_list::_T0::new(v)));
        let non_empty_parameter_list_parser = non_empty_parameter_list_parser_unmapped.boxed();

        // OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
        let override_specifier_parser_unmapped = terminal("override")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(
                just('(')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(identifier_path_parser.clone())
                                    .repeated()
                                    .at_most(1usize - 1),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(override_specifier::_T2::new(v))),
                    )
                    .then(just(')').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(override_specifier::_T1::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(override_specifier::_T0::new(v)));
        let override_specifier_parser = override_specifier_parser_unmapped.boxed();

        // ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
        let parameter_list_parser_unmapped = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(parameter_list::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(parameter_list::_T0::new(v)));
        let parameter_list_parser = parameter_list_parser_unmapped.boxed();

        // SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
        let selecting_import_directive_parser_unmapped = just('{')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                selected_import_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(selected_import_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(selecting_import_directive::_T1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                terminal("from")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>()),
            )
            .then(ignore_parser.clone())
            .then(import_path_parser.clone())
            .map(|v| Box::new(selecting_import_directive::_T0::new(v)));
        let selecting_import_directive_parser = selecting_import_directive_parser_unmapped.boxed();

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser_unmapped = import_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("as")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(identifier_parser.clone())
                    .map(|v| Box::new(simple_import_directive::_T2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(simple_import_directive::_T0::new(v)));
        let simple_import_directive_parser = simple_import_directive_parser_unmapped.boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser_unmapped = just('*')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(terminal("as").ignored().map(|_| FixedTerminal::<2usize>()))
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(
                terminal("from")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>()),
            )
            .then(ignore_parser.clone())
            .then(import_path_parser.clone())
            .map(|v| Box::new(star_import_directive::_T0::new(v)));
        let star_import_directive_parser = star_import_directive_parser_unmapped.boxed();

        // YulExpression = YulPath | YulFunctionCall | YulLiteral ;
        yul_expression_parser_unmapped.define(choice((
            yul_path_parser
                .clone()
                .map(|v| Box::new(yul_expression::_T0::YulPath(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_expression::_T0::YulFunctionCall(v))),
            yul_literal_parser
                .clone()
                .map(|v| Box::new(yul_expression::_T0::YulLiteral(v))),
        )));
        yul_expression_parser.define(yul_expression_parser_unmapped.boxed());

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser_unmapped = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                choice((
                    positional_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_T1::PositionalArgumentList(v))),
                    named_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::_T1::NamedArgumentList(v))),
                ))
                .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(argument_list::_T0::new(v)));
        let argument_list_parser = argument_list_parser_unmapped.boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
        let catch_clause_parser_unmapped = terminal("catch")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(
                identifier_parser
                    .clone()
                    .or_not()
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(catch_clause::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(catch_clause::_T0::new(v)));
        let catch_clause_parser = catch_clause_parser_unmapped.boxed();

        // FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
        let function_type_parser_unmapped = terminal("function")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
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
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_type::_T2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(function_type::_T0::new(v)));
        let function_type_parser = function_type_parser_unmapped.boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser_unmapped = terminal("import")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(choice((
                simple_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::SimpleImportDirective(v))),
                star_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::StarImportDirective(v))),
                selecting_import_directive_parser
                    .clone()
                    .map(|v| Box::new(import_directive::_T1::SelectingImportDirective(v))),
            )))
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(import_directive::_T0::new(v)));
        let import_directive_parser = import_directive_parser_unmapped.boxed();

        // MethodAttribute = 'virtual' | OverrideSpecifier ;
        let method_attribute_parser_unmapped = choice((
            terminal("virtual")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(method_attribute::_T0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(method_attribute::_T0::OverrideSpecifier(v))),
        ));
        let method_attribute_parser = method_attribute_parser_unmapped.boxed();

        // StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
        let state_variable_attribute_parser_unmapped = choice((
            choice::<_, ErrorType>((
                terminal("constant").map(|_| 8usize),
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("rivate").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            ))
            .map(|v| Box::new(state_variable_attribute::_T0::_0(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(state_variable_attribute::_T0::OverrideSpecifier(v))),
            terminal("immutable")
                .ignored()
                .map(|_| FixedTerminal::<9usize>())
                .map(|v| Box::new(state_variable_attribute::_T0::Immutable(v))),
        ));
        let state_variable_attribute_parser = state_variable_attribute_parser_unmapped.boxed();

        // YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
        let yul_assignment_parser_unmapped = yul_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(choice((
                terminal(":=")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(yul_expression_parser.clone())
                    .map(|v| Box::new(yul_assignment::_T2::new(v)))
                    .map(|v| Box::new(yul_assignment::_T1::_T2(v))),
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(yul_path_parser.clone())
                    .map(|v| Box::new(yul_assignment::_T5::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()))
                    .then(yul_function_call_parser.clone())
                    .map(|v| Box::new(yul_assignment::_T3::new(v)))
                    .map(|v| Box::new(yul_assignment::_T1::_T3(v))),
            )))
            .map(|v| Box::new(yul_assignment::_T0::new(v)));
        let yul_assignment_parser = yul_assignment_parser_unmapped.boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser_unmapped = terminal("for")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .then(ignore_parser.clone())
            .then(yul_expression_parser.clone())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_for_statement::_T0::new(v)));
        let yul_for_statement_parser = yul_for_statement_parser_unmapped.boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser_unmapped = terminal("if")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(yul_expression_parser.clone())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_if_statement::_T0::new(v)));
        let yul_if_statement_parser = yul_if_statement_parser_unmapped.boxed();

        // YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
        let yul_switch_statement_parser_unmapped = terminal("switch")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(yul_expression_parser.clone())
            .then(ignore_parser.clone())
            .then(choice((
                terminal("case")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>())
                    .then(yul_literal_parser.clone())
                    .then(yul_block_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_T4::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(
                        terminal("default")
                            .ignored()
                            .map(|_| FixedTerminal::<7usize>())
                            .then(yul_block_parser.clone())
                            .map(|v| Box::new(yul_switch_statement::_T5::new(v)))
                            .or_not(),
                    )
                    .map(|v| Box::new(yul_switch_statement::_T2::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_T1::_T2(v))),
                terminal("default")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(yul_block_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_T6::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_T1::_T6(v))),
            )))
            .map(|v| Box::new(yul_switch_statement::_T0::new(v)));
        let yul_switch_statement_parser = yul_switch_statement_parser_unmapped.boxed();

        // YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
        let yul_variable_declaration_parser_unmapped = terminal("let")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(yul_identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(
                choice((
                    terminal(":=")
                        .ignored()
                        .map(|_| FixedTerminal::<2usize>())
                        .then(yul_expression_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_T2::new(v)))
                        .map(|v| Box::new(yul_variable_declaration::_T1::_T2(v))),
                    just(',')
                        .map(|_| FixedTerminal::<1>())
                        .then(yul_identifier_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_T4::new(v)))
                        .or_not()
                        .then(
                            terminal(":=")
                                .ignored()
                                .map(|_| FixedTerminal::<2usize>())
                                .then(yul_function_call_parser.clone())
                                .map(|v| Box::new(yul_variable_declaration::_T5::new(v)))
                                .or_not(),
                        )
                        .map(|v| Box::new(yul_variable_declaration::_T3::new(v)))
                        .map(|v| Box::new(yul_variable_declaration::_T1::_T3(v))),
                ))
                .or_not(),
            )
            .map(|v| Box::new(yul_variable_declaration::_T0::new(v)));
        let yul_variable_declaration_parser = yul_variable_declaration_parser_unmapped.boxed();

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser_unmapped = identifier_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(inheritance_specifier::_T0::new(v)));
        let inheritance_specifier_parser = inheritance_specifier_parser_unmapped.boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser_unmapped = identifier_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(modifier_invocation::_T0::new(v)));
        let modifier_invocation_parser = modifier_invocation_parser_unmapped.boxed();

        // TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
        type_name_parser_unmapped.define(
            choice((
                elementary_type_with_payable_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::ElementaryTypeWithPayable(v))),
                function_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::FunctionType(v))),
                mapping_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::MappingType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(type_name::_T1::IdentifierPath(v))),
            ))
            .then(ignore_parser.clone())
            .then(
                just('[')
                    .map(|_| FixedTerminal::<1>())
                    .then(expression_parser.clone().or_not())
                    .then(just(']').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(type_name::_T3::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(type_name::_T0::new(v))),
        );
        type_name_parser.define(type_name_parser_unmapped.boxed());

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        let yul_statement_parser_unmapped = choice((
            yul_block_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulBlock(v))),
            yul_variable_declaration_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulVariableDeclaration(v))),
            yul_function_definition_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulFunctionDefinition(v))),
            yul_assignment_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulAssignment(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulFunctionCall(v))),
            yul_if_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulIfStatement(v))),
            yul_for_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulForStatement(v))),
            yul_switch_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::_T0::YulSwitchStatement(v))),
            choice::<_, ErrorType>((
                terminal("break").map(|_| 5usize),
                terminal("continue").map(|_| 8usize),
                terminal("leave").map(|_| 5usize),
            ))
            .map(|v| Box::new(yul_statement::_T0::_8(v))),
        ));
        let yul_statement_parser = yul_statement_parser_unmapped.boxed();

        // ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
        let constructor_attribute_parser_unmapped = choice((
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(constructor_attribute::_T0::ModifierInvocation(v))),
            choice::<_, ErrorType>((
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            ))
            .map(|v| Box::new(constructor_attribute::_T0::_1(v))),
        ));
        let constructor_attribute_parser = constructor_attribute_parser_unmapped.boxed();

        // ErrorParameter = TypeName [ «Identifier» ] ;
        let error_parameter_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(error_parameter::_T0::new(v)));
        let error_parameter_parser = error_parameter_parser_unmapped.boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("indexed")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(event_parameter::_T0::new(v)));
        let event_parameter_parser = event_parameter_parser_unmapped.boxed();

        // Expression = 1…*{ Expression / AssignmentOperator } | Expression [ '?' Expression ':' Expression ] | 1…*{ Expression / '||' } | 1…*{ Expression / '&&' } | 1…*{ Expression / EqualityComparisonOperator } | 1…*{ Expression / OrderComparisonOperator } | 1…*{ Expression / '|' } | 1…*{ Expression / '^' } | 1…*{ Expression / '&' } | 1…*{ Expression / ShiftOperator } | 1…*{ Expression / AddSubOperator } | 1…*{ Expression / MulDivModOperator } | 1…*{ Expression / '**' } | Expression [ UnarySuffixOperator ] | [ UnaryPrefixOperator ] Expression | Expression { ArgumentList } | Expression { '{' 1…*{ NamedArgument / ',' } '}' } | Expression { '.' ( «Identifier» | 'address' ) } | Expression { '[' [ Expression ] [ ':' [ Expression ] ] ']' } | 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
        expression_parser_unmapped.define({
            let choice_0 = expression_parser
                .clone()
                .then(
                    choice::<_, ErrorType>((
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
                    .then(expression_parser.clone())
                    .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::AssignmentExpression::new(v)));
            let choice_1 = expression_parser
                .clone()
                .then(
                    just('?')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .then(just(':').map(|_| FixedTerminal::<1>()))
                        .then(expression_parser.clone())
                        .map(|v| Box::new(expression::_T1::new(v)))
                        .or_not(),
                )
                .map(|v| Box::new(expression::ConditionalExpression::new(v)));
            let choice_2 = expression_parser
                .clone()
                .then(
                    terminal("||")
                        .ignored()
                        .map(|_| FixedTerminal::<2usize>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::OrExpression::new(v)));
            let choice_3 = expression_parser
                .clone()
                .then(
                    terminal("&&")
                        .ignored()
                        .map(|_| FixedTerminal::<2usize>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::AndExpression::new(v)));
            let choice_4 = expression_parser
                .clone()
                .then(
                    choice::<_, ErrorType>((terminal("!=").ignored(), terminal("==").ignored()))
                        .map(|_| FixedTerminal::<2usize>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::EqualityComparisonExpression::new(v)));
            let choice_5 = expression_parser
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
                    .then(expression_parser.clone())
                    .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::OrderComparisonExpression::new(v)));
            let choice_6 = expression_parser
                .clone()
                .then(
                    just('|')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::BitOrExpression::new(v)));
            let choice_7 = expression_parser
                .clone()
                .then(
                    just('^')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::BitXOrExpression::new(v)));
            let choice_8 = expression_parser
                .clone()
                .then(
                    just('&')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::BitAndExpression::new(v)));
            let choice_9 = expression_parser
                .clone()
                .then(
                    choice::<_, ErrorType>((
                        terminal("<<").map(|_| 2usize),
                        terminal(">>").ignore_then(choice((
                            terminal(">").map(|_| 3usize),
                            empty().map(|_| 2usize),
                        ))),
                    ))
                    .then(expression_parser.clone())
                    .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::ShiftExpression::new(v)));
            let choice_10 = expression_parser
                .clone()
                .then(
                    filter(|&c: &char| c == '+' || c == '-')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::AddSubExpression::new(v)));
            let choice_11 = expression_parser
                .clone()
                .then(
                    filter(|&c: &char| c == '*' || c == '/' || c == '%')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::MulDivModExpression::new(v)));
            let choice_12 = expression_parser
                .clone()
                .then(
                    terminal("**")
                        .ignored()
                        .map(|_| FixedTerminal::<2usize>())
                        .then(expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(expression::ExponentiationExpression::new(v)));
            let choice_13 = expression_parser
                .clone()
                .then(
                    choice::<_, ErrorType>((terminal("++").ignored(), terminal("--").ignored()))
                        .map(|_| FixedTerminal::<2usize>())
                        .or_not(),
                )
                .map(|v| Box::new(expression::UnarySuffixExpression::new(v)));
            let choice_14 = choice::<_, ErrorType>((
                terminal("!").map(|_| 1usize),
                terminal("++").map(|_| 2usize),
                terminal("-").ignore_then(choice((
                    terminal("-").map(|_| 2usize),
                    empty().map(|_| 1usize),
                ))),
                terminal("delete").map(|_| 6usize),
                terminal("~").map(|_| 1usize),
            ))
            .or_not()
            .then(expression_parser.clone())
            .map(|v| Box::new(expression::UnaryPrefixExpression::new(v)));
            let choice_15 = expression_parser
                .clone()
                .then(argument_list_parser.clone().repeated())
                .map(|v| Box::new(expression::FunctionCallExpression::new(v)));
            let choice_16 = expression_parser
                .clone()
                .then(
                    just('{')
                        .map(|_| FixedTerminal::<1>())
                        .then(
                            named_argument_parser
                                .clone()
                                .then(
                                    just(',')
                                        .map(|_| FixedTerminal::<1>())
                                        .then(named_argument_parser.clone())
                                        .repeated(),
                                )
                                .map(repetition_mapper)
                                .map(|v| Box::new(expression::_T5::new(v))),
                        )
                        .then(just('}').map(|_| FixedTerminal::<1>()))
                        .map(|v| Box::new(expression::_T4::new(v)))
                        .repeated(),
                )
                .map(|v| Box::new(expression::FunctionCallOptionsExpression::new(v)));
            let choice_17 = expression_parser
                .clone()
                .then(
                    just('.')
                        .map(|_| FixedTerminal::<1>())
                        .then(choice((
                            identifier_parser
                                .clone()
                                .map(|v| Box::new(expression::_T8::Identifier(v))),
                            terminal("address")
                                .ignored()
                                .map(|_| FixedTerminal::<7usize>())
                                .map(|v| Box::new(expression::_T8::Address(v))),
                        )))
                        .map(|v| Box::new(expression::_T7::new(v)))
                        .repeated(),
                )
                .map(|v| Box::new(expression::MemberAccessExpression::new(v)));
            let choice_18 = expression_parser
                .clone()
                .then(
                    just('[')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone().or_not())
                        .then(
                            just(':')
                                .map(|_| FixedTerminal::<1>())
                                .then(expression_parser.clone().or_not())
                                .map(|v| Box::new(expression::_T11::new(v)))
                                .or_not(),
                        )
                        .then(just(']').map(|_| FixedTerminal::<1>()))
                        .map(|v| Box::new(expression::_T10::new(v)))
                        .repeated(),
                )
                .map(|v| Box::new(expression::IndexAccessExpression::new(v)));
            let choice_19 = terminal("payable")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .then(argument_list_parser.clone())
                .map(|v| Box::new(expression::_T12::new(v)));
            let choice_20 = terminal("type")
                .ignored()
                .map(|_| FixedTerminal::<4usize>())
                .then(just('(').map(|_| FixedTerminal::<1>()))
                .then(type_name_parser.clone())
                .then(just(')').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(expression::_T13::new(v)));
            let choice_21 = terminal("new")
                .ignored()
                .map(|_| FixedTerminal::<3usize>())
                .then(type_name_parser.clone())
                .map(|v| Box::new(expression::_T14::new(v)));
            let choice_22 = just('(')
                .map(|_| FixedTerminal::<1>())
                .then(
                    expression_parser
                        .clone()
                        .or_not()
                        .then(
                            just(',')
                                .map(|_| FixedTerminal::<1>())
                                .then(expression_parser.clone().or_not())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(expression::_T16::new(v))),
                )
                .then(just(')').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(expression::_T15::new(v)));
            let choice_23 = just('[')
                .map(|_| FixedTerminal::<1>())
                .then(
                    expression_parser
                        .clone()
                        .then(
                            just(',')
                                .map(|_| FixedTerminal::<1>())
                                .then(expression_parser.clone())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(expression::_T18::new(v))),
                )
                .then(just(']').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(expression::_T17::new(v)));
            let choice_24 = identifier_parser.clone();
            let choice_25 = literal_parser.clone();
            let choice_26 = elementary_type_without_payable_parser.clone();
            choice_0
        });
        expression_parser.define(expression_parser_unmapped.boxed());

        // FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let fallback_function_attribute_parser_unmapped = choice((
            choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ure").map(|_| 4usize),
                ))),
                terminal("view").map(|_| 4usize),
            ))
            .map(|v| Box::new(fallback_function_attribute::_T0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_T0::ModifierInvocation(v))),
            terminal("virtual")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(fallback_function_attribute::_T0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_T0::OverrideSpecifier(v))),
        ));
        let fallback_function_attribute_parser =
            fallback_function_attribute_parser_unmapped.boxed();

        // FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let function_attribute_parser_unmapped = choice((
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
            .map(|v| Box::new(function_attribute::_T0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(function_attribute::_T0::ModifierInvocation(v))),
            terminal("virtual")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(function_attribute::_T0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::_T0::OverrideSpecifier(v))),
        ));
        let function_attribute_parser = function_attribute_parser_unmapped.boxed();

        // InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
        let inheritance_specifier_list_parser_unmapped = terminal("is")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(
                inheritance_specifier_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(inheritance_specifier_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(inheritance_specifier_list::_T1::new(v))),
            )
            .map(|v| Box::new(inheritance_specifier_list::_T0::new(v)));
        let inheritance_specifier_list_parser = inheritance_specifier_list_parser_unmapped.boxed();

        // ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let receive_function_attribute_parser_unmapped = choice((
            choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("payable").map(|_| 7usize),
            ))
            .map(|v| Box::new(receive_function_attribute::_T0::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_T0::ModifierInvocation(v))),
            terminal("virtual")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(receive_function_attribute::_T0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_T0::OverrideSpecifier(v))),
        ));
        let receive_function_attribute_parser = receive_function_attribute_parser_unmapped.boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
        let struct_definition_parser_unmapped = terminal("struct")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                type_name_parser
                    .clone()
                    .then(identifier_parser.clone())
                    .then(just(';').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(struct_definition::_T2::new(v)))
                    .repeated()
                    .at_least(1usize),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(struct_definition::_T0::new(v)));
        let struct_definition_parser = struct_definition_parser_unmapped.boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser_unmapped = terminal("using")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(choice((
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_T1::IdentifierPath(v))),
                just('{')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(identifier_path_parser.clone())
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(using_directive::_T3::new(v))),
                    )
                    .then(just('}').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(using_directive::_T2::new(v)))
                    .map(|v| Box::new(using_directive::_T1::_T2(v))),
            )))
            .then(ignore_parser.clone())
            .then(terminal("for").ignored().map(|_| FixedTerminal::<3usize>()))
            .then(ignore_parser.clone())
            .then(choice((
                just('*')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(using_directive::_T4::StarChar(v))),
                type_name_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_T4::TypeName(v))),
            )))
            .then(ignore_parser.clone())
            .then(
                terminal("global")
                    .ignored()
                    .map(|_| FixedTerminal::<6usize>())
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(using_directive::_T0::new(v)));
        let using_directive_parser = using_directive_parser_unmapped.boxed();

        // VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
        let variable_declaration_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                ))
                .or_not(),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .map(|v| Box::new(variable_declaration::_T0::new(v)));
        let variable_declaration_parser = variable_declaration_parser_unmapped.boxed();

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser_unmapped.define(
            just('{')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(yul_statement_parser.clone().repeated())
                .then(ignore_parser.clone())
                .then(just('}').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(yul_block::_T0::new(v))),
        );
        yul_block_parser.define(yul_block_parser_unmapped.boxed());

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser_unmapped = terminal("assembly")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(
                terminal("\"evmasm\"")
                    .ignored()
                    .map(|_| FixedTerminal::<8usize>())
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(assembly_flags_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(assembly_statement::_T0::new(v)));
        let assembly_statement_parser = assembly_statement_parser_unmapped.boxed();

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("constant")
                    .ignored()
                    .map(|_| FixedTerminal::<8usize>()),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('=').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(constant_definition::_T0::new(v)));
        let constant_definition_parser = constant_definition_parser_unmapped.boxed();

        // Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
        let directive_parser_unmapped = choice((
            pragma_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::PragmaDirective(v))),
            import_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::ImportDirective(v))),
            using_directive_parser
                .clone()
                .map(|v| Box::new(directive::_T0::UsingDirective(v))),
        ));
        let directive_parser = directive_parser_unmapped.boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser_unmapped = terminal("do")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(statement_parser.clone())
            .then(ignore_parser.clone())
            .then(
                terminal("while")
                    .ignored()
                    .map(|_| FixedTerminal::<5usize>()),
            )
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(do_while_statement::_T0::new(v)));
        let do_while_statement_parser = do_while_statement_parser_unmapped.boxed();

        // EmitStatement = 'emit' Expression ArgumentList ';' ;
        let emit_statement_parser_unmapped = terminal("emit")
            .ignored()
            .map(|_| FixedTerminal::<4usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(emit_statement::_T0::new(v)));
        let emit_statement_parser = emit_statement_parser_unmapped.boxed();

        // ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
        let error_definition_parser_unmapped = terminal("error")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                error_parameter_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(error_parameter_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(error_definition::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(error_definition::_T0::new(v)));
        let error_definition_parser = error_definition_parser_unmapped.boxed();

        // EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
        let event_definition_parser_unmapped = terminal("event")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                event_parameter_parser
                    .clone()
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(event_parameter_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(event_definition::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                terminal("anonymous")
                    .ignored()
                    .map(|_| FixedTerminal::<9usize>())
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(event_definition::_T0::new(v)));
        let event_definition_parser = event_definition_parser_unmapped.boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser_unmapped = expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(expression_statement::_T0::new(v)));
        let expression_statement_parser = expression_statement_parser_unmapped.boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser_unmapped = terminal("if")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(statement_parser.clone())
            .then(ignore_parser.clone())
            .then(
                terminal("else")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>())
                    .then(statement_parser.clone())
                    .map(|v| Box::new(if_statement::_T1::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(if_statement::_T0::new(v)));
        let if_statement_parser = if_statement_parser_unmapped.boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser_unmapped = terminal("return")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(return_statement::_T0::new(v)));
        let return_statement_parser = return_statement_parser_unmapped.boxed();

        // RevertStatement = 'revert' Expression ArgumentList ';' ;
        let revert_statement_parser_unmapped = terminal("revert")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(revert_statement::_T0::new(v)));
        let revert_statement_parser = revert_statement_parser_unmapped.boxed();

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        let state_variable_declaration_parser_unmapped = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(state_variable_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(
                just('=')
                    .map(|_| FixedTerminal::<1>())
                    .then(expression_parser.clone())
                    .map(|v| Box::new(state_variable_declaration::_T2::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(state_variable_declaration::_T0::new(v)));
        let state_variable_declaration_parser = state_variable_declaration_parser_unmapped.boxed();

        // TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block 1…*{ CatchClause } ;
        let try_statement_parser_unmapped = terminal("try")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(try_statement::_T1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .then(ignore_parser.clone())
            .then(catch_clause_parser.clone().repeated().at_least(1usize))
            .map(|v| Box::new(try_statement::_T0::new(v)));
        let try_statement_parser = try_statement_parser_unmapped.boxed();

        // VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
        let variable_declaration_tuple_parser_unmapped = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
            )
            .then(ignore_parser.clone())
            .then(variable_declaration_parser.clone())
            .then(ignore_parser.clone())
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(variable_declaration_parser.clone().or_not())
                    .map(|v| Box::new(variable_declaration_tuple::_T3::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(variable_declaration_tuple::_T0::new(v)));
        let variable_declaration_tuple_parser = variable_declaration_tuple_parser_unmapped.boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser_unmapped = terminal("while")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(statement_parser.clone())
            .map(|v| Box::new(while_statement::_T0::new(v)));
        let while_statement_parser = while_statement_parser_unmapped.boxed();

        // VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
        let variable_declaration_statement_parser_unmapped = choice((
            variable_declaration_parser
                .clone()
                .then(
                    just('=')
                        .map(|_| FixedTerminal::<1>())
                        .then(expression_parser.clone())
                        .map(|v| Box::new(variable_declaration_statement::_T3::new(v)))
                        .or_not(),
                )
                .map(|v| Box::new(variable_declaration_statement::_T2::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_T1::_T2(v))),
            variable_declaration_tuple_parser
                .clone()
                .then(just('=').map(|_| FixedTerminal::<1>()))
                .then(expression_parser.clone())
                .map(|v| Box::new(variable_declaration_statement::_T4::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_T1::_T4(v))),
        ))
        .then(ignore_parser.clone())
        .then(just(';').map(|_| FixedTerminal::<1>()))
        .map(|v| Box::new(variable_declaration_statement::_T0::new(v)));
        let variable_declaration_statement_parser =
            variable_declaration_statement_parser_unmapped.boxed();

        // SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser_unmapped = choice((
            variable_declaration_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_T0::VariableDeclarationStatement(v))),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_T0::ExpressionStatement(v))),
        ));
        let simple_statement_parser = simple_statement_parser_unmapped.boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser_unmapped = terminal("for")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(choice((
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_T1::SimpleStatement(v))),
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(for_statement::_T1::SemicolonChar(v))),
            )))
            .then(ignore_parser.clone())
            .then(choice((
                expression_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_T2::ExpressionStatement(v))),
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(for_statement::_T2::SemicolonChar(v))),
            )))
            .then(ignore_parser.clone())
            .then(expression_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(statement_parser.clone())
            .map(|v| Box::new(for_statement::_T0::new(v)));
        let for_statement_parser = for_statement_parser_unmapped.boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
        statement_parser_unmapped.define(choice((
            block_parser
                .clone()
                .map(|v| Box::new(statement::_T0::Block(v))),
            simple_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::SimpleStatement(v))),
            if_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::IfStatement(v))),
            for_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::ForStatement(v))),
            while_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::WhileStatement(v))),
            do_while_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::DoWhileStatement(v))),
            continue_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::ContinueStatement(v))),
            break_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::BreakStatement(v))),
            try_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::TryStatement(v))),
            return_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::ReturnStatement(v))),
            emit_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::EmitStatement(v))),
            revert_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::RevertStatement(v))),
            assembly_statement_parser
                .clone()
                .map(|v| Box::new(statement::_T0::AssemblyStatement(v))),
        )));
        statement_parser.define(statement_parser_unmapped.boxed());

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser_unmapped.define(
            just('{')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::_T2::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::_T2::UncheckedBlock(v))),
                    ))
                    .repeated(),
                )
                .then(ignore_parser.clone())
                .then(just('}').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(block::_T0::new(v))),
        );
        block_parser.define(block_parser_unmapped.boxed());

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser_unmapped = terminal("constructor")
            .ignored()
            .map(|_| FixedTerminal::<11usize>())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(constructor_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(constructor_definition::_T0::new(v)));
        let constructor_definition_parser = constructor_definition_parser_unmapped.boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser_unmapped = terminal("fallback")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(fallback_function_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(fallback_function_definition::_T2::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(fallback_function_definition::_T3::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(fallback_function_definition::_T3::Block(v))),
            )))
            .map(|v| Box::new(fallback_function_definition::_T0::new(v)));
        let fallback_function_definition_parser =
            fallback_function_definition_parser_unmapped.boxed();

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let function_definition_parser_unmapped = terminal("function")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(choice((
                identifier_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_T1::Identifier(v))),
                choice::<_, ErrorType>((
                    terminal("fallback").map(|_| 8usize),
                    terminal("receive").map(|_| 7usize),
                ))
                .map(|v| Box::new(function_definition::_T1::_1(v))),
            )))
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(function_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_definition::_T3::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(function_definition::_T4::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_T4::Block(v))),
            )))
            .map(|v| Box::new(function_definition::_T0::new(v)));
        let function_definition_parser = function_definition_parser_unmapped.boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
        let modifier_definition_parser_unmapped = terminal("modifier")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(method_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(choice((
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(modifier_definition::_T2::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(modifier_definition::_T2::Block(v))),
            )))
            .map(|v| Box::new(modifier_definition::_T0::new(v)));
        let modifier_definition_parser = modifier_definition_parser_unmapped.boxed();

        // ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser_unmapped = terminal("receive")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(receive_function_attribute_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(choice((
                just(';')
                    .map(|_| FixedTerminal::<1>())
                    .map(|v| Box::new(receive_function_definition::_T2::SemicolonChar(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(receive_function_definition::_T2::Block(v))),
            )))
            .map(|v| Box::new(receive_function_definition::_T0::new(v)));
        let receive_function_definition_parser =
            receive_function_definition_parser_unmapped.boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        let contract_body_element_parser_unmapped = choice((
            using_directive_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::UsingDirective(v))),
            constructor_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ConstructorDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::FunctionDefinition(v))),
            fallback_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::FallbackFunctionDefinition(v))),
            receive_function_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ReceiveFunctionDefinition(v))),
            modifier_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ModifierDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::EnumDefinition(v))),
            user_defined_value_type_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::_T0::UserDefinedValueTypeDefinition(
                    v,
                ))
            }),
            event_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::EventDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::ErrorDefinition(v))),
            state_variable_declaration_parser
                .clone()
                .map(|v| Box::new(contract_body_element::_T0::StateVariableDeclaration(v))),
        ));
        let contract_body_element_parser = contract_body_element_parser_unmapped.boxed();

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser_unmapped = terminal("abstract")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .or_not()
            .then(ignore_parser.clone())
            .then(
                terminal("contract")
                    .ignored()
                    .map(|_| FixedTerminal::<8usize>()),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(inheritance_specifier_list_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(contract_body_element_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(contract_definition::_T0::new(v)));
        let contract_definition_parser = contract_definition_parser_unmapped.boxed();

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser_unmapped = terminal("interface")
            .ignored()
            .map(|_| FixedTerminal::<9usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(inheritance_specifier_list_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(contract_body_element_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(interface_definition::_T0::new(v)));
        let interface_definition_parser = interface_definition_parser_unmapped.boxed();

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser_unmapped = terminal("library")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(contract_body_element_parser.clone().repeated())
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(library_definition::_T0::new(v)));
        let library_definition_parser = library_definition_parser_unmapped.boxed();

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        let definition_parser_unmapped = choice((
            contract_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ContractDefinition(v))),
            interface_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::InterfaceDefinition(v))),
            library_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::LibraryDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::FunctionDefinition(v))),
            constant_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ConstantDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::EnumDefinition(v))),
            user_defined_value_type_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::UserDefinedValueTypeDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(definition::_T0::ErrorDefinition(v))),
        ));
        let definition_parser = definition_parser_unmapped.boxed();

        // SourceUnit = «IGNORE» { Directive | Definition } $ ;
        let source_unit_parser_unmapped = ignore_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_T2::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_T2::Definition(v))),
                ))
                .repeated(),
            )
            .then(ignore_parser.clone())
            .then(end())
            .map(|v| Box::new(source_unit::_T0::new(v)));
        let source_unit_parser = source_unit_parser_unmapped.boxed();

        Self {
            ascii_escape: ascii_escape_parser.boxed(),
            boolean_literal: boolean_literal_parser.boxed(),
            comment: comment_parser.boxed(),
            decimal_integer: decimal_integer_parser.boxed(),
            fixed_bytes_type: fixed_bytes_type_parser.boxed(),
            fixed_type: fixed_type_parser.boxed(),
            hex_character: hex_character_parser.boxed(),
            identifier_start: identifier_start_parser.boxed(),
            line_comment: line_comment_parser.boxed(),
            number_unit: number_unit_parser.boxed(),
            pragma_directive: pragma_directive_parser.boxed(),
            reserved_keyword: reserved_keyword_parser.boxed(),
            signed_integer_type: signed_integer_type_parser.boxed(),
            whitespace: whitespace_parser.boxed(),
            yul_decimal_number_literal: yul_decimal_number_literal_parser.boxed(),
            yul_evm_builtin_function_name: yul_evm_builtin_function_name_parser.boxed(),
            yul_hex_literal: yul_hex_literal_parser.boxed(),
            yul_keyword: yul_keyword_parser.boxed(),
            decimal_exponent: decimal_exponent_parser.boxed(),
            decimal_float: decimal_float_parser.boxed(),
            hex_byte_escape: hex_byte_escape_parser.boxed(),
            hex_number: hex_number_parser.boxed(),
            ignore: ignore_parser.boxed(),
            identifier_part: identifier_part_parser.boxed(),
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser
                .boxed(),
            ufixed_type: ufixed_type_parser.boxed(),
            unicode_escape: unicode_escape_parser.boxed(),
            unsigned_integer_type: unsigned_integer_type_parser.boxed(),
            yul_reserved_word: yul_reserved_word_parser.boxed(),
            add_sub_operator: add_sub_operator_parser.boxed(),
            assignment_operator: assignment_operator_parser.boxed(),
            break_statement: break_statement_parser.boxed(),
            continue_statement: continue_statement_parser.boxed(),
            data_location: data_location_parser.boxed(),
            decimal_number: decimal_number_parser.boxed(),
            elementary_type: elementary_type_parser.boxed(),
            equality_comparison_operator: equality_comparison_operator_parser.boxed(),
            escape_sequence: escape_sequence_parser.boxed(),
            hex_string_literal: hex_string_literal_parser.boxed(),
            keyword: keyword_parser.boxed(),
            mul_div_mod_operator: mul_div_mod_operator_parser.boxed(),
            order_comparison_operator: order_comparison_operator_parser.boxed(),
            positional_argument_list: positional_argument_list_parser.boxed(),
            raw_identifier: raw_identifier_parser.boxed(),
            shift_operator: shift_operator_parser.boxed(),
            state_mutability_specifier: state_mutability_specifier_parser.boxed(),
            unary_prefix_operator: unary_prefix_operator_parser.boxed(),
            unary_suffix_operator: unary_suffix_operator_parser.boxed(),
            unchecked_block: unchecked_block_parser.boxed(),
            visibility_specifier: visibility_specifier_parser.boxed(),
            yul_break_statement: yul_break_statement_parser.boxed(),
            yul_continue_statement: yul_continue_statement_parser.boxed(),
            yul_leave_statement: yul_leave_statement_parser.boxed(),
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
            expression: expression_parser.boxed(),
            fallback_function_attribute: fallback_function_attribute_parser.boxed(),
            function_attribute: function_attribute_parser.boxed(),
            inheritance_specifier_list: inheritance_specifier_list_parser.boxed(),
            receive_function_attribute: receive_function_attribute_parser.boxed(),
            struct_definition: struct_definition_parser.boxed(),
            using_directive: using_directive_parser.boxed(),
            variable_declaration: variable_declaration_parser.boxed(),
            yul_block: yul_block_parser.boxed(),
            assembly_statement: assembly_statement_parser.boxed(),
            constant_definition: constant_definition_parser.boxed(),
            directive: directive_parser.boxed(),
            do_while_statement: do_while_statement_parser.boxed(),
            emit_statement: emit_statement_parser.boxed(),
            error_definition: error_definition_parser.boxed(),
            event_definition: event_definition_parser.boxed(),
            expression_statement: expression_statement_parser.boxed(),
            if_statement: if_statement_parser.boxed(),
            return_statement: return_statement_parser.boxed(),
            revert_statement: revert_statement_parser.boxed(),
            state_variable_declaration: state_variable_declaration_parser.boxed(),
            try_statement: try_statement_parser.boxed(),
            variable_declaration_tuple: variable_declaration_tuple_parser.boxed(),
            while_statement: while_statement_parser.boxed(),
            variable_declaration_statement: variable_declaration_statement_parser.boxed(),
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
