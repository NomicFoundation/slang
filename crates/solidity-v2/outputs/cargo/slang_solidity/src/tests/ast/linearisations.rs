use super::fixtures;
use crate::ast::ContractBase;
use crate::define_fixture;

define_fixture!(
    Overrides,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Base
{
    function in_base() internal pure {}
    function override_me() virtual external view {}
}

contract Middle is Base {
    function in_middle() external pure {}
    function override_me() virtual override public view {}
}

contract Inherited is Middle
{
    function override_me() override public pure {}
}
"#,
);

#[test]
fn test_contract_compute_linearised_bases() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.compute_linearised_bases();
    assert_eq!(bases.len(), 3);

    let ContractBase::Contract(counter) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(counter.name().name(), "Counter");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
    let ContractBase::Contract(ownable) = &bases[2] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");
}

#[test]
fn test_contract_compute_linearised_state_variables() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let state_variables = counter.compute_linearised_state_variables();
    assert_eq!(state_variables.len(), 4);

    assert_eq!(state_variables[0].name().name(), "_owner");
    assert_eq!(state_variables[1].name().name(), "_state");
    assert_eq!(state_variables[2].name().name(), "count");
    assert_eq!(state_variables[3].name().name(), "_clickers");
}

#[test]
fn test_contract_compute_linearised_functions() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let functions = counter.compute_linearised_functions();
    assert_eq!(functions.len(), 5);

    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "click"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "disable"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "enable"));
    assert!(functions[3]
        .name()
        .is_some_and(|name| name.name() == "increment"));
    assert!(functions[4]
        .name()
        .is_some_and(|name| name.name() == "isEnabled"));
}

#[test]
fn test_contract_compute_linearised_functions_with_overrides() {
    let unit = Overrides::build_compilation_unit();

    let inherited = unit
        .find_contract_by_name("Inherited")
        .expect("can find contract");
    let functions = inherited.compute_linearised_functions();
    assert_eq!(functions.len(), 3);
    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "in_base"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "in_middle"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "override_me"));
}
