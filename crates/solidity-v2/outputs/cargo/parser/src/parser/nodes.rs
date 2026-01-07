//! This module contains certain nodes and functions used internally by the parser.
//!
//! They shouldn't be used outside of the parser, and should be transformed into AST nodes.

use bumpalo::boxed::Box;
use bumpalo::collections::{CollectIn, Vec};
use bumpalo::Bump;
use slang_solidity_v2_cst::structured_cst::nodes::*;

/// An IndexAccessPath represents a path or elementary type followed by
/// zero or more index accesses, e.g. foo.bar[0][1:3] or uint256[5][]
///
/// It's heavily inspired by solc
/// https://github.com/argotorg/solidity/blob/194b114664c7daebc2ff68af3c573272f5d28913/libsolidity/parsing/Parser.h#L198-L209
#[derive(Debug)]
pub(crate) struct IndexAccessPath<'arena> {
    pub path: Path<'arena>,
    pub indices: Vec<'arena, Index<'arena>>,
}

#[derive(Debug)]
pub(crate) enum Path<'arena> {
    IdentifierPath(IdentifierPath<'arena>),
    ElementaryType(ElementaryType<'arena>),
}

#[derive(Debug)]
pub(crate) struct Index<'arena> {
    pub open_bracket: OpenBracket<'arena>,
    pub start: Option<Expression<'arena>>,
    pub end: Option<IndexAccessEnd<'arena>>,
    pub close_bracket: CloseBracket<'arena>,
}

/// Given an IAP it adds a new index to it
pub(crate) fn index_access_path_add_index<'arena>(
    _arena: &'arena Bump,
    mut iap: IndexAccessPath<'arena>,
    open_bracket: OpenBracket<'arena>,
    start: Option<Expression<'arena>>,
    end: Option<IndexAccessEnd<'arena>>,
    close_bracket: CloseBracket<'arena>,
) -> IndexAccessPath<'arena> {
    iap.indices.push(Index {
        open_bracket,
        start,
        end,
        close_bracket,
    });
    iap
}

/// Creates an IAP from an identifier path
pub(crate) fn new_index_access_path_from_identifier_path<'arena>(
    arena: &'arena Bump,
    identifier_path: IdentifierPath<'arena>,
) -> IndexAccessPath<'arena> {
    IndexAccessPath {
        path: Path::IdentifierPath(identifier_path),
        indices: Vec::new_in(arena),
    }
}

/// Creates a new IAP from an elementary type
pub(crate) fn new_index_access_path_from_elementary_type<'arena>(
    arena: &'arena Bump,
    elementary_type: ElementaryType<'arena>,
) -> IndexAccessPath<'arena> {
    IndexAccessPath {
        path: Path::ElementaryType(elementary_type),
        indices: Vec::new_in(arena),
    }
}

/// Consumes an IAP and creates a TypeName
///
/// TODO(v2): Return an error if any index has slicing rather than panicing
pub(crate) fn new_type_name_index_access_path<'arena>(
    arena: &'arena Bump,
    index_access_path: IndexAccessPath<'arena>,
) -> TypeName<'arena> {
    let IndexAccessPath { path, indices } = index_access_path;

    let mut type_name = match path {
        Path::IdentifierPath(path) => new_type_name_identifier_path(arena, path),
        Path::ElementaryType(elem_type) => new_type_name_elementary_type(arena, elem_type),
    };

    for index in indices.into_iter() {
        assert!(
            index.end.is_none(),
            "Slicing is not supported in type names yet"
        );
        let array_type = new_array_type_name(
            arena,
            type_name,
            index.open_bracket,
            index.start,
            index.close_bracket,
        );
        type_name = new_type_name_array_type_name(arena, array_type);
    }

    type_name
}

/// Consumes an IAP and returns an Expression
pub(crate) fn new_expression_index_access_path<'arena>(
    arena: &'arena Bump,
    index_access_path: IndexAccessPath<'arena>,
) -> Expression<'arena> {
    let IndexAccessPath { path, indices } = index_access_path;

    let mut expression = match path {
        Path::IdentifierPath(path) => new_expression_identifier_path(arena, path),
        Path::ElementaryType(elem_type) => new_expression_elementary_type(arena, elem_type),
    };

    for index in indices.into_iter() {
        let array_expression = new_index_access_expression(
            arena,
            expression,
            index.open_bracket,
            index.start,
            index.end,
            index.close_bracket,
        );
        expression = new_expression_index_access_expression(arena, array_expression);
    }

    expression
}

#[derive(Debug)]
pub(crate) struct ProtoTuple<'arena> {
    // Do we care about range in source code?
    pub _open_paren: OpenParen<'arena>,
    pub _elements: Vec<'arena, ProtoTupleElement<'arena>>,
    pub _close_paren: CloseParen<'arena>,
}

pub(crate) fn new_proto_tuple<'arena>(
    _arena: &'arena Bump,
    _open_paren: OpenParen<'arena>,
    _elements: Vec<'arena, ProtoTupleElement<'arena>>,
    _close_paren: CloseParen<'arena>,
) -> ProtoTuple<'arena> {
    ProtoTuple {
        _open_paren,
        _elements,
        _close_paren,
    }
}

pub(crate) fn new_tuple_expression_from_proto_tuple<'arena>(
    arena: &'arena Bump,
    proto_tuple: ProtoTuple<'arena>,
) -> TupleExpression<'arena> {
    let elements: Vec<'arena, TupleValue<'arena>> = proto_tuple
        ._elements
        .into_iter()
        .map(|element| match element {
            ProtoTupleElement::Expression(name) => new_tuple_value(arena, Some(name)),
            ProtoTupleElement::Declaration(_) => panic!("Tuples can't have declarations"),
            ProtoTupleElement::StorageLocation(_, _) => {
                panic!("Tuples can't have storage locations")
            }
            ProtoTupleElement::Empty => new_tuple_value(arena, None),
        })
        .collect_in(arena);
    new_tuple_expression(
        arena,
        proto_tuple._open_paren,
        new_tuple_values(arena, elements),
        proto_tuple._close_paren,
    )
}

pub(crate) fn new_tuple_deconstruction_statement_from_proto_tuple<'arena>(
    arena: &'arena Bump,
    proto_tuple: ProtoTuple<'arena>,
    equal: Equal<'arena>,
    expression: Expression<'arena>,
    semicolon: Semicolon<'arena>,
) -> TupleDeconstructionStatement<'arena> {
    let elements: Vec<'arena, TupleDeconstructionElement<'arena>> = proto_tuple
        ._elements
        .into_iter()
        .map(|element| match element {
            ProtoTupleElement::Expression(name) => match name {
                Expression::Identifier(name) => new_tuple_deconstruction_element(
                    arena,
                    Some(new_tuple_member_untyped_tuple_member(
                        arena,
                        new_untyped_tuple_member(arena, None, name),
                    )),
                ),
                _ => panic!("Tuples deconstruction can only be used with identifiers"),
            },
            ProtoTupleElement::Declaration(decl) => {
                let VariableDeclarationStruct {
                    storage_location,
                    name,
                    variable_type,
                } = Box::into_inner(decl);
                let tuple_member = match variable_type {
                    VariableDeclarationType::TypeName(type_name) => {
                        new_tuple_member_typed_tuple_member(
                            arena,
                            new_typed_tuple_member(arena, type_name, storage_location, name),
                        )
                    }
                    VariableDeclarationType::VarKeyword(_) => {
                        new_tuple_member_untyped_tuple_member(
                            arena,
                            new_untyped_tuple_member(arena, storage_location, name),
                        )
                    }
                };
                new_tuple_deconstruction_element(arena, Some(tuple_member))
            }
            ProtoTupleElement::StorageLocation(storage_location, name) => {
                new_tuple_deconstruction_element(
                    arena,
                    Some(new_tuple_member_untyped_tuple_member(
                        arena,
                        new_untyped_tuple_member(arena, Some(storage_location), name),
                    )),
                )
            }
            ProtoTupleElement::Empty => new_tuple_deconstruction_element(arena, None),
        })
        .collect_in(arena);
    new_tuple_deconstruction_statement(
        arena,
        None,
        proto_tuple._open_paren,
        new_tuple_deconstruction_elements(arena, elements),
        proto_tuple._close_paren,
        equal,
        expression,
        semicolon,
    )
}

#[derive(Debug)]
pub(crate) enum ProtoTupleElement<'arena> {
    Expression(Expression<'arena>),
    Declaration(VariableDeclaration<'arena>),
    StorageLocation(StorageLocation<'arena>, Identifier<'arena>),
    Empty,
}

pub(crate) fn new_proto_tuple_element_expression<'arena>(
    _arena: &'arena Bump,
    name: Expression<'arena>,
) -> ProtoTupleElement<'arena> {
    ProtoTupleElement::Expression(name)
}

pub(crate) fn new_proto_tuple_element_declaration<'arena>(
    _arena: &'arena Bump,
    decl: VariableDeclaration<'arena>,
) -> ProtoTupleElement<'arena> {
    ProtoTupleElement::Declaration(decl)
}

pub(crate) fn new_proto_tuple_element_storage_location<'arena>(
    _arena: &'arena Bump,
    storage_location: StorageLocation<'arena>,
    name: Identifier<'arena>,
) -> ProtoTupleElement<'arena> {
    ProtoTupleElement::StorageLocation(storage_location, name)
}

pub(crate) fn new_proto_tuple_element_empty<'arena>(
    _arena: &'arena Bump,
) -> ProtoTupleElement<'arena> {
    ProtoTupleElement::Empty
}

pub(crate) fn new_expression_identifier_path<'arena>(
    arena: &'arena Bump,
    identifier_path: IdentifierPath<'arena>,
) -> Expression<'arena> {
    let IdentifierPathStruct { head, tail } = Box::into_inner(identifier_path);
    match tail {
        None => new_expression_identifier(arena, head),
        Some(tail) => Box::into_inner(tail).elements.elements.into_iter().fold(
            new_expression_identifier(arena, head),
            |acc, id| {
                new_expression_member_access_expression(
                    arena,
                    new_member_access_expression(
                        arena,
                        acc,
                        Period {
                            l: 0,
                            r: 0,
                            phantom: std::marker::PhantomData,
                        },
                        match id {
                            MemberAccessIdentifier::AddressKeyword(ak) => {
                                new_member_access_identifier_address_keyword(arena, ak)
                            }
                            MemberAccessIdentifier::Identifier(id) => {
                                new_member_access_identifier_identifier(arena, id)
                            }
                        },
                    ),
                )
            },
        ),
    }
}

/// We use this function to share attributes between a state variable that has a function type.
/// We find and split the attributes from the function type as needed
/// TODO(v2) fail gracefully if a wrong attribute is found
pub(crate) fn extract_extra_attributes<'arena>(
    arena: &'arena Bump,
    mut fun_type: FunctionType<'arena>,
) -> (
    FunctionType<'arena>,
    Vec<'arena, StateVariableAttribute<'arena>>,
) {
    // Move all matching attributes to extra_attributes if duplicate_found, else only the first occurrence
    let mut seen_constant = false;
    let mut seen_internal = false;
    let mut seen_private = false;
    let mut seen_public = false;
    let mut duplicate_found = false;

    let mut extra_attributes: Vec<'arena, StateVariableAttribute<'arena>> = Vec::new_in(arena);
    fn add_to_extra<'arena>(
        attr: FunctionTypeAttribute<'arena>,
        extra_attributes: &mut Vec<'arena, StateVariableAttribute<'arena>>,
    ) {
        match attr {
      FunctionTypeAttribute::ConstantKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::ConstantKeyword(terminal));
      }
      FunctionTypeAttribute::InternalKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::InternalKeyword(terminal));
      }
      FunctionTypeAttribute::PrivateKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::PrivateKeyword(terminal));
      }
      FunctionTypeAttribute::PublicKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::PublicKeyword(terminal));
      }
      _ => panic!("This is wrong, I don't really know what to do for now, but it should fail gracefully (like a parser error)")
    }
    }

    // This works like extract_if, but we don't have that in this vector
    let mut i = 0;
    let vec = &mut fun_type.attributes.elements;

    while i < vec.len() {
        if duplicate_found {
            let val = vec.remove(i);
            add_to_extra(val, &mut extra_attributes);
        } else {
            let seen = match vec[i] {
                FunctionTypeAttribute::ConstantKeyword(_) => &mut seen_constant,
                FunctionTypeAttribute::InternalKeyword(_) => &mut seen_internal,
                FunctionTypeAttribute::PrivateKeyword(_) => &mut seen_private,
                FunctionTypeAttribute::PublicKeyword(_) => &mut seen_public,
                _ => {
                    i += 1;
                    continue;
                }
            };

            if *seen {
                duplicate_found = true;
                let val = vec.remove(i);
                add_to_extra(val, &mut extra_attributes);
            } else {
                *seen = true;
                i += 1;
            }
        }
    }

    (fun_type, extra_attributes)
}
