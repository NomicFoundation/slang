// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m() {
        _;
    }
}

contract B is A {
    function f() public {
        // A modifier is not reachable through the contract name in an
        // expression.
        A.m;
    }
}
