// SPDX-License-Identifier: MIT
pragma solidity *;

// `a` fills every slot but the last one, which `b` only partially uses.
// Rounding the struct up to a whole slot then overflows storage.
struct S {
    uint256[2 ** 256 - 1] a;
    uint8 b;
}

contract C {
    S s;
}
