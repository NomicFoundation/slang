// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // An explicit type conversion is not folded by the constant evaluator, so
    // it is not accepted as an array length even when the operand is a valid
    // literal.
    uint256[uint256(1)] a;
    uint256[int256(2)] b;
}
