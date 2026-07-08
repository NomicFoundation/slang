// SPDX-License-Identifier: MIT
pragma solidity *;

// An explicit type conversion is not folded by the constant evaluator, so the
// base slot cannot be evaluated at compile time.
contract C layout at uint256(42) {}
