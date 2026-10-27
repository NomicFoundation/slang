// SPDX-License-Identifier: MIT
pragma solidity *;

// `S` packs into a single slot, so the variables fit in `2**256 - 2` slots.
struct S {
    uint8 a;
    uint8 b;
}

contract C {
    uint256[2 ** 256 - 3] x;
    S s;
}
