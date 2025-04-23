use anyhow::{anyhow, Result};
use slang_solidity::backend::ast;
use slang_solidity::backend::ast::visitor::Visitor;
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[derive(Default)]
struct CounterVisitor {
    contract: usize,
    ctor: usize,
    function: usize,
    modifier: usize,
    state_vars: usize,

    enter_contracts: bool,
}

impl CounterVisitor {
    fn new(enter_contracts: bool) -> Self {
        Self {
            enter_contracts,
            ..Self::default()
        }
    }
}

impl Visitor for CounterVisitor {
    fn enter_contract_definition(&mut self, _node: &ast::ContractDefinition) -> bool {
        self.enter_contracts
    }
    fn leave_contract_definition(&mut self, _node: &ast::ContractDefinition) {
        self.contract += 1;
    }
    fn leave_constructor_definition(&mut self, _node: &ast::ConstructorDefinition) {
        self.ctor += 1;
    }
    fn leave_function_definition(&mut self, _node: &ast::FunctionDefinition) {
        self.function += 1;
    }
    fn leave_modifier_definition(&mut self, _node: &ast::ModifierDefinition) {
        self.modifier += 1;
    }
    fn leave_state_variable_definition(&mut self, _node: &ast::StateVariableDefinition) {
        self.state_vars += 1;
    }
}

#[test]
fn test_ast_visitor() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >= 0.8.17;

function add(uint a, uint b) pure returns (uint) {
    return a + b;
}

abstract contract Ownable {
    address _owner;
    constructor() {
         _owner = msg.sender;
    }
    modifier onlyOwner() {
         require(_owner == msg.sender);
         _;
    }
}

contract Counter is Ownable {
    uint _count;
    constructor(uint initial_value) {
         _count = initial_value;
    }
    function count() view public returns (uint) {
         return _count;
    }
    function increment(uint delta) public onlyOwner returns (uint) {
         _count = add(_count, delta);
         return _count;
    }
}
    "###,
    );
    assert!(output.is_valid());

    let ast =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;

    let mut visitor = CounterVisitor::new(true);
    ast::visitor::accept_source_unit(&ast, &mut visitor);

    assert_eq!(2, visitor.contract);
    assert_eq!(2, visitor.ctor);
    assert_eq!(1, visitor.modifier);
    assert_eq!(2, visitor.state_vars);
    assert_eq!(3, visitor.function);

    let mut shallow_visitor = CounterVisitor::new(false);
    ast::visitor::accept_source_unit(&ast, &mut shallow_visitor);

    assert_eq!(2, shallow_visitor.contract);
    assert_eq!(0, shallow_visitor.ctor);
    assert_eq!(0, shallow_visitor.modifier);
    assert_eq!(0, shallow_visitor.state_vars);
    assert_eq!(1, shallow_visitor.function);

    Ok(())
}
