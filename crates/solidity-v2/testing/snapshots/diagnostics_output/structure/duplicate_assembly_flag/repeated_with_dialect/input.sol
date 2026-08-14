// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedWithDialect() public pure {
        // The dialect label doesn't change the rule.
        assembly "evmasm" ("memory-safe", "memory-safe") {}
    }
}
