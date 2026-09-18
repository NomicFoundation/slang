//! The JSON-ABI serialization, pinned against solc's `--abi` output for the same sources (0.8.34 and
//! 0.8.35 agree): the compact strings below are solc's verbatim, so key order and `internalType`
//! spelling are covered along with the entries themselves.

use crate::abi::ContractAbi;
use crate::define_fixture;
use crate::tests::fixtures;

fn json(abi: &ContractAbi) -> String {
    serde_json::to_string(&abi.json()).expect("the ABI serializes")
}

#[test]
fn tuples_and_getters_match_solc() {
    let unit = super::AbiWithTuples::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Test")
        .next()
        .expect("contract Test exists")
        .compute_abi()
        .expect("the ABI is computable");

    assert_eq!(
        json(&abi),
        r#"[{"inputs":[{"components":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256[]","name":"b","type":"uint256[]"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T[]","name":"c","type":"tuple[]"}],"internalType":"struct Test.S","name":"","type":"tuple"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"","type":"tuple"},{"internalType":"uint256","name":"x","type":"uint256"}],"name":"f","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"g","outputs":[{"components":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256[]","name":"b","type":"uint256[]"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T[]","name":"c","type":"tuple[]"}],"internalType":"struct Test.S","name":"s","type":"tuple"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"t","type":"tuple"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"t","outputs":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"t_components","outputs":[{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"t_struct","outputs":[{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"","type":"tuple"}],"stateMutability":"view","type":"function"}]"#
    );
}

#[test]
fn library_matches_solc() {
    let unit = super::LibraryAbi::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");

    // `f` and `g` write state and `i` is internal: none of them is in the ABI.
    assert_eq!(
        json(&abi),
        r#"[{"inputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"name":"h","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"L.U","name":"u","type":"uint64"}],"name":"j","outputs":[],"stateMutability":"pure","type":"function"}]"#
    );
}

/// A `view` function that takes or returns a storage reference is reachable only by
/// `DELEGATECALL`; solc filters on both halves of the signature.
#[test]
fn library_functions_with_storage_in_the_signature_are_left_out() {
    let unit = super::LibraryStorageSignature::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");

    assert_eq!(
        json(&abi),
        r#"[{"inputs":[{"internalType":"uint256","name":"k","type":"uint256"}],"name":"value","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#
    );
}

define_fixture!(
    InternalTypeSpellings,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
interface IFoo { function f() external; }
type Wad is uint256;
enum E { A }
contract U {
    enum E { A, B }
    struct S { uint256 x; E e; }
    S public s;
    mapping(address => S) public m;
    function g(address payable p, IFoo i, E e, Wad w, S[] calldata arr) external pure returns (uint8) {}
}
contract F {
    function a(function(uint256) external cb) external pure {}
    function b(function(uint256) external view returns (bool) cb) external pure {}
    function c(function() external payable cb, E e, uint256[3] memory xs, string calldata s, bytes calldata bs) external pure {}
}
"#,
);

#[test]
fn internal_types_match_solc() {
    let unit = InternalTypeSpellings::build_compilation_unit();

    // Kind prefixes, `address payable`, a user-defined value type by name, and a getter over a
    // struct flattened to its members and named after them.
    let user_defined = unit
        .find_contract_by_name("U")
        .next()
        .expect("contract U exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(&user_defined),
        r#"[{"inputs":[{"internalType":"address payable","name":"p","type":"address"},{"internalType":"contract IFoo","name":"i","type":"address"},{"internalType":"enum U.E","name":"e","type":"uint8"},{"internalType":"Wad","name":"w","type":"uint256"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"enum U.E","name":"e","type":"uint8"}],"internalType":"struct U.S[]","name":"arr","type":"tuple[]"}],"name":"g","outputs":[{"internalType":"uint8","name":"","type":"uint8"}],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"address","name":"","type":"address"}],"name":"m","outputs":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"enum U.E","name":"e","type":"uint8"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"s","outputs":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"enum U.E","name":"e","type":"uint8"}],"stateMutability":"view","type":"function"}]"#
    );

    // Function types spell their parameters, mutability, visibility and returns; `nonpayable`
    // is implied.
    let function_types = unit
        .find_contract_by_name("F")
        .next()
        .expect("contract F exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(&function_types),
        r#"[{"inputs":[{"internalType":"function (uint256) external","name":"cb","type":"function"}],"name":"a","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"function (uint256) view external returns (bool)","name":"cb","type":"function"}],"name":"b","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"function () payable external","name":"cb","type":"function"},{"internalType":"enum E","name":"e","type":"uint8"},{"internalType":"uint256[3]","name":"xs","type":"uint256[3]"},{"internalType":"string","name":"s","type":"string"},{"internalType":"bytes","name":"bs","type":"bytes"}],"name":"c","outputs":[],"stateMutability":"pure","type":"function"}]"#
    );
}

define_fixture!(
    Overloads,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
contract Overloaded {
    function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata data) external {}
    function safeTransferFrom(address from, address to, uint256 tokenId) external {}
    function transferFromAndCall(address from, address to, uint256 value) external returns (bool) {}
    function transferFromAndCall(address from, address to, uint256 value, bytes calldata data) external returns (bool) {}
}
"#,
);

/// solc lists overloads in ascending selector order, not declaration order: the three-parameter
/// `safeTransferFrom` (`0x42842e0e`) precedes the four-parameter one (`0xb88d4fde`), while the
/// four-parameter `transferFromAndCall` (`0xc1d34b89`) precedes the three-parameter one
/// (`0xd8fbe994`).
#[test]
fn overloads_follow_selector_order() {
    let unit = Overloads::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Overloaded")
        .next()
        .expect("contract Overloaded exists")
        .compute_abi()
        .expect("the ABI is computable");

    assert_eq!(
        json(&abi),
        r#"[{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"value","type":"uint256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"transferFromAndCall","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"value","type":"uint256"}],"name":"transferFromAndCall","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"nonpayable","type":"function"}]"#
    );
}

/// A getter's inputs carry the mapping key names, an array index stays unnamed, and its output
/// carries the innermost value name unless struct members name the outputs.
#[test]
fn getters_carry_mapping_names() {
    let unit = super::NamedMappings::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("Named")
        .next()
        .expect("contract Named exists")
        .compute_abi()
        .expect("the ABI is computable");

    assert_eq!(
        json(&abi),
        r#"[{"inputs":[{"internalType":"address","name":"funder","type":"address"}],"name":"addressToAmountFunded","outputs":[{"internalType":"uint256","name":"amountFunded","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"key","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"name":"arrays","outputs":[{"internalType":"uint256","name":"values","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"id","type":"uint256"}],"name":"nested","outputs":[{"internalType":"bool","name":"ok","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"","type":"uint256"}],"name":"plain","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"key","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"name":"structArrays","outputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"key","type":"uint256"}],"name":"structs","outputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"","type":"uint256"}],"name":"unnamedKey","outputs":[{"internalType":"uint256","name":"value","type":"uint256"}],"stateMutability":"view","type":"function"}]"#
    );
}

/// A library lists its constants' getters, errors, events and `view`/`pure` functions; `mut_fn`
/// writes state and is left out.
#[test]
fn library_lists_constants_and_read_only_functions() {
    let unit = super::LibraryMembers::build_compilation_unit();
    let library = fixtures::find_library(&unit, "LJ")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(&library),
        r#"[{"inputs":[],"name":"E","type":"error"},{"anonymous":false,"inputs":[],"name":"Ev","type":"event"},{"inputs":[],"name":"X","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"name":"view_fn","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#
    );
}

/// An abstract contract cannot run its constructor, so solc leaves it out.
#[test]
fn abstract_contract_has_no_constructor() {
    let unit = super::AbstractConstructor::build_compilation_unit();
    let abstract_contract = unit
        .find_contract_by_name("AB")
        .next()
        .expect("contract AB exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(&abstract_contract),
        r#"[{"inputs":[],"name":"extra","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#
    );
}
