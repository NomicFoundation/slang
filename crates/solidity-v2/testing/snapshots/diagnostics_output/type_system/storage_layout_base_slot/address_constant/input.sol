// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant has type `address`, not an integer, so the constant evaluator
// treats it as non-foldable and reports a non-constant base slot.
address constant x = 0xdCad3a6d3569DF655070DEd06cb7A1b2Ccd1D3AF;
contract C layout at x {}
