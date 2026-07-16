// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {}
contract B {}

// Valid: a single inheritance list is not flagged.
contract Valid is A, B {}

// Two inheritance lists: the second `is` is flagged.
contract Duplicated is A, B is B {}

// Several repeated inheritance lists: every `is` after the first is flagged.
contract Repeated is A is B is B is A {}
