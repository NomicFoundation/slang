// SPDX-License-Identifier: MIT
pragma solidity *;

// Intermediate results are computed with unlimited precision, so expressions
// that fold to an in-range integer are valid base slots even when intermediate
// values overflow uint256.
contract A layout at 1.23e100 / 2e50 {}
contract B layout at 2 ** 256 * (500e-3) {}
contract D layout at (2 ** 255 * 2) - (2 ** 256 + 1) + 1 {}
