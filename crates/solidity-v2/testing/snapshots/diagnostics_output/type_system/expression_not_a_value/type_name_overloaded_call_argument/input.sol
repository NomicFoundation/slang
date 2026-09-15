// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: an argument that is not a value cannot select an overload, so
// the call is left alone rather than reported as matching no candidate on top.

function f(uint256 x) pure returns (bool) {
    return x > 0;
}

function f(bool x) pure returns (uint256) {
    return x ? 1 : 0;
}

function call() pure {
    f(uint);
}
