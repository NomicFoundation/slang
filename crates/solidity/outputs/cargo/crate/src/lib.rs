mod generated;

pub use generated::*;

#[cfg(feature = "__experimental_bindings_api")]
pub fn transform_built_ins_node(node: &generated::cst::Node) -> generated::cst::Node {
    use std::rc::Rc;

    use generated::cst::{Edge, Node, NonterminalNode, TerminalNode};

    use crate::cst::TerminalKind;

    match node {
        Node::Nonterminal(nonterminal) => {
            let NonterminalNode {
                kind,
                text_len,
                children,
            } = nonterminal.as_ref();
            let children = children
                .iter()
                .map(|edge| Edge {
                    label: edge.label,
                    node: transform_built_ins_node(&edge.node),
                })
                .collect();
            let nonterminal = Rc::new(NonterminalNode {
                kind: *kind,
                text_len: *text_len,
                children,
            });
            Node::Nonterminal(nonterminal)
        }
        Node::Terminal(terminal) => {
            let TerminalNode { kind, text } = terminal.as_ref();
            let terminal = match terminal.as_ref().kind {
                TerminalKind::Identifier => Rc::new(TerminalNode {
                    kind: *kind,
                    text: text.replace('$', "%"),
                }),
                _ => Rc::clone(terminal),
            };
            Node::Terminal(terminal)
        }
    }
}
