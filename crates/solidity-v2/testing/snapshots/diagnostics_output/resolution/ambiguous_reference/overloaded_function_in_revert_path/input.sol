// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure {}

function f(uint256) pure {}

contract C {
    // An overload set has no members, and is reported rather than `E`.
    function g() public pure {
        revert f.E();
    }
}
