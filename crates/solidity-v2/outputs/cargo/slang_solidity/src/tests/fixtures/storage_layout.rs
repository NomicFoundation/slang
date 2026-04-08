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
"#,
);
