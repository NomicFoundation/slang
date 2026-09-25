//! The errors and events solc adds to an ABI beyond those the hierarchy declares: the ones the
//! code reachable from the contract's constructor, entry points and function pointers reverts
//! with or emits, wherever they are declared.

use crate::abi::{AbiEntry, ContractAbi};
use crate::ast::Definition;
use crate::compilation::CompilationUnit;
use crate::tests::fixtures;

fn errors_and_events(abi: &ContractAbi) -> Vec<String> {
    abi.entries()
        .iter()
        .filter_map(|entry| match entry {
            AbiEntry::Error(error) => Some(format!("error {}", error.name())),
            AbiEntry::Event(event) => Some(format!("event {}", event.name())),
            _ => None,
        })
        .collect()
}

fn contract_abi(unit: &CompilationUnit, name: &str) -> ContractAbi {
    unit.all_definitions()
        .find_map(|definition| match definition {
            Definition::Contract(contract) if contract.name().name() == name => Some(contract),
            _ => None,
        })
        .unwrap_or_else(|| panic!("contract `{name}` is declared"))
        .compute_abi()
        .expect("the ABI is computable")
}

/// Left out, as solc does: `SelectorOnly` (only its selector is read), `Unreached` and
/// `EvUnreached` (no entry point reaches them), `InPublicLib` (a public library function runs
/// in the library, not in `C`) and `InLibCallee` (only the library's own external function
/// reaches it).
#[test]
fn a_contract_lists_what_its_reachable_code_uses() {
    let unit = super::ReachedErrorsAndEvents::build_compilation_unit();
    assert_eq!(
        errors_and_events(&contract_abi(&unit, "C")),
        [
            "error InConstructor",
            "error InInternalLib",
            "error InModifier",
            "error Qualified",
            "error ViaPointer",
            "error ViaRequire",
            "event EvFree",
            "event QEv",
        ]
    );
}

#[test]
fn a_library_lists_what_its_external_functions_reach() {
    let unit = super::ReachedErrorsAndEvents::build_compilation_unit();
    let abi = |name| {
        fixtures::find_library(&unit, name)
            .compute_abi()
            .expect("the ABI is computable")
    };
    assert_eq!(errors_and_events(&abi("Pub")), ["error InPublicLib"]);
    assert_eq!(errors_and_events(&abi("Int")), ["error InLibCallee"]);
}

/// An abstract contract lists what its own entry points reach; a derived contract adds what
/// its overrides reach in the base.
#[test]
fn a_derived_contract_adds_what_its_overrides_reach() {
    let unit = super::ReachedFromAbstractBase::build_compilation_unit();
    assert_eq!(
        errors_and_events(&contract_abi(&unit, "A")),
        ["error FromAbstract", "event EvAbstract"]
    );
    assert_eq!(
        errors_and_events(&contract_abi(&unit, "B")),
        [
            "error FromAbstract",
            "error FromBaseUsedByChild",
            "event EvAbstract"
        ]
    );
}

/// A program creating a contract gets its errors and events from the analysis that finds the
/// bytecode dependencies, the others on the first request.
#[test]
fn a_program_creating_a_contract_lists_them_too() {
    let unit = super::ReachedWithContractReference::build_compilation_unit();
    assert_eq!(
        errors_and_events(&contract_abi(&unit, "Factory")),
        ["error FromFactory", "event Deployed"]
    );
}
