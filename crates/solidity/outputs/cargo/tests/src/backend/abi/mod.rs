use anyhow::Result;
use slang_solidity::backend::semantic::ast::FunctionKind;

use crate::backend::fixtures;

#[test]
fn test_get_contracts_abi() -> Result<()> {
    let compilation_unit = fixtures::Counter::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let contracts = semantic_analysis.get_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name);
    assert_eq!("main.sol", contracts[0].file_id);
    assert_eq!(7, contracts[0].functions.len());

    let functions = &contracts[0].functions;

    assert!(matches!(functions[0].kind, FunctionKind::Constructor));
    assert!(functions[0].name.is_none());

    assert!(matches!(functions[1].kind, FunctionKind::Regular));
    assert!(functions[1]
        .name
        .as_ref()
        .is_some_and(|name| name == "click"));

    assert!(matches!(functions[2].kind, FunctionKind::Regular));
    assert!(functions[2]
        .name
        .as_ref()
        .is_some_and(|name| name == "count"));

    assert!(matches!(functions[3].kind, FunctionKind::Regular));
    assert!(functions[3]
        .name
        .as_ref()
        .is_some_and(|name| name == "disable"));

    assert!(matches!(functions[4].kind, FunctionKind::Regular));
    assert!(functions[4]
        .name
        .as_ref()
        .is_some_and(|name| name == "enable"));

    assert!(matches!(functions[5].kind, FunctionKind::Regular));
    assert!(functions[5]
        .name
        .as_ref()
        .is_some_and(|name| name == "increment"));

    assert!(matches!(functions[6].kind, FunctionKind::Regular));
    assert!(functions[6]
        .name
        .as_ref()
        .is_some_and(|name| name == "isEnabled"));

    Ok(())
}

#[test]
fn test_storage_layout() -> Result<()> {
    let compilation_unit = fixtures::Counter::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let counter = semantic_analysis
        .find_contract_by_name("Counter")
        .expect("contract can be found");
    let counter_abi = counter
        .abi_with_file_id("main.sol")
        .expect("can compute ABI");
    let layout = &counter_abi.storage_layout;

    assert_eq!(layout.len(), 4);

    assert_eq!(layout[0].label, "_owner");
    assert_eq!(layout[0].r#type, "address");
    assert_eq!(layout[0].slot, 0);
    assert_eq!(layout[0].offset, 0);

    assert_eq!(layout[1].label, "_state");
    assert_eq!(layout[1].r#type, "State");
    assert_eq!(layout[1].slot, 0);
    assert_eq!(layout[1].offset, 20);

    assert_eq!(layout[2].label, "count");
    assert_eq!(layout[2].r#type, "uint256");
    assert_eq!(layout[2].slot, 1);
    assert_eq!(layout[2].offset, 0);

    assert_eq!(layout[3].label, "_clickers");
    assert_eq!(layout[3].r#type, "mapping(address => uint256)");
    assert_eq!(layout[3].slot, 2);
    assert_eq!(layout[3].offset, 0);

    Ok(())
}
