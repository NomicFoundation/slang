// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedWithHexEscape() public pure {
        // Escape sequences are decoded before flags are compared.
        assembly ("memory-safe", "\x6demory-safe") {}
    }
}
