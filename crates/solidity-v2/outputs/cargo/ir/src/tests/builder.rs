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
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::LATEST);

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
    expect_variant!(&ir_root.members[0], ir::SourceUnitMember::PragmaDirective);
    expect_variant!(
        &ir_root.members[1],
        ir::SourceUnitMember::ContractDefinition
    );

    // MyContract contract
    let contract = expect_variant!(
        &ir_root.members[1],
        ir::SourceUnitMember::ContractDefinition
    );
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    // MyContract.owner state variable
    let state_var = expect_variant!(
        &contract.members[0],
        ir::ContractMember::StateVariableDefinition
    );
    assert_eq!("owner", state_var.name.unparse());

    // MyContract constructor
    let constructor = expect_variant!(&contract.members[1], ir::ContractMember::FunctionDefinition);
    assert_eq!(constructor.kind, ir::FunctionKind::Constructor);
    assert!(constructor.body.is_some());
    let constructor_body = constructor.body.as_ref().unwrap();
    assert_eq!(1, constructor_body.statements.len());

    // MyContract.test() function
    let function = expect_variant!(&contract.members[2], ir::ContractMember::FunctionDefinition);
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
fn test_build_ir_node_kind_histogram() {
    const CONTENTS: &str = r###"
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
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::LATEST);
    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();
    let ir::BuildOutput { diagnostics, .. } =
        ir::build("test.sol", &source_unit, &CONTENTS, &mut id_generator);
    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    let histogram = id_generator.histogram();

    // Every allocated node is recorded exactly once, so the histogram total
    // must match the number of IDs handed out.
    assert_eq!(histogram.total() as usize, id_generator.allocated_count());

    // Spot-check a few known kinds in the snippet above.
    assert_eq!(1, histogram.count(ir::NodeKind::ContractDefinition));
    assert_eq!(1, histogram.count(ir::NodeKind::StateVariableDefinition));
    // The constructor and `test()` both lower to a `FunctionDefinition`.
    assert_eq!(2, histogram.count(ir::NodeKind::FunctionDefinition));
    // Several identifiers appear (e.g. `owner`, `msg`, `sender`).
    assert_eq!(9, histogram.count(ir::NodeKind::Identifier));
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
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::LATEST);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build("test.sol", &source_unit, &CONTENTS, &mut id_generator);

    let sentinel_node_id = id_generator.next_id_of(ir::NodeKind::SourceUnit);

    assert_eq!(2, ir_root.members.len());
    assert!(ir_root.id() < sentinel_node_id);

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    assert_eq!(2, ir_root.members.len());

    let base_contract = expect_variant!(
        &ir_root.members[0],
        ir::SourceUnitMember::ContractDefinition
    );
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());
    assert!(base_contract.id() < sentinel_node_id);
    assert!(base_contract.name.id() < sentinel_node_id);

    let test_contract = expect_variant!(
        &ir_root.members[1],
        ir::SourceUnitMember::ContractDefinition
    );
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

#[test]
fn test_build_ir_distinguishes_index_access_and_slice() {
    const CONTENTS: &str = r###"
contract Test {
    function test(bytes calldata a) public pure {
        a[1];
        a[1:];
    }
}
    "###;

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse("test.sol", CONTENTS, LanguageVersion::LATEST);

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

    let contract = expect_variant!(
        &ir_root.members[0],
        ir::SourceUnitMember::ContractDefinition
    );
    let function = expect_variant!(&contract.members[0], ir::ContractMember::FunctionDefinition);
    let body = function.body.as_ref().expect("function has a body");
    assert_eq!(2, body.statements.len());

    // `a[1]` is a plain index access, not a slice.
    let statement = expect_variant!(&body.statements[0], ir::Statement::ExpressionStatement);
    let index_access =
        expect_variant!(&statement.expression, ir::Expression::IndexAccessExpression);
    assert!(
        !index_access.is_slice,
        "`a[1]` should not be a slice expression"
    );
    assert!(index_access.start.is_some());
    assert!(index_access.end.is_none());

    // `a[1:]` is a slice expression.
    let statement = expect_variant!(&body.statements[1], ir::Statement::ExpressionStatement);
    let slice_access =
        expect_variant!(&statement.expression, ir::Expression::IndexAccessExpression);
    assert!(
        slice_access.is_slice,
        "`a[1:]` should be a slice expression"
    );
    assert!(slice_access.start.is_some());
    assert!(slice_access.end.is_none());
}
