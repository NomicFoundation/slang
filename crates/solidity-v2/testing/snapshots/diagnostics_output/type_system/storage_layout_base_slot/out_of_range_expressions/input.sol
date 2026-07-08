// SPDX-License-Identifier: MIT
pragma solidity *;

// Expressions that fold above 2**256 - 1 or below zero are outside the uint256
// range.
contract A layout at 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF + 1 {}
contract B layout at 2 ** 256 {}
contract C layout at 0 - 1 {}
contract D layout at 2 ** 8 - 2 ** 16 {}
