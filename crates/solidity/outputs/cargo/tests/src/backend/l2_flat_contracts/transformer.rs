use anyhow::{anyhow, Result};
use slang_solidity::backend::l2_flat_contracts::transformer::Transformer;
use slang_solidity::backend::{l1_typed_cst, l2_flat_contracts};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_build_l2_from_l1() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###,
    );
    assert!(output.is_valid());

    let l1 = l1_typed_cst::builder::build_source_unit(output.tree()).map_err(|s| anyhow!(s))?;

    let mut transformer = l2_flat_contracts::FromL1 {};
    let l2 = transformer.transform_source_unit(&l1);

    assert_eq!(2, l2.members.len());

    let l2_flat_contracts::SourceUnitMember::ContractDefinition(base_contract) = &l2.members[0]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let l2_flat_contracts::SourceUnitMember::ContractDefinition(test_contract) = &l2.members[1]
    else {
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
