// SPDX-License-Identifier: MIT
pragma solidity *;

// A mapping stores its values out of line, so a struct with a mapping to its
// own type is finite and accepted.

contract Test {
    struct MyStruct {
        address addr;
        uint256 count;
        mapping(uint => MyStruct) x;
    }
}
