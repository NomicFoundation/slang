// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeated() public pure {
        // The `memory-safe` flag can only be listed once.
        assembly ("memory-safe", "memory-safe") {}
    }

    function repeatedWithDialect() public pure {
        // The dialect label doesn't change the rule.
        assembly "evmasm" ("memory-safe", "memory-safe") {}
    }

    function repeatedThrice() public pure {
        // Every repetition past the first is reported on its own.
        assembly ("memory-safe", "memory-safe", "memory-safe") {}
    }

    // Control: listing the flag once is allowed.
    function once() public pure {
        assembly ("memory-safe") {}
    }

    // Control: an assembly statement need not list any flags.
    function none() public pure {
        assembly {}
    }
}
