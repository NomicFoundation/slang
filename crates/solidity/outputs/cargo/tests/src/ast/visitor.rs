use anyhow::{anyhow, Result};
use slang_solidity::ast;
use slang_solidity::ast::visitor::Visitor;
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[derive(Default)]
struct CounterVisitor {
    contract: usize,
    ctor: usize,
    function: usize,
}

impl Visitor for CounterVisitor {
    fn visit_contract_definition(&mut self, _target: &ast::ContractDefinition) {
        self.contract += 1;
    }
    fn visit_constructor_definition(&mut self, _target: &ast::ConstructorDefinition) {
        self.ctor += 1;
    }
    fn visit_function_definition(&mut self, _target: &ast::FunctionDefinition) {
        self.function += 1;
    }
}

#[test]
fn test_ast_visitor() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >= 0.8.17;

contract MyContract {
    address owner;
    constructor() {
         owner = msg.sender;
    }
    function test() view public returns (bool) {
         return owner == msg.sender;
    }
}
    "###,
    );
    assert!(output.is_valid());

    let ast =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;

    let mut visitor = CounterVisitor::default();
    ast.accept(&mut visitor);

    assert_eq!(1, visitor.contract);
    assert_eq!(1, visitor.ctor);
    assert_eq!(1, visitor.function);

    Ok(())
}
