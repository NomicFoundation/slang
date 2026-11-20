// SPDX-License-Identifier: MIT
pragma solidity *;

// The getter returns the mapping value, a struct with every member omitted.
contract C {
    struct S {
        mapping(uint256 => uint256) m;
    }

    mapping(uint256 => S) public values;
}
