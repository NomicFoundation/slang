//! The JSON ABI's `internalType` spells the type the way solc's does. The fixture builds at `LanguageVersion::LATEST`; the expected spellings were taken by
//! hand from solc 0.8.34 `--abi` on the same source, not from a solc run in the test.
// TODO: compare these spellings against a solc run at the fixture's own version instead of
// pinning them by hand. Until then, a failure after a `LATEST` bump means re-deriving the
// expectation from solc, not editing it.

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
    let json = serde_json::to_value(&abi).expect("the ABI serializes");
    let entry = json
        .as_array()
        .expect("the ABI is an array")
        .iter()
        .find(|entry| entry["type"] == "function" && entry["name"] == function)
        .expect("function is in the ABI");
    entry["inputs"]
        .as_array()
        .expect("a function has inputs")
        .iter()
        .map(|input| {
            input["internalType"]
                .as_str()
                .expect("an input has an internal type")
                .to_owned()
        })
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
