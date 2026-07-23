use crate::ast::{self, AssemblyStatement};
use crate::define_fixture;

define_fixture!(
    AssemblyReferences,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Test {
    uint256 stateVar;

    function f() public {
        uint256 localVar = 1;
        assembly {
            let x := add(sload(stateVar.slot), localVar)
            function helper(a) -> b { b := a }
        }
    }
}
"#,
);

#[derive(Default)]
struct AssemblyCollector {
    statements: Vec<AssemblyStatement>,
}

impl ast::visitor::Visitor for AssemblyCollector {
    fn enter_assembly_statement(&mut self, node: &AssemblyStatement) -> bool {
        self.statements.push(node.clone());
        true
    }
}

#[test]
fn test_assembly_referenced_definitions() {
    let unit = AssemblyReferences::build_compilation_unit();
    let main_ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut collector = AssemblyCollector::default();
    ast::visitor::accept_source_unit(&main_ast, &mut collector);

    assert_eq!(collector.statements.len(), 1);
    let assembly = &collector.statements[0];

    // The block references the state variable and the local, but not the Yul
    // definitions (`x`, `helper`, `a`, `b`) nor the Yul built-ins.
    let mut names: Vec<String> = assembly
        .referenced_definitions()
        .iter()
        .map(|definition| definition.identifier().name().to_string())
        .collect();
    names.sort();
    assert_eq!(names, vec!["localVar".to_string(), "stateVar".to_string()]);
}
