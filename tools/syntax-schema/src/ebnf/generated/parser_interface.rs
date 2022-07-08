use chumsky::prelude::{BoxedParser, Simple};
pub type ErrorType = Simple<char>;
pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
use super::tree_interface::*;

#[allow(dead_code)]
pub struct Parsers {
    pub comment: ParserType<Comment>,
    pub number: ParserType<Number>,
    pub raw_identifier: ParserType<RawIdentifier>,
    pub string_char: ParserType<StringChar>,
    pub whitespace: ParserType<Whitespace>,
    pub identifier: ParserType<Identifier>,
    pub leading_trivia: ParserType<LeadingTrivia>,
    pub single_char_string: ParserType<SingleCharString>,
    pub string: ParserType<String>,
    pub trailing_trivia: ParserType<TrailingTrivia>,
    pub char_range: ParserType<CharRange>,
    pub grouped: ParserType<Grouped>,
    pub optional: ParserType<Optional>,
    pub production_reference: ParserType<ProductionReference>,
    pub repetition_prefix: ParserType<RepetitionPrefix>,
    pub repetition_separator: ParserType<RepetitionSeparator>,
    pub repeated: ParserType<Repeated>,
    pub primary: ParserType<Primary>,
    pub negation: ParserType<Negation>,
    pub difference: ParserType<Difference>,
    pub sequence: ParserType<Sequence>,
    pub expression: ParserType<Expression>,
    pub production: ParserType<Production>,
    pub grammar: ParserType<Grammar>,
}
