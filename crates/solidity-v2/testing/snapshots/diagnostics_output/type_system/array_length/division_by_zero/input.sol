// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The length divides by a constant equal to zero, which cannot be folded.
    // Routing the zero through a constant (rather than a literal `4 / 0`) makes
    // solc evaluate the expression instead of rejecting the literal operands
    // directly, so both tools report it as an invalid array length.
    uint256 constant A = 4 / B;
    uint256 constant B = 0;
    uint256[A] x;
}
