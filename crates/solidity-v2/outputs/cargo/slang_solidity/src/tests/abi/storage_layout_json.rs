//! The JSON storage layout, pinned as compact strings against solc 0.8.37's
//! `storageLayout` and `transientStorageLayout` output for the same sources.
//! Node ids are not solc's, so [`json`] replaces them with `N`.

use crate::abi::StorageLayout;
use crate::define_fixture;

/// The prefixes of the identifiers that end with a definition's node id.
const USER_DEFINED_PREFIXES: [&str; 4] = [
    "t_contract(",
    "t_enum(",
    "t_struct(",
    "t_userDefinedValueType(",
];

/// Serializes `layout`, replacing each `astId` and each node id that ends a
/// user-defined type identifier with `N`.
fn json(layout: &StorageLayout) -> String {
    let json = serde_json::to_string(layout).expect("the storage layout serializes");
    let mut normalized = String::with_capacity(json.len());
    let mut rest = json.as_str();
    while !rest.is_empty() {
        let id_start = if let Some(after) = rest.strip_prefix("\"astId\":") {
            normalized.push_str("\"astId\":");
            Some(after)
        } else if USER_DEFINED_PREFIXES
            .iter()
            .any(|prefix| rest.starts_with(prefix))
        {
            let name_end = rest.find(')').expect("the name is parenthesized") + 1;
            normalized.push_str(&rest[..name_end]);
            Some(&rest[name_end..])
        } else {
            None
        };
        if let Some(after) = id_start {
            normalized.push('N');
            rest = after.trim_start_matches(|c: char| c.is_ascii_digit());
        } else {
            let mut chars = rest.chars();
            normalized.extend(chars.next());
            rest = chars.as_str();
        }
    }
    normalized
}

define_fixture!(
    ValueTypes,
    file: "main.sol", r#"
pragma solidity *;
interface I {}
type U is uint64;

contract Base {
    uint8 small;
    address payable owner;
}

contract Values is Base {
    enum E { A, B }
    bool flag;
    int24 signedInt;
    bytes7 shortBytes;
    uint256 word;
    I i;
    U u;
    E e;
    ufixed128x18 fx;
    uint256 constant C = 1;
    uint256 transient lock;
    address transient caller;
}

contract Empty {
    uint256 immutable x = 1;
}
"#,
);

// Value types pack into shared slots, inherited variables come first under the
// derived contract's name, and transient variables have a layout of their own.
// A contract without stored variables has no type table.
#[test]
fn value_types_serialize() {
    let unit = ValueTypes::build_compilation_unit();
    let values = unit
        .find_contract_by_name("Values")
        .next()
        .expect("contract Values exists");
    assert_eq!(
        json(
            &values
                .compute_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[{"astId":N,"contract":"main.sol:Values","label":"small","offset":0,"slot":"0","type":"t_uint8"},{"astId":N,"contract":"main.sol:Values","label":"owner","offset":1,"slot":"0","type":"t_address_payable"},{"astId":N,"contract":"main.sol:Values","label":"flag","offset":21,"slot":"0","type":"t_bool"},{"astId":N,"contract":"main.sol:Values","label":"signedInt","offset":22,"slot":"0","type":"t_int24"},{"astId":N,"contract":"main.sol:Values","label":"shortBytes","offset":25,"slot":"0","type":"t_bytes7"},{"astId":N,"contract":"main.sol:Values","label":"word","offset":0,"slot":"1","type":"t_uint256"},{"astId":N,"contract":"main.sol:Values","label":"i","offset":0,"slot":"2","type":"t_contract(I)N"},{"astId":N,"contract":"main.sol:Values","label":"u","offset":20,"slot":"2","type":"t_userDefinedValueType(U)N"},{"astId":N,"contract":"main.sol:Values","label":"e","offset":28,"slot":"2","type":"t_enum(E)N"},{"astId":N,"contract":"main.sol:Values","label":"fx","offset":0,"slot":"3","type":"t_ufixed128x18"}],"types":{"t_address_payable":{"encoding":"inplace","label":"address payable","numberOfBytes":"20"},"t_bool":{"encoding":"inplace","label":"bool","numberOfBytes":"1"},"t_bytes7":{"encoding":"inplace","label":"bytes7","numberOfBytes":"7"},"t_contract(I)N":{"encoding":"inplace","label":"contract I","numberOfBytes":"20"},"t_enum(E)N":{"encoding":"inplace","label":"enum Values.E","numberOfBytes":"1"},"t_int24":{"encoding":"inplace","label":"int24","numberOfBytes":"3"},"t_ufixed128x18":{"encoding":"inplace","label":"ufixed128x18","numberOfBytes":"16"},"t_uint256":{"encoding":"inplace","label":"uint256","numberOfBytes":"32"},"t_uint8":{"encoding":"inplace","label":"uint8","numberOfBytes":"1"},"t_userDefinedValueType(U)N":{"encoding":"inplace","label":"U","numberOfBytes":"8"}}}"#
    );
    assert_eq!(
        json(
            &values
                .compute_transient_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[{"astId":N,"contract":"main.sol:Values","label":"lock","offset":0,"slot":"0","type":"t_uint256"},{"astId":N,"contract":"main.sol:Values","label":"caller","offset":0,"slot":"1","type":"t_address"}],"types":{"t_address":{"encoding":"inplace","label":"address","numberOfBytes":"20"},"t_uint256":{"encoding":"inplace","label":"uint256","numberOfBytes":"32"}}}"#
    );

    let empty = unit
        .find_contract_by_name("Empty")
        .next()
        .expect("contract Empty exists");
    assert_eq!(
        json(
            &empty
                .compute_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[],"types":null}"#
    );
    assert_eq!(
        json(
            &empty
                .compute_transient_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[],"types":null}"#
    );
}

define_fixture!(
    ReferenceTypes,
    file: "main.sol", r#"
pragma solidity *;
struct Node {
    uint128 value;
    address payable next;
    Node[] children;
    mapping(uint256 => Node) byId;
}

contract References {
    mapping(string => uint256) byName;
    mapping(bytes => mapping(address => Node)) nested;
    string name;
    bytes data;
    uint256[] dynamic;
    uint16[3] packed;
    Node[2][] grid;
}
"#,
);

// Mapping keys of reference types are memory pointers, everything else stored
// is in storage. A recursive struct lists its members once.
#[test]
fn reference_types_serialize() {
    let unit = ReferenceTypes::build_compilation_unit();
    let references = unit
        .find_contract_by_name("References")
        .next()
        .expect("contract References exists");
    assert_eq!(
        json(
            &references
                .compute_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[{"astId":N,"contract":"main.sol:References","label":"byName","offset":0,"slot":"0","type":"t_mapping(t_string_memory_ptr,t_uint256)"},{"astId":N,"contract":"main.sol:References","label":"nested","offset":0,"slot":"1","type":"t_mapping(t_bytes_memory_ptr,t_mapping(t_address,t_struct(Node)N_storage))"},{"astId":N,"contract":"main.sol:References","label":"name","offset":0,"slot":"2","type":"t_string_storage"},{"astId":N,"contract":"main.sol:References","label":"data","offset":0,"slot":"3","type":"t_bytes_storage"},{"astId":N,"contract":"main.sol:References","label":"dynamic","offset":0,"slot":"4","type":"t_array(t_uint256)dyn_storage"},{"astId":N,"contract":"main.sol:References","label":"packed","offset":0,"slot":"5","type":"t_array(t_uint16)3_storage"},{"astId":N,"contract":"main.sol:References","label":"grid","offset":0,"slot":"6","type":"t_array(t_array(t_struct(Node)N_storage)2_storage)dyn_storage"}],"types":{"t_address":{"encoding":"inplace","label":"address","numberOfBytes":"20"},"t_address_payable":{"encoding":"inplace","label":"address payable","numberOfBytes":"20"},"t_array(t_array(t_struct(Node)N_storage)2_storage)dyn_storage":{"base":"t_array(t_struct(Node)N_storage)2_storage","encoding":"dynamic_array","label":"struct Node[2][]","numberOfBytes":"32"},"t_array(t_struct(Node)N_storage)2_storage":{"base":"t_struct(Node)N_storage","encoding":"inplace","label":"struct Node[2]","numberOfBytes":"256"},"t_array(t_struct(Node)N_storage)dyn_storage":{"base":"t_struct(Node)N_storage","encoding":"dynamic_array","label":"struct Node[]","numberOfBytes":"32"},"t_array(t_uint16)3_storage":{"base":"t_uint16","encoding":"inplace","label":"uint16[3]","numberOfBytes":"32"},"t_array(t_uint256)dyn_storage":{"base":"t_uint256","encoding":"dynamic_array","label":"uint256[]","numberOfBytes":"32"},"t_bytes_memory_ptr":{"encoding":"bytes","label":"bytes","numberOfBytes":"32"},"t_bytes_storage":{"encoding":"bytes","label":"bytes","numberOfBytes":"32"},"t_mapping(t_address,t_struct(Node)N_storage)":{"encoding":"mapping","key":"t_address","label":"mapping(address => struct Node)","numberOfBytes":"32","value":"t_struct(Node)N_storage"},"t_mapping(t_bytes_memory_ptr,t_mapping(t_address,t_struct(Node)N_storage))":{"encoding":"mapping","key":"t_bytes_memory_ptr","label":"mapping(bytes => mapping(address => struct Node))","numberOfBytes":"32","value":"t_mapping(t_address,t_struct(Node)N_storage)"},"t_mapping(t_string_memory_ptr,t_uint256)":{"encoding":"mapping","key":"t_string_memory_ptr","label":"mapping(string => uint256)","numberOfBytes":"32","value":"t_uint256"},"t_mapping(t_uint256,t_struct(Node)N_storage)":{"encoding":"mapping","key":"t_uint256","label":"mapping(uint256 => struct Node)","numberOfBytes":"32","value":"t_struct(Node)N_storage"},"t_string_memory_ptr":{"encoding":"bytes","label":"string","numberOfBytes":"32"},"t_string_storage":{"encoding":"bytes","label":"string","numberOfBytes":"32"},"t_struct(Node)N_storage":{"encoding":"inplace","label":"struct Node","members":[{"astId":N,"contract":"main.sol:References","label":"value","offset":0,"slot":"0","type":"t_uint128"},{"astId":N,"contract":"main.sol:References","label":"next","offset":0,"slot":"1","type":"t_address_payable"},{"astId":N,"contract":"main.sol:References","label":"children","offset":0,"slot":"2","type":"t_array(t_struct(Node)N_storage)dyn_storage"},{"astId":N,"contract":"main.sol:References","label":"byId","offset":0,"slot":"3","type":"t_mapping(t_uint256,t_struct(Node)N_storage)"}],"numberOfBytes":"128"},"t_uint128":{"encoding":"inplace","label":"uint128","numberOfBytes":"16"},"t_uint16":{"encoding":"inplace","label":"uint16","numberOfBytes":"2"},"t_uint256":{"encoding":"inplace","label":"uint256","numberOfBytes":"32"}}}"#
    );
}

define_fixture!(
    FunctionTypes,
    file: "main.sol", r#"
pragma solidity *;
contract Functions {
    struct S { uint256 a; }
    function (uint256[][] memory, S storage, bytes calldata) internal returns (string memory, S[] memory) internalFn;
    function () external externalFn;
    function () internal pure returns (uint256) pureFn;
    function (uint256) external payable returns (uint256, bool) payableFn;
}
"#,
);

// The parameters and returns of a function type are pointers, wherever they
// point to.
#[test]
fn function_types_serialize() {
    let unit = FunctionTypes::build_compilation_unit();
    let functions = unit
        .find_contract_by_name("Functions")
        .next()
        .expect("contract Functions exists");
    assert_eq!(
        json(
            &functions
                .compute_storage_layout()
                .expect("the layout is computable")
        ),
        r#"{"storage":[{"astId":N,"contract":"main.sol:Functions","label":"internalFn","offset":0,"slot":"0","type":"t_function_internal_nonpayable(t_array(t_array(t_uint256)dyn_memory_ptr)dyn_memory_ptr,t_struct(S)N_storage_ptr,t_bytes_calldata_ptr)returns(t_string_memory_ptr,t_array(t_struct(S)N_memory_ptr)dyn_memory_ptr)"},{"astId":N,"contract":"main.sol:Functions","label":"externalFn","offset":8,"slot":"0","type":"t_function_external_nonpayable()returns()"},{"astId":N,"contract":"main.sol:Functions","label":"pureFn","offset":0,"slot":"1","type":"t_function_internal_pure()returns(t_uint256)"},{"astId":N,"contract":"main.sol:Functions","label":"payableFn","offset":8,"slot":"1","type":"t_function_external_payable(t_uint256)returns(t_uint256,t_bool)"}],"types":{"t_function_external_nonpayable()returns()":{"encoding":"inplace","label":"function () external","numberOfBytes":"24"},"t_function_external_payable(t_uint256)returns(t_uint256,t_bool)":{"encoding":"inplace","label":"function (uint256) payable external returns (uint256,bool)","numberOfBytes":"24"},"t_function_internal_nonpayable(t_array(t_array(t_uint256)dyn_memory_ptr)dyn_memory_ptr,t_struct(S)N_storage_ptr,t_bytes_calldata_ptr)returns(t_string_memory_ptr,t_array(t_struct(S)N_memory_ptr)dyn_memory_ptr)":{"encoding":"inplace","label":"function (uint256[][],struct Functions.S,bytes) returns (string,struct Functions.S[])","numberOfBytes":"8"},"t_function_internal_pure()returns(t_uint256)":{"encoding":"inplace","label":"function () pure returns (uint256)","numberOfBytes":"8"}}}"#
    );
}
