// SPDX-License-Identifier: MIT
pragma solidity *;

// Boolean and string base slots do not evaluate to an integer.
contract A layout at true {}
contract B layout at "abc" {}
