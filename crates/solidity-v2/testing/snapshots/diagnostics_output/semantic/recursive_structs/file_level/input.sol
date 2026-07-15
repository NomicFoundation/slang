// SPDX-License-Identifier: MIT
pragma solidity *;

// File-level structs are checked for recursion the same way as ones nested in a
// contract.

struct S1 {
    S2 x;
}
struct S2 {
    S1 y;
}

contract C {}
