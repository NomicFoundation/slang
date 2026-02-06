use anyhow::Result;
use slang_solidity::backend::abi::{AbiEntry, ParameterComponent};

use crate::backend::fixtures;

#[test]
fn test_get_contracts_abi() -> Result<()> {
    let compilation_unit = fixtures::Counter::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let contracts = semantic_analysis.get_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name);
    assert_eq!("main.sol", contracts[0].file_id);
    assert_eq!(7, contracts[0].entries.len());

    let entries = &contracts[0].entries;

    assert!(matches!(entries[0], AbiEntry::Constructor { .. }));
    assert!(matches!(entries[1], AbiEntry::Function { ref name, .. } if name == "click"));
    assert!(matches!(entries[2], AbiEntry::Function { ref name, .. } if name == "count"));
    assert!(matches!(entries[3], AbiEntry::Function { ref name, .. } if name == "disable"));
    assert!(matches!(entries[4], AbiEntry::Function { ref name, .. } if name == "enable"));
    assert!(matches!(entries[5], AbiEntry::Function { ref name, .. } if name == "increment"));
    assert!(matches!(entries[6], AbiEntry::Function { ref name, .. } if name == "isEnabled"));

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

#[test]
fn test_full_abi_with_events_and_errors() -> Result<()> {
    let compilation_unit = fixtures::FullAbi::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let contracts_abi = semantic_analysis.get_contracts_abi();
    assert_eq!(contracts_abi.len(), 1);
    assert_eq!(contracts_abi[0].name, "Test");

    let entries = &contracts_abi[0].entries;
    assert_eq!(entries.len(), 9);

    assert!(matches!(entries[0], AbiEntry::Constructor { .. }));
    assert!(
        matches!(entries[1], AbiEntry::Error { ref name, .. } if name == "InsufficientBalance")
    );
    assert!(matches!(entries[2], AbiEntry::Error { ref name, .. } if name == "SomethingWrong"));
    assert!(matches!(entries[3], AbiEntry::Event { ref name, .. } if name == "BaseEvent"));
    assert!(matches!(entries[4], AbiEntry::Event { ref name, .. } if name == "Event"));
    assert!(matches!(entries[5], AbiEntry::Fallback { .. }));
    assert!(matches!(entries[6], AbiEntry::Function { ref name, .. } if name == "b"));
    assert!(matches!(entries[7], AbiEntry::Function { ref name, .. } if name == "foo"));
    assert!(matches!(entries[8], AbiEntry::Receive { .. }));

    Ok(())
}

#[test]
fn test_abi_entries_with_tuples() -> Result<()> {
    let compilation_unit = fixtures::AbiWithTuples::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let contracts_abi = semantic_analysis.get_contracts_abi();
    assert_eq!(contracts_abi.len(), 1);
    assert_eq!(contracts_abi[0].name, "Test");

    let entries = &contracts_abi[0].entries;

    let AbiEntry::Function {
        name,
        inputs,
        outputs,
        ..
    } = &entries[0]
    else {
        panic!("expected ABI for function");
    };
    assert_eq!(name, "f");
    assert_eq!(inputs.len(), 3);
    assert!(inputs[0].name.is_none());
    assert_eq!(inputs[0].r#type, "tuple");
    assert_eq!(
        inputs[0].components,
        vec![
            ParameterComponent {
                name: "a".to_string(),
                r#type: "uint256".to_string(),
                components: Vec::new()
            },
            ParameterComponent {
                name: "b".to_string(),
                r#type: "uint256[]".to_string(),
                components: Vec::new()
            },
            ParameterComponent {
                name: "c".to_string(),
                r#type: "tuple[]".to_string(),
                components: vec![
                    ParameterComponent {
                        name: "x".to_string(),
                        r#type: "uint256".to_string(),
                        components: Vec::new()
                    },
                    ParameterComponent {
                        name: "y".to_string(),
                        r#type: "uint256".to_string(),
                        components: Vec::new()
                    },
                ]
            },
        ]
    );

    assert!(inputs[1].name.is_none());
    assert_eq!(inputs[1].r#type, "tuple");
    assert_eq!(
        inputs[1].components,
        vec![
            ParameterComponent {
                name: "x".to_string(),
                r#type: "uint256".to_string(),
                components: Vec::new()
            },
            ParameterComponent {
                name: "y".to_string(),
                r#type: "uint256".to_string(),
                components: Vec::new()
            },
        ]
    );

    assert!(inputs[2].name.is_none());
    assert_eq!(inputs[2].r#type, "uint256");
    assert!(inputs[2].components.is_empty());
    assert!(outputs.is_empty());

    let AbiEntry::Function {
        name,
        inputs,
        outputs,
        ..
    } = &entries[1]
    else {
        panic!("expected ABI for function");
    };
    assert_eq!(name, "g");
    assert!(inputs.is_empty());
    assert_eq!(outputs.len(), 3);
    assert!(outputs[0].name.is_none());
    assert_eq!(outputs[0].r#type, "tuple");
    assert_eq!(outputs[0].components.len(), 3);
    assert!(outputs[1].name.is_none());
    assert_eq!(outputs[1].r#type, "tuple");
    assert_eq!(outputs[1].components.len(), 2);
    assert!(outputs[2].name.is_none());
    assert_eq!(outputs[2].r#type, "uint256");
    assert!(outputs[2].components.is_empty());

    Ok(())
}

#[test]
fn test_selectors_for_functions_with_tuple_parameters() -> Result<()> {
    let compilation_unit = fixtures::AbiWithTuples::build_compilation_unit()?;
    let semantic_analysis = compilation_unit.semantic_analysis();

    let test = semantic_analysis
        .find_contract_by_name("Test")
        .expect("contract is found");
    let functions = test.functions();

    // f((uint256,uint256[],(uint256,uint256)[]),(uint256,uint256),uint256)
    assert_eq!(functions[0].compute_selector(), Some(0x6f2b_e728_u32));
    // g()
    assert_eq!(functions[1].compute_selector(), Some(0xe217_9b8e_u32));

    Ok(())
}
