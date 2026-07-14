// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports this as TypeError 5700 ("Constructor must be implemented if
// declared") during type checking, whereas slang rejects it earlier at the
// parser: the v2 grammar requires a constructor to have a body block.
contract C {
    constructor();
}
