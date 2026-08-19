// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external {}

    function f(uint256) external {}

    function g() external {}
}

contract B {
    function ambiguous() external {
        // Reached through the contract's type name, and never called.
        A.f;
    }

    function unambiguous() external {
        A.g;
    }
}
