// SPDX-License-Identifier: MIT
pragma solidity *;

// A member of the struct's own type makes the struct infinitely sized, so the
// definition is rejected as recursive.

contract Test {
    struct MyStruct {
        address addr;
        MyStruct x;
    }
}
