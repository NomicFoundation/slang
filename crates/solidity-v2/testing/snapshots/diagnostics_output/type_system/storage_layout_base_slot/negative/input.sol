// SPDX-License-Identifier: MIT
pragma solidity *;

// A negative base slot is outside the uint256 range.
contract C layout at -100000 {}
