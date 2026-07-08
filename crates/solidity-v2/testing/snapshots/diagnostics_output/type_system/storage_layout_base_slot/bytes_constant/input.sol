// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant has type `bytes32`, not an integer, so the constant evaluator
// treats it as non-foldable and reports a non-constant base slot.
bytes32 constant x = 0x000000000000000000000000dcad3a6d3569ded06cb7a1b2ccd1d3afdeadbeef;
contract C layout at x {}
