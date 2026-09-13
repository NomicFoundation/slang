//! The JSON-ABI serialization, pinned against solc 0.8.34's `--abi` output for the same sources:
//! the compact strings below are solc's verbatim, so key order and `internalType` spelling are
//! covered along with the entries themselves.

use crate::abi::AbiEntry;
use crate::define_fixture;
use crate::tests::fixtures;

fn json(entries: &[AbiEntry]) -> String {
    serde_json::to_string(entries).expect("the ABI serializes")
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
        json(abi.entries()),
        r#"[{"inputs":[{"components":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256[]","name":"b","type":"uint256[]"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T[]","name":"c","type":"tuple[]"}],"internalType":"struct Test.S","name":"","type":"tuple"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"","type":"tuple"},{"internalType":"uint256","name":"x","type":"uint256"}],"name":"f","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"g","outputs":[{"components":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256[]","name":"b","type":"uint256[]"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T[]","name":"c","type":"tuple[]"}],"internalType":"struct Test.S","name":"s","type":"tuple"},{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"t","type":"tuple"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"t","outputs":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"t_components","outputs":[{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"t_struct","outputs":[{"components":[{"internalType":"uint256","name":"x","type":"uint256"},{"internalType":"uint256","name":"y","type":"uint256"}],"internalType":"struct Test.T","name":"","type":"tuple"}],"stateMutability":"view","type":"function"}]"#
    );
}

#[test]
fn library_matches_solc() {
    let unit = super::LibraryAbi::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");

    // `f` and `g` take storage references and `i` is internal: none of them is in the ABI.
    assert_eq!(
        json(abi.entries()),
        r#"[{"inputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"name":"h","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"L.U","name":"u","type":"uint64"}],"name":"j","outputs":[],"stateMutability":"pure","type":"function"}]"#
    );
}

define_fixture!(
    InterfaceHierarchy,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
interface IBase {
    function ping(uint256 v) external returns (uint256);
    event Pinged(uint256 v);
}
interface IDerived is IBase {
    function pong() external;
    function ping(uint256 v) external override returns (uint256);
}
abstract contract AB is IBase {
    function extra() external virtual;
}
"#,
);

#[test]
fn interface_hierarchy_matches_solc() {
    let unit = InterfaceHierarchy::build_compilation_unit();

    let base = fixtures::find_interface(&unit, "IBase")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(base.entries()),
        r#"[{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"v","type":"uint256"}],"name":"Pinged","type":"event"},{"inputs":[{"internalType":"uint256","name":"v","type":"uint256"}],"name":"ping","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"nonpayable","type":"function"}]"#
    );

    // The inherited event and the overridden `ping` appear once each.
    let derived = fixtures::find_interface(&unit, "IDerived")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(derived.entries()),
        r#"[{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"v","type":"uint256"}],"name":"Pinged","type":"event"},{"inputs":[{"internalType":"uint256","name":"v","type":"uint256"}],"name":"ping","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"pong","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#
    );

    let abstract_contract = unit
        .find_contract_by_name("AB")
        .next()
        .expect("contract AB exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(abstract_contract.entries()),
        r#"[{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"v","type":"uint256"}],"name":"Pinged","type":"event"},{"inputs":[],"name":"extra","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"v","type":"uint256"}],"name":"ping","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"nonpayable","type":"function"}]"#
    );
}

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
    let unit = InternalTypes::build_compilation_unit();

    // Kind prefixes, `address payable`, a user-defined value type by name, and a getter over a
    // struct flattened to its members and named after them.
    let user_defined = unit
        .find_contract_by_name("U")
        .next()
        .expect("contract U exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert_eq!(
        json(user_defined.entries()),
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
        json(function_types.entries()),
        r#"[{"inputs":[{"internalType":"function (uint256) external","name":"cb","type":"function"}],"name":"a","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"function (uint256) view external returns (bool)","name":"cb","type":"function"}],"name":"b","outputs":[],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"function () payable external","name":"cb","type":"function"},{"internalType":"enum E","name":"e","type":"uint8"},{"internalType":"uint256[3]","name":"xs","type":"uint256[3]"},{"internalType":"string","name":"s","type":"string"},{"internalType":"bytes","name":"bs","type":"bytes"}],"name":"c","outputs":[],"stateMutability":"pure","type":"function"}]"#
    );
}
