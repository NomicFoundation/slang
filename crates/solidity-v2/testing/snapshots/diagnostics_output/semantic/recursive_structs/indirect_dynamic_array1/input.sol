// SPDX-License-Identifier: MIT
pragma solidity *;

// A dynamic array on one edge of the cycle breaks it, so both structs are
// accepted.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B[] x;
    }
    struct B {
        A x;
    }
}
