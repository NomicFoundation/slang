use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;
use crate::ir::TextRange;

#[test]
fn test_calculate_text_range() {
    const CONTENTS: &str = r###"
contract MyContract {
    constructor() {}
    function withParams(uint256 first, bool second) public {}
}"###;

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&"test.sol".into(), CONTENTS, LanguageVersion::LATEST);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(
        &"test.sol".into(),
        &source_unit,
        &CONTENTS,
        &mut id_generator,
    );

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    let ir::SourceUnitMember::ContractDefinition(ref contract) = ir_root.members[0] else {
        panic!("Expected ContractDefinition");
    };

    // A sequence node reports the range of its source text.
    let contract_range = contract.calculate_text_range().unwrap();
    assert_eq!(
        &CONTENTS[contract_range.clone()],
        CONTENTS.trim_start(),
        "Contract range should cover the whole definition"
    );

    // A choice node delegates to its inner node.
    assert_eq!(
        ir_root.members[0].calculate_text_range(),
        Some(contract_range)
    );

    // A terminal node reports the range of its text.
    let name_range = contract.name.calculate_text_range().unwrap();
    assert_eq!(&CONTENTS[name_range], "MyContract");

    let ir::ContractMember::FunctionDefinition(ref constructor) = contract.members[0] else {
        panic!("Expected FunctionDefinition for constructor");
    };
    let ir::ContractMember::FunctionDefinition(ref function) = contract.members[1] else {
        panic!("Expected FunctionDefinition");
    };

    // An empty collection has no range.
    assert_eq!(constructor.parameters.calculate_text_range(), None);

    // A non-empty collection spans from its first to its last element.
    let parameters_range = function.parameters.calculate_text_range().unwrap();
    assert_eq!(&CONTENTS[parameters_range], "uint256 first, bool second");

    // External nodes are not represented in the source code.
    assert_eq!(constructor.kind.calculate_text_range(), None);

    // Optional fields report the range of their value, if any.
    assert_eq!(constructor.name.calculate_text_range(), None);
    let body_range = function.body.calculate_text_range().unwrap();
    assert_eq!(&CONTENTS[body_range], "{}");
}
