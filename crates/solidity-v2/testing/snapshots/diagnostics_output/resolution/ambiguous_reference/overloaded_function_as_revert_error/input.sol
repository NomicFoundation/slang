// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure {}

function f(uint256) pure {}

contract C {
    // Errors cannot be overloaded, so the arguments do not narrow this down.
    function g() public pure {
        revert f();
    }
}
