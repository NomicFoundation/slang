//! An interface's ABI, as solc emits it: the functions, errors and events of its whole
//! hierarchy, an overriding declaration listed in place of the one it overrides.

use crate::abi::{AbiEntry, ContractAbi};
use crate::tests::{fixtures, support};

fn entry_names(abi: &ContractAbi) -> Vec<String> {
    abi.entries()
        .iter()
        .map(|entry| match entry {
            AbiEntry::Function(function) => format!("function {}", function.name()),
            AbiEntry::Error(error) => format!("error {}", error.name()),
            AbiEntry::Event(event) => format!("event {}", event.name()),
            AbiEntry::Fallback(_) => "fallback".to_owned(),
            AbiEntry::Receive(_) => "receive".to_owned(),
            AbiEntry::Constructor(_) => panic!("an interface has no constructor"),
        })
        .collect()
}

#[test]
fn a_base_interface_lists_its_own_members() {
    let unit = super::InterfaceHierarchy::build_compilation_unit();
    let abi = fixtures::find_interface(&unit, "IBase")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        entry_names(&abi),
        [
            "error Denied",
            "event Moved",
            "function balance",
            "function move"
        ]
    );
}

#[test]
fn a_derived_interface_lists_the_hierarchy_once() {
    let unit = super::InterfaceHierarchy::build_compilation_unit();
    let derived = fixtures::find_interface(&unit, "IDerived");
    let abi = derived.compute_abi().expect("the ABI is computable");
    assert_eq!(
        entry_names(&abi),
        [
            "error Denied",
            "event Moved",
            "event Paused",
            "fallback",
            "function balance",
            "function move",
            "function pause",
            "receive",
        ]
    );

    let own_move = derived
        .functions()
        .into_iter()
        .find(|function| function.name().is_some_and(|name| name.name() == "move"))
        .expect("`IDerived` declares `move`");
    let listed_move = abi
        .entries()
        .iter()
        .find(|entry| matches!(entry, AbiEntry::Function(function) if function.name() == "move"))
        .expect("`move` is listed");
    assert_eq!(listed_move.node_id(), own_move.node_id());
}

/// solc rejects an interface with a contract base; Slang has no validation for it yet.
#[test]
fn an_interface_inheriting_a_contract_has_no_abi() {
    let unit = support::compile([(
        "main.sol".into(),
        "pragma solidity ^0.8.0;\ncontract C { function c() external {} }\ninterface I is C { function i() external; }\n",
    )]);
    assert!(fixtures::find_interface(&unit, "I").compute_abi().is_none());
}
