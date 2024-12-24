use std::rc::Rc;

use metaslang_cst::text_index::TextIndex;
use semver::Version;

use crate::bindings::built_ins::get_built_ins_contents;
use crate::bindings::BindingGraph;
use crate::cst::{Edge, Node, NonterminalNode, TerminalKind, TerminalNode};
use crate::parser::{Parser, ParserInitializationError};

static BUILT_INS_PARSER_VERSION: Version = Version::new(0, 8, 28);

pub fn add_built_ins(
    binding_graph: &mut BindingGraph,
    version: &Version,
) -> Result<(), ParserInitializationError> {
    let parser = Parser::create(BUILT_INS_PARSER_VERSION.clone())?;
    let built_ins_parse_output = parser.parse(Parser::ROOT_KIND, get_built_ins_contents(version));
    assert!(
        built_ins_parse_output.is_valid(),
        "built-ins parse without errors"
    );

    let built_ins_cursor =
        transform(built_ins_parse_output.tree()).cursor_with_offset(TextIndex::ZERO);

    binding_graph.add_system_file("built_ins.sol", built_ins_cursor);
    Ok(())
}

fn transform(node: &Node) -> Node {
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
                    node: transform(&edge.node),
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
