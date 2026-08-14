// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedAcrossLineContinuation() public pure {
        // A line continuation decodes to nothing, joining both halves.
        assembly ("memory-safe", "memory-\
safe") {}
    }
}
