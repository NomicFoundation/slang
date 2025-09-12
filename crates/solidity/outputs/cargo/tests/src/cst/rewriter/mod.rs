use std::rc::Rc;

use slang_solidity::cst::{
    BaseRewriter, Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode, TerminalKind,
    TerminalNode,
};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

fn parse(kind: NonterminalKind, input: &str) -> Rc<NonterminalNode> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    Rc::clone(parser.parse_nonterminal(kind, input).tree())
}

#[test]
fn rewrite_terminal_node() {
    struct IdRewriter;
    impl BaseRewriter for IdRewriter {
        fn rewrite_identifier(&mut self, terminal_node: &Rc<TerminalNode>) -> Option<Node> {
            Some(Node::terminal(
                terminal_node.kind,
                terminal_node.unparse().to_string() + "New",
            ))
        }
    }

    let terminal_node = Node::terminal(TerminalKind::Identifier, "test".to_string());
    let mut rewriter = IdRewriter;
    let result = rewriter.rewrite_node(&terminal_node).unwrap();
    let result = result.as_terminal().unwrap();
    assert_eq!(result.kind, TerminalKind::Identifier);
    assert_eq!(result.unparse(), "testNew");
}

#[test]
fn rewrite_nonterminal_node() {
    struct ContractNameRewriter;
    impl BaseRewriter for ContractNameRewriter {
        fn rewrite_contract_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
            let new_children: Vec<Edge> = node
                .children
                .iter()
                .map(|edge| {
                    if edge.label == EdgeLabel::Name {
                        Edge {
                            label: EdgeLabel::Name,
                            node: Node::terminal(TerminalKind::Identifier, "NewName".to_string()),
                        }
                    } else {
                        edge.clone()
                    }
                })
                .collect();
            Some(Node::nonterminal(
                NonterminalKind::ContractDefinition,
                new_children,
            ))
        }
    }

    let node = parse(NonterminalKind::ContractDefinition, "contract AContract {}");
    let mut rewriter = ContractNameRewriter;
    let result = rewriter
        .rewrite_node(&node.into())
        .expect("To return a node");
    assert_nonterminal_node(
        &result,
        NonterminalKind::ContractDefinition,
        "contract NewName {}",
    );
}

#[test]
fn rewrite_nonterminal_node_deep() {
    struct ContractNameRewriter {
        inside_contract: bool,
    }
    impl BaseRewriter for ContractNameRewriter {
        fn rewrite_identifier(&mut self, terminal_node: &Rc<TerminalNode>) -> Option<Node> {
            if self.inside_contract {
                self.inside_contract = false;
                Some(
                    TerminalNode::create(
                        terminal_node.kind,
                        terminal_node.unparse().to_string() + "New",
                    )
                    .into(),
                )
            } else {
                Some(Node::Terminal(Rc::clone(terminal_node)))
            }
        }
        fn rewrite_contract_definition(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
            self.inside_contract = true;
            Some(self.rewrite_children(node))
        }
    }

    let source = r#"
    contract AContract {
      function aFun() public {}
    }
    library ALib {}
    "#;

    let expected = r#"
    contract AContractNew {
      function aFun() public {}
    }
    library ALib {}
    "#;

    let node = parse(NonterminalKind::SourceUnit, source);
    let mut rewriter = ContractNameRewriter {
        inside_contract: false,
    };
    let result = rewriter
        .rewrite_node(&node.into())
        .expect("To return a node");
    assert_nonterminal_node(&result, NonterminalKind::SourceUnit, expected);
}

#[test]
fn remove_nonterminal_node() {
    struct RemovalRewriter;
    impl BaseRewriter for RemovalRewriter {
        fn rewrite_function_definition(&mut self, _node: &Rc<NonterminalNode>) -> Option<Node> {
            None
        }
    }

    let contract = r#"
    contract AContract {
      function test() {
      }
    }
    "#;
    let expected = r#"
    contract AContract {
    }
    "#;
    let node = parse(NonterminalKind::ContractDefinition, contract);
    let mut rewriter = RemovalRewriter;
    let result = rewriter
        .rewrite_node(&node.into())
        .expect("To return a node");
    assert_nonterminal_node(&result, NonterminalKind::ContractDefinition, expected);
}

#[test]
fn adding_nonterminal_node() {
    struct AdderRewriter;
    impl BaseRewriter for AdderRewriter {
        fn rewrite_contract_members(&mut self, node: &Rc<NonterminalNode>) -> Option<Node> {
            // Parse the new function member as a NonterminalNode
            let new_fun = parse(
                NonterminalKind::ContractMember,
                "      function newFun() {}\n",
            );
            // Clone the children and push the new member
            let mut children = node.children.clone();
            children.push(Edge {
                label: EdgeLabel::Item,
                node: Node::Nonterminal(Rc::clone(&new_fun)),
            });
            // Return a new ContractMember node with the updated children
            Some(Node::nonterminal(NonterminalKind::ContractMember, children))
        }
    }

    let contract = r#"
    contract AContract {
    }
    "#;
    let expected = r#"
    contract AContract {
      function newFun() {}
    }
    "#;
    let node = parse(NonterminalKind::ContractDefinition, contract);
    let mut rewriter = AdderRewriter;
    let result = rewriter
        .rewrite_node(&node.into())
        .expect("To return a node");
    assert_nonterminal_node(&result, NonterminalKind::ContractDefinition, expected);
}

#[test]
fn noop() {
    struct NoopRewriter;
    impl BaseRewriter for NoopRewriter {}

    let source = r#"
    contract AContract {
    }
    "#;

    let node = parse(NonterminalKind::ContractDefinition, source);
    let node_id = node.id();
    let mut rewriter = NoopRewriter;
    let result = rewriter
        .rewrite_node(&node.into())
        .expect("To return a node");
    let result_node = assert_nonterminal_node(&result, NonterminalKind::ContractDefinition, source);
    assert_eq!(node_id, result_node.id());
}

fn assert_nonterminal_node(node: &Node, kind: NonterminalKind, text: &str) -> Rc<NonterminalNode> {
    let node = node.as_nonterminal().expect("Node should be a nonterminal");
    assert_eq!(
        node.kind, kind,
        "expected non-terminal of kind {kind}, got {node:?}"
    );
    assert_eq!(node.unparse(), text);
    Rc::clone(node)
}
