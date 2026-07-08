// SPDX-License-Identifier: MIT
pragma solidity *;

// A signed constant that folds to a negative value is outside the uint256
// range.
int256 constant x = -42;
contract C layout at x {}
