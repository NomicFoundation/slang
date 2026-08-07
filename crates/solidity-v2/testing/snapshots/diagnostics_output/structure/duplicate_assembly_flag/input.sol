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

    function repeatedWithHexEscape() public pure {
        // Escape sequences are decoded before flags are compared.
        assembly ("memory-safe", "\x6demory-safe") {}
    }

    function repeatedWithUnicodeEscape() public pure {
        // The same flag, spelled with a unicode escape.
        assembly ("\u006demory-safe", "memory-safe") {}
    }

    function repeatedFullyEncoded() public pure {
        // Every character encoded still decodes to `memory-safe`.
        assembly ("memory-safe", "\x6d\x65\x6d\x6f\x72\x79\x2d\x73\x61\x66\x65") {}
    }

    function repeatedAcrossLineContinuation() public pure {
        // A line continuation decodes to nothing, joining both halves.
        assembly ("memory-safe", "memory-\
safe") {}
    }

    // Control: listing the flag once is allowed.
    function once() public pure {
        assembly ("memory-safe") {}
    }

    // Control: an encoded flag listed once is still a single flag.
    function onceEncoded() public pure {
        assembly ("\x6demory-safe") {}
    }

    // Control: an assembly statement need not list any flags.
    function none() public pure {
        assembly {}
    }
}
