//! Hand-written adjustments to the tree produced by the generated parser, for the few constructs
//! that the language definition cannot express.

use std::rc::Rc;

use crate::cst::{Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode, TerminalKind};
use crate::parser::Parser;

impl Parser {
    /// Applied to the tree of every parse, after the generated parser functions.
    /// It never adds or removes errors.
    pub(crate) fn fix_up(&self, tree: Rc<NonterminalNode>) -> Rc<NonterminalNode> {
        // This parsing mechanism should only be applied on >0.4.22
        if !self.version_is_at_least_0_4_22 {
            return tree;
        }

        fix_up_function_type_visibility(&tree).unwrap_or(tree)
    }
}

/// State variables with function types need to properly capture attributes for either the
/// type itself, or the variable. In `function() external public x;`, `external` belongs to
/// the function type, while `public` belongs to the variable `x`.
///
/// The generated parser consumes all of them as `FunctionTypeAttributes`, so this moves them
/// to the `StateVariableAttributes` that encloses it, if any.
///
/// Returns a fixed copy of `node`, or `None` when nothing under it changed.
fn fix_up_function_type_visibility(node: &Rc<NonterminalNode>) -> Option<Rc<NonterminalNode>> {
    match node.kind {
        NonterminalKind::StateVariableDefinition => {
            // Don't build on top of a declaration that the parser had to recover from:
            if contains_errors(node) {
                return None;
            }

            move_second_visibility(node)
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
                if let Some(fixed) = edge
                    .node
                    .as_nonterminal()
                    .and_then(fix_up_function_type_visibility)
                {
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
fn move_second_visibility(variable: &Rc<NonterminalNode>) -> Option<Rc<NonterminalNode>> {
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
    let split = split_index(&function_attributes.children)?;

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
    let function_type = replace_child(function_type, EdgeLabel::Attributes, function_attributes);
    let type_name = replace_child(type_name, EdgeLabel::Variant, function_type);

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

    let variable = replace_child(variable, EdgeLabel::TypeName, type_name);
    Some(replace_child(
        &variable,
        EdgeLabel::Attributes,
        variable_attributes,
    ))
}

/// Returns the index of the first visibility that follows an `internal` or `external` one.
fn split_index(items: &[Edge]) -> Option<usize> {
    let mut armed = false;

    for (index, item) in items.iter().enumerate() {
        match first_terminal_kind(&item.node)? {
            TerminalKind::InternalKeyword
            | TerminalKind::ExternalKeyword
            | TerminalKind::PrivateKeyword
            | TerminalKind::PublicKeyword
                if armed =>
            {
                return Some(index);
            }
            TerminalKind::InternalKeyword | TerminalKind::ExternalKeyword => armed = true,
            _ => {}
        }
    }

    None
}

/// Returns the first child of `node` under the edge `label`.
fn child_with_label(node: &Rc<NonterminalNode>, label: EdgeLabel) -> Option<&Node> {
    node.children
        .iter()
        .find(|edge| edge.label == label)
        .map(|edge| &edge.node)
}

/// Returns a copy of `node`, with the child under the edge `label` replaced by `child`.
fn replace_child(
    node: &Rc<NonterminalNode>,
    label: EdgeLabel,
    child: Rc<NonterminalNode>,
) -> Rc<NonterminalNode> {
    let mut children = node.children.clone();
    if let Some(edge) = children.iter_mut().find(|edge| edge.label == label) {
        edge.node = Node::Nonterminal(child);
    }

    NonterminalNode::create(node.kind, children)
}

/// Returns whether `node` contains a `MISSING` or `UNRECOGNIZED` terminal.
fn contains_errors(node: &Rc<NonterminalNode>) -> bool {
    Rc::clone(node).descendants().any(|edge| !edge.is_valid())
}

/// Returns the kind of the first terminal in `node`, skipping trivia.
fn first_terminal_kind(node: &Node) -> Option<TerminalKind> {
    node.clone()
        .descendants()
        .find(|edge| edge.is_terminal() && !edge.is_trivia())
        .and_then(|edge| edge.as_terminal().map(|terminal| terminal.kind))
}
