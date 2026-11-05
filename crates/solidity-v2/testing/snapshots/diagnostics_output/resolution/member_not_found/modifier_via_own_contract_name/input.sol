// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m() {
        _;
    }

    function f() public {
        // A modifier is not reachable through the contract name in an
        // expression, not even from within its own contract.
        C.m;
    }
}
