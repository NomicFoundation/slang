// SPDX-License-Identifier: MIT
pragma solidity *;

// A multi-dimensional fixed-size array keeps the struct finite only if every
// dimension is fixed, so this cycle is still recursive.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B[1][1] x;
    }
    struct B {
        A x;
    }
}
