// SPDX-License-Identifier: MIT
pragma solidity *;

// Holding two members of the same non-recursive struct is not a cycle and is
// accepted.

contract Test {
    struct S1 {
        uint a;
    }
    struct S2 {
        S1 x;
        S1 y;
    }
}
