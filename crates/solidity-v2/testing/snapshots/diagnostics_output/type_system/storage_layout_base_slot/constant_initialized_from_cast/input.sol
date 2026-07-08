// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant is initialized from a cast, which the evaluator cannot fold, so
// the base slot is not a compile-time constant.
uint256 constant x = uint256(42);
contract C layout at x {}
