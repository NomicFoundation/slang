// SPDX-License-Identifier: MIT
pragma solidity *;

// A signed constant that folds to a non-negative value is within the uint256
// range, so it is a valid base slot.
int256 constant x = 42;
int16 constant y = 64;
contract C layout at x {}
contract D layout at y {}
