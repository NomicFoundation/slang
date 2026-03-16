use std::ops::Range;

use anyhow::{anyhow, Result};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::{builder, ir};

#[test]
fn test_build_ir_tree() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let unparse = |range: &Range<usize>| &CONTENTS[range.clone()];

    let version = LanguageVersion::V0_8_30;
    let source_unit_cst =
        Parser::parse(CONTENTS, version).map_err(|message| anyhow!(format!("{message:?}")))?;
    let source_unit = builder::run(&source_unit_cst);

    assert_eq!(2, source_unit.members.len());

    let ir::SourceUnitMember::ContractDefinition(base_contract) = &source_unit.members[0] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", unparse(&base_contract.name.range));
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let ir::SourceUnitMember::ContractDefinition(test_contract) = &source_unit.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Test", unparse(&test_contract.name.range));
    assert_eq!(1, test_contract.inheritance_types.len());
    assert_eq!(
        "Base",
        test_contract.inheritance_types[0]
            .type_name
            .iter()
            .map(|node| match node {
                ir::IdentifierPathElement::AddressKeyword => "address",
                ir::IdentifierPathElement::Identifier(identifier) => unparse(&identifier.range),
            })
            .collect::<Vec<_>>()
            .join(".")
    );
    assert!(test_contract.storage_layout.is_some());

    Ok(())
}
