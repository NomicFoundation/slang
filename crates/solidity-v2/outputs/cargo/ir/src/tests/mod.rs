use anyhow::{anyhow, Result};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::cst_consumer;

use crate::ir;

fn assert_pipeline_equivalence(source: &str, version: LanguageVersion) {
    let cst = cst_consumer::parse(source, version).expect("cst parse failed");
    let ir_via_cst = ir::build(&cst, &source);
    let ir_direct = ir::ir_consumer::parse(source, version).expect("ir_consumer parse failed");
    assert_eq!(
        ir_via_cst, ir_direct,
        "pipelines diverged for source:\n{source}"
    );
}

#[test]
fn test_ir_consumer_equivalence_minimal() {
    assert_pipeline_equivalence(
        "contract Base {}\ncontract Test is Base layout at 0 {}\n",
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_function_kinds() {
    assert_pipeline_equivalence(
        r###"
contract C {
    constructor() public payable {}
    fallback() external payable {}
    receive() external payable {}
    modifier m() virtual {
        _;
    }
    function f() public pure returns (uint256) { return 0; }
}
"###,
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_state_variables() {
    assert_pipeline_equivalence(
        r###"
contract C {
    uint constant X = 1;
    uint public constant Y = 2;
    uint private z;
    uint internal immutable w;
}
"###,
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_events_errors() {
    assert_pipeline_equivalence(
        r###"
contract C {
    event E(uint indexed a, string b);
    error Err(uint a);
}
"###,
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_imports() {
    assert_pipeline_equivalence(
        "import \"x\" as y;\n",
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_expressions() {
    assert_pipeline_equivalence(
        r###"
contract C {
    function f() public {
        uint[] storage arr;
        arr[0] = 1;
        x.y.z;
    }
}
"###,
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_ir_consumer_equivalence_mappings() {
    assert_pipeline_equivalence(
        r###"
contract C {
    mapping(uint k => address v) m;
}
"###,
        LanguageVersion::V0_8_30,
    );
}

#[test]
fn test_build_ir_tree() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let version = LanguageVersion::V0_8_30;
    let source_unit_cst =
        cst_consumer::parse(CONTENTS, version).map_err(|message| anyhow!(format!("{message:?}")))?;
    let source_unit = ir::build(&source_unit_cst, &CONTENTS);

    assert_eq!(2, source_unit.members.len());

    let ir::SourceUnitMember::ContractDefinition(base_contract) = &source_unit.members[0] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let ir::SourceUnitMember::ContractDefinition(test_contract) = &source_unit.members[1] else {
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

    Ok(())
}
