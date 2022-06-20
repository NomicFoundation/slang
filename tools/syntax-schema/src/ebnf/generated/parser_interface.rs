use chumsky::prelude::{BoxedParser, Simple};
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
use super::tree_interface::*;
#[allow(dead_code)]
pub struct Parsers {
    pub comment: ParserType<comment::N>,
    pub eof: ParserType<eof::N>,
    pub hex_digit: ParserType<hex_digit::N>,
    pub identifier_start: ParserType<identifier_start::N>,
    pub number: ParserType<number::N>,
    pub whitespace: ParserType<whitespace::N>,
    pub ignore: ParserType<ignore::N>,
    pub identifier_follow: ParserType<identifier_follow::N>,
    pub string_char: ParserType<string_char::N>,
    pub raw_identifier: ParserType<raw_identifier::N>,
    pub single_char_string: ParserType<single_char_string::N>,
    pub string: ParserType<string::N>,
    pub grouped: ParserType<grouped::N>,
    pub optional: ParserType<optional::N>,
    pub repetition_prefix: ParserType<repetition_prefix::N>,
    pub repetition_separator: ParserType<repetition_separator::N>,
    pub identifier: ParserType<identifier::N>,
    pub char_range: ParserType<char_range::N>,
    pub repeated: ParserType<repeated::N>,
    pub production_reference: ParserType<production_reference::N>,
    pub primary: ParserType<primary::N>,
    pub negation: ParserType<negation::N>,
    pub difference: ParserType<difference::N>,
    pub sequence: ParserType<sequence::N>,
    pub expression: ParserType<expression::N>,
    pub production: ParserType<production::N>,
    pub grammar: ParserType<grammar::N>,
}
