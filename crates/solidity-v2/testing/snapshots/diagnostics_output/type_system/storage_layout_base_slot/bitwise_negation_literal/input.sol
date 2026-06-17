// SPDX-License-Identifier: MIT
pragma solidity *;

// Bitwise negation of a literal folds to a negative value, which is outside the
// uint256 range.
contract C layout at ~0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE {}
