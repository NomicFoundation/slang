// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeated() public pure {
        // The `memory-safe` flag can only be listed once.
        assembly ("memory-safe", "memory-safe") {}
    }
}
