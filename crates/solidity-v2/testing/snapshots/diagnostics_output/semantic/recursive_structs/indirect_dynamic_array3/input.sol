// SPDX-License-Identifier: MIT
pragma solidity *;

// Dynamic arrays on both edges of the cycle break it.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B[] x;
    }
    struct B {
        A[] x;
    }
}
