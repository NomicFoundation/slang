use ruint::uint;

use crate::abi::{self, StorageKind, StorageSize, StorageType, StorageTypeKind};
use crate::ast::TypeId;
use crate::define_fixture;

// Sample adapted from: https://docs.soliditylang.org/en/v0.8.33/internals/layout_in_storage.html#layout-of-state-variables-in-storage-and-transient-storage
define_fixture!(
    StorageLayout,
    file: "main.sol", r#"
pragma solidity *;
struct S {
    int32 x;
    bool y;
}
struct T {
    uint256 z;
    uint32 w;
}

contract A {
    uint a;
    uint constant c = 10;
    uint immutable d = 12;
}

contract B {
    uint8[] e;
    mapping(uint => S) f;
    uint16 g;
    uint16 h;
    S s;
    int8 k;
}

contract C is A, B {
    bytes21 l;
    uint8[10] m;
    bytes5[8] n;
    T[2] t;
    bytes5 o;
}

contract D is A layout at 42 {
    uint p;
}

uint constant BASE = 5;

contract E layout at BASE * 2 + 10 {
    int8 q;
    int8 transient qt;
    bytes5 r;
    bytes5 transient rt;
}

contract F layout at erc7201("example.main") {
    uint256 u;
    uint256 v;
}
"#,
);

macro_rules! assert_layout_item_eq {
    ($layout:ident[$index:expr], $name:expr, $slot:expr, $offset:expr, $type:expr) => {
        let item = &$layout.items()[$index];
        let item_type = $layout
            .storage_type(item.type_id())
            .expect("the item's type is in the table");
        assert_eq!(item.name(), $name);
        assert_eq!(item_type.label(), $type);
        assert_eq!(item.slot(), $slot);
        assert_eq!(item.offset(), $offset);
    };
}

#[test]
fn test_storage_layout() {
    let unit = StorageLayout::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let counter_abi = counter.compute_abi().expect("can compute ABI");
    let layout = counter_abi.storage_layout();

    assert_eq!(layout.items().len(), 12);

    assert_layout_item_eq!(layout[0], "a", uint!(0_U256), 0, "uint256");
    assert_layout_item_eq!(layout[1], "e", uint!(1_U256), 0, "uint8[]");
    assert_layout_item_eq!(
        layout[2],
        "f",
        uint!(2_U256),
        0,
        "mapping(uint256 => struct S)"
    );
    assert_layout_item_eq!(layout[3], "g", uint!(3_U256), 0, "uint16");
    assert_layout_item_eq!(layout[4], "h", uint!(3_U256), 2, "uint16");
    assert_layout_item_eq!(layout[5], "s", uint!(4_U256), 0, "struct S");
    assert_layout_item_eq!(layout[6], "k", uint!(5_U256), 0, "int8");
    assert_layout_item_eq!(layout[7], "l", uint!(5_U256), 1, "bytes21");
    assert_layout_item_eq!(layout[8], "m", uint!(6_U256), 0, "uint8[10]");
    assert_layout_item_eq!(layout[9], "n", uint!(7_U256), 0, "bytes5[8]");
    assert_layout_item_eq!(layout[10], "t", uint!(9_U256), 0, "struct T[2]");
    assert_layout_item_eq!(layout[11], "o", uint!(13_U256), 0, "bytes5");

    let transient_layout = counter_abi.transient_storage_layout();
    assert!(transient_layout.items().is_empty());
}

#[test]
fn test_transient_and_custom_storage_layout() {
    let unit = StorageLayout::build_compilation_unit();

    let d_contract = unit
        .find_contract_by_name("D")
        .next()
        .expect("contract can be found");
    let d_abi = d_contract.compute_abi().expect("can compute ABI");
    let d_layout = d_abi.storage_layout();

    assert_eq!(d_layout.items().len(), 2);
    assert_layout_item_eq!(d_layout[0], "a", 42, 0, "uint256");
    assert_layout_item_eq!(d_layout[1], "p", 43, 0, "uint256");

    let e_contract = unit
        .find_contract_by_name("E")
        .next()
        .expect("contract can be found");
    let e_abi = e_contract.compute_abi().expect("can compute ABI");
    let e_layout = e_abi.storage_layout();
    let e_transient_layout = e_abi.transient_storage_layout();

    assert_eq!(e_layout.items().len(), 2);
    assert_layout_item_eq!(e_layout[0], "q", 20, 0, "int8");
    assert_layout_item_eq!(e_layout[1], "r", 20, 1, "bytes5");

    assert_eq!(e_transient_layout.items().len(), 2);
    assert_layout_item_eq!(e_transient_layout[0], "qt", 0, 0, "int8");
    assert_layout_item_eq!(e_transient_layout[1], "rt", 0, 1, "bytes5");
}

// A struct whose members fill a slot exactly must occupy a single slot. The
// two `uint128` members (16 bytes each) pack into one 32-byte slot, so the
// following `tail` variable lands at slot 1.
//
// Regression: a member that fits perfectly in the remaining bytes used to be
// pushed to the next slot, inflating `PerfectFit` to two slots and shifting
// `tail` to slot 2.
define_fixture!(
    PerfectlyPackedStruct,
    file: "main.sol", r#"
pragma solidity *;
struct PerfectFit {
    uint128 a;
    uint128 b;
}

contract G {
    PerfectFit s;
    uint256 tail;
}
"#,
);

#[test]
fn test_struct_members_packing_into_a_full_slot_occupy_one_slot() {
    let unit = PerfectlyPackedStruct::build_compilation_unit();

    let g_contract = unit
        .find_contract_by_name("G")
        .next()
        .expect("contract can be found");
    let g_abi = g_contract.compute_abi().expect("can compute ABI");
    let layout = g_abi.storage_layout();

    assert_eq!(layout.items().len(), 2);
    assert_layout_item_eq!(layout[0], "s", uint!(0_U256), 0, "struct PerfectFit");
    assert_layout_item_eq!(layout[1], "tail", uint!(1_U256), 0, "uint256");
}

#[test]
fn test_erc7201_storage_layout() {
    let unit = StorageLayout::build_compilation_unit();

    let f_contract = unit
        .find_contract_by_name("F")
        .next()
        .expect("contract can be found");
    let f_abi = f_contract.compute_abi().expect("can compute ABI");
    let f_layout = f_abi.storage_layout();

    // EIP-7201 test vector: `erc7201("example.main")` →
    // 0x183a6125c38840424c4a85fa12bab2ab606c4b6d0e7cc73c0c06ba5300eab500.
    let base_slot = uint!(0x183a6125c38840424c4a85fa12bab2ab606c4b6d0e7cc73c0c06ba5300eab500_U256);

    assert_eq!(f_layout.items().len(), 2);
    assert_layout_item_eq!(f_layout[0], "u", base_slot, 0, "uint256");
    assert_layout_item_eq!(f_layout[1], "v", base_slot + uint!(1_U256), 0, "uint256");
}

// A fixed-size array can occupy far more than a machine word of slots. `b`
// takes `2**64` slots, so `c` lands at slot `2**64 + 1`.
define_fixture!(
    HugeArray,
    file: "main.sol", r#"
pragma solidity *;
contract C {
    uint256 a;
    uint256[2 ** 64] b;
    uint8 c;
}
"#,
);

#[test]
fn test_huge_array_shifts_following_slot() {
    let unit = HugeArray::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");
    let layout = abi.storage_layout();

    assert_eq!(layout.items().len(), 3);
    assert_layout_item_eq!(layout[0], "a", uint!(0_U256), 0, "uint256");
    assert_layout_item_eq!(
        layout[1],
        "b",
        uint!(1_U256),
        0,
        "uint256[18446744073709551616]"
    );
    assert_layout_item_eq!(layout[2], "c", uint!(18446744073709551617_U256), 0, "uint8");
}

// The maximum legal length is `2**256 - 1` slots, which lays out at slot 0.
define_fixture!(
    MaxLegalArray,
    file: "main.sol", r#"
pragma solidity *;
contract C {
    uint256[2 ** 256 - 1] x;
}
"#,
);

#[test]
fn test_max_length_array_lays_out() {
    let unit = MaxLegalArray::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");
    let layout = abi.storage_layout();

    assert_eq!(layout.items().len(), 1);
    assert_layout_item_eq!(
        layout[0],
        "x",
        uint!(0_U256),
        0,
        "uint256[115792089237316195423570985008687907853269984665640564039457584007913129639935]"
    );
}

// A struct whose members overflow storage has no computable size. `a` spans
// `2**256 - 1` slots, so rounding up past `b` reaches `2**256`.
define_fixture!(
    OversizedStruct,
    file: "main.sol", r#"
pragma solidity *;
struct Oversized {
    uint256[2 ** 256 - 1] a;
    uint256 b;
}

contract C {
    Oversized s;
}
"#,
);

#[test]
fn test_oversized_struct_has_no_layout() {
    let unit = OversizedStruct::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    assert!(contract.compute_abi().is_none());
}

// `Big` occupies `2**255` slots, so `Big[2]` would need `2**256`. That
// overflows storage, so no layout can be computed.
define_fixture!(
    OversizedArrayOfStruct,
    file: "main.sol", r#"
pragma solidity *;
struct Big {
    uint256[2 ** 255] x;
}

contract C {
    Big[2] arr;
}
"#,
);

#[test]
fn test_oversized_array_of_struct_has_no_layout() {
    let unit = OversizedArrayOfStruct::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    assert!(contract.compute_abi().is_none());
}

// `Big` overflows storage, but `Big[]` itself occupies one slot. The items can
// still be laid out; the types table cannot describe `Big`, so there is no
// layout nor ABI.
define_fixture!(
    OversizedStructBehindDynamicArray,
    file: "main.sol", r#"
pragma solidity *;
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

contract C {
    Big[] xs;
}
"#,
);

#[test]
fn test_oversized_struct_behind_dynamic_array_has_items_but_no_layout() {
    let unit = OversizedStructBehindDynamicArray::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");

    let items = contract
        .compute_storage_items(StorageKind::Persistent)
        .expect("can compute the storage items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name(), "xs");
    assert_eq!(items[0].slot(), uint!(0_U256));
    assert_eq!(items[0].offset(), 0);

    assert!(
        contract
            .compute_storage_layout(StorageKind::Persistent)
            .is_none()
    );
    assert!(contract.compute_abi().is_none());
}

// A custom base slot pushes the layout past the end of storage. `x` spans
// `2**256 - 1` slots from base slot 1, reaching `2**256`, so no layout exists.
define_fixture!(
    OversizedBaseSlot,
    file: "main.sol", r#"
pragma solidity *;
contract C layout at 1 {
    uint256[2 ** 256 - 1] x;
}
"#,
);

#[test]
fn test_oversized_base_slot_has_no_layout() {
    let unit = OversizedBaseSlot::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    assert!(contract.compute_abi().is_none());
}

// A struct declared inside a contract is laid out under its scope-qualified
// name.
define_fixture!(
    NestedStructLayout,
    file: "main.sol", r#"
pragma solidity *;
contract C {
    struct Inner {
        uint256 a;
    }

    Inner nested;
}
"#,
);

#[test]
fn test_nested_struct_lays_out_under_its_qualified_name() {
    let unit = NestedStructLayout::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let contract_abi = contract.compute_abi().expect("can compute ABI");
    let layout = contract_abi.storage_layout();

    assert_eq!(layout.items().len(), 1);
    assert_layout_item_eq!(layout[0], "nested", uint!(0_U256), 0, "struct C.Inner");
}

// Expected type names are solc 0.8.35's `storageLayout` labels for this source.
define_fixture!(
    TypeLabels,
    file: "main.sol", r#"
pragma solidity *;
interface I {}
struct S { uint256 a; }
type U is uint64;

contract Store {
    struct TokenData { uint256 id; }
    enum E { A }

    function (uint256, TokenData memory) view returns (string memory) internal renderer;
    mapping(bytes4 => function (bytes memory)) internal handlers;
    function (uint256) external returns (bool) ext;
    function () pure internal noargs;
    function (address payable) payable external pay;
    mapping(uint256 => S) structs;
    S[] structArray;
    TokenData[2] fixedStructs;
    E e;
    mapping(E => I[]) enumKeys;
    I i;
    address payable owner;
    address plain;
    U u;
    mapping(address => mapping(uint256 => S)) nested;
}
"#,
);

#[test]
fn test_type_names_match_solc_labels() {
    let unit = TypeLabels::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("Store")
        .next()
        .expect("contract can be found");
    let contract_abi = contract.compute_abi().expect("can compute ABI");
    let layout = contract_abi.storage_layout();
    let type_names: Vec<_> = layout
        .items()
        .iter()
        .map(|item| {
            let item_type = layout
                .storage_type(item.type_id())
                .expect("the item's type is in the table");
            (item.name(), item_type.label())
        })
        .collect();

    assert_eq!(
        type_names,
        [
            (
                "renderer",
                "function (uint256,struct Store.TokenData) view returns (string)"
            ),
            ("handlers", "mapping(bytes4 => function (bytes))"),
            ("ext", "function (uint256) external returns (bool)"),
            ("noargs", "function () pure"),
            ("pay", "function (address payable) payable external"),
            ("structs", "mapping(uint256 => struct S)"),
            ("structArray", "struct S[]"),
            ("fixedStructs", "struct Store.TokenData[2]"),
            ("e", "enum Store.E"),
            ("enumKeys", "mapping(enum Store.E => contract I[])"),
            ("i", "contract I"),
            ("owner", "address payable"),
            ("plain", "address"),
            ("u", "U"),
            ("nested", "mapping(address => mapping(uint256 => struct S))"),
        ]
    );
}

/// Renders a layout's types table as `label: size, kind` lines, naming the
/// types each entry refers to by their labels, sorted so the expectations do
/// not depend on the order types are registered in.
fn describe_storage_types(layout: &abi::StorageLayout) -> Vec<String> {
    let label_of = |type_id: TypeId| {
        layout
            .storage_type(type_id)
            .expect("a referenced type is in the table")
            .label()
            .to_owned()
    };
    let describe = |storage_type: &StorageType| {
        let size = match storage_type.size() {
            StorageSize::Bytes(bytes) => format!("{bytes} bytes"),
            StorageSize::Slots(slots) => format!("{slots} slots"),
        };
        let kind = match storage_type.kind() {
            StorageTypeKind::Value => "value".to_owned(),
            StorageTypeKind::Bytes => "bytes".to_owned(),
            StorageTypeKind::DynamicArray { element } => {
                format!("dynamic array of {}", label_of(*element))
            }
            StorageTypeKind::FixedSizeArray { element } => {
                format!("fixed-size array of {}", label_of(*element))
            }
            StorageTypeKind::Mapping { key, value } => {
                format!("mapping from {} to {}", label_of(*key), label_of(*value))
            }
            StorageTypeKind::Struct { members } => {
                let members: Vec<_> = members
                    .iter()
                    .map(|member| {
                        format!(
                            "{} at {}+{}: {}",
                            member.name,
                            member.position.slot,
                            member.position.offset,
                            label_of(member.type_id)
                        )
                    })
                    .collect();
                format!("struct {{ {} }}", members.join(", "))
            }
        };
        format!("{}: {size}, {kind}", storage_type.label())
    };
    let mut lines: Vec<_> = layout.types().map(describe).collect();
    lines.sort();
    lines
}

// Expected entries are solc 0.8.37's `types` for each layout of this source.
// solc also keeps a mapping's `string` and `bytes` keys apart from the storage
// ones, so each spelling appears twice.
define_fixture!(
    StorageTypes,
    file: "main.sol", r#"
pragma solidity *;
type Price is uint64;
interface IOracle {}

contract Types {
    struct Node {
        string name;
        bytes data;
        Node[] children;
        mapping(string => Node) byName;
        function () external callback;
        uint8 tag;
    }
    enum Kind { A, B }

    Node root;
    mapping(bytes => uint8)[2] flags;
    uint8[3][] grid;
    Node[] nodes;
    Kind kind;
    Price price;
    IOracle oracle;
    Types self;
    function (uint256) internal view returns (bool) check;
    bool paused;
    mapping(address => mapping(Kind => IOracle[])) registry;
    uint8 transient lock;
    uint64 transient counter;
}
"#,
);

#[test]
fn test_storage_types_table() {
    let unit = StorageTypes::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("Types")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");

    assert_eq!(
        describe_storage_types(abi.storage_layout()),
        [
            "Price: 8 bytes, value",
            "address: 20 bytes, value",
            "bool: 1 bytes, value",
            "bytes: 1 slots, bytes",
            "bytes: 1 slots, bytes",
            "contract IOracle: 20 bytes, value",
            "contract IOracle[]: 1 slots, dynamic array of contract IOracle",
            "contract Types: 20 bytes, value",
            "enum Types.Kind: 1 bytes, value",
            "function () external: 24 bytes, value",
            "function (uint256) view returns (bool): 8 bytes, value",
            "mapping(address => mapping(enum Types.Kind => contract IOracle[])): 1 slots, \
             mapping from address to mapping(enum Types.Kind => contract IOracle[])",
            "mapping(bytes => uint8): 1 slots, mapping from bytes to uint8",
            "mapping(bytes => uint8)[2]: 2 slots, fixed-size array of mapping(bytes => uint8)",
            "mapping(enum Types.Kind => contract IOracle[]): 1 slots, \
             mapping from enum Types.Kind to contract IOracle[]",
            "mapping(string => struct Types.Node): 1 slots, mapping from string to struct Types.Node",
            "string: 1 slots, bytes",
            "string: 1 slots, bytes",
            "struct Types.Node: 5 slots, struct { name at 0+0: string, data at 1+0: bytes, \
             children at 2+0: struct Types.Node[], byName at 3+0: mapping(string => struct Types.Node), \
             callback at 4+0: function () external, tag at 4+24: uint8 }",
            "struct Types.Node[]: 1 slots, dynamic array of struct Types.Node",
            "uint8: 1 bytes, value",
            "uint8[3]: 1 slots, fixed-size array of uint8",
            "uint8[3][]: 1 slots, dynamic array of uint8[3]",
        ]
    );
    // The value types share slots 9 and 10, as solc packs them.
    let layout = abi.storage_layout();
    assert_layout_item_eq!(layout[4], "kind", uint!(9_U256), 0, "enum Types.Kind");
    assert_layout_item_eq!(layout[5], "price", uint!(9_U256), 1, "Price");
    assert_layout_item_eq!(layout[6], "oracle", uint!(9_U256), 9, "contract IOracle");
    assert_layout_item_eq!(layout[7], "self", uint!(10_U256), 0, "contract Types");
    assert_layout_item_eq!(
        layout[8],
        "check",
        uint!(10_U256),
        20,
        "function (uint256) view returns (bool)"
    );
    assert_layout_item_eq!(layout[9], "paused", uint!(10_U256), 28, "bool");
    assert_layout_item_eq!(
        layout[10],
        "registry",
        uint!(11_U256),
        0,
        "mapping(address => mapping(enum Types.Kind => contract IOracle[]))"
    );

    let type_ids: Vec<_> = abi
        .storage_layout()
        .types()
        .map(StorageType::type_id)
        .collect();
    assert!(type_ids.is_sorted(), "the table is ordered by type id");
    // `uint8` is in both tables: each layout describes its own types.
    assert_eq!(
        describe_storage_types(abi.transient_storage_layout()),
        ["uint64: 8 bytes, value", "uint8: 1 bytes, value"]
    );
}

// A struct member and a state variable of the same type name one entry.
#[test]
fn test_storage_types_share_member_and_variable_types() {
    let unit = StorageTypes::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("Types")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");
    let layout = abi.storage_layout();
    let nodes = &layout.items()[3];
    assert_eq!(nodes.name(), "nodes");

    let root_type = layout
        .storage_type(layout.items()[0].type_id())
        .expect("the variable's type is in the table");
    let StorageTypeKind::Struct { members } = root_type.kind() else {
        panic!("`root` is a struct");
    };
    let children = &members[2];
    assert_eq!(children.name, "children");
    assert_eq!(children.type_id, nodes.type_id());
}

// The size is exact, even where a byte count would not fit a `uint256`.
#[test]
fn test_storage_types_keep_the_exact_size() {
    let unit = MaxLegalArray::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");
    let layout = abi.storage_layout();
    let array_type = layout
        .storage_type(layout.items()[0].type_id())
        .expect("the variable's type is in the table");

    assert_eq!(
        array_type.size(),
        StorageSize::Slots(uint!(2_U256).pow(uint!(256_U256)) - uint!(1_U256))
    );
}

// Constants and immutables occupy no storage, so their types are left out, and
// a layout without variables has no types.
#[test]
fn test_storage_types_leave_out_constants_and_immutables() {
    let unit = StorageLayout::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("A")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");

    assert_eq!(
        describe_storage_types(abi.storage_layout()),
        ["uint256: 32 bytes, value"]
    );
    assert_eq!(abi.transient_storage_layout().types().len(), 0);
}

// The layouts and items computed on their own match the ones in the ABI,
// custom base slot included.
#[test]
fn test_storage_layouts_without_the_abi() {
    let unit = StorageLayout::build_compilation_unit();
    for name in ["C", "D", "E"] {
        let contract = unit
            .find_contract_by_name(name)
            .next()
            .expect("contract can be found");
        let abi = contract.compute_abi().expect("can compute ABI");
        let storage_layout = contract
            .compute_storage_layout(StorageKind::Persistent)
            .expect("can compute the storage layout");
        let transient_storage_layout = contract
            .compute_storage_layout(StorageKind::Transient)
            .expect("can compute the transient storage layout");
        assert_eq!(&storage_layout, abi.storage_layout());
        assert_eq!(&transient_storage_layout, abi.transient_storage_layout());

        let storage_items = contract
            .compute_storage_items(StorageKind::Persistent)
            .expect("can compute the storage items");
        let transient_storage_items = contract
            .compute_storage_items(StorageKind::Transient)
            .expect("can compute the transient storage items");
        assert_eq!(storage_items, abi.storage_layout().items());
        assert_eq!(
            transient_storage_items,
            abi.transient_storage_layout().items()
        );
    }
}

// Structs declared in another file and reached only through a base contract's
// variable, a nested member, a fixed-size array and a mapping value still name
// one entry per type.
define_fixture!(
    ImportedStorageTypes,
    file: "types.sol", r#"
pragma solidity *;
struct Inner {
    string name;
    uint8[] tags;
}
struct Outer {
    Inner[2] pair;
    mapping(uint256 => Inner) byId;
}
"#,
    file: "main.sol", r#"
pragma solidity *;
import {Inner, Outer} from "types.sol";

contract Base {
    Outer internal outer;
}

contract Derived is Base {
    Inner[] extra;
}
"#,
);

#[test]
fn test_storage_types_reached_through_other_files_share_entries() {
    let unit = ImportedStorageTypes::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("Derived")
        .next()
        .expect("contract can be found");
    let layout = contract
        .compute_storage_layout(StorageKind::Persistent)
        .expect("can compute the storage layout");
    let table = describe_storage_types(&layout);

    let mut labels: Vec<_> = layout.types().map(abi::StorageType::label).collect();
    labels.sort_unstable();
    let entries = labels.len();
    labels.dedup();
    assert_eq!(labels.len(), entries, "one entry per type: {table:#?}");
}
