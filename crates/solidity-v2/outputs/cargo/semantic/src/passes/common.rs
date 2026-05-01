use std::rc::Rc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, ImportDefinition, Reference, Resolution, ScopeId};

/// Resolves an `IdentifierPath` starting from the given scope, creating
/// `Reference`s for all its elements. It will follow through
/// contracts/interfaces/libraries as well as imports and treat them as
/// namespaces. Returns the resolution of the last reference.
pub(super) fn resolve_identifier_path_in_scope(
    binder: &mut Binder,
    identifier_path: &ir::IdentifierPath,
    starting_scope_id: ScopeId,
) -> Resolution {
    let mut scope_id = Some(starting_scope_id);
    let mut use_lexical_resolution = true;
    let mut last_resolution: Resolution = Resolution::Unresolved;

    for identifier in identifier_path {
        let symbol = identifier.unparse();
        let resolution = if let Some(scope_id) = scope_id {
            if use_lexical_resolution {
                binder.resolve_in_scope(scope_id, symbol)
            } else {
                binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
        } else {
            Resolution::Unresolved
        };

        let reference = Reference::new(Rc::clone(identifier), resolution.clone());
        binder.insert_reference(reference);

        // Unless we used namespace resolution and in order to continue
        // resolving the identifier path, we should ensure we've followed
        // through any symbol alias (ie. import deconstruction symbol). This
        // is not needed for namespaced resolution because there cannot be
        // import directives inside contracts, interfaces or libraries which
        // changes the lookup mode (see below).
        let resolution = if use_lexical_resolution {
            binder.follow_symbol_aliases(&resolution)
        } else {
            resolution
        };

        // recurse into file scopes pointed by the resolved definition
        // to resolve the next identifier in the path
        scope_id = resolution
            .as_definition_id()
            .and_then(|node_id| binder.find_definition_by_id(node_id))
            .and_then(|definition| match definition {
                Definition::Import(ImportDefinition {
                    resolved_file_id, ..
                }) => resolved_file_id
                    .as_ref()
                    .and_then(|resolved_file_id| binder.scope_id_for_file_id(resolved_file_id)),
                Definition::Contract(_) | Definition::Interface(_) | Definition::Library(_) => {
                    use_lexical_resolution = false;
                    binder.scope_id_for_node_id(definition.node_id())
                }
                _ => None,
            });

        last_resolution = resolution;
    }
    last_resolution
}

/// Since `StringExpression` nodes are plain enums and each variant is in
/// turn a collection of terminals, they don't have a `NodeId`. So we pick
/// the `NodeId` of the first terminal of the collection.
pub(super) fn node_id_for_string_expression_typing(node: &ir::StringExpression) -> NodeId {
    match node {
        ir::StringExpression::StringLiterals(strings) => strings[0].id(),
        ir::StringExpression::HexStringLiterals(hex_strings) => hex_strings[0].id(),
        ir::StringExpression::UnicodeStringLiterals(unicode_strings) => unicode_strings[0].id(),
    }
}

/// Returns the `NodeId` of an `ir::Expression`, dispatching across the variants
/// sub-expression types. This is `NodeId` is what the typing pass uses to
/// register the typing of the `Expression`.
pub(super) fn node_id_for_expression_typing(node: &ir::Expression) -> Option<NodeId> {
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

        // Other expression variants don't register typing by `NodeId`
        ir::Expression::ElementaryType(_)
        | ir::Expression::PayableKeyword
        | ir::Expression::ThisKeyword
        | ir::Expression::SuperKeyword
        | ir::Expression::TrueKeyword
        | ir::Expression::FalseKeyword => None,
    }
}
