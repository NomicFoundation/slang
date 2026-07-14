// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports TypeError 9348 ("Interfaces do not need the \"abstract\"
// keyword...") during type checking, whereas slang rejects it at the parser:
// the grammar only allows the `abstract` keyword on contracts.
abstract interface I {}
