// SPDX-License-Identifier: MIT
pragma solidity *;

// The cast inside the bitwise negation prevents the evaluator from folding the
// base slot expression.
contract C layout at ~uint256(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) {}
