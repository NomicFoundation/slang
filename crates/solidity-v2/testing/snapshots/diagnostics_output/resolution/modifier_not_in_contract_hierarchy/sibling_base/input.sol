// SPDX-License-Identifier: MIT
pragma solidity *;

// `C` and `D` share a base and a descendant, but `C` is still not a base of
// `D`, so `D` cannot invoke its modifier.
contract A {}

contract C is A {
    modifier m() {
        _;
    }
}

contract D is A {
    function f() public C.m {}
}

contract T is D, C {}
