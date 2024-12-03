mod generated;

pub use generated::*;

#[cfg(feature = "__experimental_bindings_api")]
pub mod bindings {
    use semver::Version;

    pub use super::generated::bindings::*;
    use crate::cst::TextIndex;
    use crate::parser::{Parser, ParserInitializationError};

    pub fn add_built_ins(
        bindings: &mut Bindings,
        version: &Version,
    ) -> Result<(), ParserInitializationError> {
        let parser = Parser::create(version.clone())?;
        let built_ins_parse_output = parser.parse(Parser::ROOT_KIND, get_built_ins(version));
        assert!(
            built_ins_parse_output.is_valid(),
            "built-ins parse without errors"
        );

        let built_ins_cursor = transform_built_ins_node(&built_ins_parse_output.tree())
            .cursor_with_offset(TextIndex::ZERO);

        bindings.add_system_file("built_ins.sol", built_ins_cursor);
        Ok(())
    }

    pub fn transform_built_ins_node(node: &crate::cst::Node) -> crate::cst::Node {
        use std::rc::Rc;

        use crate::cst::{Edge, Node, NonterminalNode, TerminalKind, TerminalNode};

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
}
