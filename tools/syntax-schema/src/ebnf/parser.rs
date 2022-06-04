use chumsky::prelude::*;
use chumsky::Parser;
pub type ErrorType = Simple<char>;

mod tree {
    type C = ();
    type S = usize;
    pub enum Choice2<T0, T1> {
        _0(T0),
        _1(T1),
    }
    pub enum Choice3<T0, T1, T2> {
        _0(T0),
        _1(T1),
        _2(T2),
    }
    pub enum Choice4<T0, T1, T2, T3> {
        _0(T0),
        _1(T1),
        _2(T2),
        _3(T3),
    }
    pub enum Choice7<T0, T1, T2, T3, T4, T5, T6> {
        _0(T0),
        _1(T1),
        _2(T2),
        _3(T3),
        _4(T4),
        _5(T5),
        _6(T6),
    }
    pub struct Grammar(Ignore, Vec<Production>, C);
    pub struct Production(Identifier, C, Expression, C);
    pub struct Expression(Sequence, Vec<(C, Sequence)>);
    pub type Sequence = Vec<Difference>;
    pub struct Difference(Negation, Option<(C, Negation)>);
    pub struct Negation(Option<C>, Primary);
    pub type Primary =
        Choice7<ProductionReference, Grouped, Optional, Repeated, CharRange, Eof, String>;
    pub type ProductionReference = Identifier;
    pub struct Grouped(C, Expression, C);
    pub struct RepetitionPrefix(
        Choice2<(Number, Option<(C, Option<Number>)>), (C, Number)>,
        C,
    );
    pub struct RepetitionSeparator(C, Expression);
    pub struct Repeated(
        Option<RepetitionPrefix>,
        C,
        Expression,
        Option<RepetitionSeparator>,
        C,
    );
    pub struct Optional(C, Expression, C);
    pub struct CharRange(SingleCharString, C, SingleCharString);
    pub type Eof = C;
    pub type Number = Vec<C>;
    pub struct String(C, Vec<StringChar>, C);
    pub type StringChar = Choice2<Choice2<C, C>, (C, Choice3<C, C, (S, Vec<HexDigit>, C)>)>;
    pub struct SingleCharString(C, StringChar, C);
    pub type HexDigit = Choice3<C, C, C>;
    pub type Identifier = Choice2<(C, RawIdentifier, C), RawIdentifier>;
    pub struct RawIdentifier(IdentifierStart, Vec<IdentifierFollow>);
    pub type IdentifierStart = Choice3<C, C, C>;
    pub type IdentifierFollow = Choice2<IdentifierStart, C>;
    pub type Ignore = Vec<Choice2<Whitespace, Comment>>;
    pub struct Comment(S, Vec<Choice2<C, (Vec<C>, Choice2<C, C>)>>, Vec<C>, C);
    pub type Whitespace = Choice4<C, C, C, C>;
}

use super::builder;

use crate::schema::Production;
pub type GrammarParserResultType = Vec<Production>;

pub fn create_grammar_parser() -> impl Parser<char, GrammarParserResultType, Error = ErrorType> {
    let mut expression_parser = Recursive::declare();

    // «Comment» = '/*' { ¬'*' | 1…*{ '*' } ¬( '*' | '/' ) } 1…*{ '*' } '/' ;
    let comment_parser = just("/*")
        .ignore_then(
            choice::<_, ErrorType>((
                filter(|&c: &char| c != '*').ignored().ignored(),
                just('*')
                    .repeated()
                    .at_least(1usize)
                    .then(filter(|&c: &char| c != '*' && c != '/'))
                    .ignored()
                    .ignored(),
            ))
            .repeated(),
        )
        .then(just('*').repeated().at_least(1usize))
        .then_ignore(just('/'))
        .ignored();

    // «EOF» = '$' ;
    let eof_parser = just('$').map(builder::eof);

    // «HexDigit» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    let hex_digit_parser =
        filter(|&c: &char| c.is_ascii_digit() || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F'));

    // «IdentifierStart» = '_' | 'a'…'z' | 'A'…'Z' ;
    let identifier_start_parser =
        filter(|&c: &char| c == '_' || c.is_ascii_lowercase() || c.is_ascii_uppercase());

    // «Number» = 1…*{ '0'…'9' } ;
    let number_parser = filter(|&c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1usize)
        .map(builder::number);

    // «Whitespace» = '\u{9}' | '\u{a}' | '\u{d}' | '\u{20}' ;
    let whitespace_parser =
        filter(|&c: &char| c == '\t' || c == '\n' || c == '\r' || c == ' ').ignored();

    // «IGNORE» = { «Whitespace» | «Comment» } ;
    let ignore_parser = choice::<_, ErrorType>((
        whitespace_parser.clone().ignored(),
        comment_parser.clone().ignored(),
    ))
    .repeated()
    .ignored();

    // «IdentifierFollow» = «IdentifierStart» | '0'…'9' ;
    let identifier_follow_parser = choice::<_, ErrorType>((
        identifier_start_parser.clone(),
        filter(|&c: &char| c.is_ascii_digit()),
    ));

    // «StringChar» = ¬( '\'' | '\\' ) | '\\' ( '\'' | '\\' | 'u{' 1…6*{ «HexDigit» } '}' ) ;
    let string_char_parser = choice::<_, ErrorType>((
        filter(|&c: &char| c != '\'' && c != '\\'),
        just('\\').ignore_then(choice::<_, ErrorType>((
            just('\''),
            just('\\'),
            just("u{")
                .ignore_then(
                    hex_digit_parser
                        .clone()
                        .repeated()
                        .at_least(1usize)
                        .at_most(6usize)
                        .map(builder::hex_digits_to_char)
                        .unwrapped(),
                )
                .then_ignore(just('}')),
        ))),
    ));

    // «RawIdentifier» = «IdentifierStart» { «IdentifierFollow» } ;
    let raw_identifier_parser = identifier_start_parser
        .clone()
        .chain(identifier_follow_parser.clone().repeated())
        .map(builder::raw_identifier);

    // «SingleCharString» = '\'' «StringChar» '\'' ;
    let single_char_string_parser = just('\'')
        .ignore_then(string_char_parser.clone())
        .then_ignore(just('\''));

    // «String» = '\'' { «StringChar» } '\'' ;
    let string_parser = just('\'')
        .ignore_then(string_char_parser.clone().repeated())
        .then_ignore(just('\''))
        .map(builder::string);

    // grouped = '(' expression ')' ;
    let grouped_parser = just('(')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(')').then_ignore(ignore_parser.clone()));

    // optional = '[' expression ']' ;
    let optional_parser = just('[')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone())
        .then_ignore(just(']').then_ignore(ignore_parser.clone()))
        .map(builder::optional);

    // repetitionPrefix = ( «Number» [ '…' [ «Number» ] ] | '…' «Number» ) '*' ;
    let repetition_prefix_parser = choice::<_, ErrorType>((
        number_parser
            .clone()
            .then_ignore(ignore_parser.clone())
            .then(
                just('…')
                    .then_ignore(ignore_parser.clone())
                    .ignore_then(
                        number_parser
                            .clone()
                            .then_ignore(ignore_parser.clone())
                            .or_not(),
                    )
                    .or_not(),
            )
            .map(builder::repetition_prefix_1),
        just('…')
            .then_ignore(ignore_parser.clone())
            .ignore_then(number_parser.clone().then_ignore(ignore_parser.clone()))
            .map(builder::repetition_prefix_2),
    ))
    .then_ignore(just('*').then_ignore(ignore_parser.clone()));

    // repetitionSeparator = '/' expression ;
    let repetition_separator_parser = just('/')
        .then_ignore(ignore_parser.clone())
        .ignore_then(expression_parser.clone());

    // «Identifier» = '«' «RawIdentifier» '»' | «RawIdentifier» ;
    let identifier_parser = choice::<_, ErrorType>((
        just('«')
            .ignore_then(raw_identifier_parser.clone())
            .then_ignore(just('»'))
            .map(builder::identifier_1),
        raw_identifier_parser.clone().map(builder::identifier_2),
    ));

    // charRange = «SingleCharString» '…' «SingleCharString» ;
    let char_range_parser = single_char_string_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('…').then_ignore(ignore_parser.clone()))
        .then(
            single_char_string_parser
                .clone()
                .then_ignore(ignore_parser.clone()),
        )
        .map(builder::char_range);

    // repeated = [ repetitionPrefix ] '{' expression [ repetitionSeparator ] '}' ;
    let repeated_parser = repetition_prefix_parser
        .clone()
        .or_not()
        .then_ignore(just('{').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then(repetition_separator_parser.clone().or_not())
        .then_ignore(just('}').then_ignore(ignore_parser.clone()))
        .map(builder::repeated);

    // productionReference = «Identifier» ;
    let production_reference_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .map(builder::production_reference);

    // primary = productionReference | grouped | optional | repeated | charRange | «EOF» | «String» ;
    let primary_parser = choice::<_, ErrorType>((
        production_reference_parser.clone(),
        grouped_parser.clone(),
        optional_parser.clone(),
        repeated_parser.clone(),
        char_range_parser.clone(),
        eof_parser.clone().then_ignore(ignore_parser.clone()),
        string_parser.clone().then_ignore(ignore_parser.clone()),
    ));

    // negation = [ '¬' ] primary ;
    let negation_parser = just('¬')
        .then_ignore(ignore_parser.clone())
        .or_not()
        .then(primary_parser.clone())
        .map(builder::negation);

    // difference = negation [ '-' negation ] ;
    let difference_parser = negation_parser
        .clone()
        .then(
            just('-')
                .then_ignore(ignore_parser.clone())
                .ignore_then(negation_parser.clone())
                .or_not(),
        )
        .map(builder::difference);

    // sequence = 1…*{ difference } ;
    let sequence_parser = difference_parser
        .clone()
        .repeated()
        .at_least(1usize)
        .map(builder::sequence);

    // expression = 1…*{ sequence / '|' } ;
    expression_parser.define(
        sequence_parser
            .clone()
            .separated_by(just('|').then_ignore(ignore_parser.clone()))
            .at_least(1usize)
            .map(builder::expression),
    );

    // production = «Identifier» '=' expression ';' ;
    let production_parser = identifier_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .then_ignore(just('=').then_ignore(ignore_parser.clone()))
        .then(expression_parser.clone())
        .then_ignore(just(';').then_ignore(ignore_parser.clone()))
        .map(builder::production);

    // grammar = «IGNORE» { production } $ ;
    let grammar_parser = ignore_parser
        .clone()
        .then_ignore(ignore_parser.clone())
        .ignore_then(production_parser.clone().repeated())
        .then_ignore(end());

    grammar_parser.recover_with(skip_then_retry_until([]))
}
