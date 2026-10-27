//! State variables with function types need to properly capture attributes for either the
//! type itself, or the variable. In `function() external public x;`, `external` belongs to
//! the function type, while `public` belongs to the variable `x`.
//!
//! The generated parser consumes all of them as `FunctionTypeAttributes`, so this moves them
//! to the `StateVariableAttributes` that encloses it, if any.

use std::rc::Rc;

use super::{child_with_label, contains_errors, first_terminal_kind, with_updated_children};
use crate::cst::{Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode, TerminalKind};

/// Returns a fixed copy of `node`, or `None` when nothing under it changed.
pub(super) fn fix_up(node: &Rc<NonterminalNode>) -> Option<Rc<NonterminalNode>> {
    match node.kind {
        NonterminalKind::StateVariableDefinition => {
            let fixed = fix_up_function_typed_state_variable_visibility(node)?;

            // Don't build on top of a declaration that the parser had to recover from.
            // Checked last, as it walks the whole declaration and it's quite slow
            if contains_errors(node) {
                return None;
            }

            Some(fixed)
        }
        // Only traverse the containers that can (transitively) hold state variables:
        NonterminalKind::SourceUnit
        | NonterminalKind::SourceUnitMembers
        | NonterminalKind::SourceUnitMember
        | NonterminalKind::ContractDefinition
        | NonterminalKind::InterfaceDefinition
        | NonterminalKind::LibraryDefinition
        | NonterminalKind::ContractMembers
        | NonterminalKind::InterfaceMembers
        | NonterminalKind::LibraryMembers
        | NonterminalKind::ContractMember => {
            let mut children: Option<Vec<Edge>> = None;

            for (index, edge) in node.children.iter().enumerate() {
                if let Some(fixed) = edge.node.as_nonterminal().and_then(fix_up) {
                    children.get_or_insert_with(|| node.children.clone())[index].node =
                        Node::Nonterminal(fixed);
                }
            }

            children.map(|children| NonterminalNode::create(node.kind, children))
        }
        _ => None,
    }
}

/// Returns `variable` with its function type's attributes split at the second visibility, and the
/// rest moved to the front of its own `StateVariableAttributes`.
///
/// Returns `None` when there is nothing to move, or when the change can't be applied:
/// - the type isn't directly a function type, as in an array of function types;
/// - the function type has `returns`, since it delimits the attributes;
/// - a moved attribute isn't one a state variable can have, such as `view`.
fn fix_up_function_typed_state_variable_visibility(
    variable: &Rc<NonterminalNode>,
) -> Option<Rc<NonterminalNode>> {
    let type_name = child_with_label(variable, EdgeLabel::TypeName)?.as_nonterminal()?;
    let function_type = child_with_label(type_name, EdgeLabel::Variant)?
        .as_nonterminal_with_kind(NonterminalKind::FunctionType)?;

    // `returns` is parsed after the attributes, so moving them would reorder the source text.
    // `solc` reports an error in that case anyway.
    if child_with_label(function_type, EdgeLabel::Returns).is_some() {
        return None;
    }

    let function_attributes =
        child_with_label(function_type, EdgeLabel::Attributes)?.as_nonterminal()?;
    let split = index_of_second_visibility_attribute(&function_attributes.children)?;

    let (kept, moved) = function_attributes.children.split_at(split);

    // `solc` reports an error for anything else, and it has no `StateVariableAttribute` to map to:
    if !moved.iter().all(|item| {
        matches!(
            first_terminal_kind(&item.node),
            Some(
                TerminalKind::InternalKeyword
                    | TerminalKind::PrivateKeyword
                    | TerminalKind::PublicKeyword
                    | TerminalKind::ConstantKeyword
            )
        )
    }) {
        return None;
    }

    let function_attributes =
        NonterminalNode::create(NonterminalKind::FunctionTypeAttributes, kept.to_vec());
    let function_type = with_updated_children(function_type, |edge| {
        if edge.label == EdgeLabel::Attributes {
            edge.node = Node::Nonterminal(Rc::clone(&function_attributes));
        }
    });
    let type_name = with_updated_children(type_name, |edge| {
        if edge.label == EdgeLabel::Variant {
            edge.node = Node::Nonterminal(Rc::clone(&function_type));
        }
    });

    let variable_attributes =
        child_with_label(variable, EdgeLabel::Attributes)?.as_nonterminal()?;
    let variable_attributes = NonterminalNode::create(
        NonterminalKind::StateVariableAttributes,
        moved
            .iter()
            .map(|item| Edge {
                label: item.label,
                node: Node::nonterminal(
                    NonterminalKind::StateVariableAttribute,
                    item.node.children().to_vec(),
                ),
            })
            .chain(variable_attributes.children.iter().cloned())
            .collect(),
    );

    Some(with_updated_children(variable, |edge| match edge.label {
        EdgeLabel::TypeName => edge.node = Node::Nonterminal(Rc::clone(&type_name)),
        EdgeLabel::Attributes => edge.node = Node::Nonterminal(Rc::clone(&variable_attributes)),
        _ => {}
    }))
}

/// Returns the index of the first visibility that follows an `internal` or `external` one.
fn index_of_second_visibility_attribute(items: &[Edge]) -> Option<usize> {
    let mut seen_function_type_visibility = false;

    for (index, item) in items.iter().enumerate() {
        match first_terminal_kind(&item.node)? {
            TerminalKind::InternalKeyword
            | TerminalKind::ExternalKeyword
            | TerminalKind::PrivateKeyword
            | TerminalKind::PublicKeyword
                if seen_function_type_visibility =>
            {
                return Some(index);
            }
            TerminalKind::InternalKeyword | TerminalKind::ExternalKeyword => {
                seen_function_type_visibility = true;
            }
            _ => {}
        }
    }

    None
}
