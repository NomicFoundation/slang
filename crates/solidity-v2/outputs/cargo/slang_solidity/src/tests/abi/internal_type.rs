//! `AbiParameter::internal_type()` spells the type the way solc's JSON-ABI `internalType` field
//! does (checked against solc 0.8.34 `--abi` for the same source).

use crate::abi::AbiEntry;
use crate::define_fixture;

define_fixture!(
    InternalTypes,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
interface IFoo { function f() external; }
type Wad is uint256;
enum E { A }
contract U {
    enum E { A, B }
    struct S { uint256 x; E e; }
    function g(address payable p, IFoo i, E e, Wad w, S[] calldata arr, uint256[3] memory xs, string calldata s) external pure returns (uint8) {}
}
contract F {
    function a(function(uint256) external cb) external pure {}
    function b(function(uint256) external view returns (bool) cb) external pure {}
    function c(function() external payable cb, E e) external pure {}
}
"#,
);

fn function_input_internal_types(contract: &str, function: &str) -> Vec<String> {
    let unit = InternalTypes::build_compilation_unit();
    let abi = unit
        .find_contract_by_name(contract)
        .next()
        .expect("contract exists")
        .compute_abi()
        .expect("the ABI is computable");
    let entry = abi
        .entries()
        .iter()
        .find(|entry| matches!(entry, AbiEntry::Function(f) if f.name() == function))
        .expect("function is in the ABI");
    let AbiEntry::Function(function) = entry else {
        unreachable!("matched a function above");
    };
    function
        .inputs()
        .iter()
        .map(|input| input.internal_type())
        .collect()
}

#[test]
fn user_defined_types_carry_their_kind_and_scope() {
    assert_eq!(
        function_input_internal_types("U", "g"),
        [
            "address payable",
            "contract IFoo",
            "enum U.E",
            "Wad",
            "struct U.S[]",
            "uint256[3]",
            "string",
        ]
    );
}

#[test]
fn function_types_spell_mutability_visibility_and_returns() {
    assert_eq!(
        function_input_internal_types("F", "a"),
        ["function (uint256) external"]
    );
    assert_eq!(
        function_input_internal_types("F", "b"),
        ["function (uint256) view external returns (bool)"]
    );
    assert_eq!(
        function_input_internal_types("F", "c"),
        ["function () payable external", "enum E"]
    );
}
