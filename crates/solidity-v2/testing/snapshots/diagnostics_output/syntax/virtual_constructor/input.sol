// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports this as TypeError 7001 ("Constructors cannot be virtual")
// during type checking, whereas slang rejects it earlier at the parser: the
// v2 grammar's constructor attributes do not include the `virtual` keyword.
contract C {
    constructor() virtual {}
}
