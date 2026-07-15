// SPDX-License-Identifier: MIT
pragma solidity *;

// Fixed-size arrays of a non-recursive struct are not a cycle and are accepted.

contract Test {
    struct S1 {
        uint a;
    }
    struct S2 {
        S1[1] x;
        S1[1] y;
    }
}
