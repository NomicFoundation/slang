// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedThrice() public pure {
        // Every repetition past the first is reported on its own.
        assembly ("memory-safe", "memory-safe", "memory-safe") {}
    }
}
