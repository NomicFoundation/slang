use std::rc::Rc;

use anyhow::{bail, ensure, Error, Result};
use semver::Version;

use slang_solidity::{
    cst::{Node, RuleNode},
    cursor::Cursor,
    kinds::{ProductionKind, RuleKind, TokenKind},
    language::Language,
    visitor::{Visitor, VisitorEntryResponse},
};

struct ContractCollector {
    contract_names: Vec<String>,
}

impl Visitor<Error> for ContractCollector {
    fn rule_enter(
        &mut self,
        node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> Result<VisitorEntryResponse> {
        if node.kind == RuleKind::ContractDefinition {
            if let Node::Token(token) = &node.children[2] {
                ensure!(token.kind == TokenKind::Identifier);
                self.contract_names.push(token.text.to_owned());
            } else {
                bail!("Expected contract identifier: {node:?}");
            };
            return Ok(VisitorEntryResponse::StepOver);
        }

        return Ok(VisitorEntryResponse::StepIn);
    }
}

#[test]
fn visitor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut collector = ContractCollector {
        contract_names: Vec::new(),
    };

    parse_output
        .parse_tree()
        .cursor()
        .drive_visitor(&mut collector)?;

    assert!(matches!(&collector.contract_names[..], [single] if single == "Foo"));

    return Ok(());
}
