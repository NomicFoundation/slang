use anyhow::Result;

use super::fixtures;
use crate::abi::AbiEntry;

/// Convenience macro to test equality of tuple components for input/output parameters.
///
/// Usage:
/// ```
/// assert_components_eq!(inputs[0].components(), [
///     ("a", "uint256", []),
///     ("c", "tuple[]", [
///         ("x", "uint256", []),
///         ("y", "uint256", []),
///     ]),
/// ]);
/// ```
macro_rules! assert_components_eq {
    ($components:expr, [ $(($name:expr, $type_name:expr, [$($sub:tt)*])),* $(,)? ]) => {
        {
            let components = $components;
            let expected: &[(&str, &str)] = &[$(($name, $type_name)),*];
            assert_eq!(
                components.len(),
                expected.len(),
                "expected {} components, got {}",
                expected.len(),
                components.len()
            );
            assert_components_eq!(@step components, 0, $(($name, $type_name, [$($sub)*]),)*);
        }
    };
    (@step $components:ident, $index:expr, ) => {};
    (@step $components:ident, $index:expr, ($name:expr, $type_name:expr, [$($sub:tt)*]), $($rest:tt)*) => {
        assert_eq!($components[$index].name(), $name);
        assert_eq!($components[$index].type_name(), $type_name);
        assert_components_eq!($components[$index].components(), [$($sub)*]);
        assert_components_eq!(@step $components, $index + 1, $($rest)*);
    };
}

#[test]
fn test_get_contracts_abi() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let contracts = unit.compute_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name());
    assert_eq!("main.sol", contracts[0].file_id());
    assert_eq!(7, contracts[0].entries().len());

    let entries = contracts[0].entries();

    assert!(matches!(entries[0], AbiEntry::Constructor(_)));
    assert!(matches!(&entries[1], AbiEntry::Function(f) if f.name() == "click"));
    assert!(matches!(&entries[2], AbiEntry::Function(f) if f.name() == "count"));
    assert!(matches!(&entries[3], AbiEntry::Function(f) if f.name() == "disable"));
    assert!(matches!(&entries[4], AbiEntry::Function(f) if f.name() == "enable"));
    assert!(matches!(&entries[5], AbiEntry::Function(f) if f.name() == "increment"));
    assert!(matches!(&entries[6], AbiEntry::Function(f) if f.name() == "isEnabled"));

    Ok(())
}

macro_rules! assert_layout_item_eq {
    ($item:expr, $name:expr, $slot:expr, $offset:expr, $type:expr) => {
        assert_eq!($item.label(), $name);
        assert_eq!($item.type_name(), $type);
        assert_eq!($item.slot(), $slot);
        assert_eq!($item.offset(), $offset);
    };
}

#[test]
fn test_storage_layout() -> Result<()> {
    let unit = fixtures::StorageLayout::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("C")
        .expect("contract can be found");
    let counter_abi = counter
        .compute_abi_with_file_id("main.sol".to_string())
        .expect("can compute ABI");
    let layout = counter_abi.storage_layout();

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

    let transient_layout = counter_abi.transient_storage_layout();
    assert!(transient_layout.is_empty());

    Ok(())
}

#[test]
fn test_transient_and_custom_storage_layout() -> Result<()> {
    let unit = fixtures::StorageLayout::build_compilation_unit()?;

    let d_contract = unit
        .find_contract_by_name("D")
        .expect("contract can be found");
    let d_abi = d_contract
        .compute_abi_with_file_id("main.sol".to_string())
        .expect("can compute ABI");
    let d_layout = d_abi.storage_layout();

    assert_eq!(d_layout.len(), 2);
    assert_layout_item_eq!(d_layout[0], "a", 42, 0, "uint256");
    assert_layout_item_eq!(d_layout[1], "p", 43, 0, "uint256");

    let e_contract = unit
        .find_contract_by_name("E")
        .expect("contract can be found");
    let e_abi = e_contract
        .compute_abi_with_file_id("main.sol".to_string())
        .expect("can compute ABI");
    let e_layout = e_abi.storage_layout();
    let e_transient_layout = e_abi.transient_storage_layout();

    assert_eq!(e_layout.len(), 2);
    assert_layout_item_eq!(e_layout[0], "q", 20, 0, "int8");
    assert_layout_item_eq!(e_layout[1], "r", 20, 1, "bytes5");

    assert_eq!(e_transient_layout.len(), 2);
    assert_layout_item_eq!(e_transient_layout[0], "qt", 0, 0, "int8");
    assert_layout_item_eq!(e_transient_layout[1], "rt", 0, 1, "bytes5");

    Ok(())
}

#[test]
fn test_function_selector() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
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
    let unit = fixtures::FullAbi::build_compilation_unit()?;

    let contracts_abi = unit.compute_contracts_abi();
    assert_eq!(contracts_abi.len(), 1);
    assert_eq!(contracts_abi[0].name(), "Test");

    let entries = contracts_abi[0].entries();
    assert_eq!(entries.len(), 9);

    assert!(matches!(entries[0], AbiEntry::Constructor(_)));
    assert!(matches!(&entries[1], AbiEntry::Error(e) if e.name() == "InsufficientBalance"));
    assert!(matches!(&entries[2], AbiEntry::Error(e) if e.name() == "SomethingWrong"));
    assert!(matches!(&entries[3], AbiEntry::Event(e) if e.name() == "BaseEvent"));
    assert!(matches!(&entries[4], AbiEntry::Event(e) if e.name() == "Event"));
    assert!(matches!(entries[5], AbiEntry::Fallback(_)));
    assert!(matches!(&entries[6], AbiEntry::Function(f) if f.name() == "b"));
    assert!(matches!(&entries[7], AbiEntry::Function(f) if f.name() == "foo"));
    assert!(matches!(entries[8], AbiEntry::Receive(_)));

    Ok(())
}

#[test]
fn test_abi_entries_with_tuples() -> Result<()> {
    let unit = fixtures::AbiWithTuples::build_compilation_unit()?;

    let contracts_abi = unit.compute_contracts_abi();
    assert_eq!(contracts_abi.len(), 1);
    assert_eq!(contracts_abi[0].name(), "Test");

    let entries = contracts_abi[0].entries();

    let AbiEntry::Function(function) = &entries[0] else {
        panic!("expected ABI for function");
    };
    assert_eq!(function.name(), "f");
    let inputs = function.inputs();
    let outputs = function.outputs();
    assert_eq!(inputs.len(), 3);
    assert!(inputs[0].name().is_none());
    assert_eq!(inputs[0].type_name(), "tuple");
    assert_components_eq!(
        inputs[0].components(),
        [
            ("a", "uint256", []),
            ("b", "uint256[]", []),
            (
                "c",
                "tuple[]",
                [("x", "uint256", []), ("y", "uint256", []),]
            ),
        ]
    );

    assert!(inputs[1].name().is_none());
    assert_eq!(inputs[1].type_name(), "tuple");
    assert_components_eq!(
        inputs[1].components(),
        [("x", "uint256", []), ("y", "uint256", []),]
    );

    assert!(inputs[2].name().is_none());
    assert_eq!(inputs[2].type_name(), "uint256");
    assert!(inputs[2].components().is_empty());
    assert!(outputs.is_empty());

    let AbiEntry::Function(function) = &entries[1] else {
        panic!("expected ABI for function");
    };
    assert_eq!(function.name(), "g");
    let inputs = function.inputs();
    let outputs = function.outputs();
    assert!(inputs.is_empty());
    assert_eq!(outputs.len(), 3);
    assert!(outputs[0].name().is_none());
    assert_eq!(outputs[0].type_name(), "tuple");
    assert_components_eq!(
        outputs[0].components(),
        [
            ("a", "uint256", []),
            ("b", "uint256[]", []),
            (
                "c",
                "tuple[]",
                [("x", "uint256", []), ("y", "uint256", []),]
            ),
        ]
    );
    assert!(outputs[1].name().is_none());
    assert_eq!(outputs[1].type_name(), "tuple");
    assert_components_eq!(
        outputs[1].components(),
        [("x", "uint256", []), ("y", "uint256", []),]
    );
    assert!(outputs[2].name().is_none());
    assert_eq!(outputs[2].type_name(), "uint256");
    assert!(outputs[2].components().is_empty());

    Ok(())
}

#[test]
fn test_selectors_for_functions_with_tuple_parameters() -> Result<()> {
    let unit = fixtures::AbiWithTuples::build_compilation_unit()?;

    let test = unit
        .find_contract_by_name("Test")
        .expect("contract is found");
    let functions = test.functions();

    // f((uint256,uint256[],(uint256,uint256)[]),(uint256,uint256),uint256)
    assert_eq!(functions[0].compute_selector(), Some(0x6f2b_e728_u32));
    // g()
    assert_eq!(functions[1].compute_selector(), Some(0xe217_9b8e_u32));

    Ok(())
}
