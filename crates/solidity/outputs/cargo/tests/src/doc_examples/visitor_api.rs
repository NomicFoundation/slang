use std::rc::Rc;

use anyhow::{bail, Error, Result};
use semver::Version;

use slang_solidity::{
    language::Language,
    syntax::{
        nodes::{Node, RuleKind, RuleNode, TextRange, TokenKind},
        parser::ProductionKind,
        visitors::{Visitable, Visitor, VisitorEntryResponse},
    },
};

struct ContractCollector {
    contract_names: Vec<String>,
}

impl Visitor<Error> for ContractCollector {
    fn enter_rule(
        &mut self,
        node: &Rc<RuleNode>,
        _path: &Vec<Rc<RuleNode>>,
        _range: &TextRange,
    ) -> Result<VisitorEntryResponse> {
        if node.kind != RuleKind::ContractDefinition {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let identifier = if let Node::Token(token) = &node.children[2] {
            assert_eq!(token.kind, TokenKind::Identifier);
            token.text.to_owned()
        } else {
            bail!("Expected contract identifier: {node:?}");
        };

        self.contract_names.push(identifier);

        return Ok(VisitorEntryResponse::StepOver);
    }
}

#[test]
fn visitor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}")?;
    let parse_tree = parse_output.parse_tree();

    let mut collector = ContractCollector {
        contract_names: Vec::new(),
    };

    parse_tree.accept_visitor(&mut collector)?;

    assert!(matches!(&collector.contract_names[..], [single] if single == "Foo"));

    return Ok(());
}
