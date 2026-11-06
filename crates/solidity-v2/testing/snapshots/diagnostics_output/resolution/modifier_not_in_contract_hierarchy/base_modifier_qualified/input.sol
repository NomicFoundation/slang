// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier of a base, named through the base, is in the hierarchy.
contract A {
    modifier m() {
        _;
    }
}

contract B is A {
    function f() public A.m {}
}
