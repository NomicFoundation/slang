use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;

#[test]
fn test_build_ir_tree() {
    const CONTENTS: &str = r###"
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
    "###;

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::V0_8_30);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build("test.sol", &source_unit, &CONTENTS, &mut id_generator);

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    assert_eq!(2, ir_root.members.len());
    assert!(matches!(
        ir_root.members[0],
        ir::SourceUnitMember::PragmaDirective(_)
    ));
    assert!(matches!(
        ir_root.members[1],
        ir::SourceUnitMember::ContractDefinition(_)
    ));

    // MyContract contract
    let ir::SourceUnitMember::ContractDefinition(ref contract) = ir_root.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    // MyContract.owner state variable
    let ir::ContractMember::StateVariableDefinition(ref state_var) = contract.members[0] else {
        panic!("Expected StateVariableDefinition");
    };
    assert_eq!("owner", state_var.name.unparse());

    // MyContract constructor
    let ir::ContractMember::FunctionDefinition(ref constructor) = contract.members[1] else {
        panic!("Expected FunctionDefinition for constructor");
    };
    assert_eq!(constructor.kind, ir::FunctionKind::Constructor);
    assert!(constructor.body.is_some());
    let constructor_body = constructor.body.as_ref().unwrap();
    assert_eq!(1, constructor_body.statements.len());

    // MyContract.test() function
    let ir::ContractMember::FunctionDefinition(ref function) = contract.members[2] else {
        panic!("Expected FunctionDefinition");
    };
    assert_eq!(function.kind, ir::FunctionKind::Regular);
    let function_name = function.name.as_ref().expect("function has a name");
    assert_eq!("test", function_name.unparse());
    assert_eq!(function.visibility, ir::FunctionVisibility::Public);
    assert_eq!(function.mutability, ir::FunctionMutability::View);
    assert_eq!(0, function.parameters.len());
    assert!(function.returns.is_some());
    assert_eq!(1, function.returns.as_ref().unwrap().len());
    assert!(function.body.is_some());
    let function_body = function.body.as_ref().unwrap();
    assert_eq!(1, function_body.statements.len());
}

#[test]
fn test_build_ir_contract_inheritance_and_storage_layout() {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::V0_8_30);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build("test.sol", &source_unit, &CONTENTS, &mut id_generator);

    let sentinel_node_id = id_generator.next_id();

    assert_eq!(2, ir_root.members.len());
    assert!(ir_root.id() < sentinel_node_id);

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    assert_eq!(2, ir_root.members.len());

    let ir::SourceUnitMember::ContractDefinition(base_contract) = &ir_root.members[0] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());
    assert!(base_contract.id() < sentinel_node_id);
    assert!(base_contract.name.id() < sentinel_node_id);

    let ir::SourceUnitMember::ContractDefinition(test_contract) = &ir_root.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Test", test_contract.name.unparse());
    assert_eq!(1, test_contract.inheritance_types.len());
    assert_eq!(
        "Base",
        test_contract.inheritance_types[0]
            .type_name
            .iter()
            .map(|node| node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    );
    assert!(test_contract.storage_layout.is_some());
    assert!(test_contract.id() < sentinel_node_id);
    assert!(test_contract.name.id() < sentinel_node_id);
}
