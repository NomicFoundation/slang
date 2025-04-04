use anyhow::{anyhow, Result};
use slang_solidity::backend::l1::transformer::Transformer;
use slang_solidity::backend::{ast, l1};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_build_l1_from_ast() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###,
    );
    assert!(output.is_valid());

    let ast_source =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;

    let mut transformer = l1::FromAst {};
    let l1 = transformer.transform_source_unit(&ast_source);

    assert_eq!(2, l1.members.len());

    let l1::SourceUnitMember::ContractDefinition(base_contract) = &l1.members[0] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let l1::SourceUnitMember::ContractDefinition(test_contract) = &l1.members[1] else {
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
