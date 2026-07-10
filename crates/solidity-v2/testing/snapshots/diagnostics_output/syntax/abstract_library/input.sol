// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports TypeError 9571 ("Libraries cannot be abstract") during type
// checking, whereas slang rejects it at the parser: the grammar only allows
// the `abstract` keyword on contracts.
abstract library L {}
