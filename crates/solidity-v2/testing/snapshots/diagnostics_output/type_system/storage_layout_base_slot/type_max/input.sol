// SPDX-License-Identifier: MIT
pragma solidity *;

// `type(uint).max` is not folded by the constant evaluator.
contract C layout at type(uint).max {}
