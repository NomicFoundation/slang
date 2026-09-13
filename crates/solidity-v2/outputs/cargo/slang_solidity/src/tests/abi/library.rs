//! A library's ABI, as solc emits it: `view`/`pure` external functions that neither take nor
//! return a storage reference, public constants' getters, errors and events.

use crate::abi::{AbiEntry, ContractAbi};
use crate::tests::fixtures;

fn entry_names(abi: &ContractAbi) -> Vec<String> {
    abi.entries()
        .iter()
        .map(|entry| match entry {
            AbiEntry::Function(function) => format!("function {}", function.name()),
            AbiEntry::Error(error) => format!("error {}", error.name()),
            AbiEntry::Event(event) => format!("event {}", event.name()),
            other => panic!("a library has no {other:?}"),
        })
        .collect()
}

#[test]
fn storage_references_and_internal_functions_are_left_out() {
    let unit = super::LibraryAbi::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");
    // `f` and `g` take storage references and `i` is internal.
    assert_eq!(entry_names(&abi), ["function h", "function j"]);
    assert!(abi.storage_layout().is_empty());
}

/// solc filters on the return types as well: a `view` function returning a storage reference is
/// reachable only by `DELEGATECALL`.
#[test]
fn storage_reference_returns_are_left_out() {
    let unit = super::LibraryStorageReturn::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(entry_names(&abi), ["function value"]);
}

#[test]
fn constants_errors_events_and_read_only_functions_are_listed() {
    let unit = super::LibraryMembers::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "LJ")
        .compute_abi()
        .expect("the ABI is computable");
    // `mut_fn` writes state and is left out.
    assert_eq!(
        entry_names(&abi),
        ["error E", "event Ev", "function X", "function view_fn"]
    );
}
