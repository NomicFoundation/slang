use ruint::uint;

use crate::define_fixture;

// Sample adapted from: https://docs.soliditylang.org/en/v0.8.33/internals/layout_in_storage.html#layout-of-state-variables-in-storage-and-transient-storage
define_fixture!(
    StorageLayout,
    file: "main.sol", r#"
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

define_fixture!(
    ExactFillStorage,
    file: "main.sol", r#"
struct ExactFill {
    int96 a;
    address b;
}

contract Test {
    ExactFill packed;
    uint256 next;
}
"#,
);

macro_rules! assert_layout_item_eq {
    ($item:expr, $name:expr, $slot:expr, $offset:expr, $type:expr) => {
        assert_eq!($item.label(), $name);
        assert_eq!($item.type_name(), $type);
        assert_eq!($item.slot(), $slot);
        assert_eq!($item.offset(), $offset);
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

    assert_eq!(layout.len(), 12);

    assert_layout_item_eq!(layout[0], "a", uint!(0_U256), 0, "uint256");
    assert_layout_item_eq!(layout[1], "e", uint!(1_U256), 0, "uint8[]");
    assert_layout_item_eq!(layout[2], "f", uint!(2_U256), 0, "mapping(uint256 => S)");
    assert_layout_item_eq!(layout[3], "g", uint!(3_U256), 0, "uint16");
    assert_layout_item_eq!(layout[4], "h", uint!(3_U256), 2, "uint16");
    assert_layout_item_eq!(layout[5], "s", uint!(4_U256), 0, "S");
    assert_layout_item_eq!(layout[6], "k", uint!(5_U256), 0, "int8");
    assert_layout_item_eq!(layout[7], "l", uint!(5_U256), 1, "bytes21");
    assert_layout_item_eq!(layout[8], "m", uint!(6_U256), 0, "uint8[10]");
    assert_layout_item_eq!(layout[9], "n", uint!(7_U256), 0, "bytes5[8]");
    assert_layout_item_eq!(layout[10], "t", uint!(9_U256), 0, "T[2]");
    assert_layout_item_eq!(layout[11], "o", uint!(13_U256), 0, "bytes5");

    let transient_layout = counter_abi.transient_storage_layout();
    assert!(transient_layout.is_empty());
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

    assert_eq!(d_layout.len(), 2);
    assert_layout_item_eq!(d_layout[0], "a", 42, 0, "uint256");
    assert_layout_item_eq!(d_layout[1], "p", 43, 0, "uint256");

    let e_contract = unit
        .find_contract_by_name("E")
        .next()
        .expect("contract can be found");
    let e_abi = e_contract.compute_abi().expect("can compute ABI");
    let e_layout = e_abi.storage_layout();
    let e_transient_layout = e_abi.transient_storage_layout();

    assert_eq!(e_layout.len(), 2);
    assert_layout_item_eq!(e_layout[0], "q", 20, 0, "int8");
    assert_layout_item_eq!(e_layout[1], "r", 20, 1, "bytes5");

    assert_eq!(e_transient_layout.len(), 2);
    assert_layout_item_eq!(e_transient_layout[0], "qt", 0, 0, "int8");
    assert_layout_item_eq!(e_transient_layout[1], "rt", 0, 1, "bytes5");
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

    assert_eq!(f_layout.len(), 2);
    assert_layout_item_eq!(f_layout[0], "u", base_slot, 0, "uint256");
    assert_layout_item_eq!(f_layout[1], "v", base_slot + uint!(1_U256), 0, "uint256");
}

#[test]
fn test_struct_member_exactly_fills_slot() {
    // int96 (12) + address (20) = 32 bytes fills one slot; `next` follows at slot 1.
    let unit = ExactFillStorage::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("Test")
        .next()
        .expect("contract can be found");
    let abi = contract.compute_abi().expect("can compute ABI");
    let layout = abi.storage_layout();

    assert_eq!(layout.len(), 2);
    assert_layout_item_eq!(layout[0], "packed", uint!(0_U256), 0, "ExactFill");
    assert_layout_item_eq!(layout[1], "next", uint!(1_U256), 0, "uint256");
}
