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
        let mut block_parser = Recursive::declare();

        let mut expression_parser = Recursive::declare();

        let mut ignore_parser = Recursive::declare();

        let mut statement_parser = Recursive::declare();

        let mut type_name_parser = Recursive::declare();

        let mut yul_block_parser = Recursive::declare();

        let mut yul_expression_parser = Recursive::declare();

        // AddSubOperator = '+' | '-' ;
        let add_sub_operator_parser = filter(|&c: &char| c == '+' || c == '-')
            .map(|_| FixedTerminal::<1>())
            .boxed();

        // AssignmentOperator = '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ;
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

        // BreakStatement = 'break' ';' ;
        let break_statement_parser = terminal("break")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(break_statement::_S0::new(v)))
            .boxed();

        // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } { '*' } '*/' ;
        let comment_parser = terminal("/*")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                choice((
                    filter(|&c: &char| c != '*')
                        .map(|_| FixedTerminal::<1>())
                        .map(|v| Box::new(comment::_C2::StarChar(v))),
                    just('*')
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .at_least(1usize)
                        .map(|v| v.len())
                        .then(
                            filter(|&c: &char| c != '*' && c != '/').map(|_| FixedTerminal::<1>()),
                        )
                        .map(|v| Box::new(comment::_S3::new(v)))
                        .map(|v| Box::new(comment::_C2::_S3(v))),
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
            .map(|v| Box::new(comment::_S0::new(v)))
            .boxed();

        // ContinueStatement = 'continue' ';' ;
        let continue_statement_parser = terminal("continue")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(continue_statement::_S0::new(v)))
            .boxed();

        // DataLocation = 'memory' | 'storage' | 'calldata' ;
        let data_location_parser = choice::<_, ErrorType>((
            terminal("calldata").map(|_| 8usize),
            terminal("memory").map(|_| 6usize),
            terminal("storage").map(|_| 7usize),
        ))
        .boxed();

        // EqualityComparisonOperator = '==' | '!=' ;
        let equality_comparison_operator_parser =
            choice::<_, ErrorType>((terminal("!=").ignored(), terminal("==").ignored()))
                .map(|_| FixedTerminal::<2usize>())
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
            .map(|v| Box::new(line_comment::_S0::new(v)))
            .boxed();

        // MulDivModOperator = '*' | '/' | '%' ;
        let mul_div_mod_operator_parser = filter(|&c: &char| c == '*' || c == '/' || c == '%')
            .map(|_| FixedTerminal::<1>())
            .boxed();

        // OrderComparisonOperator = '<' | '>' | '<=' | '>=' ;
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

        // PositionalArgumentList = 1…*{ Expression / ',' } ;
        let positional_argument_list_parser = expression_parser
            .clone()
            .then(ignore_parser.clone())
            .map(|v| Box::new(positional_argument_list::_S1::new(v)))
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(positional_argument_list::_S2::new(v)))
                    .then(
                        expression_parser
                            .clone()
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(positional_argument_list::_S1::new(v))),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(positional_argument_list::_S0::new(v)))
            .boxed();

        // ShiftOperator = '<<' | '>>' | '>>>' ;
        let shift_operator_parser = choice::<_, ErrorType>((
            terminal("<<").map(|_| 2usize),
            terminal(">>").ignore_then(choice((
                terminal(">").map(|_| 3usize),
                empty().map(|_| 2usize),
            ))),
        ))
        .boxed();

        // StateMutabilitySpecifier = 'pure' | 'view' | 'payable' ;
        let state_mutability_specifier_parser = choice::<_, ErrorType>((
            terminal("p").ignore_then(choice((
                terminal("ayable").map(|_| 7usize),
                terminal("ure").map(|_| 4usize),
            ))),
            terminal("view").map(|_| 4usize),
        ))
        .boxed();

        // UnaryPrefixOperator = '++' | '--' | '!' | '~' | 'delete' | '-' ;
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

        // UnarySuffixOperator = '++' | '--' ;
        let unary_suffix_operator_parser =
            choice::<_, ErrorType>((terminal("++").ignored(), terminal("--").ignored()))
                .map(|_| FixedTerminal::<2usize>())
                .boxed();

        // UncheckedBlock = 'unchecked' Block ;
        let unchecked_block_parser = terminal("unchecked")
            .ignored()
            .map(|_| FixedTerminal::<9usize>())
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(unchecked_block::_S0::new(v)))
            .boxed();

        // VisibilitySpecifier = 'internal' | 'external' | 'private' | 'public' ;
        let visibility_specifier_parser = choice::<_, ErrorType>((
            terminal("external").map(|_| 8usize),
            terminal("internal").map(|_| 8usize),
            terminal("p").ignore_then(choice((
                terminal("rivate").map(|_| 7usize),
                terminal("ublic").map(|_| 6usize),
            ))),
        ))
        .boxed();

        // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' | '\u{d}' | '\u{a}' } ;
        let whitespace_parser = filter(|&c: &char| c == ' ' || c == '\t' || c == '\r' || c == '\n')
            .map(|_| FixedTerminal::<1>())
            .repeated()
            .at_least(1usize)
            .map(|v| v.len())
            .boxed();

        // YulBreakStatement = 'break' ;
        let yul_break_statement_parser = terminal("break")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .boxed();

        // YulContinueStatement = 'continue' ;
        let yul_continue_statement_parser = terminal("continue")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .boxed();

        // YulLeaveStatement = 'leave' ;
        let yul_leave_statement_parser = terminal("leave")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .boxed();

        // «IGNORE» = { «Whitespace» | «Comment» | «LineComment» } ;
        ignore_parser.define(
            choice((
                whitespace_parser
                    .clone()
                    .map(|v| Box::new(ignore::_C1::Whitespace(v))),
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
            .map(|v| Box::new(decimal_integer::_S0::new(v)))
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
            .map(|v| Box::new(fixed_bytes_type::_S0::new(v)))
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
                    .map(|v| Box::new(fixed_type::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(fixed_type::_S0::new(v)))
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

        // «PragmaDirective» = 'pragma' ¬';' { ¬';' } ';' ;
        let pragma_directive_parser = terminal("pragma")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(filter(|&c: &char| c != ';').map(|_| FixedTerminal::<1>()))
            .then(
                filter(|&c: &char| c != ';')
                    .map(|_| FixedTerminal::<1>())
                    .repeated()
                    .map(|v| v.len()),
            )
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(pragma_directive::_S0::new(v)))
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
            .map(|v| Box::new(signed_integer_type::_S0::new(v)))
            .boxed();

        // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
        let yul_decimal_number_literal_parser = choice((
            terminal("0")
                .ignored()
                .map(|_| FixedTerminal::<1usize>())
                .map(|v| Box::new(yul_decimal_number_literal::_C0::Zero(v))),
            filter(|&c: &char| ('1' <= c && c <= '9'))
                .map(|_| FixedTerminal::<1>())
                .then(
                    filter(|&c: &char| ('0' <= c && c <= '9'))
                        .map(|_| FixedTerminal::<1>())
                        .repeated()
                        .map(|v| v.len()),
                )
                .map(|v| Box::new(yul_decimal_number_literal::_S1::new(v)))
                .map(|v| Box::new(yul_decimal_number_literal::_C0::_S1(v))),
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

        // «YulHexLiteral» = '0x' ( '0'…'9' | 'a'…'f' | 'A'…'F' ) { '0'…'9' | 'a'…'f' | 'A'…'F' } ;
        let yul_hex_literal_parser = terminal("0x")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>()),
            )
            .then(
                filter(|&c: &char| {
                    ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
                })
                .map(|_| FixedTerminal::<1>())
                .repeated()
                .map(|v| v.len()),
            )
            .map(|v| Box::new(yul_hex_literal::_S0::new(v)))
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
            .map(|v| Box::new(decimal_exponent::_S0::new(v)))
            .boxed();

        // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
        let decimal_float_parser = decimal_integer_parser
            .clone()
            .or_not()
            .then(just('.').map(|_| FixedTerminal::<1>()))
            .then(decimal_integer_parser.clone())
            .map(|v| Box::new(decimal_float::_S0::new(v)))
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
            .map(|v| Box::new(hex_byte_escape::_S0::new(v)))
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
                .map(|v| Box::new(hex_number::_S1::new(v))),
            )
            .map(|v| Box::new(hex_number::_S0::new(v)))
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
        .map(|v| Box::new(possibly_separated_pairs_of_hex_digits::_S0::new(v)))
        .boxed();

        // «UfixedType» = 'u' «FixedType» ;
        let ufixed_type_parser = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(fixed_type_parser.clone())
            .map(|v| Box::new(ufixed_type::_S0::new(v)))
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
            .map(|v| Box::new(unicode_escape::_S0::new(v)))
            .boxed();

        // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
        let unsigned_integer_type_parser = just('u')
            .map(|_| FixedTerminal::<1>())
            .then(signed_integer_type_parser.clone())
            .map(|v| Box::new(unsigned_integer_type::_S0::new(v)))
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

        // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
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

        // ElementaryType = 'bool' | 'string' | 'bytes' | «SignedIntegerType» | «UnsignedIntegerType» | «FixedBytesType» | «FixedType» | «UfixedType» ;
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

        // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
        let escape_sequence_parser = just('\\')
            .map(|_| FixedTerminal::<1>())
            .then(choice((
                choice::<_, ErrorType>((
                    terminal("\n").ignored(),
                    terminal("\r").ignored(),
                    terminal("\"").ignored(),
                    terminal("'").ignored(),
                    terminal("\\").ignored(),
                    terminal("n").ignored(),
                    terminal("r").ignored(),
                    terminal("t").ignored(),
                ))
                .map(|_| FixedTerminal::<1usize>())
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
                    .map(|v| Box::new(hex_string_literal::_S2::new(v)))
                    .map(|v| Box::new(hex_string_literal::_C1::_S2(v))),
                just('\'')
                    .map(|_| FixedTerminal::<1>())
                    .then(
                        possibly_separated_pairs_of_hex_digits_parser
                            .clone()
                            .or_not(),
                    )
                    .then(just('\'').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(hex_string_literal::_S4::new(v)))
                    .map(|v| Box::new(hex_string_literal::_C1::_S4(v))),
            )))
            .map(|v| Box::new(hex_string_literal::_S0::new(v)))
            .boxed();

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
        .map(|v| Box::new(raw_identifier::_S0::new(v)))
        .boxed();

        // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
        let double_quoted_ascii_string_literal_parser = just('"')
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
            .map(|v| Box::new(double_quoted_ascii_string_literal::_S0::new(v)))
            .boxed();

        // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
        let double_quoted_unicode_string_literal_parser = terminal("unicode\"")
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
            .map(|v| Box::new(double_quoted_unicode_string_literal::_S0::new(v)))
            .boxed();

        // ElementaryTypeWithPayable = 'address' [ 'payable' ] | ElementaryType ;
        let elementary_type_with_payable_parser = choice((
            terminal("address")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .then(ignore_parser.clone())
                .then(
                    terminal("payable")
                        .ignored()
                        .map(|_| FixedTerminal::<7usize>())
                        .or_not(),
                )
                .map(|v| Box::new(elementary_type_with_payable::_S1::new(v)))
                .map(|v| Box::new(elementary_type_with_payable::_C0::_S1(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_with_payable::_C0::ElementaryType(v))),
        ))
        .boxed();

        // ElementaryTypeWithoutPayable = 'address' | ElementaryType ;
        let elementary_type_without_payable_parser = choice((
            terminal("address")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(elementary_type_without_payable::_C0::Address(v))),
            elementary_type_parser
                .clone()
                .map(|v| Box::new(elementary_type_without_payable::_C0::ElementaryType(v))),
        ))
        .boxed();

        // NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
        let numeric_literal_parser = choice((
            decimal_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_C1::DecimalNumber(v))),
            hex_number_parser
                .clone()
                .map(|v| Box::new(numeric_literal::_C1::HexNumber(v))),
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
        .map(|v| Box::new(numeric_literal::_S0::new(v)))
        .boxed();

        // «ReservedWord» = «Keyword» | «ReservedKeyword» | «NumberUnit» | «BooleanLiteral» ;
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

        // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_ascii_string_literal_parser = just('\'')
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
            .map(|v| Box::new(single_quoted_ascii_string_literal::_S0::new(v)))
            .boxed();

        // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
        let single_quoted_unicode_string_literal_parser = terminal("unicode'")
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
            .map(|v| Box::new(single_quoted_unicode_string_literal::_S0::new(v)))
            .boxed();

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
            single_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_C0::SingleQuotedAsciiStringLiteral(v))),
            double_quoted_ascii_string_literal_parser
                .clone()
                .map(|v| Box::new(ascii_string_literal::_C0::DoubleQuotedAsciiStringLiteral(v))),
        ))
        .boxed();

        // AssemblyFlags = '(' 1…*{ «DoubleQuotedAsciiStringLiteral» / ',' } ')' ;
        let assembly_flags_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                double_quoted_ascii_string_literal_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(assembly_flags::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(assembly_flags::_S3::new(v)))
                            .then(
                                double_quoted_ascii_string_literal_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(assembly_flags::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(assembly_flags::_S1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(assembly_flags::_S0::new(v)))
            .boxed();

        // «Identifier» = «RawIdentifier» - «ReservedWord» ;
        let identifier_parser =
            difference(raw_identifier_parser.clone(), reserved_word_parser.clone()).boxed();

        // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
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

        // YulFunctionCall = ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) '(' { YulExpression / ',' } ')' ;
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
        .then(ignore_parser.clone())
        .then(just('(').map(|_| FixedTerminal::<1>()))
        .then(ignore_parser.clone())
        .then(
            yul_expression_parser
                .clone()
                .then(ignore_parser.clone())
                .map(|v| Box::new(yul_function_call::_S3::new(v)))
                .then(
                    just(',')
                        .map(|_| FixedTerminal::<1>())
                        .then(ignore_parser.clone())
                        .map(|v| Box::new(yul_function_call::_S4::new(v)))
                        .then(
                            yul_expression_parser
                                .clone()
                                .then(ignore_parser.clone())
                                .map(|v| Box::new(yul_function_call::_S3::new(v))),
                        )
                        .repeated(),
                )
                .map(repetition_mapper)
                .map(|v| Box::new(yul_function_call::_S2::new(v)))
                .or_not(),
        )
        .then(ignore_parser.clone())
        .then(just(')').map(|_| FixedTerminal::<1>()))
        .map(|v| Box::new(yul_function_call::_S0::new(v)))
        .boxed();

        // YulFunctionDefinition = 'function' «YulIdentifier» '(' { «YulIdentifier» / ',' } ')' [ '->' 1…*{ «YulIdentifier» / ',' } ] YulBlock ;
        let yul_function_definition_parser = terminal("function")
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
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(yul_function_definition::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(yul_function_definition::_S3::new(v)))
                            .then(
                                yul_identifier_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(yul_function_definition::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(yul_function_definition::_S1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                terminal("->")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(
                        yul_identifier_parser
                            .clone()
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(yul_function_definition::_S7::new(v)))
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(yul_function_definition::_S8::new(v)))
                                    .then(
                                        yul_identifier_parser
                                            .clone()
                                            .then(ignore_parser.clone())
                                            .map(|v| {
                                                Box::new(yul_function_definition::_S7::new(v))
                                            }),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(yul_function_definition::_S6::new(v))),
                    )
                    .map(|v| Box::new(yul_function_definition::_S5::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_function_definition::_S0::new(v)))
            .boxed();

        // YulPath = «YulIdentifier» { '.' ( «YulIdentifier» | «YulEVMBuiltinFunctionName» ) } ;
        let yul_path_parser = yul_identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('.')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
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
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(yul_path::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(yul_path::_S0::new(v)))
            .boxed();

        // EnumDefinition = 'enum' «Identifier» '{' 1…*{ «Identifier» / ',' } '}' ;
        let enum_definition_parser = terminal("enum")
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
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(enum_definition::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(enum_definition::_S3::new(v)))
                            .then(
                                identifier_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(enum_definition::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(enum_definition::_S1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(enum_definition::_S0::new(v)))
            .boxed();

        // IdentifierPath = 1…*{ «Identifier» / '.' } ;
        let identifier_path_parser = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .map(|v| Box::new(identifier_path::_S1::new(v)))
            .then(
                just('.')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(identifier_path::_S2::new(v)))
                    .then(
                        identifier_parser
                            .clone()
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(identifier_path::_S1::new(v))),
                    )
                    .repeated(),
            )
            .map(repetition_mapper)
            .map(|v| Box::new(identifier_path::_S0::new(v)))
            .boxed();

        // ImportPath = «AsciiStringLiteral» ;
        let import_path_parser = ascii_string_literal_parser.clone().boxed();

        // Literal = «AsciiStringLiteral» | «UnicodeStringLiteral» | NumericLiteral | «HexStringLiteral» | «BooleanLiteral» ;
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

        // NamedArgument = «Identifier» ':' Expression ;
        let named_argument_parser = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just(':').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .map(|v| Box::new(named_argument::_S0::new(v)))
            .boxed();

        // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
        let parameter_declaration_parser = type_name_parser
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
            .map(|v| Box::new(parameter_declaration::_S0::new(v)))
            .boxed();

        // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
        let selected_import_parser = identifier_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("as")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(identifier_parser.clone())
                    .map(|v| Box::new(selected_import::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(selected_import::_S0::new(v)))
            .boxed();

        // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryTypeWithPayable ';' ;
        let user_defined_value_type_definition_parser = terminal("type")
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
            .map(|v| Box::new(user_defined_value_type_definition::_S0::new(v)))
            .boxed();

        // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
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

        // MappingType = 'mapping' '(' ( ElementaryTypeWithoutPayable | IdentifierPath ) '=>' TypeName ')' ;
        let mapping_type_parser = terminal("mapping")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(choice((
                elementary_type_without_payable_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_C1::ElementaryTypeWithoutPayable(v))),
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(mapping_type::_C1::IdentifierPath(v))),
            )))
            .then(ignore_parser.clone())
            .then(terminal("=>").ignored().map(|_| FixedTerminal::<2usize>()))
            .then(ignore_parser.clone())
            .then(type_name_parser.clone())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(mapping_type::_S0::new(v)))
            .boxed();

        // NamedArgumentList = '{' { NamedArgument / ',' } '}' ;
        let named_argument_list_parser = just('{')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                named_argument_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(named_argument_list::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(named_argument_list::_S3::new(v)))
                            .then(
                                named_argument_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(named_argument_list::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(named_argument_list::_S1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(named_argument_list::_S0::new(v)))
            .boxed();

        // NonEmptyParameterList = '(' 1…*{ ParameterDeclaration / ',' } ')' ;
        let non_empty_parameter_list_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(non_empty_parameter_list::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(non_empty_parameter_list::_S3::new(v)))
                            .then(
                                parameter_declaration_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(non_empty_parameter_list::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(non_empty_parameter_list::_S1::new(v))),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(non_empty_parameter_list::_S0::new(v)))
            .boxed();

        // OverrideSpecifier = 'override' [ '(' 1…1*{ IdentifierPath / ',' } ')' ] ;
        let override_specifier_parser = terminal("override")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(
                just('(')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(identifier_path_parser.clone().repeated().exactly(1usize))
                    .then(ignore_parser.clone())
                    .then(just(')').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(override_specifier::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(override_specifier::_S0::new(v)))
            .boxed();

        // ParameterList = '(' { ParameterDeclaration / ',' } ')' ;
        let parameter_list_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                parameter_declaration_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(parameter_list::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(parameter_list::_S3::new(v)))
                            .then(
                                parameter_declaration_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(parameter_list::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(parameter_list::_S1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(parameter_list::_S0::new(v)))
            .boxed();

        // SelectingImportDirective = '{' 1…*{ SelectedImport / ',' } '}' 'from' ImportPath ;
        let selecting_import_directive_parser = just('{')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                selected_import_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(selecting_import_directive::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(selecting_import_directive::_S3::new(v)))
                            .then(
                                selected_import_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(selecting_import_directive::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(selecting_import_directive::_S1::new(v))),
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
            .map(|v| Box::new(selecting_import_directive::_S0::new(v)))
            .boxed();

        // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
        let simple_import_directive_parser = import_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("as")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(identifier_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(simple_import_directive::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(simple_import_directive::_S0::new(v)))
            .boxed();

        // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
        let star_import_directive_parser = just('*')
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
            .map(|v| Box::new(star_import_directive::_S0::new(v)))
            .boxed();

        // YulExpression = YulPath | YulFunctionCall | YulLiteral ;
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

        // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
        let argument_list_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
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
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(argument_list::_S0::new(v)))
            .boxed();

        // CatchClause = 'catch' [ [ «Identifier» ] NonEmptyParameterList ] Block ;
        let catch_clause_parser = terminal("catch")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(
                identifier_parser
                    .clone()
                    .or_not()
                    .then(ignore_parser.clone())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(catch_clause::_S2::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(catch_clause::_S0::new(v)))
            .boxed();

        // FunctionType = 'function' ParameterList { VisibilitySpecifier | StateMutabilitySpecifier } [ 'returns' NonEmptyParameterList ] ;
        let function_type_parser = terminal("function")
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
                .then(ignore_parser.clone())
                .map(|v| Box::new(function_type::_S2::new(v)))
                .repeated(),
            )
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(ignore_parser.clone())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_type::_S4::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(function_type::_S0::new(v)))
            .boxed();

        // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
        let import_directive_parser = terminal("import")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
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
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(import_directive::_S0::new(v)))
            .boxed();

        // MethodAttribute = 'virtual' | OverrideSpecifier ;
        let method_attribute_parser = choice((
            terminal("virtual")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(method_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(method_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();

        // StateVariableAttribute = 'public' | 'private' | 'internal' | 'constant' | OverrideSpecifier | 'immutable' ;
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
                .ignored()
                .map(|_| FixedTerminal::<9usize>())
                .map(|v| Box::new(state_variable_attribute::_C0::Immutable(v))),
        ))
        .boxed();

        // YulAssignment = YulPath ( ':=' YulExpression | 1…*{ ',' YulPath } ':=' YulFunctionCall ) ;
        let yul_assignment_parser = yul_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(choice((
                terminal(":=")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(yul_expression_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S2::new(v)))
                    .map(|v| Box::new(yul_assignment::_C1::_S2(v))),
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(yul_path_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S5::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(ignore_parser.clone())
                    .then(terminal(":=").ignored().map(|_| FixedTerminal::<2usize>()))
                    .then(ignore_parser.clone())
                    .then(yul_function_call_parser.clone())
                    .map(|v| Box::new(yul_assignment::_S3::new(v)))
                    .map(|v| Box::new(yul_assignment::_C1::_S3(v))),
            )))
            .map(|v| Box::new(yul_assignment::_S0::new(v)))
            .boxed();

        // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
        let yul_for_statement_parser = terminal("for")
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
            .map(|v| Box::new(yul_for_statement::_S0::new(v)))
            .boxed();

        // YulIfStatement = 'if' YulExpression YulBlock ;
        let yul_if_statement_parser = terminal("if")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(yul_expression_parser.clone())
            .then(ignore_parser.clone())
            .then(yul_block_parser.clone())
            .map(|v| Box::new(yul_if_statement::_S0::new(v)))
            .boxed();

        // YulSwitchStatement = 'switch' YulExpression ( 1…*{ 'case' YulLiteral YulBlock } [ 'default' YulBlock ] | 'default' YulBlock ) ;
        let yul_switch_statement_parser = terminal("switch")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(yul_expression_parser.clone())
            .then(ignore_parser.clone())
            .then(choice((
                terminal("case")
                    .ignored()
                    .map(|_| FixedTerminal::<4usize>())
                    .then(ignore_parser.clone())
                    .then(yul_literal_parser.clone())
                    .then(ignore_parser.clone())
                    .then(yul_block_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_S4::new(v)))
                    .repeated()
                    .at_least(1usize)
                    .then(ignore_parser.clone())
                    .then(
                        terminal("default")
                            .ignored()
                            .map(|_| FixedTerminal::<7usize>())
                            .then(ignore_parser.clone())
                            .then(yul_block_parser.clone())
                            .map(|v| Box::new(yul_switch_statement::_S6::new(v)))
                            .or_not(),
                    )
                    .map(|v| Box::new(yul_switch_statement::_S2::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_C1::_S2(v))),
                terminal("default")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(ignore_parser.clone())
                    .then(yul_block_parser.clone())
                    .map(|v| Box::new(yul_switch_statement::_S7::new(v)))
                    .map(|v| Box::new(yul_switch_statement::_C1::_S7(v))),
            )))
            .map(|v| Box::new(yul_switch_statement::_S0::new(v)))
            .boxed();

        // YulVariableDeclaration = 'let' «YulIdentifier» [ ':=' YulExpression | [ ',' «YulIdentifier» ] [ ':=' YulFunctionCall ] ] ;
        let yul_variable_declaration_parser = terminal("let")
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
                        .then(ignore_parser.clone())
                        .then(yul_expression_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_S3::new(v)))
                        .map(|v| Box::new(yul_variable_declaration::_C2::_S3(v))),
                    just(',')
                        .map(|_| FixedTerminal::<1>())
                        .then(ignore_parser.clone())
                        .then(yul_identifier_parser.clone())
                        .map(|v| Box::new(yul_variable_declaration::_S6::new(v)))
                        .or_not()
                        .then(ignore_parser.clone())
                        .then(
                            terminal(":=")
                                .ignored()
                                .map(|_| FixedTerminal::<2usize>())
                                .then(ignore_parser.clone())
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

        // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
        let inheritance_specifier_parser = identifier_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(inheritance_specifier::_S0::new(v)))
            .boxed();

        // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
        let modifier_invocation_parser = identifier_path_parser
            .clone()
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone().or_not())
            .map(|v| Box::new(modifier_invocation::_S0::new(v)))
            .boxed();

        // TypeName = ( ElementaryTypeWithPayable | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
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
            .then(ignore_parser.clone())
            .then(
                just('[')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone().or_not())
                    .then(ignore_parser.clone())
                    .then(just(']').map(|_| FixedTerminal::<1>()))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(type_name::_S3::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(type_name::_S0::new(v)))
            .boxed(),
        );

        // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignment | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
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

        // ConstructorAttribute = ModifierInvocation | 'payable' | 'internal' | 'public' ;
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

        // ErrorParameter = TypeName [ «Identifier» ] ;
        let error_parameter_parser = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(identifier_parser.clone().or_not())
            .map(|v| Box::new(error_parameter::_S0::new(v)))
            .boxed();

        // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
        let event_parameter_parser = type_name_parser
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
            .map(|v| Box::new(event_parameter::_S0::new(v)))
            .boxed();

        // FallbackFunctionAttribute = 'external' | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
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
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(fallback_function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(fallback_function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();

        // FunctionAttribute = VisibilitySpecifier | StateMutabilitySpecifier | ModifierInvocation | 'virtual' | OverrideSpecifier ;
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
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();

        // InheritanceSpecifierList = 'is' 1…*{ InheritanceSpecifier / ',' } ;
        let inheritance_specifier_list_parser = terminal("is")
            .ignored()
            .map(|_| FixedTerminal::<2usize>())
            .then(ignore_parser.clone())
            .then(
                inheritance_specifier_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(inheritance_specifier_list::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(inheritance_specifier_list::_S3::new(v)))
                            .then(
                                inheritance_specifier_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(inheritance_specifier_list::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(inheritance_specifier_list::_S1::new(v))),
            )
            .map(|v| Box::new(inheritance_specifier_list::_S0::new(v)))
            .boxed();

        // PrimaryExpression = 'payable' ArgumentList | 'type' '(' TypeName ')' | 'new' TypeName | '(' 1…*{ [ Expression ] / ',' } ')' | '[' 1…*{ Expression / ',' } ']' | «Identifier» | Literal | ElementaryTypeWithoutPayable ;
        let primary_expression_parser = choice((
            terminal("payable")
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .then(ignore_parser.clone())
                .then(argument_list_parser.clone())
                .map(|v| Box::new(primary_expression::_S1::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S1(v))),
            terminal("type")
                .ignored()
                .map(|_| FixedTerminal::<4usize>())
                .then(ignore_parser.clone())
                .then(just('(').map(|_| FixedTerminal::<1>()))
                .then(ignore_parser.clone())
                .then(type_name_parser.clone())
                .then(ignore_parser.clone())
                .then(just(')').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(primary_expression::_S2::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S2(v))),
            terminal("new")
                .ignored()
                .map(|_| FixedTerminal::<3usize>())
                .then(ignore_parser.clone())
                .then(type_name_parser.clone())
                .map(|v| Box::new(primary_expression::_S3::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S3(v))),
            just('(')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(
                    expression_parser
                        .clone()
                        .or_not()
                        .then(ignore_parser.clone())
                        .map(|v| Box::new(primary_expression::_S7::new(v)))
                        .then(
                            just(',')
                                .map(|_| FixedTerminal::<1>())
                                .then(ignore_parser.clone())
                                .map(|v| Box::new(primary_expression::_S8::new(v)))
                                .then(
                                    expression_parser
                                        .clone()
                                        .or_not()
                                        .then(ignore_parser.clone())
                                        .map(|v| Box::new(primary_expression::_S7::new(v))),
                                )
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(primary_expression::_S5::new(v))),
                )
                .then(ignore_parser.clone())
                .then(just(')').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(primary_expression::_S4::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S4(v))),
            just('[')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(
                    expression_parser
                        .clone()
                        .then(ignore_parser.clone())
                        .map(|v| Box::new(primary_expression::_S11::new(v)))
                        .then(
                            just(',')
                                .map(|_| FixedTerminal::<1>())
                                .then(ignore_parser.clone())
                                .map(|v| Box::new(primary_expression::_S12::new(v)))
                                .then(
                                    expression_parser
                                        .clone()
                                        .then(ignore_parser.clone())
                                        .map(|v| Box::new(primary_expression::_S11::new(v))),
                                )
                                .repeated(),
                        )
                        .map(repetition_mapper)
                        .map(|v| Box::new(primary_expression::_S10::new(v))),
                )
                .then(ignore_parser.clone())
                .then(just(']').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(primary_expression::_S9::new(v)))
                .map(|v| Box::new(primary_expression::_C0::_S9(v))),
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

        // ReceiveFunctionAttribute = 'external' | 'payable' | ModifierInvocation | 'virtual' | OverrideSpecifier ;
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
                .ignored()
                .map(|_| FixedTerminal::<7usize>())
                .map(|v| Box::new(receive_function_attribute::_C0::Virtual(v))),
            override_specifier_parser
                .clone()
                .map(|v| Box::new(receive_function_attribute::_C0::OverrideSpecifier(v))),
        ))
        .boxed();

        // StructDefinition = 'struct' «Identifier» '{' 1…*{ TypeName «Identifier» ';' } '}' ;
        let struct_definition_parser = terminal("struct")
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
                    .then(ignore_parser.clone())
                    .then(identifier_parser.clone())
                    .then(ignore_parser.clone())
                    .then(just(';').map(|_| FixedTerminal::<1>()))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(struct_definition::_S2::new(v)))
                    .repeated()
                    .at_least(1usize),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(struct_definition::_S0::new(v)))
            .boxed();

        // UsingDirective = 'using' ( IdentifierPath | '{' 1…*{ IdentifierPath / ',' } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
        let using_directive_parser = terminal("using")
            .ignored()
            .map(|_| FixedTerminal::<5usize>())
            .then(ignore_parser.clone())
            .then(choice((
                identifier_path_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_C1::IdentifierPath(v))),
                just('{')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(
                        identifier_path_parser
                            .clone()
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(using_directive::_S4::new(v)))
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(using_directive::_S5::new(v)))
                                    .then(
                                        identifier_path_parser
                                            .clone()
                                            .then(ignore_parser.clone())
                                            .map(|v| Box::new(using_directive::_S4::new(v))),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(using_directive::_S3::new(v))),
                    )
                    .then(ignore_parser.clone())
                    .then(just('}').map(|_| FixedTerminal::<1>()))
                    .map(|v| Box::new(using_directive::_S2::new(v)))
                    .map(|v| Box::new(using_directive::_C1::_S2(v))),
            )))
            .then(ignore_parser.clone())
            .then(terminal("for").ignored().map(|_| FixedTerminal::<3usize>()))
            .then(ignore_parser.clone())
            .then(choice((
                terminal("*")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(using_directive::_C6::Star(v))),
                type_name_parser
                    .clone()
                    .map(|v| Box::new(using_directive::_C6::TypeName(v))),
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
            .map(|v| Box::new(using_directive::_S0::new(v)))
            .boxed();

        // VariableDeclaration = TypeName [ DataLocation ] «Identifier» ;
        let variable_declaration_parser = type_name_parser
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
            .map(|v| Box::new(variable_declaration::_S0::new(v)))
            .boxed();

        // YulBlock = '{' { YulStatement } '}' ;
        yul_block_parser.define(
            just('{')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(
                    yul_statement_parser
                        .clone()
                        .then(ignore_parser.clone())
                        .map(|v| Box::new(yul_block::_S2::new(v)))
                        .repeated(),
                )
                .then(ignore_parser.clone())
                .then(just('}').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(yul_block::_S0::new(v)))
                .boxed(),
        );

        // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
        let assembly_statement_parser = terminal("assembly")
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
            .map(|v| Box::new(assembly_statement::_S0::new(v)))
            .boxed();

        // Directive = «PragmaDirective» | ImportDirective | UsingDirective ;
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

        // ErrorDefinition = 'error' «Identifier» '(' { ErrorParameter / ',' } ')' ';' ;
        let error_definition_parser = terminal("error")
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
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(error_definition::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(error_definition::_S3::new(v)))
                            .then(
                                error_parameter_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(error_definition::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(error_definition::_S1::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(error_definition::_S0::new(v)))
            .boxed();

        // EventDefinition = 'event' «Identifier» '(' { EventParameter / ',' } ')' [ 'anonymous' ] ';' ;
        let event_definition_parser = terminal("event")
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
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(event_definition::_S2::new(v)))
                    .then(
                        just(',')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(event_definition::_S3::new(v)))
                            .then(
                                event_parameter_parser
                                    .clone()
                                    .then(ignore_parser.clone())
                                    .map(|v| Box::new(event_definition::_S2::new(v))),
                            )
                            .repeated(),
                    )
                    .map(repetition_mapper)
                    .map(|v| Box::new(event_definition::_S1::new(v)))
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
            .map(|v| Box::new(event_definition::_S0::new(v)))
            .boxed();

        // IndexAccessExpression = PrimaryExpression { '[' [ Expression ] [ ':' [ Expression ] ] ']' } ;
        let index_access_expression_parser = primary_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('[')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone().or_not())
                    .then(ignore_parser.clone())
                    .then(
                        just(':')
                            .map(|_| FixedTerminal::<1>())
                            .then(ignore_parser.clone())
                            .then(expression_parser.clone().or_not())
                            .map(|v| Box::new(index_access_expression::_S5::new(v)))
                            .or_not(),
                    )
                    .then(ignore_parser.clone())
                    .then(just(']').map(|_| FixedTerminal::<1>()))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(index_access_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(index_access_expression::_S0::new(v)))
            .boxed();

        // VariableDeclarationTuple = '(' { ',' } VariableDeclaration { ',' [ VariableDeclaration ] } ')' ;
        let variable_declaration_tuple_parser = just('(')
            .map(|_| FixedTerminal::<1>())
            .then(ignore_parser.clone())
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(variable_declaration_tuple::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(variable_declaration_parser.clone())
            .then(ignore_parser.clone())
            .then(
                just(',')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(variable_declaration_parser.clone().or_not())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(variable_declaration_tuple::_S4::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(variable_declaration_tuple::_S0::new(v)))
            .boxed();

        // MemberAccessExpression = IndexAccessExpression { '.' ( «Identifier» | 'address' ) } ;
        let member_access_expression_parser = index_access_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('.')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(choice((
                        identifier_parser
                            .clone()
                            .map(|v| Box::new(member_access_expression::_C3::Identifier(v))),
                        terminal("address")
                            .ignored()
                            .map(|_| FixedTerminal::<7usize>())
                            .map(|v| Box::new(member_access_expression::_C3::Address(v))),
                    )))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(member_access_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(member_access_expression::_S0::new(v)))
            .boxed();

        // FunctionCallOptionsExpression = MemberAccessExpression { '{' 1…*{ NamedArgument / ',' } '}' } ;
        let function_call_options_expression_parser = member_access_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('{')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(
                        named_argument_parser
                            .clone()
                            .then(ignore_parser.clone())
                            .map(|v| Box::new(function_call_options_expression::_S4::new(v)))
                            .then(
                                just(',')
                                    .map(|_| FixedTerminal::<1>())
                                    .then(ignore_parser.clone())
                                    .map(|v| {
                                        Box::new(function_call_options_expression::_S5::new(v))
                                    })
                                    .then(
                                        named_argument_parser
                                            .clone()
                                            .then(ignore_parser.clone())
                                            .map(|v| {
                                                Box::new(
                                                    function_call_options_expression::_S4::new(v),
                                                )
                                            }),
                                    )
                                    .repeated(),
                            )
                            .map(repetition_mapper)
                            .map(|v| Box::new(function_call_options_expression::_S3::new(v))),
                    )
                    .then(ignore_parser.clone())
                    .then(just('}').map(|_| FixedTerminal::<1>()))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(function_call_options_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(function_call_options_expression::_S0::new(v)))
            .boxed();

        // FunctionCallExpression = FunctionCallOptionsExpression { ArgumentList } ;
        let function_call_expression_parser = function_call_options_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                argument_list_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(function_call_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(function_call_expression::_S0::new(v)))
            .boxed();

        // UnaryPrefixExpression = [ UnaryPrefixOperator ] FunctionCallExpression ;
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
        .or_not()
        .then(ignore_parser.clone())
        .then(function_call_expression_parser.clone())
        .map(|v| Box::new(unary_prefix_expression::_S0::new(v)))
        .boxed();

        // UnarySuffixExpression = UnaryPrefixExpression [ UnarySuffixOperator ] ;
        let unary_suffix_expression_parser = unary_prefix_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice::<_, ErrorType>((terminal("++").ignored(), terminal("--").ignored()))
                    .map(|_| FixedTerminal::<2usize>())
                    .or_not(),
            )
            .map(|v| Box::new(unary_suffix_expression::_S0::new(v)))
            .boxed();

        // ExpExpression = UnarySuffixExpression { '**' Expression } ;
        let exp_expression_parser = unary_suffix_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("**")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(exp_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(exp_expression::_S0::new(v)))
            .boxed();

        // MulDivModExpression = ExpExpression { MulDivModOperator ExpExpression } ;
        let mul_div_mod_expression_parser = exp_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                filter(|&c: &char| c == '*' || c == '/' || c == '%')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(exp_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(mul_div_mod_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(mul_div_mod_expression::_S0::new(v)))
            .boxed();

        // AddSubExpression = MulDivModExpression { AddSubOperator MulDivModExpression } ;
        let add_sub_expression_parser = mul_div_mod_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                filter(|&c: &char| c == '+' || c == '-')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(mul_div_mod_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(add_sub_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(add_sub_expression::_S0::new(v)))
            .boxed();

        // ShiftExpression = AddSubExpression { ShiftOperator AddSubExpression } ;
        let shift_expression_parser = add_sub_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice::<_, ErrorType>((
                    terminal("<<").map(|_| 2usize),
                    terminal(">>").ignore_then(choice((
                        terminal(">").map(|_| 3usize),
                        empty().map(|_| 2usize),
                    ))),
                ))
                .then(ignore_parser.clone())
                .then(add_sub_expression_parser.clone())
                .then(ignore_parser.clone())
                .map(|v| Box::new(shift_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(shift_expression::_S0::new(v)))
            .boxed();

        // BitAndExpression = ShiftExpression { '&' ShiftExpression } ;
        let bit_and_expression_parser = shift_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('&')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(shift_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(bit_and_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_and_expression::_S0::new(v)))
            .boxed();

        // BitXOrExpression = BitAndExpression { '^' BitAndExpression } ;
        let bit_x_or_expression_parser = bit_and_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('^')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(bit_and_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(bit_x_or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_x_or_expression::_S0::new(v)))
            .boxed();

        // BitOrExpression = BitXOrExpression { '|' BitXOrExpression } ;
        let bit_or_expression_parser = bit_x_or_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('|')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(bit_x_or_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(bit_or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(bit_or_expression::_S0::new(v)))
            .boxed();

        // OrderComparisonExpression = BitOrExpression { OrderComparisonOperator BitOrExpression } ;
        let order_comparison_expression_parser = bit_or_expression_parser
            .clone()
            .then(ignore_parser.clone())
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
                .then(ignore_parser.clone())
                .then(bit_or_expression_parser.clone())
                .then(ignore_parser.clone())
                .map(|v| Box::new(order_comparison_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(order_comparison_expression::_S0::new(v)))
            .boxed();

        // EqualityComparisonExpression = OrderComparisonExpression { EqualityComparisonOperator OrderComparisonExpression } ;
        let equality_comparison_expression_parser = order_comparison_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice::<_, ErrorType>((terminal("!=").ignored(), terminal("==").ignored()))
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(order_comparison_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(equality_comparison_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(equality_comparison_expression::_S0::new(v)))
            .boxed();

        // AndExpression = EqualityComparisonExpression { '&&' EqualityComparisonExpression } ;
        let and_expression_parser = equality_comparison_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("&&")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(equality_comparison_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(and_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(and_expression::_S0::new(v)))
            .boxed();

        // OrExpression = AndExpression { '||' AndExpression } ;
        let or_expression_parser = and_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                terminal("||")
                    .ignored()
                    .map(|_| FixedTerminal::<2usize>())
                    .then(ignore_parser.clone())
                    .then(and_expression_parser.clone())
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(or_expression::_S2::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(or_expression::_S0::new(v)))
            .boxed();

        // ConditionalExpression = OrExpression [ '?' Expression ':' Expression ] ;
        let conditional_expression_parser = or_expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                just('?')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone())
                    .then(ignore_parser.clone())
                    .then(just(':').map(|_| FixedTerminal::<1>()))
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone())
                    .map(|v| Box::new(conditional_expression::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(conditional_expression::_S0::new(v)))
            .boxed();

        // AssignmentExpression = ConditionalExpression { AssignmentOperator Expression } ;
        let assignment_expression_parser = conditional_expression_parser
            .clone()
            .then(ignore_parser.clone())
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
                .then(ignore_parser.clone())
                .then(expression_parser.clone())
                .then(ignore_parser.clone())
                .map(|v| Box::new(assignment_expression::_S2::new(v)))
                .repeated(),
            )
            .map(|v| Box::new(assignment_expression::_S0::new(v)))
            .boxed();

        // Expression = AssignmentExpression ;
        expression_parser.define(assignment_expression_parser.clone().boxed());

        // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
        let constant_definition_parser = type_name_parser
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
            .map(|v| Box::new(constant_definition::_S0::new(v)))
            .boxed();

        // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
        let do_while_statement_parser = terminal("do")
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
            .map(|v| Box::new(do_while_statement::_S0::new(v)))
            .boxed();

        // EmitStatement = 'emit' Expression ArgumentList ';' ;
        let emit_statement_parser = terminal("emit")
            .ignored()
            .map(|_| FixedTerminal::<4usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(emit_statement::_S0::new(v)))
            .boxed();

        // ExpressionStatement = Expression ';' ;
        let expression_statement_parser = expression_parser
            .clone()
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(expression_statement::_S0::new(v)))
            .boxed();

        // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
        let if_statement_parser = terminal("if")
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
                    .then(ignore_parser.clone())
                    .then(statement_parser.clone())
                    .map(|v| Box::new(if_statement::_S2::new(v)))
                    .or_not(),
            )
            .map(|v| Box::new(if_statement::_S0::new(v)))
            .boxed();

        // ReturnStatement = 'return' [ Expression ] ';' ;
        let return_statement_parser = terminal("return")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(return_statement::_S0::new(v)))
            .boxed();

        // RevertStatement = 'revert' Expression ArgumentList ';' ;
        let revert_statement_parser = terminal("revert")
            .ignored()
            .map(|_| FixedTerminal::<6usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(argument_list_parser.clone())
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(revert_statement::_S0::new(v)))
            .boxed();

        // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
        let state_variable_declaration_parser = type_name_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                state_variable_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(state_variable_declaration::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(
                just('=')
                    .map(|_| FixedTerminal::<1>())
                    .then(ignore_parser.clone())
                    .then(expression_parser.clone())
                    .map(|v| Box::new(state_variable_declaration::_S4::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(just(';').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(state_variable_declaration::_S0::new(v)))
            .boxed();

        // TryStatement = 'try' Expression [ 'returns' NonEmptyParameterList ] Block CatchClause { CatchClause } ;
        let try_statement_parser = terminal("try")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(expression_parser.clone())
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(ignore_parser.clone())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(try_statement::_S2::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .then(ignore_parser.clone())
            .then(catch_clause_parser.clone())
            .then(ignore_parser.clone())
            .then(
                catch_clause_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(try_statement::_S4::new(v)))
                    .repeated(),
            )
            .map(|v| Box::new(try_statement::_S0::new(v)))
            .boxed();

        // VariableDeclarationStatement = ( VariableDeclaration [ '=' Expression ] | VariableDeclarationTuple '=' Expression ) ';' ;
        let variable_declaration_statement_parser = choice((
            variable_declaration_parser
                .clone()
                .then(ignore_parser.clone())
                .then(
                    just('=')
                        .map(|_| FixedTerminal::<1>())
                        .then(ignore_parser.clone())
                        .then(expression_parser.clone())
                        .map(|v| Box::new(variable_declaration_statement::_S4::new(v)))
                        .or_not(),
                )
                .map(|v| Box::new(variable_declaration_statement::_S2::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_C1::_S2(v))),
            variable_declaration_tuple_parser
                .clone()
                .then(ignore_parser.clone())
                .then(just('=').map(|_| FixedTerminal::<1>()))
                .then(ignore_parser.clone())
                .then(expression_parser.clone())
                .map(|v| Box::new(variable_declaration_statement::_S5::new(v)))
                .map(|v| Box::new(variable_declaration_statement::_C1::_S5(v))),
        ))
        .then(ignore_parser.clone())
        .then(just(';').map(|_| FixedTerminal::<1>()))
        .map(|v| Box::new(variable_declaration_statement::_S0::new(v)))
        .boxed();

        // WhileStatement = 'while' '(' Expression ')' Statement ;
        let while_statement_parser = terminal("while")
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
            .map(|v| Box::new(while_statement::_S0::new(v)))
            .boxed();

        // SimpleStatement = VariableDeclarationStatement | ExpressionStatement ;
        let simple_statement_parser = choice((
            variable_declaration_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_C0::VariableDeclarationStatement(v))),
            expression_statement_parser
                .clone()
                .map(|v| Box::new(simple_statement::_C0::ExpressionStatement(v))),
        ))
        .boxed();

        // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
        let for_statement_parser = terminal("for")
            .ignored()
            .map(|_| FixedTerminal::<3usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(choice((
                simple_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_C1::SimpleStatement(v))),
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(for_statement::_C1::Semicolon(v))),
            )))
            .then(ignore_parser.clone())
            .then(choice((
                expression_statement_parser
                    .clone()
                    .map(|v| Box::new(for_statement::_C2::ExpressionStatement(v))),
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(for_statement::_C2::Semicolon(v))),
            )))
            .then(ignore_parser.clone())
            .then(expression_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(statement_parser.clone())
            .map(|v| Box::new(for_statement::_S0::new(v)))
            .boxed();

        // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | AssemblyStatement ;
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

        // Block = '{' { Statement | UncheckedBlock } '}' ;
        block_parser.define(
            just('{')
                .map(|_| FixedTerminal::<1>())
                .then(ignore_parser.clone())
                .then(
                    choice((
                        statement_parser
                            .clone()
                            .map(|v| Box::new(block::_C2::Statement(v))),
                        unchecked_block_parser
                            .clone()
                            .map(|v| Box::new(block::_C2::UncheckedBlock(v))),
                    ))
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(block::_S3::new(v)))
                    .repeated(),
                )
                .then(ignore_parser.clone())
                .then(just('}').map(|_| FixedTerminal::<1>()))
                .map(|v| Box::new(block::_S0::new(v)))
                .boxed(),
        );

        // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
        let constructor_definition_parser = terminal("constructor")
            .ignored()
            .map(|_| FixedTerminal::<11usize>())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(
                constructor_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(constructor_definition::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(block_parser.clone())
            .map(|v| Box::new(constructor_definition::_S0::new(v)))
            .boxed();

        // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let fallback_function_definition_parser = terminal("fallback")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(
                fallback_function_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(fallback_function_definition::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(ignore_parser.clone())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(fallback_function_definition::_S4::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(fallback_function_definition::_C5::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(fallback_function_definition::_C5::Block(v))),
            )))
            .map(|v| Box::new(fallback_function_definition::_S0::new(v)))
            .boxed();

        // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' NonEmptyParameterList ] ( ';' | Block ) ;
        let function_definition_parser = terminal("function")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
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
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone())
            .then(ignore_parser.clone())
            .then(
                function_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(function_definition::_S3::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(
                terminal("returns")
                    .ignored()
                    .map(|_| FixedTerminal::<7usize>())
                    .then(ignore_parser.clone())
                    .then(non_empty_parameter_list_parser.clone())
                    .map(|v| Box::new(function_definition::_S5::new(v)))
                    .or_not(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(function_definition::_C6::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(function_definition::_C6::Block(v))),
            )))
            .map(|v| Box::new(function_definition::_S0::new(v)))
            .boxed();

        // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { MethodAttribute } ( ';' | Block ) ;
        let modifier_definition_parser = terminal("modifier")
            .ignored()
            .map(|_| FixedTerminal::<8usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(parameter_list_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(
                method_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(modifier_definition::_S3::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(modifier_definition::_C4::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(modifier_definition::_C4::Block(v))),
            )))
            .map(|v| Box::new(modifier_definition::_S0::new(v)))
            .boxed();

        // ReceiveFunctionDefinition = 'receive' '(' ')' { ReceiveFunctionAttribute } ( ';' | Block ) ;
        let receive_function_definition_parser = terminal("receive")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(just('(').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(just(')').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                receive_function_attribute_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(receive_function_definition::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(choice((
                terminal(";")
                    .ignored()
                    .map(|_| FixedTerminal::<1usize>())
                    .map(|v| Box::new(receive_function_definition::_C3::Semicolon(v))),
                block_parser
                    .clone()
                    .map(|v| Box::new(receive_function_definition::_C3::Block(v))),
            )))
            .map(|v| Box::new(receive_function_definition::_S0::new(v)))
            .boxed();

        // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
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

        // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let contract_definition_parser = terminal("abstract")
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
            .then(
                contract_body_element_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(contract_definition::_S4::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(contract_definition::_S0::new(v)))
            .boxed();

        // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
        let interface_definition_parser = terminal("interface")
            .ignored()
            .map(|_| FixedTerminal::<9usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(inheritance_specifier_list_parser.clone().or_not())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                contract_body_element_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(interface_definition::_S3::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(interface_definition::_S0::new(v)))
            .boxed();

        // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
        let library_definition_parser = terminal("library")
            .ignored()
            .map(|_| FixedTerminal::<7usize>())
            .then(ignore_parser.clone())
            .then(identifier_parser.clone())
            .then(ignore_parser.clone())
            .then(just('{').map(|_| FixedTerminal::<1>()))
            .then(ignore_parser.clone())
            .then(
                contract_body_element_parser
                    .clone()
                    .then(ignore_parser.clone())
                    .map(|v| Box::new(library_definition::_S2::new(v)))
                    .repeated(),
            )
            .then(ignore_parser.clone())
            .then(just('}').map(|_| FixedTerminal::<1>()))
            .map(|v| Box::new(library_definition::_S0::new(v)))
            .boxed();

        // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
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

        // SourceUnit = «IGNORE» { Directive | Definition } $ ;
        let source_unit_parser = ignore_parser
            .clone()
            .then(ignore_parser.clone())
            .then(
                choice((
                    directive_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_C2::Directive(v))),
                    definition_parser
                        .clone()
                        .map(|v| Box::new(source_unit::_C2::Definition(v))),
                ))
                .then(ignore_parser.clone())
                .map(|v| Box::new(source_unit::_S3::new(v)))
                .repeated(),
            )
            .then(ignore_parser.clone())
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
