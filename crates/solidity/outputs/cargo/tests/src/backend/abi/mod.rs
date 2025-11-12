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
