// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public view {
        // A contract instance cannot be indexed.
        this[0];
    }
}
