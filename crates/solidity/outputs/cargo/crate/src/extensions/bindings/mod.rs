use std::rc::Rc;

use metaslang_cst::text_index::TextIndex;
use semver::Version;

use crate::bindings::built_ins::get_built_ins_contents;
use crate::bindings::BindingGraphBuilder;
use crate::cst::{Edge, Node, NonterminalNode, TerminalKind, TerminalNode};
use crate::parser::{Parser, ParserInitializationError};

pub fn add_built_ins(
    builder: &mut BindingGraphBuilder,
    version: Version,
) -> Result<(), ParserInitializationError> {
    let source = get_built_ins_contents(&version);
    let parser = Parser::create(version)?;
    let parse_output = parser.parse(Parser::ROOT_KIND, source);

    let built_ins_cursor = transform(&Node::Nonterminal(Rc::clone(parse_output.tree())))
        .cursor_with_offset(TextIndex::ZERO);

    builder.add_system_file("built_ins.sol", built_ins_cursor);
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
