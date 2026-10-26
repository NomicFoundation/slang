//! Hand-written adjustments to the tree produced by the generated parser, for the few constructs
//! that the language definition cannot express. Each one lives in its own submodule.

mod function_typed_state_variable_visibility;

use std::rc::Rc;

use crate::cst::{Edge, EdgeLabel, Node, NonterminalNode, TerminalKind};
use crate::parser::Parser;

impl Parser {
    /// Applied to the tree of every parse, after the generated parser functions.
    /// It never adds or removes errors.
    pub(crate) fn fix_up(&self, tree: Rc<NonterminalNode>) -> Rc<NonterminalNode> {
        // This parsing mechanism should only be applied on >0.4.22
        if !self.version_is_at_least_0_4_22 {
            return tree;
        }

        function_typed_state_variable_visibility::fix_up(&tree).unwrap_or(tree)
    }
}

/// Returns the first child of `node` under the edge `label`.
fn child_with_label(node: &Rc<NonterminalNode>, label: EdgeLabel) -> Option<&Node> {
    node.children
        .iter()
        .find(|edge| edge.label == label)
        .map(|edge| &edge.node)
}

/// Returns a copy of `node`, after letting `update` modify each of its children in place.
fn with_updated_children(
    node: &Rc<NonterminalNode>,
    update: impl FnMut(&mut Edge),
) -> Rc<NonterminalNode> {
    let mut children = node.children.clone();
    children.iter_mut().for_each(update);

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
