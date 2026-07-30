// SPDX-License-Identifier: MIT
pragma solidity *;

// The built-in error name `Panic` is reserved and must be flagged.
error Panic(uint256 code);

// A custom error with any other name is valid and must not be flagged.
error MyError(uint256 code);
