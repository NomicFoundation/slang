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
        // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
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
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «BooleanLiteral» = 'true' | 'false' ;
        let boolean_literal_parser = choice::<_, ErrorType>((
            terminal("false").map(|_| 5usize),
            terminal("true").map(|_| 4usize),
        ))
        .boxed();

        // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let comment_parser = terminal("/*")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedTerminal::<1>())
                        .map(|v| Box::new(comment::Comment::NotStarChar(v))),
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .then(
                            filter(|&c: &char| c != '*' && c != '/').map(|_| FixedTerminal::<1>()),
                        )
                        .map(|v| comment::_T2::new(v))
                        .map(|v| Box::new(comment::Comment::_T2(v))),
                ))
                .repeated()
                .then(
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .map(|v| v.len()),
                )
                .map(|v| comment::Content::new(v)),
            )
            .then(terminal("*/").ignored().map(|_| FixedTerminal::<2usize>()))
            .map(|v| comment::_T0::new(v))
            .boxed();

        // «DecimalInteger» = 1…*{ '0'…'9' / [ '_' ] } ;
        let decimal_integer_parser = filter(|&c: &char| ('0' <= c && c <= '9'))
            .map(|_| FixedTerminal::<1>())
            .then(
                just('_')
                    .map(|_| FixedTerminal::<1>())
                    .or_not()
                    .then(filter(|&c: &char| ('0' <= c && c <= '9')).map(|_| FixedTerminal::<1>()))
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(decimal_integer::_T0::new(v)))
            .boxed();

        // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
        let fixed_bytes_type_parser = terminal("bytes")
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
            .map(|v| fixed_bytes_type::_T0::new(v))
            .boxed();

        // «FixedType» = 'fixed' [ '1'…'9' { '0'…'9' } 'x' '1'…'9' { '0'…'9' } ] ;
        let fixed_type_parser = terminal("fixed")
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
                    .map(|v| fixed_type::_T1::new(v))
                    .or_not(),
            )
            .map(|v| fixed_type::_T0::new(v))
            .boxed();

        // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
        let hex_character_parser = filter(|&c: &char| {
            ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
        })
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
        let identifier_start_parser = filter(|&c: &char| {
            c == '_' || c == '$' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
        })
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «LineComment» = '//' { ¬( '\u{a}' | '\u{d}' ) } ;
        let line_comment_parser = terminal("//")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                filter(|&c: &char| c != '\n' && c != '\r')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
            )
            .map(|v| line_comment::_T0::new(v))
            .boxed();

        // «NumberUnit» = 'wei' | 'gwei' | 'ether' | 'seconds' | 'minutes' | 'hours' | 'days' | 'weeks' | 'years' ;
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

        // «PragmaDirective» = 'pragma' 1…*{ ¬';' } ';' ;
        let pragma_directive_parser = terminal("pragma")
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
            .map(|v| pragma_directive::_T0::new(v))
            .boxed();

        // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
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

        // «SignedIntegerType» = 'int' ( '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ) ;
        let signed_integer_type_parser = terminal("int")
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
            .map(|v| signed_integer_type::_T0::new(v))
            .boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
        let whitespace_parser = filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n')
            .map(|_| FixedTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| v.len())
            .boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser = choice((
            just('0').map(|_| FixedTerminal::<1>()).map(|v| {
                Box::new(yul_decimal_number_literal::YulDecimalNumberLiteral::ZeroChar(v))
            }),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .map(|_| FixedTerminal::<1>())
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .map(|v| v.len()),
                )
                .map(|v| yul_decimal_number_literal::_T0::new(v))
                .map(|v| Box::new(yul_decimal_number_literal::YulDecimalNumberLiteral::_T0(v))),
        ))
        .boxed();

        // «YulEVMBuiltinFunctionName» = 'stop' | 'add' | 'sub' | 'mul' | 'div' | 'sdiv' | 'mod' | 'smod' | 'exp' | 'not' | 'lt' | 'gt' | 'slt' | 'sgt' | 'eq' | 'iszero' | 'and' | 'or' | 'xor' | 'byte' | 'shl' | 'shr' | 'sar' | 'addmod' | 'mulmod' | 'signextend' | 'keccak256' | 'pop' | 'mload' | 'mstore' | 'mstore8' | 'sload' | 'sstore' | 'msize' | 'gas' | 'address' | 'balance' | 'selfbalance' | 'caller' | 'callvalue' | 'calldataload' | 'calldatasize' | 'calldatacopy' | 'extcodesize' | 'extcodecopy' | 'returndatasize' | 'returndatacopy' | 'extcodehash' | 'create' | 'create2' | 'call' | 'callcode' | 'delegatecall' | 'staticcall' | 'return' | 'revert' | 'selfdestruct' | 'invalid' | 'log0' | 'log1' | 'log2' | 'log3' | 'log4' | 'chainid' | 'origin' | 'gasprice' | 'Blockhash' | 'coinbase' | 'timestamp' | 'number' | 'difficulty' | 'gaslimit' | 'basefee' ;
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

        // «YulHexLiteral» = '0x' 1…*{ '0'…'9' | 'a'…'f' | 'A'…'F' } ;
        let yul_hex_literal_parser = terminal("0x")
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
            .map(|v| yul_hex_literal::_T0::new(v))
            .boxed();

        // «YulKeyword» = 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'if' | 'leave' | 'let' | 'switch' | 'hex' ;
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

        // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
        let decimal_exponent_parser = filter(|&c: &char| c == 'e' || c == 'E')
            .map(|_| FixedTerminal::<1>())
            .then(just('-').map(|_| FixedTerminal::<1>()).or_not())
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_exponent::_T0::new(v))
            .boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.').map(|_| FixedTerminal::<1>()))
            .then(decimal_integer_parser.clone())
            .map(|v| decimal_float::_T0::new(v))
            .boxed();

        // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
        let hex_byte_escape_parser = just('x')
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
            .map(|v| hex_byte_escape::_T0::new(v))
            .boxed();

        // «HexNumber» = '0' 'x' 1…*{ «HexCharacter» / [ '_' ] } ;
        let hex_number_parser = just('0')
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
            .map(|v| hex_number::_T0::new(v))
            .boxed();

        // «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
        let ignore_parser = choice((
            whitespace_parser
                .clone()
                .map(|v| Box::new(ignore::Ignore::Whitespace(v))),
            comment_parser
                .clone()
                .map(|v| Box::new(ignore::Ignore::Comment(v))),
            line_comment_parser
                .clone()
                .map(|v| Box::new(ignore::Ignore::LineComment(v))),
        ))
        .repeated()
        .boxed();

        // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
        let identifier_part_parser = filter(|&c: &char| {
            c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')
                || ('0' <= c && c <= '9')
        })
        .map(|_| FixedTerminal::<1>())
        .boxed();

        // «PossiblySeparatedPairsOfHexDigits» = 1…*{ 2…2*{ «HexCharacter» } / [ '_' ] } ;
        let possibly_separated_pairs_of_hex_digits_parser = filter(|&c: &char| {
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
        .map(|v| Box::new(possibly_separated_pairs_of_hex_digits::_T0::new(v)))
        .boxed();

        // «UfixedType» = 'u' «FixedType» ;
        let ufixed_type_parser = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(fixed_type_parser.clone())
            .map(|v| ufixed_type::_T0::new(v))
            .boxed();

        // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
        let unicode_escape_parser = just('u')
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
            .map(|v| unicode_escape::_T0::new(v))
            .boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(signed_integer_type_parser.clone())
            .map(|v| unsigned_integer_type::_T0::new(v))
            .boxed();

        // «YulReservedWord» = «YulKeyword» | «YulEVMBuiltinFunctionName» | «BooleanLiteral» ;
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

        // AddSubExpression = Expression ( '+' | '-' ) Expression ;
        let add_sub_expression_parser = todo!().boxed();

        // AndExpression = Expression '&&' Expression ;
        let and_expression_parser = todo!().boxed();

        // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
        let assignment_expression_parser = todo!().boxed();

        // BitAndExpression = Expression '&' Expression ;
        let bit_and_expression_parser = todo!().boxed();

        // BitOrExpression = Expression '|' Expression ;
        let bit_or_expression_parser = todo!().boxed();

        // BitXOrExpression = Expression '^' Expression ;
        let bit_x_or_expression_parser = todo!().boxed();

        // BreakStatement = 'break' ';' ;
        let break_statement_parser = with_noise(
            terminal("break")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| break_statement::_T0::new(v))
        .boxed();

        // ConditionalExpression = Expression '?' Expression ':' Expression ;
        let conditional_expression_parser = todo!().boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser = with_noise(
            terminal("continue")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| continue_statement::_T0::new(v))
        .boxed();

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        let data_location_parser = with_noise(choice::<_, ErrorType>((
            terminal("calldata").map(|_| 8usize),
            terminal("memory").map(|_| 6usize),
            terminal("storage").map(|_| 7usize),
        )))
        .boxed();

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
        let decimal_number_parser = choice((
            decimal_integer_parser
                .clone()
                .map(|v| Box::new(decimal_number::DecimalNumber::DecimalInteger(v))),
            decimal_float_parser
                .clone()
                .map(|v| Box::new(decimal_number::DecimalNumber::DecimalFloat(v))),
        ))
        .then(decimal_exponent_parser.clone().or_not())
        .map(|v| decimal_number::_T0::new(v))
        .boxed();

        // ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
        let elementary_type_parser = choice((
            with_noise(choice::<_, ErrorType>((
                terminal("b").ignore_then(choice((
                    terminal("ool").map(|_| 4usize),
                    terminal("ytes").map(|_| 5usize),
                ))),
                terminal("string").map(|_| 6usize),
            )))
            .map(|v| Box::new(elementary_type::ElementaryType::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::FixedBytesType(v))),
            fixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::FixedType(v))),
            ufixed_type_parser
                .clone()
                .map(|v| Box::new(elementary_type::ElementaryType::UfixedType(v))),
        ))
        .boxed();

        // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
        let equality_comparison_expression_parser = todo!().boxed();

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser = just('\\')
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
                .map(|v| Box::new(escape_sequence::EscapeSequence::_0(v))),
                hex_byte_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::EscapeSequence::HexByteEscape(v))),
                unicode_escape_parser
                    .clone()
                    .map(|v| Box::new(escape_sequence::EscapeSequence::UnicodeEscape(v))),
            )))
            .map(|v| escape_sequence::_T0::new(v))
            .boxed();

        // ExponentiationExpression = Expression '**' Expression ;
        let exponentiation_expression_parser = todo!().boxed();

        // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
        let hex_string_literal_parser = terminal("hex")
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
                    .map(|v| hex_string_literal::_T1::new(v))
                    .map(|v| Box::new(hex_string_literal::HexStringLiteral::_T1(v))),
                just('\'')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('\'').map(|_| FixedTerminal::<1>()))
                    .map(|v| hex_string_literal::_T2::new(v))
                    .map(|v| Box::new(hex_string_literal::HexStringLiteral::_T2(v))),
            )))
            .map(|v| hex_string_literal::_T0::new(v))
            .boxed();

        // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
        let index_access_expression_parser = todo!().boxed();

        // «Keyword» = 'pragma' | 'abstract' | 'anonymous' | 'address' | 'as' | 'assembly' | 'bool' | 'break' | 'bytes' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | 'fixed' | 'ufixed' ;
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
            .map(|v| Box::new(keyword::Keyword::_0(v))),
            signed_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::SignedIntegerType(v))),
            unsigned_integer_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::UnsignedIntegerType(v))),
            fixed_bytes_type_parser
                .clone()
                .map(|v| Box::new(keyword::Keyword::FixedBytesType(v))),
            choice::<_, ErrorType>((
                terminal("fixed").map(|_| 5usize),
                terminal("ufixed").map(|_| 6usize),
            ))
            .map(|v| Box::new(keyword::Keyword::_4(v))),
        ))
        .boxed();

        // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
        let mul_div_mod_expression_parser = todo!().boxed();

        // OrExpression = Expression '||' Expression ;
        let or_expression_parser = todo!().boxed();

        // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
        let order_comparison_expression_parser = todo!().boxed();

        // PositionalArgumentList = 1…*{ Expression / ',' } ;
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(
                with_noise(just(',').map(|_| FixedTerminal::<1>()))
                    .then(expression_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(positional_argument_list::_T0::new(v)))
            .boxed();

        // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
        let raw_identifier_parser = filter(|&c: &char| {
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
        .map(|v| raw_identifier::_T0::new(v))
        .boxed();

        // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
        let shift_expression_parser = todo!().boxed();

        // StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;
        let state_mutability_specifier_parser = with_noise(choice::<_, ErrorType>((
            terminal("p").ignore_then(choice((
                terminal("ayable").map(|_| 7usize),
                terminal("ure").map(|_| 4usize),
            ))),
            terminal("view").map(|_| 4usize),
        )))
        .boxed();

        // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | 'delete' | '-' ) Expression ;
        let unary_prefix_expression_parser = todo!().boxed();

        // UnarySuffixExpression = Expression ( '++' | '--' ) ;
        let unary_suffix_expression_parser = todo!().boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser = with_noise(
            terminal("unchecked")
                .ignored()
                .map(|_| FixedTerminal::<9usize>()),
        )
        .then(block_parser.clone())
        .map(|v| unchecked_block::_T0::new(v))
        .boxed();

        // VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;
        let visibility_specifier_parser = with_noise(choice::<_, ErrorType>((
            terminal("external").map(|_| 8usize),
            terminal("internal").map(|_| 8usize),
            terminal("p").ignore_then(choice((
                terminal("rivate").map(|_| 7usize),
                terminal("ublic").map(|_| 6usize),
            ))),
        )))
        .boxed();

        // YulBreakStatement = 'break' ;
        let yul_break_statement_parser = with_noise(
            terminal("break")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .boxed();

        // YulContinueStatement = 'continue' ;
        let yul_continue_statement_parser = with_noise(
            terminal("continue")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .boxed();

        // YulLeaveStatement = 'leave' ;
        let yul_leave_statement_parser = with_noise(
            terminal("leave")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser = just ('"') . map (| _ | FixedTerminal :: < 1 > ()) . then (choice ((filter (| & c : & char | (' ' <= c && c <= '~') && c != '"' && c != '\\') . map (| _ | FixedTerminal :: < 1 > ()) . repeated () . at_least (1usize) . map (| v | v . len ()) . map (| v | Box :: new (double_quoted_ascii_string_literal :: DoubleQuotedAsciiStringLiteral :: Chars (v))) , escape_sequence_parser . clone () . map (| v | Box :: new (double_quoted_ascii_string_literal :: DoubleQuotedAsciiStringLiteral :: EscapeSequence (v))))) . repeated ()) . then (just ('"') . map (| _ | FixedTerminal :: < 1 > ())) . map (| v | double_quoted_ascii_string_literal :: _T0 :: new (v)) . boxed () ;

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser = terminal ("unicode\"") . ignored () . map (| _ | FixedTerminal :: < 8usize > ()) . then (choice ((filter (| & c : & char | c != '"' && c != '\\' && c != '\n' && c != '\r') . map (| _ | FixedTerminal :: < 1 > ()) . repeated () . at_least (1usize) . map (| v | v . len ()) . map (| v | Box :: new (double_quoted_unicode_string_literal :: DoubleQuotedUnicodeStringLiteral :: Chars (v))) , escape_sequence_parser . clone () . map (| v | Box :: new (double_quoted_unicode_string_literal :: DoubleQuotedUnicodeStringLiteral :: EscapeSequence (v))))) . repeated ()) . then (just ('"') . map (| _ | FixedTerminal :: < 1 > ())) . map (| v | double_quoted_unicode_string_literal :: _T0 :: new (v)) . boxed () ;

        // ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
        let elementary_type_with_payable_parser = choice((
            with_noise(
                terminal("address")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .then(
                with_noise(
                    terminal("payable")
                        .ignored()
                        .map(|_| FixedTerminal::<7usize>()),
                )
                .or_not(),
            )
            .map(|v| elementary_type_with_payable::_T0::new(v))
            .map(|v| Box::new(elementary_type_with_payable::ElementaryTypeWithPayable::_T0(v))),
            elementary_type_parser.clone().map(|v| {
                Box::new(elementary_type_with_payable::ElementaryTypeWithPayable::ElementaryType(v))
            }),
        ))
        .boxed();

        // ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
        let elementary_type_without_payable_parser = choice((
            with_noise(
                terminal("address")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .map(|v| {
                Box::new(elementary_type_without_payable::ElementaryTypeWithoutPayable::Address(v))
            }),
            elementary_type_parser.clone().map(|v| {
                Box::new(
                    elementary_type_without_payable::ElementaryTypeWithoutPayable::ElementaryType(
                        v,
                    ),
                )
            }),
        ))
        .boxed();

        // NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::NumericLiteral::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::NumericLiteral::HexNumber(v))),
        ))
        .then(
            with_noise(choice::<_, ErrorType>((
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
            )))
            .or_not(),
        )
        .map(|v| numeric_literal::_T0::new(v))
        .boxed();

        // «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
        let reserved_word_parser = choice((
            keyword_parser
                .clone()
                .map(|v| Box::new(reserved_word::ReservedWord::Keyword(v))),
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
            .map(|v| Box::new(reserved_word::ReservedWord::_1(v))),
        ))
        .boxed();

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser = just ('\'') . map (| _ | FixedTerminal :: < 1 > ()) . then (choice ((filter (| & c : & char | (' ' <= c && c <= '~') && c != '\'' && c != '\\') . map (| _ | FixedTerminal :: < 1 > ()) . repeated () . at_least (1usize) . map (| v | v . len ()) . map (| v | Box :: new (single_quoted_ascii_string_literal :: SingleQuotedAsciiStringLiteral :: Chars (v))) , escape_sequence_parser . clone () . map (| v | Box :: new (single_quoted_ascii_string_literal :: SingleQuotedAsciiStringLiteral :: EscapeSequence (v))))) . repeated ()) . then (just ('\'') . map (| _ | FixedTerminal :: < 1 > ())) . map (| v | single_quoted_ascii_string_literal :: _T0 :: new (v)) . boxed () ;

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser = terminal ("unicode'") . ignored () . map (| _ | FixedTerminal :: < 8usize > ()) . then (choice ((filter (| & c : & char | c != '\'' && c != '\\' && c != '\n' && c != '\r') . map (| _ | FixedTerminal :: < 1 > ()) . repeated () . at_least (1usize) . map (| v | v . len ()) . map (| v | Box :: new (single_quoted_unicode_string_literal :: SingleQuotedUnicodeStringLiteral :: Chars (v))) , escape_sequence_parser . clone () . map (| v | Box :: new (single_quoted_unicode_string_literal :: SingleQuotedUnicodeStringLiteral :: EscapeSequence (v))))) . repeated ()) . then (just ('\'') . map (| _ | FixedTerminal :: < 1 > ())) . map (| v | single_quoted_unicode_string_literal :: _T0 :: new (v)) . boxed () ;

        // «YulIdentifier» = «RawIdentifier» - «YulReservedWord» ;
        let yul_identifier_parser = difference(
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
        )
        .boxed();

        // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
        let ascii_string_literal_parser = choice((
            single_quoted_ascii_string_literal_parser.clone().map(|v| {
                Box::new(
                    ascii_string_literal::AsciiStringLiteral::SingleQuotedAsciiStringLiteral(v),
                )
            }),
            double_quoted_ascii_string_literal_parser.clone().map(|v| {
                Box::new(
                    ascii_string_literal::AsciiStringLiteral::DoubleQuotedAsciiStringLiteral(v),
                )
            }),
        ))
        .boxed();

        // AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
        let assembly_flags_parser = with_noise(just('(').map(|_| FixedTerminal::<1>()))
            .then(
                double_quoted_ascii_string_literal_parser
                    .clone()
                    .then(
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(double_quoted_ascii_string_literal_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(assembly_flags::_T1::new(v))),
            )
            .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
            .map(|v| assembly_flags::_T0::new(v))
            .boxed();

        // «Identifier» = «RawIdentifier» - «ReservedWord» ;
        let identifier_parser =
            difference(raw_identifier_parser.clone(), reserved_word_parser.clone()).boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
        let unicode_string_literal_parser = choice ((single_quoted_unicode_string_literal_parser . clone () . map (| v | Box :: new (unicode_string_literal :: UnicodeStringLiteral :: SingleQuotedUnicodeStringLiteral (v))) , double_quoted_unicode_string_literal_parser . clone () . map (| v | Box :: new (unicode_string_literal :: UnicodeStringLiteral :: DoubleQuotedUnicodeStringLiteral (v))))) . boxed () ;

        // YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
        let yul_function_call_parser = choice((
            yul_identifier_parser
                .clone()
                .map(|v| Box::new(yul_function_call::YulFunctionCall::YulIdentifier(v))),
            with_noise(choice::<_, ErrorType>((
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
            )))
            .map(|v| Box::new(yul_function_call::YulFunctionCall::_1(v))),
        ))
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(
            yul_expression_parser
                .clone()
                .then(
                    with_noise(just(',').map(|_| FixedTerminal::<1>()))
                        .then(yul_expression_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(yul_function_call::_T1::new(v)))
                .or_not(),
        )
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .map(|v| yul_function_call::_T0::new(v))
        .boxed();

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
        let yul_function_definition_parser = with_noise(
            terminal("function")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(yul_identifier_parser.clone())
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(
            yul_identifier_parser
                .clone()
                .then(
                    with_noise(just(',').map(|_| FixedTerminal::<1>()))
                        .then(yul_identifier_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(yul_function_definition::_T1::new(v)))
                .or_not(),
        )
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .then(
            with_noise(terminal("->").ignored().map(|_| FixedTerminal::<2usize>()))
                .then(
                    yul_identifier_parser
                        .clone()
                        .then(
                            with_noise(just(',').map(|_| FixedTerminal::<1>()))
                                .then(yul_identifier_parser.clone())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(yul_function_definition::_T3::new(v))),
                )
                .map(|v| yul_function_definition::_T2::new(v))
                .or_not(),
        )
        .then(yul_block_parser.clone())
        .map(|v| yul_function_definition::_T0::new(v))
        .boxed();

        // YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
        let yul_path_parser = yul_identifier_parser
            .clone()
            .then(
                with_noise(just('.').map(|_| FixedTerminal::<1>()))
                    .then(choice((
                        yul_identifier_parser
                            .clone()
                            .map(|v| Box::new(yul_path::YulPath::YulIdentifier(v))),
                        with_noise(choice::<_, ErrorType>((
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
                        )))
                        .map(|v| Box::new(yul_path::YulPath::_1(v))),
                    )))
                    .map(|v| yul_path::_T2::new(v))
                    .repeated(),
            )
            .map(|v| yul_path::_T0::new(v))
            .boxed();

        // EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
        let enum_definition_parser = with_noise(
            terminal("enum")
                .ignored()
                .map(|_| FixedTerminal::<4usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(just('{').map(|_| FixedTerminal::<1>())))
        .then(
            identifier_parser
                .clone()
                .then(
                    with_noise(just(',').map(|_| FixedTerminal::<1>()))
                        .then(identifier_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(enum_definition::_T1::new(v))),
        )
        .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
        .map(|v| enum_definition::_T0::new(v))
        .boxed();

        // IdentifierPath = 1…*{ «Identifier» / '.' } ;
        let identifier_path_parser = identifier_parser
            .clone()
            .then(
                with_noise(just('.').map(|_| FixedTerminal::<1>()))
                    .then(identifier_parser.clone())
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(identifier_path::_T0::new(v)))
            .boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser = ascii_string_literal_parser.clone().boxed();

        // Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
        let literal_parser = choice((
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::Literal::AsciiStringLiteral(v))),
            unicode_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::Literal::UnicodeStringLiteral(v))),
            numeric_literal_parser
                .clone()
                .map(|v| Box::new(literal::Literal::NumericLiteral(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(literal::Literal::HexStringLiteral(v))),
            with_noise(choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            )))
            .map(|v| Box::new(literal::Literal::_4(v))),
        ))
        .boxed();

        // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
        let member_access_expression_parser = todo!().boxed();

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser = identifier_parser
            .clone()
            .then(with_noise(just(':').map(|_| FixedTerminal::<1>())))
            .then(expression_parser.clone())
            .map(|v| named_argument::_T0::new(v))
            .boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser = type_name_parser
            .clone()
            .then(
                with_noise(choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                )))
                .or_not(),
            )
            .then(identifier_parser.clone().or_not())
            .map(|v| parameter_declaration::_T0::new(v))
            .boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser = identifier_parser
            .clone()
            .then(
                with_noise(terminal("as").ignored().map(|_| FixedTerminal::<2usize>()))
                    .then(identifier_parser.clone())
                    .map(|v| selected_import::_T1::new(v))
                    .or_not(),
            )
            .map(|v| selected_import::_T0::new(v))
            .boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
        let user_defined_value_type_definition_parser = with_noise(
            terminal("type")
                .ignored()
                .map(|_| FixedTerminal::<4usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(
            terminal("is").ignored().map(|_| FixedTerminal::<2usize>()),
        ))
        .then(elementary_type_with_payable_parser.clone())
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| user_defined_value_type_definition::_T0::new(v))
        .boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
        let yul_literal_parser = choice((
            yul_decimal_number_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::YulLiteral::YulDecimalNumberLiteral(v))),
            yul_hex_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::YulLiteral::YulHexLiteral(v))),
            ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::YulLiteral::AsciiStringLiteral(v))),
            with_noise(choice::<_, ErrorType>((
                terminal("false").map(|_| 5usize),
                terminal("true").map(|_| 4usize),
            )))
            .map(|v| Box::new(yul_literal::YulLiteral::_3(v))),
            hex_string_literal_parser
                .clone()
                .map(|v| Box::new(yul_literal::YulLiteral::HexStringLiteral(v))),
        ))
        .boxed();

        // FunctionCallOptionsExpression = Expression '{' 1…*{ NamedArgument / ',' } '}' ;
        let function_call_options_expression_parser = todo!().boxed();

        // MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser = with_noise(
            terminal("mapping")
                .ignored()
                .map(|_| FixedTerminal::<7usize>()),
        )
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(choice((
            elementary_type_without_payable_parser
                .clone()
                .map(|v| Box::new(mapping_type::MappingType::ElementaryTypeWithoutPayable(v))),
            identifier_path_parser
                .clone()
                .map(|v| Box::new(mapping_type::MappingType::IdentifierPath(v))),
        )))
        .then(with_noise(
            terminal("=>").ignored().map(|_| FixedTerminal::<2usize>()),
        ))
        .then(type_name_parser.clone())
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .map(|v| mapping_type::_T0::new(v))
        .boxed();

        // NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
        let named_argument_list_parser = with_noise(just('{').map(|_| FixedTerminal::<1>()))
            .then(
                named_argument_parser
                    .clone()
                    .then(
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(named_argument_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(named_argument_list::_T1::new(v)))
                    .or_not(),
            )
            .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
            .map(|v| named_argument_list::_T0::new(v))
            .boxed();

        // NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
        let non_empty_parameter_list_parser = with_noise(just('(').map(|_| FixedTerminal::<1>()))
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(non_empty_parameter_list::_T1::new(v))),
            )
            .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
            .map(|v| non_empty_parameter_list::_T0::new(v))
            .boxed();

        // OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
        let override_specifier_parser = with_noise(
            terminal("override")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(
            with_noise(just('(').map(|_| FixedTerminal::<1>()))
                .then(
                    identifier_path_parser
                        .clone()
                        .then(
                            with_noise(just(',').map(|_| FixedTerminal::<1>()))
                                .then(identifier_path_parser.clone())
                                .repeated()
                                .at_most(1usize - 1),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(override_specifier::_T2::new(v))),
                )
                .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
                .map(|v| override_specifier::_T1::new(v))
                .or_not(),
        )
        .map(|v| override_specifier::_T0::new(v))
        .boxed();

        // ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
        let parameter_list_parser = with_noise(just('(').map(|_| FixedTerminal::<1>()))
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(parameter_declaration_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(parameter_list::_T1::new(v)))
                    .or_not(),
            )
            .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
            .map(|v| parameter_list::_T0::new(v))
            .boxed();

        // SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
        let selecting_import_directive_parser = with_noise(just('{').map(|_| FixedTerminal::<1>()))
            .then(
                selected_import_parser
                    .clone()
                    .then(
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(selected_import_parser.clone())
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(selecting_import_directive::_T1::new(v))),
            )
            .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
            .then(with_noise(
                terminal("from")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>()),
            ))
            .then(import_path_parser.clone())
            .map(|v| selecting_import_directive::_T0::new(v))
            .boxed();

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(
                with_noise(terminal("as").ignored().map(|_| FixedTerminal::<2usize>()))
                    .then(identifier_parser.clone())
                    .map(|v| simple_import_directive::_T2::new(v))
                    .repeated(),
            )
            .map(|v| simple_import_directive::_T0::new(v))
            .boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser = with_noise(just('*').map(|_| FixedTerminal::<1>()))
            .then(with_noise(
                terminal("as").ignored().map(|_| FixedTerminal::<2usize>()),
            ))
            .then(identifier_parser.clone())
            .then(with_noise(
                terminal("from")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>()),
            ))
            .then(import_path_parser.clone())
            .map(|v| star_import_directive::_T0::new(v))
            .boxed();

        // YulExpression = YulPath | YulFunctionCall | YulLiteral ;
        yul_expression_parser.define(
            choice((
                yul_path_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulPath(v))),
                yul_function_call_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulFunctionCall(v))),
                yul_literal_parser
                    .clone()
                    .map(|v| Box::new(yul_expression::YulExpression::YulLiteral(v))),
            ))
            .boxed(),
        );

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser = with_noise(just('(').map(|_| FixedTerminal::<1>()))
            .then(
                choice((
                    positional_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::ArgumentList::PositionalArgumentList(v))),
                    named_argument_list_parser
                        .clone()
                        .map(|v| Box::new(argument_list::ArgumentList::NamedArgumentList(v))),
                ))
                .or_not(),
            )
            .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
            .map(|v| argument_list::_T0::new(v))
            .boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
        let catch_clause_parser = with_noise(
            terminal("catch")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(
            identifier_parser
                .clone()
                .or_not()
                .then(non_empty_parameter_list_parser.clone())
                .map(|v| catch_clause::_T1::new(v))
                .or_not(),
        )
        .then(block_parser.clone())
        .map(|v| catch_clause::_T0::new(v))
        .boxed();

        // FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
        let function_type_parser = with_noise(
            terminal("function")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(parameter_list_parser.clone())
        .then(
            with_noise(choice::<_, ErrorType>((
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
            )))
            .repeated(),
        )
        .then(
            with_noise(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .then(non_empty_parameter_list_parser.clone())
            .map(|v| function_type::_T2::new(v))
            .or_not(),
        )
        .map(|v| function_type::_T0::new(v))
        .boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser = with_noise(
            terminal("import")
                .ignored()
                .map(|_| FixedTerminal::<6usize>()),
        )
        .then(choice((
            simple_import_directive_parser
                .clone()
                .map(|v| Box::new(import_directive::ImportDirective::SimpleImportDirective(v))),
            star_import_directive_parser
                .clone()
                .map(|v| Box::new(import_directive::ImportDirective::StarImportDirective(v))),
            selecting_import_directive_parser.clone().map(|v| {
                Box::new(import_directive::ImportDirective::SelectingImportDirective(
                    v,
                ))
            }),
        )))
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| import_directive::_T0::new(v))
        .boxed();

        // MethodAttribute = 'virtual' | OverrideSpecifier ;
        let method_attribute_parser = choice((
            with_noise(
                terminal("virtual")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .map(|v| Box::new(method_attribute::MethodAttribute::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(method_attribute::MethodAttribute::OverrideSpecifier(v))),
        ))
        .boxed();

        // StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
        let state_variable_attribute_parser = choice((
            with_noise(choice::<_, ErrorType>((
                terminal("constant").map(|_| 8usize),
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("rivate").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            )))
            .map(|v| Box::new(state_variable_attribute::StateVariableAttribute::_0(v))),
            override_specifier_parser.clone().map(|v| {
                Box::new(state_variable_attribute::StateVariableAttribute::OverrideSpecifier(v))
            }),
            with_noise(
                terminal("immutable")
                    .ignored()
                    .map(|_| FixedTerminal::<9usize>()),
            )
            .map(|v| {
                Box::new(state_variable_attribute::StateVariableAttribute::Immutable(
                    v,
                ))
            }),
        ))
        .boxed();

        // YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
        let yul_assignment_parser = yul_path_parser
            .clone()
            .then(choice((
                with_noise(terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()))
                    .then(yul_expression_parser.clone())
                    .map(|v| yul_assignment::_T1::new(v))
                    .map(|v| Box::new(yul_assignment::YulAssignment::_T1(v))),
                with_noise(just(',').map(|_| FixedTerminal::<1>()))
                    .then(yul_path_parser.clone())
                    .map(|v| yul_assignment::_T4::new(v))
                    .repeated()
                    .at_least(1usize)
                    .then(with_noise(
                        terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()),
                    ))
                    .then(yul_function_call_parser.clone())
                    .map(|v| yul_assignment::_T2::new(v))
                    .map(|v| Box::new(yul_assignment::YulAssignment::_T2(v))),
            )))
            .map(|v| yul_assignment::_T0::new(v))
            .boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser =
            with_noise(terminal("for").ignored().map(|_| FixedTerminal::<3usize>()))
                .then(yul_block_parser.clone())
                .then(yul_expression_parser.clone())
                .then(yul_block_parser.clone())
                .then(yul_block_parser.clone())
                .map(|v| yul_for_statement::_T0::new(v))
                .boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser =
            with_noise(terminal("if").ignored().map(|_| FixedTerminal::<2usize>()))
                .then(yul_expression_parser.clone())
                .then(yul_block_parser.clone())
                .map(|v| yul_if_statement::_T0::new(v))
                .boxed();

        // YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
        let yul_switch_statement_parser = with_noise(
            terminal("switch")
                .ignored()
                .map(|_| FixedTerminal::<6usize>()),
        )
        .then(yul_expression_parser.clone())
        .then(choice((
            with_noise(
                terminal("case")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>()),
            )
            .then(yul_literal_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| yul_switch_statement::_T3::new(v))
            .repeated()
            .at_least(1usize)
            .then(
                with_noise(
                    terminal("default")
                        .ignored()
                        .map(|_| FixedTerminal::<7usize>()),
                )
                .then(yul_block_parser.clone())
                .map(|v| yul_switch_statement::_T4::new(v))
                .or_not(),
            )
            .map(|v| yul_switch_statement::_T1::new(v))
            .map(|v| Box::new(yul_switch_statement::YulSwitchStatement::_T1(v))),
            with_noise(
                terminal("default")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .then(yul_block_parser.clone())
            .map(|v| yul_switch_statement::_T5::new(v))
            .map(|v| Box::new(yul_switch_statement::YulSwitchStatement::_T5(v))),
        )))
        .map(|v| yul_switch_statement::_T0::new(v))
        .boxed();

        // YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
        let yul_variable_declaration_parser =
            with_noise(terminal("let").ignored().map(|_| FixedTerminal::<3usize>()))
                .then(yul_identifier_parser.clone())
                .then(
                    choice((
                        with_noise(terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()))
                            .then(yul_expression_parser.clone())
                            .map(|v| yul_variable_declaration::_T1::new(v))
                            .map(|v| {
                                Box::new(yul_variable_declaration::YulVariableDeclaration::_T1(v))
                            }),
                        with_noise(just(',').map(|_| FixedTerminal::<1>()))
                            .then(yul_identifier_parser.clone())
                            .map(|v| yul_variable_declaration::_T3::new(v))
                            .or_not()
                            .then(
                                with_noise(
                                    terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()),
                                )
                                .then(yul_function_call_parser.clone())
                                .map(|v| yul_variable_declaration::_T4::new(v))
                                .or_not(),
                            )
                            .map(|v| yul_variable_declaration::_T2::new(v))
                            .map(|v| {
                                Box::new(yul_variable_declaration::YulVariableDeclaration::_T2(v))
                            }),
                    ))
                    .or_not(),
                )
                .map(|v| yul_variable_declaration::_T0::new(v))
                .boxed();

        // FunctionCallExpression = Expression ArgumentList ;
        let function_call_expression_parser = todo!().boxed();

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| inheritance_specifier::_T0::new(v))
            .boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(argument_list_parser.clone().or_not())
            .map(|v| modifier_invocation::_T0::new(v))
            .boxed();

        // TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
        type_name_parser.define(
            choice((
                elementary_type_with_payable_parser
                    .clone()
                    .map(|v| Box::new(type_name::TypeName::ElementaryTypeWithPayable(v))),
                function_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::TypeName::FunctionType(v))),
                mapping_type_parser
                    .clone()
                    .map(|v| Box::new(type_name::TypeName::MappingType(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(type_name::TypeName::IdentifierPath(v))),
            ))
            .then(
                with_noise(just('[').map(|_| FixedTerminal::<1>()))
                    .then(expression_parser.clone().or_not())
                    .then(with_noise(just(']').map(|_| FixedTerminal::<1>())))
                    .map(|v| type_name::_T2::new(v))
                    .repeated(),
            )
            .map(|v| type_name::_T0::new(v))
            .boxed(),
        );

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
        let yul_statement_parser = choice((
            yul_block_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulBlock(v))),
            yul_variable_declaration_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulVariableDeclaration(v))),
            yul_function_definition_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulFunctionDefinition(v))),
            yul_assignment_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulAssignment(v))),
            yul_function_call_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulFunctionCall(v))),
            yul_if_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulIfStatement(v))),
            yul_for_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulForStatement(v))),
            yul_switch_statement_parser
                .clone()
                .map(|v| Box::new(yul_statement::YulStatement::YulSwitchStatement(v))),
            with_noise(choice::<_, ErrorType>((
                terminal("break").map(|_| 5usize),
                terminal("continue").map(|_| 8usize),
                terminal("leave").map(|_| 5usize),
            )))
            .map(|v| Box::new(yul_statement::YulStatement::_8(v))),
        ))
        .boxed();

        // ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
        let constructor_attribute_parser = choice((
            modifier_invocation_parser.clone().map(|v| {
                Box::new(constructor_attribute::ConstructorAttribute::ModifierInvocation(v))
            }),
            with_noise(choice::<_, ErrorType>((
                terminal("internal").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ublic").map(|_| 6usize),
                ))),
            )))
            .map(|v| Box::new(constructor_attribute::ConstructorAttribute::_1(v))),
        ))
        .boxed();

        // ErrorParameter = TypeName [ «Identifier» ] ;
        let error_parameter_parser = type_name_parser
            .clone()
            .then(identifier_parser.clone().or_not())
            .map(|v| error_parameter::_T0::new(v))
            .boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser = type_name_parser
            .clone()
            .then(
                with_noise(
                    terminal("indexed")
                        .ignored()
                        .map(|_| FixedTerminal::<7usize>()),
                )
                .or_not(),
            )
            .then(identifier_parser.clone().or_not())
            .map(|v| event_parameter::_T0::new(v))
            .boxed();

        // FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let fallback_function_attribute_parser = choice((
            with_noise(choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("p").ignore_then(choice((
                    terminal("ayable").map(|_| 7usize),
                    terminal("ure").map(|_| 4usize),
                ))),
                terminal("view").map(|_| 4usize),
            )))
            .map(|v| {
                Box::new(fallback_function_attribute::FallbackFunctionAttribute::_0(
                    v,
                ))
            }),
            modifier_invocation_parser.clone().map(|v| {
                Box::new(
                    fallback_function_attribute::FallbackFunctionAttribute::ModifierInvocation(v),
                )
            }),
            with_noise(
                terminal("virtual")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .map(|v| Box::new(fallback_function_attribute::FallbackFunctionAttribute::Virtual(v))),
            override_specifier_parser.clone().map(|v| {
                Box::new(
                    fallback_function_attribute::FallbackFunctionAttribute::OverrideSpecifier(v),
                )
            }),
        ))
        .boxed();

        // FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let function_attribute_parser = choice((
            with_noise(choice::<_, ErrorType>((
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
            )))
            .map(|v| Box::new(function_attribute::FunctionAttribute::_0(v))),
            modifier_invocation_parser
                .clone()
                .map(|v| Box::new(function_attribute::FunctionAttribute::ModifierInvocation(v))),
            with_noise(
                terminal("virtual")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .map(|v| Box::new(function_attribute::FunctionAttribute::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::FunctionAttribute::OverrideSpecifier(v))),
        ))
        .boxed();

        // InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
        let inheritance_specifier_list_parser =
            with_noise(terminal("is").ignored().map(|_| FixedTerminal::<2usize>()))
                .then(
                    inheritance_specifier_parser
                        .clone()
                        .then(
                            with_noise(just(',').map(|_| FixedTerminal::<1>()))
                                .then(inheritance_specifier_parser.clone())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(inheritance_specifier_list::_T1::new(v))),
                )
                .map(|v| inheritance_specifier_list::_T0::new(v))
                .boxed();

        // PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
        let primary_expression_parser = todo!().boxed();

        // ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
        let receive_function_attribute_parser = choice((
            with_noise(choice::<_, ErrorType>((
                terminal("external").map(|_| 8usize),
                terminal("payable").map(|_| 7usize),
            )))
            .map(|v| Box::new(receive_function_attribute::ReceiveFunctionAttribute::_0(v))),
            modifier_invocation_parser.clone().map(|v| {
                Box::new(
                    receive_function_attribute::ReceiveFunctionAttribute::ModifierInvocation(v),
                )
            }),
            with_noise(
                terminal("virtual")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .map(|v| Box::new(receive_function_attribute::ReceiveFunctionAttribute::Virtual(v))),
            override_specifier_parser.clone().map(|v| {
                Box::new(receive_function_attribute::ReceiveFunctionAttribute::OverrideSpecifier(v))
            }),
        ))
        .boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
        let struct_definition_parser = with_noise(
            terminal("struct")
                .ignored()
                .map(|_| FixedTerminal::<6usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(just('{').map(|_| FixedTerminal::<1>())))
        .then(
            type_name_parser
                .clone()
                .then(identifier_parser.clone())
                .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
                .map(|v| struct_definition::_T2::new(v))
                .repeated()
                .at_least(1usize),
        )
        .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
        .map(|v| struct_definition::_T0::new(v))
        .boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser = with_noise(
            terminal("using")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(choice((
            identifier_path_parser
                .clone()
                .map(|v| Box::new(using_directive::UsingDirective::IdentifierPath(v))),
            with_noise(just('{').map(|_| FixedTerminal::<1>()))
                .then(
                    identifier_path_parser
                        .clone()
                        .then(
                            with_noise(just(',').map(|_| FixedTerminal::<1>()))
                                .then(identifier_path_parser.clone())
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(using_directive::_T2::new(v))),
                )
                .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
                .map(|v| using_directive::_T1::new(v))
                .map(|v| Box::new(using_directive::UsingDirective::_T1(v))),
        )))
        .then(with_noise(
            terminal("for").ignored().map(|_| FixedTerminal::<3usize>()),
        ))
        .then(choice((
            with_noise(just('*').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(using_directive::UsingDirective::StarChar(v))),
            type_name_parser
                .clone()
                .map(|v| Box::new(using_directive::UsingDirective::TypeName(v))),
        )))
        .then(
            with_noise(
                terminal("global")
                    .ignored()
                    .map(|_| FixedTerminal::<6usize>()),
            )
            .or_not(),
        )
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| using_directive::_T0::new(v))
        .boxed();

        // VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
        let variable_declaration_parser = type_name_parser
            .clone()
            .then(
                with_noise(choice::<_, ErrorType>((
                    terminal("calldata").map(|_| 8usize),
                    terminal("memory").map(|_| 6usize),
                    terminal("storage").map(|_| 7usize),
                )))
                .or_not(),
            )
            .then(identifier_parser.clone())
            .map(|v| variable_declaration::_T0::new(v))
            .boxed();

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser.define(
            with_noise(just('{').map(|_| FixedTerminal::<1>()))
                .then(yul_statement_parser.clone().repeated())
                .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
                .map(|v| yul_block::_T0::new(v))
                .boxed(),
        );

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser = with_noise(
            terminal("assembly")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(
            with_noise(
                terminal("\"evmasm\"")
                    .ignored()
                    .map(|_| FixedTerminal::<8usize>()),
            )
            .or_not(),
        )
        .then(assembly_flags_parser.clone().or_not())
        .then(yul_block_parser.clone())
        .map(|v| assembly_statement::_T0::new(v))
        .boxed();

        // Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
        let directive_parser = choice((
            pragma_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::PragmaDirective(v))),
            import_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::ImportDirective(v))),
            using_directive_parser
                .clone()
                .map(|v| Box::new(directive::Directive::UsingDirective(v))),
        ))
        .boxed();

        // ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
        let error_definition_parser = with_noise(
            terminal("error")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(
            error_parameter_parser
                .clone()
                .then(
                    with_noise(just(',').map(|_| FixedTerminal::<1>()))
                        .then(error_parameter_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(error_definition::_T1::new(v)))
                .or_not(),
        )
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| error_definition::_T0::new(v))
        .boxed();

        // EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
        let event_definition_parser = with_noise(
            terminal("event")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(
            event_parameter_parser
                .clone()
                .then(
                    with_noise(just(',').map(|_| FixedTerminal::<1>()))
                        .then(event_parameter_parser.clone())
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(event_definition::_T1::new(v)))
                .or_not(),
        )
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .then(
            with_noise(
                terminal("anonymous")
                    .ignored()
                    .map(|_| FixedTerminal::<9usize>()),
            )
            .or_not(),
        )
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| event_definition::_T0::new(v))
        .boxed();

        // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | FunctionCallOptionsExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
        expression_parser.define(todo!().boxed());

        // VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
        let variable_declaration_tuple_parser = with_noise(just('(').map(|_| FixedTerminal::<1>()))
            .then(
                with_noise(just(',').map(|_| FixedTerminal::<1>()))
                    .repeated()
                    .map(|v| v.len()),
            )
            .then(variable_declaration_parser.clone())
            .then(
                with_noise(just(',').map(|_| FixedTerminal::<1>()))
                    .then(variable_declaration_parser.clone().or_not())
                    .map(|v| variable_declaration_tuple::_T3::new(v))
                    .repeated(),
            )
            .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
            .map(|v| variable_declaration_tuple::_T0::new(v))
            .boxed();

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser = type_name_parser
            .clone()
            .then(with_noise(
                terminal("constant")
                    .ignored()
                    .map(|_| FixedTerminal::<8usize>()),
            ))
            .then(identifier_parser.clone())
            .then(with_noise(just('=').map(|_| FixedTerminal::<1>())))
            .then(expression_parser.clone())
            .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
            .map(|v| constant_definition::_T0::new(v))
            .boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser =
            with_noise(terminal("do").ignored().map(|_| FixedTerminal::<2usize>()))
                .then(statement_parser.clone())
                .then(with_noise(
                    terminal("while")
                        .ignored()
                        .map(|_| FixedTerminal::<5usize>()),
                ))
                .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
                .then(expression_parser.clone())
                .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
                .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
                .map(|v| do_while_statement::_T0::new(v))
                .boxed();

        // EmitStatement = 'emit' Expression ArgumentList ';' ;
        let emit_statement_parser = with_noise(
            terminal("emit")
                .ignored()
                .map(|_| FixedTerminal::<4usize>()),
        )
        .then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| emit_statement::_T0::new(v))
        .boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser = expression_parser
            .clone()
            .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
            .map(|v| expression_statement::_T0::new(v))
            .boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser =
            with_noise(terminal("if").ignored().map(|_| FixedTerminal::<2usize>()))
                .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
                .then(expression_parser.clone())
                .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
                .then(statement_parser.clone())
                .then(
                    with_noise(
                        terminal("else")
                            .ignored()
                            .map(|_| FixedTerminal::<4usize>()),
                    )
                    .then(statement_parser.clone())
                    .map(|v| if_statement::_T1::new(v))
                    .or_not(),
                )
                .map(|v| if_statement::_T0::new(v))
                .boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser = with_noise(
            terminal("return")
                .ignored()
                .map(|_| FixedTerminal::<6usize>()),
        )
        .then(expression_parser.clone().or_not())
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| return_statement::_T0::new(v))
        .boxed();

        // RevertStatement = 'revert' Expression ArgumentList ';' ;
        let revert_statement_parser = with_noise(
            terminal("revert")
                .ignored()
                .map(|_| FixedTerminal::<6usize>()),
        )
        .then(expression_parser.clone())
        .then(argument_list_parser.clone())
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| revert_statement::_T0::new(v))
        .boxed();

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        let state_variable_declaration_parser = type_name_parser
            .clone()
            .then(state_variable_attribute_parser.clone().repeated())
            .then(identifier_parser.clone())
            .then(
                with_noise(just('=').map(|_| FixedTerminal::<1>()))
                    .then(expression_parser.clone())
                    .map(|v| state_variable_declaration::_T2::new(v))
                    .or_not(),
            )
            .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
            .map(|v| state_variable_declaration::_T0::new(v))
            .boxed();

        // TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block 1…*{ CatchClause } ;
        let try_statement_parser =
            with_noise(terminal("try").ignored().map(|_| FixedTerminal::<3usize>()))
                .then(expression_parser.clone())
                .then(
                    with_noise(
                        terminal("returns")
                            .ignored()
                            .map(|_| FixedTerminal::<7usize>()),
                    )
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| try_statement::_T1::new(v))
                    .or_not(),
                )
                .then(block_parser.clone())
                .then(catch_clause_parser.clone().repeated().at_least(1usize))
                .map(|v| try_statement::_T0::new(v))
                .boxed();

        // VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
        let variable_declaration_statement_parser = choice((
            variable_declaration_parser
                .clone()
                .then(
                    with_noise(just('=').map(|_| FixedTerminal::<1>()))
                        .then(expression_parser.clone())
                        .map(|v| variable_declaration_statement::_T2::new(v))
                        .or_not(),
                )
                .map(|v| variable_declaration_statement::_T1::new(v))
                .map(|v| {
                    Box::new(variable_declaration_statement::VariableDeclarationStatement::_T1(v))
                }),
            variable_declaration_tuple_parser
                .clone()
                .then(with_noise(just('=').map(|_| FixedTerminal::<1>())))
                .then(expression_parser.clone())
                .map(|v| variable_declaration_statement::_T3::new(v))
                .map(|v| {
                    Box::new(variable_declaration_statement::VariableDeclarationStatement::_T3(v))
                }),
        ))
        .then(with_noise(just(';').map(|_| FixedTerminal::<1>())))
        .map(|v| variable_declaration_statement::_T0::new(v))
        .boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser = with_noise(
            terminal("while")
                .ignored()
                .map(|_| FixedTerminal::<5usize>()),
        )
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(expression_parser.clone())
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .then(statement_parser.clone())
        .map(|v| while_statement::_T0::new(v))
        .boxed();

        // SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser = choice((
            variable_declaration_statement_parser.clone().map(|v| {
                Box::new(simple_statement::SimpleStatement::VariableDeclarationStatement(v))
            }),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::SimpleStatement::ExpressionStatement(v))),
        ))
        .boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser =
            with_noise(terminal("for").ignored().map(|_| FixedTerminal::<3usize>()))
                .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
                .then(choice((
                    simple_statement_parser
                        .clone()
                        .map(|v| Box::new(for_statement::ForStatement::SimpleStatement(v))),
                    with_noise(just(';').map(|_| FixedTerminal::<1>()))
                        .map(|v| Box::new(for_statement::ForStatement::SemicolonChar(v))),
                )))
                .then(choice((
                    expression_statement_parser
                        .clone()
                        .map(|v| Box::new(for_statement::ForStatement::ExpressionStatement(v))),
                    with_noise(just(';').map(|_| FixedTerminal::<1>()))
                        .map(|v| Box::new(for_statement::ForStatement::SemicolonChar(v))),
                )))
                .then(expression_parser.clone().or_not())
                .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
                .then(statement_parser.clone())
                .map(|v| for_statement::_T0::new(v))
                .boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
        statement_parser.define(
            choice((
                block_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::Block(v))),
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::SimpleStatement(v))),
                if_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::IfStatement(v))),
                for_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ForStatement(v))),
                while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::WhileStatement(v))),
                do_while_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::DoWhileStatement(v))),
                continue_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ContinueStatement(v))),
                break_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::BreakStatement(v))),
                try_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::TryStatement(v))),
                return_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::ReturnStatement(v))),
                emit_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::EmitStatement(v))),
                revert_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::RevertStatement(v))),
                assembly_statement_parser
                    .clone()
                    .map(|v| Box::new(statement::Statement::AssemblyStatement(v))),
            ))
            .boxed(),
        );

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser.define(
            with_noise(just('{').map(|_| FixedTerminal::<1>()))
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::Block::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::Block::UncheckedBlock(v))),
                    ))
                    .repeated(),
                )
                .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
                .map(|v| block::_T0::new(v))
                .boxed(),
        );

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser = with_noise(
            terminal("constructor")
                .ignored()
                .map(|_| FixedTerminal::<11usize>()),
        )
        .then(parameter_list_parser.clone())
        .then(constructor_attribute_parser.clone().repeated())
        .then(block_parser.clone())
        .map(|v| constructor_definition::_T0::new(v))
        .boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser = with_noise(
            terminal("fallback")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(parameter_list_parser.clone())
        .then(fallback_function_attribute_parser.clone().repeated())
        .then(
            with_noise(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .then(non_empty_parameter_list_parser.clone())
            .map(|v| fallback_function_definition::_T2::new(v))
            .or_not(),
        )
        .then(choice((
            with_noise(just(';').map(|_| FixedTerminal::<1>())).map(|v| {
                Box::new(fallback_function_definition::FallbackFunctionDefinition::SemicolonChar(v))
            }),
            block_parser.clone().map(|v| {
                Box::new(fallback_function_definition::FallbackFunctionDefinition::Block(v))
            }),
        )))
        .map(|v| fallback_function_definition::_T0::new(v))
        .boxed();

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let function_definition_parser = with_noise(
            terminal("function")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(choice((
            identifier_parser
                .clone()
                .map(|v| Box::new(function_definition::FunctionDefinition::Identifier(v))),
            with_noise(choice::<_, ErrorType>((
                terminal("fallback").map(|_| 8usize),
                terminal("receive").map(|_| 7usize),
            )))
            .map(|v| Box::new(function_definition::FunctionDefinition::_1(v))),
        )))
        .then(parameter_list_parser.clone())
        .then(function_attribute_parser.clone().repeated())
        .then(
            with_noise(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>()),
            )
            .then(non_empty_parameter_list_parser.clone())
            .map(|v| function_definition::_T2::new(v))
            .or_not(),
        )
        .then(choice((
            with_noise(just(';').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(function_definition::FunctionDefinition::SemicolonChar(v))),
            block_parser
                .clone()
                .map(|v| Box::new(function_definition::FunctionDefinition::Block(v))),
        )))
        .map(|v| function_definition::_T0::new(v))
        .boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
        let modifier_definition_parser = with_noise(
            terminal("modifier")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .then(identifier_parser.clone())
        .then(parameter_list_parser.clone().or_not())
        .then(method_attribute_parser.clone().repeated())
        .then(choice((
            with_noise(just(';').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(modifier_definition::ModifierDefinition::SemicolonChar(v))),
            block_parser
                .clone()
                .map(|v| Box::new(modifier_definition::ModifierDefinition::Block(v))),
        )))
        .map(|v| modifier_definition::_T0::new(v))
        .boxed();

        // ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser = with_noise(
            terminal("receive")
                .ignored()
                .map(|_| FixedTerminal::<7usize>()),
        )
        .then(with_noise(just('(').map(|_| FixedTerminal::<1>())))
        .then(with_noise(just(')').map(|_| FixedTerminal::<1>())))
        .then(receive_function_attribute_parser.clone().repeated())
        .then(choice((
            with_noise(just(';').map(|_| FixedTerminal::<1>())).map(|v| {
                Box::new(receive_function_definition::ReceiveFunctionDefinition::SemicolonChar(v))
            }),
            block_parser.clone().map(|v| {
                Box::new(receive_function_definition::ReceiveFunctionDefinition::Block(v))
            }),
        )))
        .map(|v| receive_function_definition::_T0::new(v))
        .boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
        let contract_body_element_parser = choice((
            using_directive_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::UsingDirective(
                    v,
                ))
            }),
            constructor_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ConstructorDefinition(v))
            }),
            function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::FunctionDefinition(v))
            }),
            fallback_function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::FallbackFunctionDefinition(v))
            }),
            receive_function_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ReceiveFunctionDefinition(v))
            }),
            modifier_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ModifierDefinition(v))
            }),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(contract_body_element::ContractBodyElement::StructDefinition(v))),
            enum_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::EnumDefinition(
                    v,
                ))
            }),
            user_defined_value_type_definition_parser.clone().map(|v| {
                Box::new(
                    contract_body_element::ContractBodyElement::UserDefinedValueTypeDefinition(v),
                )
            }),
            event_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::EventDefinition(
                    v,
                ))
            }),
            error_definition_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::ErrorDefinition(
                    v,
                ))
            }),
            state_variable_declaration_parser.clone().map(|v| {
                Box::new(contract_body_element::ContractBodyElement::StateVariableDeclaration(v))
            }),
        ))
        .boxed();

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser = with_noise(
            terminal("abstract")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        )
        .or_not()
        .then(with_noise(
            terminal("contract")
                .ignored()
                .map(|_| FixedTerminal::<8usize>()),
        ))
        .then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then(with_noise(just('{').map(|_| FixedTerminal::<1>())))
        .then(contract_body_element_parser.clone().repeated())
        .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
        .map(|v| contract_definition::_T0::new(v))
        .boxed();

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser = with_noise(
            terminal("interface")
                .ignored()
                .map(|_| FixedTerminal::<9usize>()),
        )
        .then(identifier_parser.clone())
        .then(inheritance_specifier_list_parser.clone().or_not())
        .then(with_noise(just('{').map(|_| FixedTerminal::<1>())))
        .then(contract_body_element_parser.clone().repeated())
        .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
        .map(|v| interface_definition::_T0::new(v))
        .boxed();

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser = with_noise(
            terminal("library")
                .ignored()
                .map(|_| FixedTerminal::<7usize>()),
        )
        .then(identifier_parser.clone())
        .then(with_noise(just('{').map(|_| FixedTerminal::<1>())))
        .then(contract_body_element_parser.clone().repeated())
        .then(with_noise(just('}').map(|_| FixedTerminal::<1>())))
        .map(|v| library_definition::_T0::new(v))
        .boxed();

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
        let definition_parser = choice((
            contract_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ContractDefinition(v))),
            interface_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::InterfaceDefinition(v))),
            library_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::LibraryDefinition(v))),
            function_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::FunctionDefinition(v))),
            constant_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ConstantDefinition(v))),
            struct_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::StructDefinition(v))),
            enum_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::EnumDefinition(v))),
            user_defined_value_type_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::UserDefinedValueTypeDefinition(v))),
            error_definition_parser
                .clone()
                .map(|v| Box::new(definition::Definition::ErrorDefinition(v))),
        ))
        .boxed();

        // SourceUnit = «IGNORE» { Directive | Definition } $ ;
        let source_unit_parser = ignore_parser
            .clone()
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::SourceUnit::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::SourceUnit::Definition(v))),
                ))
                .repeated(),
            )
            .then(end())
            .map(|v| source_unit::_T0::new(v))
            .boxed();

        Self {
            ascii_escape: ascii_escape_parser,
            boolean_literal: boolean_literal_parser,
            comment: comment_parser,
            decimal_integer: decimal_integer_parser,
            fixed_bytes_type: fixed_bytes_type_parser,
            fixed_type: fixed_type_parser,
            hex_character: hex_character_parser,
            identifier_start: identifier_start_parser,
            line_comment: line_comment_parser,
            number_unit: number_unit_parser,
            pragma_directive: pragma_directive_parser,
            reserved_keyword: reserved_keyword_parser,
            signed_integer_type: signed_integer_type_parser,
            whitespace: whitespace_parser,
            yul_decimal_number_literal: yul_decimal_number_literal_parser,
            yul_evm_builtin_function_name: yul_evm_builtin_function_name_parser,
            yul_hex_literal: yul_hex_literal_parser,
            yul_keyword: yul_keyword_parser,
            decimal_exponent: decimal_exponent_parser,
            decimal_float: decimal_float_parser,
            hex_byte_escape: hex_byte_escape_parser,
            hex_number: hex_number_parser,
            ignore: ignore_parser,
            identifier_part: identifier_part_parser,
            possibly_separated_pairs_of_hex_digits: possibly_separated_pairs_of_hex_digits_parser,
            ufixed_type: ufixed_type_parser,
            unicode_escape: unicode_escape_parser,
            unsigned_integer_type: unsigned_integer_type_parser,
            yul_reserved_word: yul_reserved_word_parser,
            add_sub_expression: add_sub_expression_parser,
            and_expression: and_expression_parser,
            assignment_expression: assignment_expression_parser,
            bit_and_expression: bit_and_expression_parser,
            bit_or_expression: bit_or_expression_parser,
            bit_x_or_expression: bit_x_or_expression_parser,
            break_statement: break_statement_parser,
            conditional_expression: conditional_expression_parser,
            continue_statement: continue_statement_parser,
            data_location: data_location_parser,
            decimal_number: decimal_number_parser,
            elementary_type: elementary_type_parser,
            equality_comparison_expression: equality_comparison_expression_parser,
            escape_sequence: escape_sequence_parser,
            exponentiation_expression: exponentiation_expression_parser,
            hex_string_literal: hex_string_literal_parser,
            index_access_expression: index_access_expression_parser,
            keyword: keyword_parser,
            mul_div_mod_expression: mul_div_mod_expression_parser,
            or_expression: or_expression_parser,
            order_comparison_expression: order_comparison_expression_parser,
            positional_argument_list: positional_argument_list_parser,
            raw_identifier: raw_identifier_parser,
            shift_expression: shift_expression_parser,
            state_mutability_specifier: state_mutability_specifier_parser,
            unary_prefix_expression: unary_prefix_expression_parser,
            unary_suffix_expression: unary_suffix_expression_parser,
            unchecked_block: unchecked_block_parser,
            visibility_specifier: visibility_specifier_parser,
            yul_break_statement: yul_break_statement_parser,
            yul_continue_statement: yul_continue_statement_parser,
            yul_leave_statement: yul_leave_statement_parser,
            double_quoted_ascii_string_literal: double_quoted_ascii_string_literal_parser,
            double_quoted_unicode_string_literal: double_quoted_unicode_string_literal_parser,
            elementary_type_with_payable: elementary_type_with_payable_parser,
            elementary_type_without_payable: elementary_type_without_payable_parser,
            numeric_literal: numeric_literal_parser,
            reserved_word: reserved_word_parser,
            single_quoted_ascii_string_literal: single_quoted_ascii_string_literal_parser,
            single_quoted_unicode_string_literal: single_quoted_unicode_string_literal_parser,
            yul_identifier: yul_identifier_parser,
            ascii_string_literal: ascii_string_literal_parser,
            assembly_flags: assembly_flags_parser,
            identifier: identifier_parser,
            unicode_string_literal: unicode_string_literal_parser,
            yul_function_call: yul_function_call_parser,
            yul_function_definition: yul_function_definition_parser,
            yul_path: yul_path_parser,
            enum_definition: enum_definition_parser,
            identifier_path: identifier_path_parser,
            import_path: import_path_parser,
            literal: literal_parser,
            member_access_expression: member_access_expression_parser,
            named_argument: named_argument_parser,
            parameter_declaration: parameter_declaration_parser,
            selected_import: selected_import_parser,
            user_defined_value_type_definition: user_defined_value_type_definition_parser,
            yul_literal: yul_literal_parser,
            function_call_options_expression: function_call_options_expression_parser,
            mapping_type: mapping_type_parser,
            named_argument_list: named_argument_list_parser,
            non_empty_parameter_list: non_empty_parameter_list_parser,
            override_specifier: override_specifier_parser,
            parameter_list: parameter_list_parser,
            selecting_import_directive: selecting_import_directive_parser,
            simple_import_directive: simple_import_directive_parser,
            star_import_directive: star_import_directive_parser,
            yul_expression: yul_expression_parser,
            argument_list: argument_list_parser,
            catch_clause: catch_clause_parser,
            function_type: function_type_parser,
            import_directive: import_directive_parser,
            method_attribute: method_attribute_parser,
            state_variable_attribute: state_variable_attribute_parser,
            yul_assignment: yul_assignment_parser,
            yul_for_statement: yul_for_statement_parser,
            yul_if_statement: yul_if_statement_parser,
            yul_switch_statement: yul_switch_statement_parser,
            yul_variable_declaration: yul_variable_declaration_parser,
            function_call_expression: function_call_expression_parser,
            inheritance_specifier: inheritance_specifier_parser,
            modifier_invocation: modifier_invocation_parser,
            type_name: type_name_parser,
            yul_statement: yul_statement_parser,
            constructor_attribute: constructor_attribute_parser,
            error_parameter: error_parameter_parser,
            event_parameter: event_parameter_parser,
            fallback_function_attribute: fallback_function_attribute_parser,
            function_attribute: function_attribute_parser,
            inheritance_specifier_list: inheritance_specifier_list_parser,
            primary_expression: primary_expression_parser,
            receive_function_attribute: receive_function_attribute_parser,
            struct_definition: struct_definition_parser,
            using_directive: using_directive_parser,
            variable_declaration: variable_declaration_parser,
            yul_block: yul_block_parser,
            assembly_statement: assembly_statement_parser,
            directive: directive_parser,
            error_definition: error_definition_parser,
            event_definition: event_definition_parser,
            expression: expression_parser,
            variable_declaration_tuple: variable_declaration_tuple_parser,
            constant_definition: constant_definition_parser,
            do_while_statement: do_while_statement_parser,
            emit_statement: emit_statement_parser,
            expression_statement: expression_statement_parser,
            if_statement: if_statement_parser,
            return_statement: return_statement_parser,
            revert_statement: revert_statement_parser,
            state_variable_declaration: state_variable_declaration_parser,
            try_statement: try_statement_parser,
            variable_declaration_statement: variable_declaration_statement_parser,
            while_statement: while_statement_parser,
            simple_statement: simple_statement_parser,
            for_statement: for_statement_parser,
            statement: statement_parser,
            block: block_parser,
            constructor_definition: constructor_definition_parser,
            fallback_function_definition: fallback_function_definition_parser,
            function_definition: function_definition_parser,
            modifier_definition: modifier_definition_parser,
            receive_function_definition: receive_function_definition_parser,
            contract_body_element: contract_body_element_parser,
            contract_definition: contract_definition_parser,
            interface_definition: interface_definition_parser,
            library_definition: library_definition_parser,
            definition: definition_parser,
            source_unit: source_unit_parser,
        }
    }
}
