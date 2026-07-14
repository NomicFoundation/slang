// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports TypeError 9239 ("Constructor cannot have visibility") during
// type checking, whereas slang rejects it at the parser: the grammar's
// constructor attributes do not include visibility keywords.
contract C {
    constructor() external {}
}
