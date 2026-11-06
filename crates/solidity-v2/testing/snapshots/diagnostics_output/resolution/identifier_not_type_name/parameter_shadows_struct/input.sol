// SPDX-License-Identifier: MIT
pragma solidity *;

// A function's parameters are visible to the types of its other parameters, so
// `S` finds the parameter.
struct S {
    uint256 a;
}

function f(uint256 S, S memory s) pure {}
