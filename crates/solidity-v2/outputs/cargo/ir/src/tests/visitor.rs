use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;

#[derive(Default)]
struct CounterVisitor {
    contracts: usize,
    constructors: usize,
    functions: usize,
    modifiers: usize,
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

impl ir::visitor::Visitor for CounterVisitor {
    fn enter_contract_definition(&mut self, _node: &ir::ContractDefinition) -> bool {
        self.contracts += 1;
        self.enter_contracts
    }
    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        match node.kind {
            ir::FunctionKind::Constructor => self.constructors += 1,
            ir::FunctionKind::Modifier => self.modifiers += 1,
            _ => self.functions += 1,
        }
    }
    fn leave_state_variable_definition(&mut self, _node: &ir::StateVariableDefinition) {
        self.state_vars += 1;
    }
}

#[test]
fn test_ir_visitor() {
    const CONTENTS: &str = r###"
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
    "###;

    let ParseOutput {
        source_unit,
        errors,
    } = Parser::parse(CONTENTS, LanguageVersion::V0_8_30);

    assert!(errors.is_empty(), "Parser errors: {errors:?}");

    let source_unit = ir::build(&source_unit, &CONTENTS);

    let mut visitor = CounterVisitor::new(true);
    ir::visitor::accept_source_unit(&source_unit, &mut visitor);

    assert_eq!(2, visitor.contracts);
    assert_eq!(2, visitor.constructors);
    assert_eq!(1, visitor.modifiers);
    assert_eq!(2, visitor.state_vars);
    assert_eq!(3, visitor.functions);

    let mut shallow_visitor = CounterVisitor::new(false);
    ir::visitor::accept_source_unit(&source_unit, &mut shallow_visitor);

    assert_eq!(2, shallow_visitor.contracts);
    assert_eq!(0, shallow_visitor.constructors);
    assert_eq!(0, shallow_visitor.modifiers);
    assert_eq!(0, shallow_visitor.state_vars);
    assert_eq!(1, shallow_visitor.functions);
}
