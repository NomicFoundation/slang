use anyhow::{anyhow, Result};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::interner::Interner;
use crate::ir;

#[test]
fn test_build_ir_tree() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let version = LanguageVersion::V0_8_30;
    let source_unit_cst =
        Parser::parse(CONTENTS, version).map_err(|message| anyhow!(format!("{message:?}")))?;
    let mut interner = Interner::new();
    let source_unit = ir::build(&source_unit_cst, CONTENTS, &mut interner);

    assert_eq!(2, source_unit.members.len());

    let ir::SourceUnitMember::ContractDefinition(base_contract) = &source_unit.members[0] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse(CONTENTS));
    assert_eq!("Base", interner.resolve(base_contract.name.string_id));
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let ir::SourceUnitMember::ContractDefinition(test_contract) = &source_unit.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Test", test_contract.name.unparse(CONTENTS));
    assert_eq!("Test", interner.resolve(test_contract.name.string_id));
    assert_eq!(1, test_contract.inheritance_types.len());
    assert_eq!(
        "Base",
        test_contract.inheritance_types[0]
            .type_name
            .iter()
            .map(|node| node.unparse(CONTENTS))
            .collect::<Vec<_>>()
            .join(".")
    );
    assert!(test_contract.storage_layout.is_some());

    // Verify that two references to the same name yield the same StringId
    let base_name_string_id = base_contract.name.string_id;
    let ir::IdentifierPathElement::Identifier(base_in_inheritance) =
        &test_contract.inheritance_types[0].type_name[0]
    else {
        panic!("Expected Identifier");
    };
    assert_eq!(base_name_string_id, base_in_inheritance.string_id);

    // Verify that different names yield different StringId
    assert_ne!(base_contract.name.string_id, test_contract.name.string_id);

    Ok(())
}
