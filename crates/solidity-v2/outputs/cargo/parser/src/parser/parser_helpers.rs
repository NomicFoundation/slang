//! This module contains certain nodes and functions used internally by the parser.
//!
//! They shouldn't be used outside of the parser, and should be transformed into AST nodes.

use std::iter::once;

use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    ExpectedArrayLengthExpression, InvalidMutability, InvalidVisibility,
};
use slang_solidity_v2_common::terminals::TerminalKind;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes::{
    CloseBracket, ElementaryType, Expression, FunctionTypeAttribute, FunctionTypeStruct,
    Identifier, IdentifierPath, IdentifierPathElement, IndexAccessEnd, OpenBracket, Period,
    StateVariableAttribute, TypeName, new_array_type_name, new_expression_elementary_type,
    new_expression_identifier, new_expression_index_access_expression,
    new_expression_member_access_expression, new_identifier_path,
    new_identifier_path_element_identifier, new_index_access_expression,
    new_member_access_expression, new_type_name_array_type_name, new_type_name_elementary_type,
    new_type_name_identifier_path,
};
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::parser::GrammarCtx;

/// An `IndexAccessPath` represents a path or elementary type followed by
/// zero or more index accesses, e.g. `foo.bar[0][1:3]` or `uint256[5][]`
///
/// It's heavily inspired by solc
/// <https://github.com/argotorg/solidity/blob/194b114664c7daebc2ff68af3c573272f5d28913/libsolidity/parsing/Parser.h#L198-L209>
#[derive(Debug)]
pub(crate) struct IndexAccessPath {
    pub path: Path,
    pub indices: Vec<Index>,
}

#[derive(Debug)]
pub(crate) enum Path {
    SeparatedIdentifierPath(SeparatedIdentifierPath),
    ElementaryType(ElementaryType),
}

/// An identifier path that keeps the `Period`s separating its elements.
///
/// The `IdentifierPath` node doesn't keep its separators, but a path can also be
/// reinterpreted as a chain of member accesses, which does need them.
#[derive(Debug)]
pub(crate) struct SeparatedIdentifierPath {
    /// Only the elements after the first one can be an `AddressKeyword`
    pub head: Identifier,
    /// The remaining elements, each one with the period that precedes it
    pub tail: Vec<(Period, IdentifierPathElement)>,
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

/// Creates an IAP from a separated identifier path
pub(crate) fn new_index_access_path_from_separated_identifier_path(
    separated_identifier_path: SeparatedIdentifierPath,
) -> IndexAccessPath {
    IndexAccessPath {
        path: Path::SeparatedIdentifierPath(separated_identifier_path),
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

/// Consumes an IAP and creates a `TypeName`
///
/// `start` is the byte offset of the beginning of the IAP (the start of its
/// path), used as the start of any reported diagnostic range.
///
/// A range/slice index access (`[start:end]`) is not a valid array length, so
/// an error is reported and recovery ignores everything after the colon
/// (i.e. treating `[start:end]` as `[start]`).
///
/// TODO(error-recovery): Once the CST allows for error nodes, the failure here should be present in there.
pub(crate) fn new_type_name_index_access_path(
    index_access_path: IndexAccessPath,
    start: usize,
    ctx: &mut GrammarCtx<'_>,
) -> TypeName {
    let IndexAccessPath { path, indices } = index_access_path;

    let mut type_name = match path {
        Path::SeparatedIdentifierPath(path) => {
            new_type_name_identifier_path(new_identifier_path_from_separated_identifier_path(path))
        }
        Path::ElementaryType(elem_type) => new_type_name_elementary_type(elem_type),
    };

    for index in indices {
        if index.end.is_some() {
            // Report from the start of the index access path up to the closing
            // bracket of the offending index, matching solc's range.
            let end = index.close_bracket.range.end;
            ctx.diagnostics.push(
                ctx.file_id.to_owned(),
                start..end,
                ExpectedArrayLengthExpression,
            );
        }
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
        Path::SeparatedIdentifierPath(path) => new_expression_separated_identifier_path(path),
        Path::ElementaryType(elem_type) => new_expression_elementary_type(elem_type),
    };

    for index in indices {
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

/// Consumes an identifier and returns a `SeparatedIdentifierPath` with a single element
pub(crate) fn new_separated_identifier_path_from_identifier(
    head: Identifier,
) -> SeparatedIdentifierPath {
    SeparatedIdentifierPath { head, tail: vec![] }
}

/// Consumes an identifier and a tail of `(Period, IdentifierPathElement)` and returns a `SeparatedIdentifierPath`
pub(crate) fn new_separated_identifier_path_from_identifier_and_tail(
    head: Identifier,
    tail: Vec<(Period, IdentifierPathElement)>,
) -> SeparatedIdentifierPath {
    SeparatedIdentifierPath { head, tail }
}

/// Consumes a `SeparatedIdentifierPath` and returns an `IdentifierPath`, dropping the separators
pub(crate) fn new_identifier_path_from_separated_identifier_path(
    separated_identifier_path: SeparatedIdentifierPath,
) -> IdentifierPath {
    let elements = once(new_identifier_path_element_identifier(
        separated_identifier_path.head,
    ))
    .chain(
        separated_identifier_path
            .tail
            .into_iter()
            .map(|(_, element)| element),
    )
    .collect();

    new_identifier_path(elements)
}

/// Consumes a separated identifier path and returns the equivalent chain of member accesses
pub(crate) fn new_expression_separated_identifier_path(
    separated_identifier_path: SeparatedIdentifierPath,
) -> Expression {
    let SeparatedIdentifierPath { head, tail } = separated_identifier_path;

    let mut expression = new_expression_identifier(head);
    for (period, element) in tail {
        expression = new_expression_member_access_expression(new_member_access_expression(
            expression, period, element,
        ));
    }

    expression
}

/// We use this function to share attributes between a state variable that has a function type.
/// We find and split the attributes from the function type as needed
///
/// Extracted attributes belong to the state variable, so the ones that are not
/// valid there (a mutability, or an `external` visibility) are reported as
/// diagnostics and dropped.
///
/// TODO(error-recovery): Once the CST allows for error nodes, the dropped attributes should be present in there.
pub(crate) fn extract_extra_attributes(
    fun_type: &mut FunctionTypeStruct,
    ctx: &mut GrammarCtx<'_>,
) -> Vec<StateVariableAttribute> {
    // Move all matching attributes to extra_attributes if duplicate_found, else only the first occurrence
    let mut seen_visibility = false;
    let mut duplicate_found = false;

    let extracted = fun_type.attributes.elements.extract_if(.., |attr| {
        if duplicate_found {
            // After the first duplicate is found, all matching attributes are extracted
            true
        } else {
            match attr {
                FunctionTypeAttribute::ExternalKeyword(_)
                | FunctionTypeAttribute::InternalKeyword(_)
                | FunctionTypeAttribute::PrivateKeyword(_)
                | FunctionTypeAttribute::PublicKeyword(_) => {
                    if seen_visibility {
                        // If a visibility attribute has already been seen, mark duplicate_found and extract it
                        duplicate_found = true;
                        true
                    } else {
                        // If it's the first time we see a visibility attribute, mark it as seen and don't extract it
                        seen_visibility = true;
                        false
                    }
                }
                _ => false,
            }
        }
    });

    extracted
        .filter_map(|attr| {
            match attr {
                FunctionTypeAttribute::InternalKeyword(terminal) => {
                    Some(StateVariableAttribute::InternalKeyword(terminal))
                }
                FunctionTypeAttribute::PrivateKeyword(terminal) => {
                    Some(StateVariableAttribute::PrivateKeyword(terminal))
                }
                FunctionTypeAttribute::PublicKeyword(terminal) => {
                    Some(StateVariableAttribute::PublicKeyword(terminal))
                }
                // A state variable cannot be `external`
                FunctionTypeAttribute::ExternalKeyword(terminal) => {
                    ctx.diagnostics.push(
                        ctx.file_id.to_owned(),
                        terminal.range,
                        InvalidVisibility {
                            valid: vec![
                                TerminalKind::PublicKeyword,
                                TerminalKind::InternalKeyword,
                                TerminalKind::PrivateKeyword,
                            ],
                        },
                    );
                    None
                }
                // Neither can it declare a function mutability
                attr @ (FunctionTypeAttribute::PureKeyword(_)
                | FunctionTypeAttribute::ViewKeyword(_)
                | FunctionTypeAttribute::PayableKeyword(_)) => {
                    let range = attr
                        .calculate_text_range()
                        .expect("Function type attributes always have a range");
                    // `transient` only exists from 0.8.27 onwards, so offering it
                    // below that would name a keyword the version check rejects.
                    let mut valid = vec![
                        TerminalKind::ConstantKeyword,
                        TerminalKind::ImmutableKeyword,
                    ];
                    if ctx.language_version >= LanguageVersion::V0_8_27 {
                        valid.push(TerminalKind::TransientKeyword);
                    }

                    ctx.diagnostics.push(
                        ctx.file_id.to_owned(),
                        range,
                        InvalidMutability { valid },
                    );
                    None
                }
            }
        })
        .collect()
}
