// SPDX-License-Identifier: MIT
pragma solidity *;

// The base slot exceeds the uint256 range.
contract C layout at 2 ** 256 {}
