use super::fixtures;
use crate::ast::ContractBase;
use crate::define_fixture;

#[test]
fn test_contract_direct_bases() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");
    let bases = counter.direct_bases();
    assert_eq!(bases.len(), 2);

    let ContractBase::Contract(ownable) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
}

#[test]
fn test_contract_constructor_and_modifiers() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");

    let constructor = counter.constructor();
    assert!(constructor.is_some());

    let modifiers = counter.modifiers();
    assert_eq!(modifiers.len(), 0);
}

define_fixture!(
    LibraryMembers,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

library L {
    uint256 public constant PUBLIC_CONSTANT = 21;

    modifier guard() { _; }

    function f(uint256 x) external pure returns (uint256) { return x; }
    function g(uint256 x) internal pure returns (uint256) { return x; }
}
"#);

#[test]
fn test_library_functions_and_state_variables() {
    let unit = LibraryMembers::build_compilation_unit();
    let library = fixtures::find_library(&unit, "L");

    let functions = library.functions();
    let [f, g] = functions.as_slice() else {
        panic!("expected two functions: the modifier is not one");
    };
    assert_eq!(f.name().expect("a library function is named").name(), "f");
    assert_eq!(g.name().expect("a library function is named").name(), "g");

    let state_variables = library.state_variables();
    let [constant] = state_variables.as_slice() else {
        panic!("expected the library to declare one state variable");
    };
    assert_eq!(constant.name().name(), "PUBLIC_CONSTANT");
}
