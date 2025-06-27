use anyhow::Result;
use slang_solidity::backend::l2_flat_contracts;
use slang_solidity::backend::passes::{p0_build_ast, p1_flatten_contracts};
use slang_solidity::compilation::InternalCompilationBuilder;
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_flatten_contracts() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let mut internal_builder = InternalCompilationBuilder::create(LanguageFacts::LATEST_VERSION)?;
    _ = internal_builder.add_file("main.sol".into(), CONTENTS);
    let compilation_unit = internal_builder.build();

    let data = p0_build_ast::run(compilation_unit);
    let data = p1_flatten_contracts::run(data);

    let l2 = &data.files["main.sol"];

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
