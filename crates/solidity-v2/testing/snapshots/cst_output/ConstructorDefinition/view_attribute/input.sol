// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports TypeError 1558 ("Constructor must be payable or non-payable...")
// during type checking, whereas slang rejects it at the parser: the grammar's
// constructor attributes do not include `view`/`pure`.
contract View {
    constructor() view {}
}
