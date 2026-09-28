// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: both operands of an operator are value positions, so a type name
// is reported in each of them rather than once for the expression.

enum E { A }

function add() pure returns (uint256) {
    return E + E;
}
