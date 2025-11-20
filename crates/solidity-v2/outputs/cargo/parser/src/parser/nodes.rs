//! This module contains certain nodes and functions used internally by the parser.
//!
//! They shouldn't be used outside of the parser, and should be transformed into AST nodes.

use std::rc::Rc;

use slang_solidity_v2_cst::structured_cst::nodes::*;

/// An IndexAccessPath represents a path or elementary type followed by
/// zero or more index accesses, e.g. foo.bar[0][1:3] or uint256[5][]
///
/// It's heavily inspired by solc
/// https://github.com/argotorg/solidity/blob/194b114664c7daebc2ff68af3c573272f5d28913/libsolidity/parsing/Parser.h#L198-L209
#[derive(Debug)]
pub(crate) struct IndexAccessPath {
    pub path: Path,
    pub indices: Vec<Index>,
}

#[derive(Debug)]
pub(crate) enum Path {
    IdentifierPath(IdentifierPath),
    ElementaryType(ElementaryType),
}

#[derive(Debug)]
pub(crate) struct Index {
    pub open_bracket: OpenBracket,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
    pub close_bracket: CloseBracket,
}

/// Given an IAP it adds a new index to it
pub(crate) fn index_access_path_add_index(
    mut iap: IndexAccessPath,
    open_bracket: OpenBracket,
    start: Option<Expression>,
    end: Option<IndexAccessEnd>,
    close_bracket: CloseBracket,
) -> IndexAccessPath {
    iap.indices.push(Index {
        open_bracket,
        start,
        end,
        close_bracket,
    });
    iap
}

/// Creates an IAP from an identifier path
pub(crate) fn new_index_access_path_from_identifier_path(
    identifier_path: IdentifierPath,
) -> IndexAccessPath {
    IndexAccessPath {
        path: Path::IdentifierPath(identifier_path),
        indices: vec![],
    }
}

/// Creates a new IAP from an elementary type
pub(crate) fn new_index_access_path_from_elementary_type(
    elementary_type: ElementaryType,
) -> IndexAccessPath {
    IndexAccessPath {
        path: Path::ElementaryType(elementary_type),
        indices: vec![],
    }
}

/// Consumes an IAP and creates a TypeName
///
/// TODO(v2): Return an error if any index has slicing rather than panicing
pub(crate) fn new_type_name_index_access_path(index_access_path: IndexAccessPath) -> TypeName {
    let IndexAccessPath { path, indices } = index_access_path;

    let mut type_name = match path {
        Path::IdentifierPath(path) => new_type_name_identifier_path(path),
        Path::ElementaryType(elem_type) => new_type_name_elementary_type(elem_type),
    };

    for index in indices.into_iter() {
        assert!(
            index.end.is_none(),
            "Slicing is not supported in type names yet"
        );
        let array_type = new_array_type_name(
            type_name,
            index.open_bracket,
            index.start,
            index.close_bracket,
        );
        type_name = new_type_name_array_type_name(array_type);
    }

    type_name
}

/// Consumes an IAP and returns an Expression
pub(crate) fn new_expression_index_access_path(index_access_path: IndexAccessPath) -> Expression {
    let IndexAccessPath { path, indices } = index_access_path;

    let mut expression = match path {
        Path::IdentifierPath(path) => new_expression_identifier_path(path),
        Path::ElementaryType(elem_type) => new_expression_elementary_type(elem_type),
    };

    for index in indices.into_iter() {
        let array_expression = new_index_access_expression(
            expression,
            index.open_bracket,
            index.start,
            index.end,
            index.close_bracket,
        );
        expression = new_expression_index_access_expression(array_expression);
    }

    expression
}

pub(crate) fn new_expression_identifier_path(identifier_path: IdentifierPath) -> Expression {
    let IdentifierPathStruct { head, tail } = Rc::unwrap_or_clone(identifier_path);
    match tail {
        None => new_expression_identifier(head),
        Some(tail) => Rc::unwrap_or_clone(tail)
            .elements
            .elements
            .into_iter()
            .fold(new_expression_identifier(head), |acc, id| {
                new_expression_member_access_expression(new_member_access_expression(
                    acc,
                    Period { range: 0..0 },
                    match id {
                        MemberAccessIdentifier::AddressKeyword(ak) => {
                            new_member_access_identifier_address_keyword(ak)
                        }
                        MemberAccessIdentifier::Identifier(id) => {
                            new_member_access_identifier_identifier(id)
                        }
                    },
                ))
            }),
    }
}

/// We use this function to share attributes between a state variable that has a function type.
/// We find and split the attributes from the function type as needed
/// TODO(v2) fail gracefully if a wrong attribute is found
pub(crate) fn extract_extra_attributes(
    fun_type: FunctionType,
) -> (FunctionType, Vec<StateVariableAttribute>) {
    // Move all matching attributes to extra_attributes if duplicate_found, else only the first occurrence
    let mut seen_constant = false;
    let mut seen_internal = false;
    let mut seen_private = false;
    let mut seen_public = false;
    let mut duplicate_found = false;

    let mut extra_attributes: Vec<StateVariableAttribute> = vec![];
    fn add_to_extra(
        attr: FunctionTypeAttribute,
        extra_attributes: &mut Vec<StateVariableAttribute>,
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
    // TODO(v2): use extract_if
    let mut i = 0;

    let FunctionTypeStruct {
        function_keyword,
        parameters,
        attributes,
        returns,
    } = Rc::unwrap_or_clone(fun_type);
    let mut vec = attributes.elements;

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

    let new_fun_type = new_function_type(
        function_keyword,
        parameters,
        new_function_type_attributes(vec),
        returns,
    );

    (new_fun_type, extra_attributes)
}
