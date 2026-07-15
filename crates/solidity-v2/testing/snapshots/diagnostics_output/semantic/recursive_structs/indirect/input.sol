// SPDX-License-Identifier: MIT
pragma solidity *;

// Two structs that contain each other by value form a cycle, so the definition
// is rejected as recursive.

contract Test {
    struct A {
        address addr;
        uint256 count;
        B x;
    }
    struct B {
        A x;
    }
}
