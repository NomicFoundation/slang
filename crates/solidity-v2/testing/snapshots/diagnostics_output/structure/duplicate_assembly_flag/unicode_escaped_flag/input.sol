// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedWithUnicodeEscape() public pure {
        // The same flag, spelled with a unicode escape.
        assembly ("\u006demory-safe", "memory-safe") {}
    }
}
