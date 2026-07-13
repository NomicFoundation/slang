// SPDX-License-Identifier: MIT
pragma solidity *;

// A dynamic array stores its elements out of line, so a struct holding a
// dynamic array of its own type is finite and accepted.

contract Test {
    struct MyStruct {
        address addr;
        MyStruct[] x;
    }
}
