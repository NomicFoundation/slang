//! solc names a getter's parameters after the declaration: mapping key names for the inputs,
//! and for the outputs the struct members the return type is built from, else the innermost
//! mapping's value name. An array index stays unnamed.

use crate::abi::{AbiEntry, AbiFunction, ContractAbi};

fn getter<'a>(abi: &'a ContractAbi, name: &str) -> &'a AbiFunction {
    abi.entries()
        .iter()
        .find_map(|entry| match entry {
            AbiEntry::Function(function) if function.name() == name => Some(function),
            _ => None,
        })
        .unwrap_or_else(|| panic!("getter `{name}` is in the ABI"))
}

fn names(parameters: &[crate::abi::AbiParameter]) -> Vec<Option<&str>> {
    parameters
        .iter()
        .map(|parameter| parameter.name())
        .collect()
}

#[test]
fn getter_inputs_carry_mapping_key_names_and_outputs_the_value_name() {
    let unit = super::NamedMappings::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Named")
        .next()
        .expect("contract Named exists")
        .compute_abi()
        .expect("the ABI is computable");

    let funded = getter(&abi, "addressToAmountFunded");
    assert_eq!(names(funded.inputs()), [Some("funder")]);
    assert_eq!(names(funded.outputs()), [Some("amountFunded")]);

    let nested = getter(&abi, "nested");
    assert_eq!(names(nested.inputs()), [Some("owner"), Some("id")]);
    assert_eq!(names(nested.outputs()), [Some("ok")]);

    // An array index is unnamed; the value name reaches through the array.
    let arrays = getter(&abi, "arrays");
    assert_eq!(names(arrays.inputs()), [Some("key"), None]);
    assert_eq!(names(arrays.outputs()), [Some("values")]);

    let unnamed_key = getter(&abi, "unnamedKey");
    assert_eq!(names(unnamed_key.inputs()), [None]);
    assert_eq!(names(unnamed_key.outputs()), [Some("value")]);

    let plain = getter(&abi, "plain");
    assert_eq!(names(plain.inputs()), [None]);
    assert_eq!(names(plain.outputs()), [None]);
}

#[test]
fn struct_getter_outputs_are_named_after_the_members() {
    let unit = super::NamedMappings::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Named")
        .next()
        .expect("contract Named exists")
        .compute_abi()
        .expect("the ABI is computable");

    // `ys` is an array member and is not returned by the getter.
    let structs = getter(&abi, "structs");
    assert_eq!(names(structs.inputs()), [Some("key")]);
    assert_eq!(names(structs.outputs()), [Some("x")]);

    let unit = super::AbiWithTuples::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Test")
        .next()
        .expect("contract Test exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(names(getter(&abi, "t").outputs()), [Some("x"), Some("y")]);
}
