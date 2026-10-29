// SPDX-License-Identifier: MIT
pragma solidity *;

// `S2` wraps `S` and still packs into a single slot, so the variables fit in
// `2**256 - 3` slots.
struct S {
    uint8 a;
    uint8 b;
}

struct S2 {
    S s;
}

contract C {
    uint256[2 ** 256 - 4] x;
    S2 s;
}
