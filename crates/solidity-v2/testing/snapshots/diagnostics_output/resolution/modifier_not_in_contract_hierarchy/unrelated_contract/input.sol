// SPDX-License-Identifier: MIT
pragma solidity *;

// A function invokes the modifier of a contract it does not derive from.
contract C {
    modifier m() {
        _;
    }
}

contract D {
    function f() public C.m {}
}
