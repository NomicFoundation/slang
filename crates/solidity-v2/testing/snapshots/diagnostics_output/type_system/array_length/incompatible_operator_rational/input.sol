// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 constant B = 2;

    // A fractional literal cannot meet a typed integer, so the division has
    // no result type. The rational operand is rendered as a normalised
    // fraction: `rational_const 1 / 3` for the source text `2 / 6`.
    uint256[(2 / 6) / B] x;
}
