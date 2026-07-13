use std::ops::Range;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use crate::context::FileNodeMapper;

/// Returns the source location of a node: the file it belongs to and the
/// text range within it.
pub(crate) fn node_location(
    node: &(impl NodeIdentity + TextRange),
    file_node_mapper: &FileNodeMapper,
) -> (FileId, Range<usize>) {
    let file_id = file_node_mapper
        .file_id_from_node_id(node.node_id().expect("node has an id"))
        .to_owned();
    let range = node.calculate_text_range().expect("node has a text range");
    (file_id, range)
}

/// Since `StringExpression` nodes are plain enums and each variant is in
/// turn a collection of terminals, they don't have a `NodeId`. So we pick
/// the `NodeId` of the first terminal of the collection.
pub(crate) fn node_id_for_string_expression_typing(node: &ir::StringExpression) -> NodeId {
    match node {
        ir::StringExpression::StringLiterals(strings) => strings[0].id(),
        ir::StringExpression::HexStringLiterals(hex_strings) => hex_strings[0].id(),
        ir::StringExpression::UnicodeStringLiterals(unicode_strings) => unicode_strings[0].id(),
    }
}

/// Returns the `NodeId` of an `ir::Expression`, dispatching across the variants
/// sub-expression types. This is `NodeId` is what the typing pass uses to
/// register the typing of the `Expression`.
pub(crate) fn node_id_for_expression_typing(node: &ir::Expression) -> Option<NodeId> {
    match node {
        ir::Expression::AssignmentExpression(e) => Some(e.id()),
        ir::Expression::ConditionalExpression(e) => Some(e.id()),
        ir::Expression::OrExpression(e) => Some(e.id()),
        ir::Expression::AndExpression(e) => Some(e.id()),
        ir::Expression::EqualityExpression(e) => Some(e.id()),
        ir::Expression::InequalityExpression(e) => Some(e.id()),
        ir::Expression::BitwiseOrExpression(e) => Some(e.id()),
        ir::Expression::BitwiseXorExpression(e) => Some(e.id()),
        ir::Expression::BitwiseAndExpression(e) => Some(e.id()),
        ir::Expression::ShiftExpression(e) => Some(e.id()),
        ir::Expression::AdditiveExpression(e) => Some(e.id()),
        ir::Expression::MultiplicativeExpression(e) => Some(e.id()),
        ir::Expression::ExponentiationExpression(e) => Some(e.id()),
        ir::Expression::PostfixExpression(e) => Some(e.id()),
        ir::Expression::PrefixExpression(e) => Some(e.id()),
        ir::Expression::FunctionCallExpression(e) => Some(e.id()),
        ir::Expression::CallOptionsExpression(e) => Some(e.id()),
        ir::Expression::MemberAccessExpression(e) => Some(e.id()),
        ir::Expression::IndexAccessExpression(e) => Some(e.id()),
        ir::Expression::NewExpression(e) => Some(e.id()),
        ir::Expression::TupleExpression(e) => Some(e.id()),
        ir::Expression::TypeExpression(e) => Some(e.id()),
        ir::Expression::ArrayExpression(e) => Some(e.id()),
        ir::Expression::HexNumberExpression(e) => Some(e.id()),
        ir::Expression::DecimalNumberExpression(e) => Some(e.id()),
        ir::Expression::StringExpression(s) => Some(node_id_for_string_expression_typing(s)),
        ir::Expression::Identifier(ident) => Some(ident.id()),
        ir::Expression::ThisKeyword(e) => Some(e.id()),
        ir::Expression::ElementaryType(e) => {
            Some(e.node_id().expect("ElementaryType should have a node id"))
        }
        ir::Expression::PayableKeyword(e) => Some(e.id()),

        // Other expression variants don't register typing by `NodeId`
        ir::Expression::SuperKeyword(_)
        | ir::Expression::TrueKeyword(_)
        | ir::Expression::FalseKeyword(_) => None,
    }
}
