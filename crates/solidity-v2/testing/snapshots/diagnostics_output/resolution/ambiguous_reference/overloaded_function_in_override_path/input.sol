// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure {}

function f(uint256) pure {}

contract C {
    // An overload set names no base contract to override from.
    function g() public override(f) {}
}
