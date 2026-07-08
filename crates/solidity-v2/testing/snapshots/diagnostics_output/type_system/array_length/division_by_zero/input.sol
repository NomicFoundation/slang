// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The length divides by a constant equal to zero, which cannot be folded.
    // The zero is routed through a typed constant: a literal `4 / 0` would be
    // rejected as an incompatible operator instead of an invalid length.
    uint256 constant A = 4 / B;
    uint256 constant B = 0;
    uint256[A] x;
}
