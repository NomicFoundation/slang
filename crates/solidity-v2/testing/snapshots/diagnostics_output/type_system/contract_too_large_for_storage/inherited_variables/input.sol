// SPDX-License-Identifier: MIT
pragma solidity *;

// `B` fits, but `C` lays out `B`'s array before its own, and `D`
// inherits both.
contract B {
    uint256[2 ** 255] a;
}

contract C is B {
    uint256[2 ** 255] b;
}

contract D is C {}
