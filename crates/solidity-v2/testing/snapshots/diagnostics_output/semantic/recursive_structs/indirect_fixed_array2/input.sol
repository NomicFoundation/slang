// SPDX-License-Identifier: MIT
pragma solidity *;

// The fixed-size array on the other edge of the cycle is still recursive.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B x;
    }
    struct B {
        A[1] x;
    }
}
