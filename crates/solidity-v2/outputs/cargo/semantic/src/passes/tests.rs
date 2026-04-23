use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::binder::{Binder, Resolution};
use crate::context::SemanticFile;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::TypeRegistry;

struct TestFile {
    id: String,
    ir_root: ir::SourceUnit,
}

impl SemanticFile for TestFile {
    fn id(&self) -> &str {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, _node_id: NodeId) -> Option<&String> {
        None
    }
}

fn build_file(name: &str, contents: &str, id_generator: &mut NodeIdGenerator) -> TestFile {
    let ParseOutput {
        source_unit,
        errors,
    } = Parser::parse(contents, LanguageVersion::V0_8_30);

    assert!(errors.is_empty(), "Parser errors: {errors:?}");

    let ir_root = ir::build(&source_unit, &contents, id_generator);
    TestFile {
        id: name.to_string(),
        ir_root,
    }
}

#[test]
fn test_collect_definitions_and_linearise_contracts() {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", CONTENTS, &mut id_generator);

    let files = [file];
    let mut binder = Binder::default();
    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);

    // Verify definitions were collected
    assert_eq!(2, binder.definitions().len());
    // Verify linearisations were computed
    assert_eq!(2, binder.linearisations().len());
}

fn get_contract_to_bases_map(binder: &Binder) -> HashMap<String, Vec<String>> {
    let mut contract_to_bases = HashMap::new();
    for (key, values) in binder.linearisations() {
        let contract_name = binder
            .find_definition_by_id(*key)
            .unwrap()
            .identifier()
            .unparse()
            .to_string();

        let base_names: Vec<String> = values
            .iter()
            .map(|value| {
                binder
                    .find_definition_by_id(*value)
                    .unwrap()
                    .identifier()
                    .unparse()
                    .to_string()
            })
            .collect();

        contract_to_bases.insert(contract_name, base_names);
    }
    contract_to_bases
}

#[test]
fn test_valid_linearisations() {
    const CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract D is A, B {}
interface C {}
abstract contract B is C {}
interface A is C {}
"#;

    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", CONTENTS, &mut id_generator);

    let files = [file];
    let mut binder = Binder::default();
    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);

    let contract_to_bases = get_contract_to_bases_map(&binder);

    let mut expected = HashMap::new();
    expected.insert(
        "D".to_string(),
        vec![
            "D".to_string(),
            "B".to_string(),
            "A".to_string(),
            "C".to_string(),
        ],
    );
    expected.insert("A".to_string(), vec!["A".to_string(), "C".to_string()]);
    expected.insert("B".to_string(), vec!["B".to_string(), "C".to_string()]);
    expected.insert("C".to_string(), vec!["C".to_string()]);

    assert_eq!(contract_to_bases, expected);
}

#[test]
fn test_linearise_with_invalid_input() {
    const CONTENTS: &str = r#"
contract Base {}

library Foo {}

// Foo is an invalid base, but it shouldn't crash the linearisation pass
contract Test is Base, Foo { // Base should resolve to the contract, not the var
    string Base;
}
"#;

    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", CONTENTS, &mut id_generator);

    let files = [file];
    let mut binder = Binder::default();
    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);

    let contract_to_bases = get_contract_to_bases_map(&binder);

    let mut expected = HashMap::new();
    expected.insert("Base".to_string(), vec!["Base".to_string()]);
    expected.insert(
        "Test".to_string(),
        vec!["Test".to_string(), "Base".to_string()],
    );

    assert_eq!(contract_to_bases, expected);
}

#[test]
fn test_type_definitions() {
    const CONTENTS: &str = r###"
contract Base {
    uint256 public x;
    function foo(uint256 a) public pure returns (uint256) {
        return a;
    }
}

contract Test is Base {
    mapping(address => uint256) balances;
    bool flag;

    struct Point {
        uint256 x;
        uint256 y;
    }

    enum Color { Red, Green, Blue }

    function bar(uint256 b) external view returns (bool) {
        return flag;
    }
}
    "###;

    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", CONTENTS, &mut id_generator);

    let files = [file];
    let mut binder = Binder::default();
    let mut types = TypeRegistry::default();

    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);

    let types_before = types.iter_types().count();
    p3_type_definitions::run(&files, &mut binder, &mut types);
    let types_after = types.iter_types().count();

    // The pass registers new types for: contracts, mappings, structs, enums,
    // function types, getter types, etc.
    let registered_types = types_after - types_before;
    assert_eq!(registered_types, 7);
}

#[test]
fn test_resolve_references() {
    const CONTENTS: &str = r###"
contract Base {
    uint256 public x;
    function foo(uint256 a) public pure returns (uint256) {
        return a;
    }
}

contract Test is Base {
    mapping(address => uint256) balances;
    bool flag;

    struct Point {
        uint256 x;
        uint256 y;
    }

    enum Color { Red, Green, Blue }

    function bar(uint256 b) external view returns (bool) {
        return flag;
    }

    function baz() public view returns (uint256) {
        Point memory p;
        p.x = 1;
        return balances[msg.sender];
    }
}
    "###;

    let mut id_generator = NodeIdGenerator::default();
    let file = build_file("test.sol", CONTENTS, &mut id_generator);

    let files = [file];
    let mut binder = Binder::default();
    let mut types = TypeRegistry::default();
    let language_version = LanguageVersion::V0_8_30;

    p1_collect_definitions::run(&files, &mut binder);
    p2_linearise_contracts::run(&files, &mut binder);
    p3_type_definitions::run(&files, &mut binder, &mut types);
    p4_resolve_references::run(&files, &mut binder, &mut types, language_version);

    // Verify that references were created and most are resolved
    let references = binder.references();
    assert!(!references.is_empty(), "expected some references");

    let unresolved_count = references
        .values()
        .filter(|r| matches!(r.resolution, Resolution::Unresolved))
        .count();
    assert_eq!(
        0, unresolved_count,
        "expected all references to be resolved"
    );
}
