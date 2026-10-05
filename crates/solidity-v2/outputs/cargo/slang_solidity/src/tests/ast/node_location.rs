use super::fixtures;
use crate::ast::{self, NodeLocation};
use crate::compilation::CompilationUnit;
use crate::define_fixture;

#[test]
fn test_get_file_id_and_text_range() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .next()
        .expect("contract is found");
    assert_eq!(ownable.get_file_id().as_str(), "ownable.sol");

    let owner = ownable
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .expect("_owner state variable is found");
    assert_eq!(owner.name().name(), "_owner");
    assert_eq!(owner.get_file_id().as_str(), "ownable.sol");

    // The state variable's range must sit within the enclosing contract's range.
    assert!(ownable.get_text_range().start <= owner.get_text_range().start);
    assert!(ownable.get_text_range().end >= owner.get_text_range().end);

    let activatable = unit
        .find_contract_by_name("Activatable")
        .next()
        .expect("contract is found");
    assert_eq!(activatable.get_file_id().as_str(), "activatable.sol");

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract is found");
    assert_eq!(counter.get_file_id().as_str(), "main.sol");
}

const MAIN: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Base} from "base.sol";

contract Test is Base {
    uint256 stateVar;

    function f(uint256 a, uint256 b) public returns (uint256) {
        g();
        string memory s = "ab" "cd";
        assembly {
            let x := sload(stateVar.slot)
        }
        return a + b;
    }
}
"#;

const BASE: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Base {
    function g() public {}
}
"#;

define_fixture!(
    NodeLocations,
    file: "main.sol", MAIN,
    file: "base.sol", BASE,
);

#[derive(Default)]
struct Collector {
    functions: Vec<ast::FunctionDefinition>,
    expressions: Vec<ast::Expression>,
    string_literals: Vec<ast::StringLiterals>,
    yul_paths: Vec<ast::YulPath>,
}

impl ast::visitor::Visitor for Collector {
    fn enter_function_definition(&mut self, node: &ast::FunctionDefinition) -> bool {
        self.functions.push(node.clone());
        true
    }

    fn enter_expression(&mut self, node: &ast::Expression) -> bool {
        self.expressions.push(node.clone());
        true
    }

    fn enter_string_literals(&mut self, items: &ast::StringLiterals) -> bool {
        self.string_literals.push(items.clone());
        true
    }

    fn enter_yul_path(&mut self, items: &ast::YulPath) -> bool {
        self.yul_paths.push(items.clone());
        true
    }
}

fn collect(unit: &CompilationUnit, file_id: &str) -> Collector {
    let source_unit = unit.file(&file_id.into()).unwrap().ast();
    let mut collector = Collector::default();
    ast::visitor::accept_source_unit(&source_unit, &mut collector);
    collector
}

/// Returns the file of `node` and the source text it spans.
fn located(node: &impl NodeLocation) -> Option<(&str, &'static str)> {
    let file_id = node.calculate_file_id()?.as_str();
    let source = match file_id {
        "main.sol" => MAIN,
        "base.sol" => BASE,
        _ => panic!("unknown file `{file_id}`"),
    };
    Some((file_id, &source[node.calculate_text_range()?]))
}

#[test]
fn test_sequence_location() {
    let unit = NodeLocations::build_compilation_unit();
    let main = collect(&unit, "main.sol");

    let f = &main.functions[0];
    assert_eq!(f.calculate_file_id(), Some(f.get_file_id()));
    assert_eq!(f.calculate_text_range(), Some(f.get_text_range().clone()));

    let (file_id, text) = located(f).unwrap();
    assert_eq!(file_id, "main.sol");
    assert!(text.starts_with("function f("));
    assert!(text.ends_with('}'));
}

#[test]
fn test_choice_location() {
    let unit = NodeLocations::build_compilation_unit();
    let main = collect(&unit, "main.sol");

    let sum = main
        .expressions
        .iter()
        .find(|expression| matches!(expression, ast::Expression::AdditiveExpression(_)))
        .expect("`a + b` is found");
    assert_eq!(located(sum), Some(("main.sol", "a + b")));

    // A string expression is a choice over collections.
    let string = main
        .expressions
        .iter()
        .find(|expression| matches!(expression, ast::Expression::StringExpression(_)))
        .expect("`\"ab\" \"cd\"` is found");
    assert_eq!(located(string), Some(("main.sol", r#""ab" "cd""#)));
}

#[test]
fn test_collection_location() {
    let unit = NodeLocations::build_compilation_unit();
    let main = collect(&unit, "main.sol");

    // A collection spans from its first item to its last.
    let slot = main
        .yul_paths
        .iter()
        .find(|path| path.len() == 2)
        .expect("`stateVar.slot` is found");
    assert_eq!(located(slot), Some(("main.sol", "stateVar.slot")));

    let literals = main
        .string_literals
        .iter()
        .find(|literals| literals.len() == 2)
        .expect("`\"ab\" \"cd\"` is found");
    assert_eq!(located(literals), Some(("main.sol", r#""ab" "cd""#)));
}

#[test]
fn test_empty_collection_has_no_location() {
    let unit = NodeLocations::build_compilation_unit();
    let main = collect(&unit, "main.sol");

    let call = main
        .expressions
        .iter()
        .find_map(|expression| match expression {
            ast::Expression::FunctionCallExpression(call) => Some(call),
            _ => None,
        })
        .expect("`g()` is found");

    let arguments = call.arguments();
    assert!(arguments.calculate_file_id().is_none());
    assert!(arguments.calculate_text_range().is_none());

    let ast::ArgumentsDeclaration::PositionalArguments(positional) = &arguments else {
        panic!("`g()` has positional arguments");
    };
    assert!(positional.is_empty());
    assert!(positional.calculate_file_id().is_none());
    assert!(positional.calculate_text_range().is_none());
}

#[test]
fn test_unit_variant_has_no_location() {
    let unit = NodeLocations::build_compilation_unit();
    let main = collect(&unit, "main.sol");

    let kind = main.functions[0].kind();
    assert!(matches!(kind, ast::FunctionKind::Regular));
    assert!(kind.calculate_file_id().is_none());
    assert!(kind.calculate_text_range().is_none());
}

#[test]
fn test_imported_file_location() {
    let unit = NodeLocations::build_compilation_unit();
    let base = collect(&unit, "base.sol");

    let g = &base.functions[0];
    assert_eq!(located(g), Some(("base.sol", "function g() public {}")));
}
