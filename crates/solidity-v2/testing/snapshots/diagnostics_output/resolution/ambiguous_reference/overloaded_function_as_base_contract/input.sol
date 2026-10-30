// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure {}

function f(uint256) pure {}

// An overload set names no contract to inherit from.
contract C is f {}
