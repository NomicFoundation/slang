//! This module contains certain nodes and functions used internally by the parser.
//!
//! They shouldn't be used outside of the parser, and should be transformed into AST nodes.

use crate::parser::consumer::ParserConsumer;

/// An `IndexAccessPath` represents a path or elementary type followed by
/// zero or more index accesses, e.g. `foo.bar[0][1:3]` or `uint256[5][]`
///
/// It's heavily inspired by solc
/// <https://github.com/argotorg/solidity/blob/194b114664c7daebc2ff68af3c573272f5d28913/libsolidity/parsing/Parser.h#L198-L209>
pub struct IndexAccessPath<C: ParserConsumer> {
    pub path: Path<C>,
    pub indices: Vec<Index<C>>,
}

pub enum Path<C: ParserConsumer> {
    IdentifierPath(C::IdentifierPath),
    ElementaryType(C::ElementaryType),
}

pub struct Index<C: ParserConsumer> {
    pub open_bracket: C::OpenBracket,
    pub start: Option<C::Expression>,
    pub end: Option<C::IndexAccessEnd>,
    pub close_bracket: C::CloseBracket,
}

/// Given an IAP it adds a new index to it
pub(crate) fn index_access_path_add_index<C: ParserConsumer>(
    mut iap: IndexAccessPath<C>,
    open_bracket: C::OpenBracket,
    start: Option<C::Expression>,
    end: Option<C::IndexAccessEnd>,
    close_bracket: C::CloseBracket,
) -> IndexAccessPath<C> {
    iap.indices.push(Index {
        open_bracket,
        start,
        end,
        close_bracket,
    });
    iap
}

/// Creates an IAP from an identifier path
pub(crate) fn new_index_access_path_from_identifier_path<C: ParserConsumer>(
    identifier_path: C::IdentifierPath,
) -> IndexAccessPath<C> {
    IndexAccessPath {
        path: Path::IdentifierPath(identifier_path),
        indices: vec![],
    }
}

/// Creates a new IAP from an elementary type
pub(crate) fn new_index_access_path_from_elementary_type<C: ParserConsumer>(
    elementary_type: C::ElementaryType,
) -> IndexAccessPath<C> {
    IndexAccessPath {
        path: Path::ElementaryType(elementary_type),
        indices: vec![],
    }
}

/// Consumes an IAP and creates a `TypeName` by delegating to the consumer
pub(crate) fn new_type_name_index_access_path<C: ParserConsumer>(
    consumer: &C,
    index_access_path: IndexAccessPath<C>,
) -> C::TypeName {
    consumer.make_type_name_from_index_access_path(index_access_path)
}

/// Consumes an IAP and returns an Expression by delegating to the consumer
pub(crate) fn new_expression_index_access_path<C: ParserConsumer>(
    consumer: &C,
    index_access_path: IndexAccessPath<C>,
) -> C::Expression {
    consumer.make_expression_from_index_access_path(index_access_path)
}
