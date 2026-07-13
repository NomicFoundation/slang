// SPDX-License-Identifier: MIT
pragma solidity *;

// Fixed-size arrays on both edges of the cycle are still recursive.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B[1] x;
    }
    struct B {
        A[1] x;
    }
}
