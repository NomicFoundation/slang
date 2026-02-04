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
    let compilation_unit = fixtures::StorageLayout::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let counter = semantic_analysis
        .find_contract_by_name("C")
        .expect("contract can be found");
    let counter_abi = counter
        .compute_abi_with_file_id("main.sol".to_string())
        .expect("can compute ABI");
    let layout = &counter_abi.storage_layout;

    // Expected layout:
    // 00 [aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]
    // 01 [eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee]
    // 02 [ffffffffffffffffffffffffffffffff]
    // 03 [                            hhgg]
    // 04 [                           sssss] (yxxxx)
    // 05 [          lllllllllllllllllllllk]
    // 06 [                      mmmmmmmmmm]
    // 07 [  nnnnnnnnnnnnnnnnnnnnnnnnnnnnnn]
    // 08 [                      nnnnnnnnnn]
    // 09 [tttttttttttttttttttttttttttttttt] (t[0]: zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz)
    // 10 [tttttttttttttttttttttttttttttttt] (t[0]:                             wwww)
    // 11 [tttttttttttttttttttttttttttttttt] (t[1]: zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz)
    // 12 [tttttttttttttttttttttttttttttttt] (t[1]:                             wwww)
    // 13 [                           ooooo]

    macro_rules! assert_layout_item_eq {
        ($item:expr, $name:expr, $slot:expr, $offset:expr, $type:expr) => {
            assert_eq!($item.label, $name);
            assert_eq!($item.r#type, $type);
            assert_eq!($item.slot, $slot);
            assert_eq!($item.offset, $offset);
        };
    }
    assert_eq!(layout.len(), 12);

    assert_layout_item_eq!(layout[0], "a", 0, 0, "uint256");
    assert_layout_item_eq!(layout[1], "e", 1, 0, "uint8[]");
    assert_layout_item_eq!(layout[2], "f", 2, 0, "mapping(uint256 => S)");
    assert_layout_item_eq!(layout[3], "g", 3, 0, "uint16");
    assert_layout_item_eq!(layout[4], "h", 3, 2, "uint16");
    assert_layout_item_eq!(layout[5], "s", 4, 0, "S");
    assert_layout_item_eq!(layout[6], "k", 5, 0, "int8");
    assert_layout_item_eq!(layout[7], "l", 5, 1, "bytes21");
    assert_layout_item_eq!(layout[8], "m", 6, 0, "uint8[10]");
    assert_layout_item_eq!(layout[9], "n", 7, 0, "bytes5[8]");
    assert_layout_item_eq!(layout[10], "t", 9, 0, "T[2]");
    assert_layout_item_eq!(layout[11], "o", 13, 0, "bytes5");

    Ok(())
}

#[test]
fn test_function_selector() -> Result<()> {
    let compilation_unit = fixtures::Counter::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let counter = semantic_analysis
        .find_contract_by_name("Counter")
        .expect("contract can be found");

    let functions = counter.compute_linearised_functions();
    assert_eq!(functions.len(), 5);

    // all the functions in the contract are public
    assert_eq!(functions[0].compute_selector(), Some(0x7d55_923d_u32)); // click()
    assert_eq!(functions[1].compute_selector(), Some(0x2f27_70db_u32)); // disable()
    assert_eq!(functions[2].compute_selector(), Some(0xa390_7d71_u32)); // enable()
    assert_eq!(functions[3].compute_selector(), Some(0x7cf5_dab0_u32)); // increment(uint256)
    assert_eq!(functions[4].compute_selector(), Some(0x6aa6_33b6_u32)); // isEnabled()

    let state_variables = counter.compute_linearised_state_variables();
    assert_eq!(state_variables.len(), 4);

    // for state variables, selectors only make sense for public getters
    assert_eq!(state_variables[0].compute_selector(), None); // _owner
    assert_eq!(state_variables[1].compute_selector(), None); // _state
    assert_eq!(state_variables[2].compute_selector(), Some(0x0666_1abd_u32)); // count()
    assert_eq!(state_variables[3].compute_selector(), None); // _clickers

    Ok(())
}
