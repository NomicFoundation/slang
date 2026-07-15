// SPDX-License-Identifier: MIT
pragma solidity *;

// A dynamic array on the other edge of the cycle also breaks it.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B x;
    }
    struct B {
        A[] x;
    }
}
