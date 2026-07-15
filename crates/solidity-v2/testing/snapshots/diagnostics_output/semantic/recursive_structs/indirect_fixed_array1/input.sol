// SPDX-License-Identifier: MIT
pragma solidity *;

// A fixed-size array on one edge of a two-struct cycle is still recursive.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B[1] x;
    }
    struct B {
        A x;
    }
}
