use super::fixtures;
use crate::ast::Definition;
use crate::define_fixture;

define_fixture!(
    BytecodeDependencies,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Created {}
contract Deployed {}
contract Read {}

library Library {
    function deploy() external returns (address) {
        return address(new Created());
    }
}

contract Creator {
    constructor() {
        new Created();
    }

    function deploy() public returns (address) {
        return address(new Deployed());
    }

    function code() public pure returns (bytes memory, bytes memory) {
        return (type(Read).runtimeCode, type(Library).runtimeCode);
    }
}
"#,
);

fn names(definitions: Vec<Definition>) -> Vec<String> {
    definitions
        .into_iter()
        .map(|definition| definition.identifier().name().to_owned())
        .collect()
}

#[test]
fn test_contract_bytecode_dependencies() {
    let unit = BytecodeDependencies::build_compilation_unit();
    let creator = unit
        .find_contract_by_name("Creator")
        .next()
        .expect("can find contract");

    assert_eq!(names(creator.creation_bytecode_dependencies()), ["Created"]);
    assert_eq!(
        names(creator.deployed_bytecode_dependencies()),
        ["Deployed", "Read", "Library"],
    );
}

#[test]
fn test_contract_without_bytecode_dependencies() {
    let unit = BytecodeDependencies::build_compilation_unit();
    let created = unit
        .find_contract_by_name("Created")
        .next()
        .expect("can find contract");

    assert!(created.creation_bytecode_dependencies().is_empty());
    assert!(created.deployed_bytecode_dependencies().is_empty());
}

#[test]
fn test_library_bytecode_dependencies() {
    let unit = BytecodeDependencies::build_compilation_unit();
    let library = fixtures::find_library(&unit, "Library");

    assert_eq!(names(library.bytecode_dependencies()), ["Created"]);
}
