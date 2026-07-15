// SPDX-License-Identifier: MIT
pragma solidity *;

// A fixed-size array of the struct's own type is still infinitely sized, so the
// definition is rejected as recursive.

contract Test {
    struct MyStruct {
        address addr;
        MyStruct[1] x;
    }
}
